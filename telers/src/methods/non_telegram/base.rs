//! Compatibility with framework-aware custom methods and request preparation.
//!
//! Generated methods implement the transport-independent trait in `telers-methods`.
//! This adapter preserves the framework trait and its `Bot` argument for existing callers.

pub use telers_methods::base::*;

use crate::{client::Bot, serialization};

/// A Telegram Bot API method accepted by [`Bot::send`].
///
/// Standalone methods automatically implement this trait. Existing custom framework
/// methods can continue implementing it directly. When importing both traits, use
/// qualified calls to distinguish their `build_request` signatures.
pub trait TelegramMethod {
    type Method: serialization::Serialize;
    type Return: serialization::DeserializeOwned;

    #[must_use]
    fn build_request<Client>(self, bot: &Bot<Client>) -> Request<Self::Method>;

    /// Parse the API response using the selected serialization backend.
    ///
    /// # Errors
    /// Returns an error if the response cannot be parsed.
    fn build_response(content: &str) -> Result<Response<Self::Return>, serialization::Error> {
        telers_methods::base::parse_response(content)
    }
}

impl<T: telers_methods::TelegramMethod> TelegramMethod for T {
    type Method = <T as telers_methods::TelegramMethod>::Method;
    type Return = <T as telers_methods::TelegramMethod>::Return;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        telers_methods::TelegramMethod::build_request(self)
    }

    fn build_response(content: &str) -> Result<Response<Self::Return>, serialization::Error> {
        <T as telers_methods::TelegramMethod>::build_response(content)
    }
}
