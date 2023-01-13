use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes an inline message sent by a `Web App <https://core.telegram.org/bots/webapps>` on behalf of a user.
/// <https://core.telegram.org/bots/api#sentwebappmessage>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// *Optional*. Identifier of the sent inline message. Available only if there is an `inline keyboard <https://core.telegram.org/bots/api#inlinekeyboardmarkup>` attached to the message.
    pub inline_message_id: Option<String>,
}
