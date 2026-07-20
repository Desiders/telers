//! Receiving updates via webhooks instead of long polling (feature `webhooks`).
//!
//! - [`axum`] (feature `axum`) mounts the bot as an `axum` router which accepts updates from
//!   the Telegram server: see [`axum::get_updates_router`] and the `examples/axum_webhook`
//!   example for a complete setup

pub(crate) mod secret;

#[cfg(feature = "axum")]
pub mod axum;
