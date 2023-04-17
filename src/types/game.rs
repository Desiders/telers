use super::{Animation, MessageEntity, PhotoSize};

use serde::Deserialize;

/// This object represents a game. Use `BotFather` to create and edit games, their short names will act as unique identifiers.
/// # Documentation
/// <https://core.telegram.org/bots/api#game>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls [`SetGameScore`](crate::methods::SetGameScore), or manually edited using [`EditMessageText`](crate::methods::EditMessageText). 0-4096 characters.
    pub text: Option<String>,
    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Animation that will be displayed in the game message in chats. Upload via [`BotFather`](https://t.me/botfather)
    pub animation: Option<Animation>,
}
