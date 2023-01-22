use super::InlineKeyboardMarkup;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a `Game <https://core.telegram.org/bots/api#games>`.
/// # Notes
/// This will only work in Telegram versions released after October 1, 2016. Older clients will not display any inline results if a game result is among them.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultgame>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be *game*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultGame {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, game_short_name: T) -> Self {
        Self {
            id: id.into(),
            game_short_name: game_short_name.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn game_short_name<T: Into<String>>(mut self, val: T) -> Self {
        self.game_short_name = val.into();
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl Default for InlineQueryResultGame {
    fn default() -> Self {
        Self {
            result_type: "game".to_string(),
            id: String::default(),
            game_short_name: String::default(),
            reply_markup: None,
        }
    }
}
