#[allow(clippy::module_inception)]
pub mod dispatcher;
pub mod event;
pub mod middlewares;
pub mod router;

#[allow(clippy::module_name_repetitions)]
pub use dispatcher::{Dispatcher, DispatcherBuilder};
pub use router::{Request as RouterRequest, Response as RouterResponse, Router};
