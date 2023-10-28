use serde::Deserialize;

/// This object represents the bot's short description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botshortdescription>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: Box<str>,
}
