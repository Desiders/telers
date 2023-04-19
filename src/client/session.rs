//! Client session for sending requests to Telegram Bot API.
//!
//! This module contains [`Session`] trait, which is used to send requests to Telegram Bot API.
//! [`Session`] trait is implemented for [`Reqwest`] struct, which is default implementation of [`Session`].
//!
//! [`Session::send_request`] method is used to send requests and returns [`ClientResponse`] instance,
//! which is used to get response from Telegram Bot API.
//! It accepts [`crate::client::Bot`] instance, which is used for building request (for example, to get token),
//! and [`crate::methods::TelegramMethod`] instance, which is used for building request and [`ClientResponse`] instance,
//! because [`crate::methods::TelegramMethod`] contains information about response type and request data.
//! Check [`crate::methods`] module for more information about methods.

pub mod base;
pub mod reqwest;

pub use self::reqwest::Reqwest;
pub use base::{ClientResponse, Session, StatusCode};
