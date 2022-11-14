use crate::{
    dispatcher::event::{
        service::BoxFuture,
        telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    },
    error::app,
};

pub trait Middleware {
    /// Execute middleware
    /// # Arguments
    /// * `handler` - Handler service
    /// * `req` - Data for handler service
    /// * `middlewares` - Middlewares for handler service
    #[must_use]
    fn call(
        &self,
        handler: &BoxedHandlerService,
        req: HandlerRequest,
        middlewares: Box<dyn Iterator<Item = Box<dyn Middleware>>>,
    ) -> BoxFuture<Result<HandlerResponse, app::Error>>;

    /// Call next middleware or handler service if all middlewares has passed
    /// # Arguments
    /// * `handler` - Handler service
    /// * `req` - Data for handler service
    /// * `middlewares` - Middlewares for handler service
    #[must_use]
    fn handler(
        &self,
        handler: &BoxedHandlerService,
        req: HandlerRequest,
        mut middlewares: Box<dyn Iterator<Item = Box<dyn Middleware>>>,
    ) -> BoxFuture<Result<HandlerResponse, app::Error>> {
        match middlewares.next() {
            Some(middleware) => middleware.call(handler, req, middlewares),
            None => handler.call(req),
        }
    }
}
