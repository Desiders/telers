//! This module contains submodules with components for sending requests to the Telegram Bot API.
//!
//! Components are:
//! - [`base`] module with basic types and traits for sending requests
//! - [`reqwest`] module with reqwest client implementation
//!
//! Check each submodule for more information.

pub mod base;
pub mod reqwest;

pub use self::reqwest::Reqwest;
pub use base::{ClientResponse, Session, StatusCode};
