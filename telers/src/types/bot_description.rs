use serde::Deserialize;

/// This object represents the bot's description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botdescription>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct BotDescription {
    /// The bot's description
    pub description: Box<str>,
}
