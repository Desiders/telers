//! This module contains the default serializers implementations for serializing the request body.
//!
//! `reqwest` serializer is enabled by default, if you want to use your own serializer, you can
//! do it for custom client and implement [`Session`] trait for it.
//! You can check example of using custom client in bot in `examples/bot_http_client`.
//!
//! [`Session`]: crate::client::Session

pub(crate) mod reqwest;
