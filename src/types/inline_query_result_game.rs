use super::InlineKeyboardMarkup;

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a [`Game`](https://core.telegram.org/bots/api#games).
/// # Notes
/// This will only work in Telegram versions released after October 1, 2016. Older clients will not display any inline results if a game result is among them.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultgame>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be *game*
    #[serde(rename = "type", default = "game")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultGame {
    #[must_use]
    pub fn new(id: impl Into<String>, game_short_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            game_short_name: game_short_name.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id(self, val: impl Into<String>) -> Self {
        Self {
            id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn game_short_name(self, val: impl Into<String>) -> Self {
        Self {
            game_short_name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl Default for InlineQueryResultGame {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: game(),
            id: String::default(),
            game_short_name: String::default(),
            reply_markup: None,
        }
    }
}

fn game() -> String {
    InlineQueryResultType::Game.into()
}
