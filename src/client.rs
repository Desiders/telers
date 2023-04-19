//! Telegram Bot API client.
//!
//! This module contains the [`Bot`] struct, which is the main entry point for the library.
//! Using this struct, you can send methods to the Telegram Bot API and receive responses.
//!
//! [`Bot`] supports different [`Session`] implementations, which are used to send requests,
//! and [`Session`]s can be customized to fit your needs. Check [`Reqwest`] for more information about default implementation.
//! You can check example of create [`Bot`] with "other" client and using it in handlers in `examples/other_client.rs`.
//!
//! [`Bot`] supports different ways to send methods to the Telegram Bot API:
//! - You can use `Bot::send` method, which accepts any type that implements [`crate::methods::TelegramMethod`].
//! This method is the most comfortable, because you can use any method from [`crate::methods`] module with
//! implemented builders and you don't need to pass all parameters to it, only required,
//! and optional by using builder methods. Builders yet can have some useful shortcuts.
//! Also, you can use your own methods, which implements [`crate::methods::TelegramMethod`].
//! - You can use `Bot::{method}` methods, which are shortcuts for `Bot::send` method,
//! where method name is the same as method name in Telegram Bot API, but in snake case.
//! For example, `Bot::send_message` is a shortcut for `Bot::send` method, where method is [`crate::methods::SendMessage`].
//! This method isn't so comfortable, because you need to pass all parameters (required and optional) to it.
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
//!     bot.send_message(
//!         123, "Hello, world!", Some(123),
//!         ..., // optional parameters,
//!         None, // request timeout
//!     ).await;
//! }
//! ```
//!
//! [`bot`] module contains [`Bot`] struct, which is the main entry point for the library.
//! Check this module documentation for more information.
//!
//! [`session`] module contains [`Session`] trait, which is used to send requests to Telegram Bot API.
//! Check this module documentation for more information or if you want to use your own client.
//!
//! [`telegram`] module contains configuration of Telegram Bot API, which is used by [`Session`] implementations
//! for building URLs for requests and set Telegram Bot API server configuration.
//! Check this module documentation for more information or if you want to use local Telegram Bot API server.

pub mod bot;
pub mod session;
pub mod telegram;

pub use bot::Bot;
pub use session::{Reqwest, Session};
