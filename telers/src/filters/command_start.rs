//! Filter for `/start` commands with optional deep-link payload validation.

use super::{Filter, FilterResult};
use crate::{filters::command::CommandObject, utils::decode_payload, Request};

use std::{convert::Infallible, future::Future};

/// Deep-link payload validation rules for [`CommandStart`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeepLink {
    /// The payload must decode as base64url.
    Encoded,
    /// The payload may be any non-empty text.
    Plain,
}

/// Matches `/start` commands that carry a payload.
///
/// If the payload must be base64url-encoded, it is validated with
/// [`decode_payload`]. To match any `/start` command without payload checks,
/// use [`Command::one`].
///
/// [`Command::one`]: crate::filters::command::Command::one
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandStart {
    deep_link: DeepLink,
}

impl CommandStart {
    /// Matches `/start` commands that carry a payload.
    #[must_use]
    pub fn with_payload(deep_link: DeepLink) -> Self {
        Self {
            deep_link,
        }
    }
}

#[allow(clippy::manual_async_fn)]
impl<Client: Send + Sync + 'static> Filter<Client> for CommandStart {
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
        async move {
            let Some(message) = request.update.message() else {
                return Ok(false);
            };
            let Some(text) = message.text().or(message.caption()) else {
                return Ok(false);
            };
            let Some(command) = CommandObject::extract(text) else {
                return Ok(false);
            };
            if !command.command.eq_ignore_ascii_case("start") {
                return Ok(false);
            }
            let Some(payload) = command.args.first() else {
                return Ok(false);
            };
            if matches!(self.deep_link, DeepLink::Encoded) && decode_payload(payload).is_err() {
                return Ok(false);
            }
            request.context.insert("command", command);
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
    };

    use std::sync::Arc;

    fn request(text: &str) -> Request<Reqwest> {
        Request {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), text),
            ))),
            bot: crate::Bot::default(),
            context: crate::Context::default(),
            extensions: crate::Extensions::default(),
        }
    }

    #[tokio::test]
    async fn rejects_other_commands() {
        let mut req = request("/help");
        assert!(!CommandStart::with_payload(DeepLink::Plain)
            .check(&mut req)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn rejects_non_commands() {
        let mut req = request("just text");
        assert!(!CommandStart::with_payload(DeepLink::Plain)
            .check(&mut req)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn start_without_payload_rejected() {
        let mut req = request("/start");
        assert!(!CommandStart::with_payload(DeepLink::Plain)
            .check(&mut req)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn plain_payload_passes() {
        let mut req = request("/start ref123");
        assert!(CommandStart::with_payload(DeepLink::Plain)
            .check(&mut req)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn encoded_payload_passes() {
        let mut req = request("/start aGVsbG8gd29ybGQ");
        assert!(CommandStart::with_payload(DeepLink::Encoded)
            .check(&mut req)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn non_encoded_payload_rejected_by_encoded_check() {
        let mut req = request("/start ref123");
        assert!(!CommandStart::with_payload(DeepLink::Encoded)
            .check(&mut req)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn invalid_encoded_payload_rejected() {
        let mut req = request("/start !!!");
        assert!(!CommandStart::with_payload(DeepLink::Encoded)
            .check(&mut req)
            .await
            .unwrap());
    }
}
