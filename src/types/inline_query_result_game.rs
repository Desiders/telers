use super::InlineKeyboardMarkup;

use serde::{Deserialize, Serialize};

/// Represents a `Game <https://core.telegram.org/bots/api#games>`.
/// **Note:** This will only work in Telegram versions released after October 1, 2016. Older clients will not display any inline results if a game result is among them.
/// <https://core.telegram.org/bots/api#inlinequeryresultgame>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be *game*
    #[serde(rename = "type", default = "game")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl Default for InlineQueryResultGame {
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
    "game".to_string()
}
