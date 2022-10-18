use super::InlineKeyboardMarkup;

use serde::{Deserialize, Serialize};

/// Represents a `Game <https://core.telegram.org/bots/api#games>`_.
/// **Note:** This will only work in Telegram versions released after October 1, 2016. Older clients will not display any inline results if a game result is among them.
/// <https://core.telegram.org/bots/api#inlinequeryresultgame>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be *game*
    #[serde(rename = "type", default = "game")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>`_ attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

fn game() -> String {
    "game".to_string()
}
