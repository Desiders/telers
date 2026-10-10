use super::{Configured, Response};
use crate::{
    enums::UpdateType,
    errors::EventErrorKind,
    event::{
        bases::PropagateEventResult,
        service::{service_fn, BoxCloneService, Service},
        telegram::{
            observer::Response as ObserverResponse, HandlerResponse, Observer as TelegramObserver,
        },
    },
    middlewares::outer::{
        wrap_to_next, BoxedCloneMiddlewareService, BoxedCloneRoutingService, MiddlewareResult,
    },
    Request,
};

use std::{mem, sync::Arc};
use tracing::{event, Level};

#[derive(Clone, Copy)]
pub(super) enum Route {
    Event(UpdateType),
    Error,
}

impl<Client> Configured<Client>
where
    Client: Send + Sync + Clone + 'static,
{
    #[must_use]
    pub(super) fn routing_service(&self, route: Route) -> BoxedCloneRoutingService<Client> {
        match route {
            Route::Event(update_type) => {
                let service = observer_service(
                    self.name,
                    self.telegram_observer_by_update_type(update_type).clone(),
                    self.sub_routers.clone(),
                    route,
                );

                update_service(self.name, self.update.clone(), Some(service))
            }
            Route::Error => observer_service(
                self.name,
                self.error.clone(),
                self.sub_routers.clone(),
                route,
            ),
        }
    }

    #[must_use]
    pub(super) fn update_service(&self) -> BoxedCloneRoutingService<Client> {
        update_service(self.name, self.update.clone(), None)
    }
}

/// The update chain surrounds the update observer and, when unhandled, typed routing.
fn update_service<Client>(
    name: &'static str,
    mut observer: TelegramObserver<Client>,
    continuation: Option<BoxedCloneRoutingService<Client>>,
) -> BoxedCloneRoutingService<Client>
where
    Client: Send + Sync + Clone + 'static,
{
    let middlewares = mem::take(&mut observer.outer_middlewares.middlewares).into_boxed_slice();
    let service = BoxCloneService::new(service_fn(move |request: Request<Client>| {
        let mut observer = observer.clone();
        let continuation = continuation.clone();

        async move {
            event!(
                Level::TRACE,
                router = name,
                "Propagate update event to router"
            );

            let response = observer.trigger(request).await?;
            let response = observer_response(response)?;

            match response.propagate_result {
                PropagateEventResult::Unhandled => match continuation {
                    Some(mut continuation) => continuation.call(response.request).await,
                    None => Ok(response),
                },
                PropagateEventResult::Handled(_) | PropagateEventResult::Rejected => Ok(response),
            }
        }
    }));

    with_outer_middlewares(service, middlewares)
}

/// A typed or error chain surrounds its observer and traversal of its children.
fn observer_service<Client>(
    name: &'static str,
    mut observer: TelegramObserver<Client>,
    sub_routers: Arc<[Configured<Client>]>,
    route: Route,
) -> BoxedCloneRoutingService<Client>
where
    Client: Send + Sync + Clone + 'static,
{
    let middlewares = mem::take(&mut observer.outer_middlewares.middlewares).into_boxed_slice();
    let service = BoxCloneService::new(service_fn(move |request: Request<Client>| {
        let mut observer = observer.clone();
        let sub_routers = sub_routers.clone();

        async move {
            event!(Level::TRACE, router = name, "Propagate event to router");

            let response = observer.trigger(request).await?;
            match response.propagate_result {
                // Handled or rejected events stop routing through this subtree.
                PropagateEventResult::Rejected | PropagateEventResult::Handled(_) => {
                    return observer_response(response);
                }
                PropagateEventResult::Unhandled => {}
            }
            let response = observer_response(response)?;

            for router in sub_routers.iter() {
                let router_response = router
                    .routing_service(route)
                    .call(response.request.clone())
                    .await?;
                match router_response.propagate_result {
                    PropagateEventResult::Unhandled => {}
                    PropagateEventResult::Handled(_) | PropagateEventResult::Rejected => {
                        return Ok(router_response);
                    }
                }
                // An unhandled child's request changes do not leak to its siblings.
            }

            Ok(response)
        }
    }));

    with_outer_middlewares(service, middlewares)
}

fn with_outer_middlewares<Client>(
    service: BoxedCloneRoutingService<Client>,
    middlewares: Box<[BoxedCloneMiddlewareService<Client>]>,
) -> BoxedCloneRoutingService<Client>
where
    Client: Send + Sync + 'static,
{
    BoxCloneService::new(service_fn(move |request: Request<Client>| {
        let next = wrap_to_next(service.clone(), middlewares.clone());

        async move { next(request).await }
    }))
}

/// Make handler failures visible as errors while outer continuations unwind.
fn observer_response<Client>(response: ObserverResponse<Client>) -> MiddlewareResult<Client>
where
    Client: Clone,
{
    let request = response.request;
    match response.propagate_result {
        PropagateEventResult::Handled(handler_response) => {
            let request = handler_response.request;
            match handler_response.result {
                Ok(result) => Ok(Response {
                    request: request.clone(),
                    propagate_result: PropagateEventResult::Handled(HandlerResponse {
                        request,
                        result: Ok(result),
                    }),
                }),
                Err(err) => Err((EventErrorKind::Handler(err), request)),
            }
        }
        PropagateEventResult::Unhandled | PropagateEventResult::Rejected => Ok(Response {
            request,
            propagate_result: PropagateEventResult::Unhandled,
        }),
    }
}

/// Restore the public handler result after every outer middleware has completed.
pub(super) fn finish<Client>(result: MiddlewareResult<Client>) -> MiddlewareResult<Client>
where
    Client: Clone,
{
    match result {
        Ok(mut response) => {
            if let PropagateEventResult::Handled(handler_response) = &mut response.propagate_result
            {
                handler_response.request = response.request.clone();
            }
            Ok(response)
        }
        Err((EventErrorKind::Handler(err), request)) => Ok(Response {
            request: request.clone(),
            propagate_result: PropagateEventResult::Handled(HandlerResponse {
                request,
                result: Err(err),
            }),
        }),
        Err(err) => Err(err),
    }
}
