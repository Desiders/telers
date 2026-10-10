use crate::{
    client::Reqwest,
    errors::EventErrorKind,
    event::service::{service_fn, BoxCloneService, Service},
    router::Response,
    Request,
};

use futures_util::future::BoxFuture;
use std::{future::Future, sync::Arc};

pub(crate) type BoxedCloneMiddlewareService<Client> = BoxCloneService<
    (Request<Client>, Next<Client>),
    MiddlewareResponse<Client>,
    (EventErrorKind, Request<Client>),
>;

pub(crate) type BoxedCloneRoutingService<Client> =
    BoxCloneService<Request<Client>, MiddlewareResponse<Client>, (EventErrorKind, Request<Client>)>;

/// Response from the remaining routing work, including the final request.
pub type MiddlewareResponse<Client = Reqwest> = Response<Client>;

/// Routing result, retaining the request if processing fails.
pub type MiddlewareResult<Client = Reqwest> =
    Result<MiddlewareResponse<Client>, (EventErrorKind, Request<Client>)>;

/// The remaining outer middlewares and routing work, called at most once.
///
/// Handler, filter and middleware failures return their error kind and request.
/// Modify [`Response::request`] to retain request changes made during cleanup.
pub type Next<Client = Reqwest> =
    Box<dyn FnOnce(Request<Client>) -> BoxFuture<'static, MiddlewareResult<Client>> + Send>;

/// Outer middlewares called before filters, inner middlewares and handlers
///
/// Prefer to use outer middlewares over inner middlewares in some cases:
/// - If you need to call middlewares before filters, inner middlewares and handlers
/// - If you need to manipulate with [`Request`] and [`crate::context::Context`] in it
/// - If you need to wrap filters and routing with setup and cleanup
///
/// Usually outer middlewares are used to manipulate with [`Request`].
///
/// Implement this trait for your own middlewares
pub trait Middleware<Client = Reqwest>: Clone + Send + Sync + 'static {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for observers, filters, handler and middlewares
    /// * `next` - Run the remaining middlewares, filters and routing work
    /// # Returns
    /// [`MiddlewareResponse`] from routing or an error together with its request
    /// # Errors
    /// If middleware or downstream processing fails, return the error with its request
    fn call(
        &mut self,
        request: Request<Client>,
        next: Next<Client>,
    ) -> impl Future<Output = MiddlewareResult<Client>> + Send;
}

/// To possible use function-like as middlewares
impl<Client, F, Fut> Middleware<Client> for F
where
    Client: Send + Sync + 'static,
    F: FnMut(Request<Client>, Next<Client>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = MiddlewareResult<Client>> + Send,
{
    fn call(
        &mut self,
        request: Request<Client>,
        next: Next<Client>,
    ) -> impl Future<Output = MiddlewareResult<Client>> + Send {
        self(request, next)
    }
}

/// Wrap routing and middlewares in a [`Next`] function.
#[must_use]
pub(crate) fn wrap_to_next<Client>(
    service: BoxedCloneRoutingService<Client>,
    middlewares: Box<[BoxedCloneMiddlewareService<Client>]>,
) -> Next<Client>
where
    Client: Send + Sync + 'static,
{
    wrap_to_next_at(service, Arc::from(middlewares), 0)
}

fn wrap_to_next_at<Client>(
    mut service: BoxedCloneRoutingService<Client>,
    middlewares: Arc<[BoxedCloneMiddlewareService<Client>]>,
    index: usize,
) -> Next<Client>
where
    Client: Send + Sync + 'static,
{
    Box::new(move |request: Request<Client>| {
        Box::pin(async move {
            let Some(middleware) = middlewares.get(index) else {
                return service.call(request).await;
            };

            let mut middleware = middleware.clone();
            let next = wrap_to_next_at(service, middlewares, index + 1);

            middleware.call((request, next)).await
        })
    })
}

pub(crate) fn boxed_middleware_factory<Client>(
    middleware: impl Middleware<Client>,
) -> BoxedCloneMiddlewareService<Client>
where
    Client: Send + Sync + 'static,
{
    BoxCloneService::new(service_fn(move |(request, next)| {
        let mut middleware = middleware.clone();

        async move { middleware.call(request, next).await }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        context::Context,
        errors::MiddlewareError,
        event::bases::PropagateEventResult,
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
        Bot, Extensions,
    };

    use anyhow::anyhow;
    use std::sync::{Arc, Mutex};
    use tokio;

    fn test_request() -> Request<Reqwest> {
        Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: Context::default(),
            extensions: Extensions::default(),
        }
    }

    fn recording_middleware(
        before: &'static str,
        after: &'static str,
        calls: Arc<Mutex<Vec<&'static str>>>,
    ) -> impl Middleware<Reqwest> {
        move |mut request: Request<Reqwest>, next: Next<Reqwest>| {
            let calls = calls.clone();

            async move {
                calls.lock().unwrap().push(before);
                request.context.insert(before, true);

                let mut result = next(request).await;

                calls.lock().unwrap().push(after);
                match &mut result {
                    Ok(response) => response.request.context.insert(after, true),
                    Err((_, request)) => request.context.insert(after, true),
                };

                result
            }
        }
    }

    #[tokio::test]
    async fn test_call() {
        let service =
            BoxCloneService::new(service_fn(|mut request: Request<Reqwest>| async move {
                assert_eq!(request.context.get::<bool>("middleware"), Some(&true));
                request.context.insert("service", true);
                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                })
            }));
        let mut middleware = |mut request: Request<Reqwest>, next: Next<Reqwest>| async move {
            request.context.insert("middleware", true);
            next(request).await
        };

        let response = Middleware::call(
            &mut middleware,
            test_request(),
            wrap_to_next(service, [].into()),
        )
        .await
        .unwrap();

        assert!(matches!(
            response.propagate_result,
            PropagateEventResult::Unhandled
        ));
        assert_eq!(response.request.context.get::<bool>("service"), Some(&true));
    }

    #[tokio::test]
    async fn test_chain_runs_setup_and_cleanup_in_order() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let service_calls = calls.clone();
        let service = BoxCloneService::new(service_fn(move |mut request: Request<Reqwest>| {
            let calls = service_calls.clone();

            async move {
                calls.lock().unwrap().push("service");
                assert_eq!(request.context.get::<bool>("before_0"), Some(&true));
                assert_eq!(request.context.get::<bool>("before_1"), Some(&true));
                request.context.insert("service", true);
                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                })
            }
        }));
        let middlewares = vec![
            boxed_middleware_factory(recording_middleware("before_0", "after_0", calls.clone())),
            boxed_middleware_factory(recording_middleware("before_1", "after_1", calls.clone())),
        ]
        .into_boxed_slice();

        let response = wrap_to_next(service, middlewares)(test_request())
            .await
            .unwrap();

        assert!(matches!(
            response.propagate_result,
            PropagateEventResult::Unhandled
        ));
        assert_eq!(response.request.context.get::<bool>("service"), Some(&true));
        assert_eq!(response.request.context.get::<bool>("after_0"), Some(&true));
        assert_eq!(response.request.context.get::<bool>("after_1"), Some(&true));
        assert_eq!(
            *calls.lock().unwrap(),
            vec!["before_0", "before_1", "service", "after_1", "after_0"]
        );
    }

    #[tokio::test]
    async fn test_middleware_can_suppress_remaining_routing() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let service_calls = calls.clone();
        let service = BoxCloneService::new(service_fn(move |request: Request<Reqwest>| {
            let calls = service_calls.clone();

            async move {
                calls.lock().unwrap().push("service");
                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                })
            }
        }));
        let middleware_calls = calls.clone();
        let middleware = move |mut request: Request<Reqwest>, _next: Next<Reqwest>| {
            let calls = middleware_calls.clone();

            async move {
                calls.lock().unwrap().push("suppressed");
                request.context.insert("suppressed", true);
                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Rejected,
                })
            }
        };
        let middlewares = vec![
            boxed_middleware_factory(recording_middleware("before_0", "after_0", calls.clone())),
            boxed_middleware_factory(middleware),
            boxed_middleware_factory(recording_middleware("before_1", "after_1", calls.clone())),
        ]
        .into_boxed_slice();

        let response = wrap_to_next(service, middlewares)(test_request())
            .await
            .unwrap();

        assert!(matches!(
            response.propagate_result,
            PropagateEventResult::Rejected
        ));
        assert_eq!(
            response.request.context.get::<bool>("before_0"),
            Some(&true)
        );
        assert_eq!(
            response.request.context.get::<bool>("suppressed"),
            Some(&true)
        );
        assert_eq!(response.request.context.get::<bool>("after_0"), Some(&true));
        assert!(response.request.context.get::<bool>("before_1").is_none());
        assert_eq!(
            *calls.lock().unwrap(),
            vec!["before_0", "suppressed", "after_0"]
        );
    }

    #[tokio::test]
    async fn test_failure_unwinds_with_downstream_request() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let service_calls = calls.clone();
        let service = BoxCloneService::new(service_fn(move |mut request: Request<Reqwest>| {
            let calls = service_calls.clone();

            async move {
                calls.lock().unwrap().push("service");
                request.context.insert("failed", true);
                Err((
                    EventErrorKind::Middleware(MiddlewareError::new(anyhow!("failure"))),
                    request,
                ))
            }
        }));
        let middlewares = vec![
            boxed_middleware_factory(recording_middleware("before_0", "after_0", calls.clone())),
            boxed_middleware_factory(recording_middleware("before_1", "after_1", calls.clone())),
        ]
        .into_boxed_slice();

        let (error, request) = wrap_to_next(service, middlewares)(test_request())
            .await
            .unwrap_err();

        assert!(matches!(error, EventErrorKind::Middleware(_)));
        assert_eq!(error.to_string(), "failure");
        assert_eq!(request.context.get::<bool>("before_0"), Some(&true));
        assert_eq!(request.context.get::<bool>("before_1"), Some(&true));
        assert_eq!(request.context.get::<bool>("failed"), Some(&true));
        assert_eq!(request.context.get::<bool>("after_0"), Some(&true));
        assert_eq!(request.context.get::<bool>("after_1"), Some(&true));
        assert_eq!(
            *calls.lock().unwrap(),
            vec!["before_0", "before_1", "service", "after_1", "after_0"]
        );
    }
}
