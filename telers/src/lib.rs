extern crate self as telers;

mod serializers;

pub(crate) mod any;

#[macro_use]
pub(crate) mod macros;

pub mod client;
pub mod context;
pub mod dispatcher;
pub mod enums;
pub mod errors;
pub mod event;
pub mod extensions;
pub mod extractor;
pub mod filters;
pub mod fsm;
pub mod methods;
pub mod middlewares;
pub mod request;
pub mod router;
pub mod types;
pub mod utils;

pub use telers_macros::{FromContext, FromEvent};

pub use client::Bot;
pub use context::Context;
pub use dispatcher::{Builder as DispatcherBuilder, Dispatcher};
pub use extensions::{Extension, Extensions};
pub use extractor::Extractor;
pub use filters::Filter;
pub use fsm::Context as FSMContext;
pub use request::Request;
pub use router::{Configured as RouterConfigured, Router};
