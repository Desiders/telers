#![allow(clippy::module_name_repetitions)]

pub mod inner;
pub mod outer;

pub use inner::{Middleware as InnerMiddleware, Middlewares as InnerMiddlewares, Next};
pub use outer::{Middleware as OuterMiddleware, Middlewares as OuterMiddlewares};
