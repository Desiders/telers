use super::session::{base::Session, Reqwest};

use crate::{
    error::session,
    methods::GetUpdates,
    types::{Update, User},
};

use std::fmt::{self, Debug, Formatter};

/// Represents a bot with a token for getting updates and sending requests to Telegram API
#[derive(Clone, Default)]
pub struct Bot {
    /// Bot token, which is used to receive updates and send requests to the Telegram API
    token: String,
    /// Bot token, which is used in `Debug` implementation for privacy
    hidden_token: String,
    /// Client for sending requests to Telegram API
    client: Reqwest,
}

/// Hide token for privacy. \
/// If token length is less than 4, then it will be hidden as `*`. \
/// For example,
/// `1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11` will be hidden as `12********11`
fn hide_token(token: &str) -> String {
    let token_len = token.len();

    if token_len < 4 {
        return "*".repeat(token_len);
    }

    let mut hidden = String::with_capacity(token_len);
    hidden.push_str(&token[..2]);
    hidden.push_str(&"*".repeat(8));
    hidden.push_str(&token[token_len - 2..]);
    hidden
}

impl Debug for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bot")
            .field("token", &self.hidden_token)
            .field("client", &self.client)
            .finish()
    }
}

/// A block of non-consuming builder
impl Bot {
    #[must_use]
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let token = token.into();
        let hidden_token = hide_token(&token);

        Self {
            token,
            hidden_token,
            client: Reqwest::default(),
        }
    }

    #[must_use]
    pub fn client(&mut self, client: Reqwest) -> &mut Self {
        self.client = client;
        self
    }
}

/// A block of unrelated methods with Telegram methods
impl Bot {
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[must_use]
    pub fn hidden_token(&self) -> &str {
        &self.hidden_token
    }
}

/// A block of Telegram methods
impl Bot {
    #[must_use]
    pub fn get_me(&self) -> User {
        todo!()
    }

    /// Use this method to receive incoming updates using long polling (`wiki <https://en.wikipedia.org/wiki/Push_technology#Long_polling>`). \
    /// # Arguments
    /// * `offset` - Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [`crate::methods::get_updates::GetUpdates`] is called with an `offset` higher than its `update_id`. The negative offset can be specified to retrieve updates starting from `-offset` update from the end of the updates queue. All previous updates will forgotten.
    /// * `limit` - Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    /// * `timeout` - Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` - A JSON-serialized list of the update types you want your bot to receive. For example, specify [`message`, `edited_channel_post`, `callback_query`] to only receive updates of these types. See [`crate::types::Update`] for a complete list of available update types. Specify an empty list to receive all update types except `chat_member` (default). If not specified, the previous setting will be used.
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_updates(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
        timeout: Option<i64>,
        allowed_updates: Option<Vec<String>>,
        request_timeout: Option<f32>,
    ) -> Result<Vec<Update>, session::ErrorKind> {
        self.client
            .make_request_and_get_result(
                self,
                &GetUpdates {
                    offset,
                    limit,
                    timeout,
                    allowed_updates,
                },
                request_timeout,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_hide_token() {
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"), "12********11");
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew1"), "12********w1");
        assert_eq!(hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew"), "12********ew");
        assert_eq!(hide_token("123"), "***");
        assert_eq!(hide_token("1234"), "12********34");
    }
}
