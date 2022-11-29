#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

mod dispatcher;
mod router;

pub mod event;
pub mod middlewares;

pub use dispatcher::{Dispatcher, DispatcherService};
pub use router::{Request as RouterRequest, Response as RouterResponse, Router, RouterService};
