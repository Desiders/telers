use super::{Location, User};

use serde::{Deserialize, Serialize};

/// Represents a `result <https://core.telegram.org/bots/api#inlinequeryresult>`_ of an inline query that was chosen by the user and sent to their chat partner.
/// **Note:** It is necessary to enable `inline feedback <https://core.telegram.org/bots/inline#collecting-feedback>`_ via `@BotFather <https://t.me/botfather>`_ in order to receive these objects in updates.
/// <https://core.telegram.org/bots/api#choseninlineresult>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// *Optional*. Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// *Optional*. Identifier of the sent inline message. Available only if there is an `inline keyboard <https://core.telegram.org/bots/api#inlinekeyboardmarkup>`_ attached to the message. Will be also received in `callback queries <https://core.telegram.org/bots/api#callbackquery>`_ and can be used to `edit <https://core.telegram.org/bots/api#updating-messages>`_ the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}

impl Default for ChosenInlineResult {
    fn default() -> Self {
        Self {
            result_id: String::default(),
            from: User::default(),
            location: None,
            inline_message_id: None,
            query: String::default(),
        }
    }
}
