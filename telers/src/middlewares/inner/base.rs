use crate::{
    client::Reqwest,
    errors::EventErrorKind,
    event::{
        service::{service_fn, BoxCloneService, Service},
        telegram::{handler::BoxedCloneHandlerService, HandlerResponse},
    },
    Request,
};

use futures_util::future::BoxFuture;
use std::{future::Future, sync::Arc};

pub(crate) type BoxedCloneMiddlewareService<Client> =
    BoxCloneService<(Request<Client>, Next<Client>), HandlerResponse<Client>, EventErrorKind>;

/// The middleware chain and the handler at the end
pub type Next<Client = Reqwest> = Box<
    dyn Fn(Request<Client>) -> BoxFuture<'static, Result<HandlerResponse<Client>, EventErrorKind>>
        + Send
        + Sync,
>;

/// Inner middlewares called after outer middlewares, after filters, but before handlers.
/// If filters aren't passed, then inner middlewares aren't called.
///
/// Prefer to use inner middlewares over outer middlewares in some cases:
/// - If you need to call middlewares after filters and before handlers
/// - If you need to manipulate with call of next middleware or handler
/// - If you need to manipulate with [`Request`] or [`HandlerResponse`]
///
/// Usually inner middlewares are more relevant than outer middlewares.
///
/// Implement this trait for your own middlewares
pub trait Middleware<Client = Reqwest>: Clone + Send + Sync + 'static {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for handler and middlewares
    /// * `next` - Call next middleware or handler, if middlewares are empty or already called
    /// # Returns
    /// [`HandlerResponse`] from handler or [`EventErrorKind`] if handler or middleware returns an error
    /// # Errors
    /// If any inner middleware returns an error
    /// If handler returns an error. Probably it's the error to extract args to the handler
    fn call(
        &mut self,
        request: Request<Client>,
        next: Next<Client>,
    ) -> impl Future<Output = Result<HandlerResponse<Client>, EventErrorKind>> + Send;
}

/// To possible use function-like as middlewares
impl<Client, F, Fut> Middleware<Client> for F
where
    Client: Send + Sync + 'static,
    F: FnMut(Request<Client>, Next<Client>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<HandlerResponse<Client>, EventErrorKind>> + Send,
{
    fn call(
        &mut self,
        request: Request<Client>,
        next: Next<Client>,
    ) -> impl Future<Output = Result<HandlerResponse<Client>, EventErrorKind>> + Send {
        self(request, next)
    }
}

/// Wrap handler and middlewares to [`Next`] function
/// # Notes
/// This function is wrap [`crate::errors::HandlerError`] to [`EventErrorKind::Handler`]
#[must_use]
pub fn wrap_to_next<Client>(
    handler: BoxedCloneHandlerService<Client>,
    middlewares: Box<[BoxedCloneMiddlewareService<Client>]>,
) -> Next<Client>
where
    Client: Send + Sync + 'static,
{
    wrap_to_next_at(handler, Arc::from(middlewares), 0)
}

fn wrap_to_next_at<Client>(
    handler: BoxedCloneHandlerService<Client>,
    middlewares: Arc<[BoxedCloneMiddlewareService<Client>]>,
    index: usize,
) -> Next<Client>
where
    Client: Send + Sync + 'static,
{
    Box::new(move |request: Request<Client>| {
        let mut handler = handler.clone();
        let middlewares = middlewares.clone();

        Box::pin(async move {
            let Some(middleware) = middlewares.get(index) else {
                return match handler.call(request).await {
                    Ok(response) => match response.result {
                        Ok(_) => Ok(response),
                        // If handler returns an error, then wrap it to event error
                        Err(err) => Err(EventErrorKind::Handler(err)),
                    },
                    // If handler service returns an error, then it's an error to extract args to the handler
                    Err(err) => Err(EventErrorKind::Extraction(err)),
                };
            };

            let mut middleware = middleware.clone();
            let next = wrap_to_next_at(handler, Arc::clone(&middlewares), index + 1);

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
        event::{telegram::handler::boxed_handler_factory, EventReturn},
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
        Bot, Extensions,
    };

    use std::{
        convert::Infallible,
        sync::{Arc, Mutex},
    };

    async fn test_middleware<Client>(
        request: Request<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        next(request).await
    }

    fn test_request() -> Request<Reqwest> {
        Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        }
    }

    #[tokio::test]
    async fn test_call() {
        let handler_service =
            boxed_handler_factory(|| async { Ok::<_, Infallible>(EventReturn::Finish) });

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };
        let response = Middleware::call(
            &mut test_middleware,
            request,
            wrap_to_next(handler_service, [].into()),
        )
        .await
        .unwrap();

        match response.result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected response"),
        }
    }

    #[tokio::test]
    async fn test_chain_calls_every_middleware_once_in_order() {
        // The chain is shared by `Arc` and walked by index, so this pins the contract that has to
        // survive that: every middleware still runs exactly once, in registration order, and the
        // handler runs last.
        let calls = Arc::new(Mutex::new(Vec::new()));

        let record = |index: usize, calls: Arc<Mutex<Vec<usize>>>| {
            move |request: Request<Reqwest>, next: Next<Reqwest>| {
                let calls = calls.clone();

                async move {
                    calls.lock().unwrap().push(index);
                    next(request).await
                }
            }
        };

        let handler_service =
            boxed_handler_factory(|| async { Ok::<_, Infallible>(EventReturn::Finish) });
        let middlewares = vec![
            boxed_middleware_factory(record(0, calls.clone())),
            boxed_middleware_factory(record(1, calls.clone())),
            boxed_middleware_factory(record(2, calls.clone())),
        ]
        .into_boxed_slice();

        let next = wrap_to_next(handler_service, middlewares);
        let response = next(test_request()).await.unwrap();

        assert!(matches!(response.result, Ok(EventReturn::Finish)));
        assert_eq!(*calls.lock().unwrap(), vec![0, 1, 2]);
    }
}
