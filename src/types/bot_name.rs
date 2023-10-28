use serde::Deserialize;

/// This object represents the bot's name.
/// # Documentation
/// <https://core.telegram.org/bots/api#botname>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct BotName {
    /// The bot's name
    pub name: Box<str>,
}
