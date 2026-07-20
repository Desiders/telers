//! An asynchronous framework for the Telegram Bot API written in Rust.
//!
//! `telers` makes it easy to create Telegram bots: it provides a system of routers, middlewares,
//! filters and handlers (inspired by [`aiogram`]), and an extractor system for handler arguments
//! similar to the ones in `axum` and `actix`.
//! Types and methods mirror the [Telegram Bot API] documentation: the same objects with the same
//! fields, plus generated helper methods and builders.
//!
//! # Quick start
//!
//! An echo bot (reads the token from the `BOT_TOKEN` environment variable):
//!
//! ```no_run
//! use telers::{
//!     enums::UpdateType,
//!     event::telegram::{Handler, HandlerResult},
//!     types::Message,
//!     Bot, Dispatcher, Router,
//! };
//!
//! async fn echo_handler(bot: Bot, message: Message) -> HandlerResult<()> {
//!     bot.send(message.to_copy_message(message.chat().id()))
//!         .await?;
//!     Ok(())
//! }
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     let bot = Bot::from_env();
//!
//!     let router = Router::new("main")
//!         .on_message(|observer| observer.register(Handler::new(echo_handler)));
//!
//!     let dispatcher = Dispatcher::builder()
//!         .main_router(router.configure_default())
//!         .bot(bot)
//!         .allowed_update(UpdateType::Message)
//!         .build();
//!
//!     dispatcher.run_polling().await.expect("Polling failed");
//! }
//! ```
//!
//! # Where to go next
//!
//! - [`router`] — how events are routed to observers and handlers, controlled by [`EventReturn`](event::EventReturn)
//! - [`dispatcher`] — polling, allowed updates, startup/shutdown events
//! - [`middlewares`] — outer and inner middlewares
//! - [`filters`] — ready-made and custom filters, including the [`SmartFilter`](filters::SmartFilter)
//! - [`extractor`] — extracting handler arguments from events and the [`Context`]
//! - [`fsm`] — finite state machine (conversations) with pluggable storage
//! - [`types`] and [`methods`] — generated Telegram Bot API objects and requests
//! - [`utils`] — text formatting and rendering helpers, and more
//!
//! More examples can be found in the [examples directory].
//!
//! [`aiogram`]: https://github.com/aiogram/aiogram/
//! [Telegram Bot API]: https://core.telegram.org/bots/api
//! [examples directory]: https://github.com/Desiders/telers/tree/dev-1.x/examples

extern crate self as telers;

mod serializers;

pub(crate) mod any;

#[macro_use]
pub(crate) mod macros;

pub mod client;
pub mod context;
pub mod dispatcher;
pub mod either;
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

#[cfg(feature = "webhooks")]
pub mod webhooks;

pub use telers_macros::{FromContext, FromEvent};

pub use client::Bot;
pub use context::Context;
pub use dispatcher::{Builder as DispatcherBuilder, Dispatcher};
pub use either::Either;
pub use extensions::{Extension, Extensions};
pub use extractor::Extractor;
pub use filters::{Filter, FilterResult};
pub use fsm::Context as FSMContext;
pub use request::Request;
pub use router::{Configured as RouterConfigured, Router};
