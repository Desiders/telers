//! Error events observer and handlers.
//!
//! When an error occurs while processing an update (a handler returns an error,
//! or argument extraction, middleware or filter fails), the dispatcher propagates
//! an [`ErrorEvent`] to the error observers of the main router and its sub routers
//! (sub routers first, so more specific handlers are called before global ones).
//!
//! Register error handlers with [`Router::on_error`](crate::Router#method.on_error):
//!
//! ```rust
//! use telers::{
//!     event::error::{ErrorEvent, Handler, HandlerResult, EventReturn},
//!     Router,
//! };
//!
//! async fn on_error(event: ErrorEvent) -> HandlerResult {
//!     tracing::error!(error = ?event.error, "Error while processing update");
//!
//!     // The error is handled, no other error handler will be called
//!     Ok(EventReturn::Finish)
//! }
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     let router: Router = Router::new("example")
//!         .on_error(|observer| observer.register(Handler::new(on_error)));
//! }
//! ```
//!
//! If an error handler returns [`EventReturn::Skip`], the error is passed
//! to the next handler. If no handler handles the error (or no handlers are registered),
//! the dispatcher logs it with the `ERROR` level.
//!
//! [`ErrorEvent`]: event::ErrorEvent

pub mod event;
pub mod handler;
pub mod observer;

pub use event::ErrorEvent;
pub use handler::{Handler, HandlerFn, HandlerResult};
pub use observer::{Observer, PropagateErrorResult};

pub use crate::event::EventReturn;
