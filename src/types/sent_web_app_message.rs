use serde::Deserialize;

/// Describes an inline message sent by a [`Web App`](https://core.telegram.org/bots/webapps) on behalf of a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#sentwebappmessage>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct SentWebAppMessage {
    /// *Optional*. Identifier of the sent inline message. Available only if there is an [`inline keyboard`](https://core.telegram.org/bots/api#inlinekeyboardmarkup) attached to the message.
    pub inline_message_id: Option<String>,
}
