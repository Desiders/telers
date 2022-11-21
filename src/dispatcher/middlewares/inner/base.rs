use crate::{
    dispatcher::event::{
        service::BoxFuture,
        telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    },
    error::app,
};

use std::rc::Rc;

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
        middlewares: Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>,
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
        mut middlewares: Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>,
    ) -> BoxFuture<Result<HandlerResponse, app::Error>> {
        match middlewares.next() {
            // Call next middleware
            Some(middleware) => middleware.call(handler, req, middlewares),
            // Call handler service
            None => handler.call(req),
        }
    }
}
