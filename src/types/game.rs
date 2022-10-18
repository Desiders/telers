use super::{Animation, MessageEntity, PhotoSize};

use serde::{Deserialize, Serialize};

/// This object represents a game. Use `BotFather` to create and edit games, their short names will act as unique identifiers.
/// <https://core.telegram.org/bots/api#game>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// *Optional*. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls :class:`aiogram_rs.methods.set_game_score.SetGameScore`, or manually edited using :class:`aiogram_rs.methods.edit_message_text.EditMessageText`. 0-4096 characters.
    pub text: Option<String>,
    /// *Optional*. Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Animation that will be displayed in the game message in chats. Upload via `BotFather <https://t.me/botfather>`_
    pub animation: Option<Animation>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            title: String::default(),
            description: String::default(),
            photo: Vec::default(),
            text: None,
            text_entities: None,
            animation: None,
        }
    }
}
