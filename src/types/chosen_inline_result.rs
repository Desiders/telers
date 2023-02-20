use super::{Location, Update, User};

use crate::error::ConvertUpdateToTypeError;

use serde::Deserialize;

/// Represents a [`result`](https://core.telegram.org/bots/api#inlinequeryresult) of an inline query that was chosen by the user and sent to their chat partner.
/// # Notes
/// It is necessary to enable [`inline feedback`](https://core.telegram.org/bots/inline#collecting-feedback) via [`@BotFather`](https://t.me/botfather) in order to receive these objects in updates.
/// # Documentation
/// <https://core.telegram.org/bots/api#choseninlineresult>
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// *Optional*. Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// *Optional*. Identifier of the sent inline message. Available only if there is an [`inline keyboard`](https://core.telegram.org/bots/api#inlinekeyboardmarkup) attached to the message. Will be also received in [`callback queries`](https://core.telegram.org/bots/api#callbackquery) and can be used to [`edit`](https://core.telegram.org/bots/api#updating-messages) the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}

impl TryFrom<Update> for ChosenInlineResult {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(chosen_inline_result) = update.chosen_inline_result {
            Ok(chosen_inline_result)
        } else {
            Err(ConvertUpdateToTypeError::new(format!(
                "Update `{update:?}` doesn't contain `ChosenInlineResult`"
            )))
        }
    }
}
