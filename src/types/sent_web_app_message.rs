use serde::{Deserialize, Serialize};

/// Describes an inline message sent by a `Web App <https://core.telegram.org/bots/webapps>`_ on behalf of a user.
/// <https://core.telegram.org/bots/api#sentwebappmessage>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// *Optional*. Identifier of the sent inline message. Available only if there is an `inline keyboard <https://core.telegram.org/bots/api#inlinekeyboardmarkup>`_ attached to the message.
    pub inline_message_id: Option<String>,
}
