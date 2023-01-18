use super::{Animation, MessageEntity, PhotoSize};

use serde::Deserialize;

/// This object represents a game. Use `BotFather` to create and edit games, their short names will act as unique identifiers.
/// <https://core.telegram.org/bots/api#game>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// *Optional*. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls `aiogram_rs.methods.set_game_score.SetGameScore`, or manually edited using `aiogram_rs.methods.edit_message_text.EditMessageText`. 0-4096 characters.
    pub text: Option<String>,
    /// *Optional*. Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Animation that will be displayed in the game message in chats. Upload via `BotFather <https://t.me/botfather>`
    pub animation: Option<Animation>,
}
