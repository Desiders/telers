use super::{Animation, MessageEntity, PhotoSize};

use serde::Deserialize;

/// This object represents a game. Use `BotFather` to create and edit games, their short names will act as unique identifiers.
/// # Documentation
/// <https://core.telegram.org/bots/api#game>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: Box<str>,
    /// Description of the game
    pub description: Box<str>,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Box<[PhotoSize]>,
    /// Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls [`SetGameScore`](crate::methods::SetGameScore), or manually edited using [`EditMessageText`](crate::methods::EditMessageText). 0-4096 characters.
    pub text: Option<Box<str>>,
    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Box<[MessageEntity]>>,
    /// Animation that will be displayed in the game message in chats. Upload via [`BotFather`](https://t.me/botfather)
    pub animation: Option<Animation>,
}
