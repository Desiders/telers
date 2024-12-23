extern crate self as telers;

mod serializers;

pub mod client;
pub mod context;
pub mod dispatcher;
pub mod enums;
pub mod errors;
pub mod event;
pub mod extensions;
pub mod extractors;
pub mod filters;
pub mod fsm;
pub mod methods;
pub mod middlewares;
pub mod router;
pub mod types;
pub mod utils;

pub use telers_macros::{FromContext, FromEvent};

pub use client::Bot;
pub use context::Context;
pub use dispatcher::{Builder as DispatcherBuilder, Dispatcher};
pub use extensions::Extensions;
pub use filters::Filter;
pub use fsm::Context as FSMContext;
pub use router::Router;
