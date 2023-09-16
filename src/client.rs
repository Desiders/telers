//! Telegram Bot API client.
//!
//! This module contains the [`Bot`] struct, which is the main entry point for the library.
//! Using this struct, you can send methods to the Telegram Bot API and receive responses.
//!
//! [`Bot`] supports different [`Session`] implementations, which are used to send requests,
//! and [`Session`]s can be customized to fit your needs. Check [`Reqwest`] for more information about default implementation.
//! You can check example of create [`Bot`] with "other" client and using it in handlers in `examples/bot_http_client`.
//!
//! You can use `Bot::send` method, which accepts any type that implements [`TelegramMethod`].
//! This method is the most comfortable, because you can use any method from [`methods module`] with
//! implemented builders and you don't need to pass all parameters to it, only required,
//! and optional by using builder methods. Builders yet can have some useful shortcuts.
//! Also, you can use your own methods, which implements [`TelegramMethod`].
//!
//! # Example
//! ```ignore
//! use telers::client::Bot;
//!
//! #[tokio::main]
//! async fn main() {
//!     let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
//!          panic!("BOT_TOKEN env variable is not set!");
//!     };
//!
//!     let bot = Bot::new(bot_token);
//!
//!     bot.send(
//!         &SendMessage::new(123, "Hello, world!").message_thread_id(123),
//!         None,
//!     ).await;
//! }
//! ```
//!
//! [`bot module`] contains [`Bot`] struct, which is the main entry point for the library.
//! Check this module documentation for more information.
//!
//! [`session module`] contains [`Session`] trait, which is used to send requests to Telegram Bot API.
//! Check this module documentation for more information or if you want to use your own client.
//!
//! [`telegram module`] contains configuration of Telegram Bot API, which is used by [`Session`] implementations
//! for building URLs for requests and set Telegram Bot API server configuration.
//! Check this module documentation for more information or if you want to use local Telegram Bot API server.
//!
//! [`TelegramMethod`]: crate::methods::TelegramMethod
//! [`methods module`]: crate::methods
//! [`bot module`]: crate::client::bot
//! [`session module`]: crate::client::session
//! [`telegram module`]: crate::client::telegram

pub mod bot;
pub mod session;
pub mod telegram;

pub use bot::Bot;
pub use session::{Reqwest, Session};
