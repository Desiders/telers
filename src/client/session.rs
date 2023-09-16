//! Client session for sending requests to Telegram Bot API.
//!
//! This module contains [`Session`] trait, which is used to send requests to Telegram Bot API.
//! [`Session`] trait is implemented for [`Reqwest`] struct, which is default implementation of [`Session`].
//!
//! [`Session::send_request`] method is used to send requests and returns [`ClientResponse`] instance,
//! which is used to get response from Telegram Bot API.
//! It accepts [`Bot`] instance, which is used for building request (for example, to get token),
//! and [`TelegramMethod`] instance, which is used for building request and [`ClientResponse`] instance,
//! because [`TelegramMethod`] contains information about response type and request data.
//! Check [`methods module`] documentation for more information about methods.
//!
//! [`Bot`]: crate::client::Bot
//! [`TelegramMethod`]: crate::methods::TelegramMethod
//! [`methods module`]: crate::methods

pub mod base;
pub mod reqwest;

pub use self::reqwest::Reqwest;
pub use base::{ClientResponse, Session, StatusCode};
