//! This module contains submodules with components for sending requests to the Telegram Bot API, its conguration
//! and the main entry point for the library - the [`Bot`] struct.
//!
//! Components are:
//! - [`bot`] module with the main entry point for the library
//! - [`session`] module with components for sending requests
//! - [`telegram`] module with configuration of the Telegram Bot API
//!
//! Check each submodule for more information.

pub mod bot;
pub mod session;
pub mod telegram;

pub use bot::{download::FileDownload, Bot, RetryPolicy};
pub use session::{Reqwest, Session};

// Re-exported because [`Bot::token`] returns a [`SecretString`], so users need these to read the
// token without adding a direct `secrecy` dependency.
pub use secrecy::{ExposeSecret, SecretString};

// Re-exported because [`Reqwest::with_proxy`] takes a [`Proxy`], so users need it to configure
// a proxy without adding a direct `reqwest` dependency.
pub use reqwest::Proxy;
