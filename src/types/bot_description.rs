use serde::Deserialize;

/// This object represents the bot's description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botdescription>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct BotDescription {
    /// The bot's description
    pub description: String,
}
