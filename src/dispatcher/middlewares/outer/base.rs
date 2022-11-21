use crate::{
    dispatcher::{
        event::{bases::EventReturn, service::BoxFuture},
        RouterRequest,
    },
    error::app,
};

pub trait Middleware {
    /// Execute middleware
    /// # Arguments
    /// * `req` - Data for router service
    #[must_use]
    fn call(&self, req: &RouterRequest) -> BoxFuture<Result<EventReturn, app::Error>>;
}
