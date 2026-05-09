use serde::{Deserialize, Serialize};
/// The reaction is based on an emoji.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontypeemoji>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactionTypeEmoji {
    /// Reaction emoji. Currently, it can be one of `❤`, `👍`, `👎`, `🔥`, `🥰`, `👏`, `😁`, `🤔`, `🤯`, `😱`, `🤬`, `😢`, `🎉`, `🤩`, `🤮`, `💩`, `🙏`, `👌`, `🕊`, `🤡`, `🥱`, `🥴`, `😍`, `🐳`, `❤‍🔥`, `🌚`, `🌭`, `💯`, `🤣`, `⚡`, `🍌`, `🏆`, `💔`, `🤨`, `😐`, `🍓`, `🍾`, `💋`, `🖕`, `😈`, `😴`, `😭`, `🤓`, `👻`, `👨‍💻`, `👀`, `🎃`, `🙈`, `😇`, `😨`, `🤝`, `✍`, `🤗`, `🫡`, `🎅`, `🎄`, `☃`, `💅`, `🤪`, `🗿`, `🆒`, `💘`, `🙉`, `🦄`, `😘`, `💊`, `🙊`, `😎`, `👾`, `🤷‍♂`, `🤷`, `🤷‍♀`, `😡`
    pub emoji: Box<str>,
}
impl ReactionTypeEmoji {
    /// Creates a new `ReactionTypeEmoji`.
    ///
    /// # Arguments
    /// * `emoji` - Reaction emoji. Currently, it can be one of `❤`, `👍`, `👎`, `🔥`, `🥰`, `👏`, `😁`, `🤔`, `🤯`, `😱`, `🤬`, `😢`, `🎉`, `🤩`, `🤮`, `💩`, `🙏`, `👌`, `🕊`, `🤡`, `🥱`, `🥴`, `😍`, `🐳`, `❤‍🔥`, `🌚`, `🌭`, `💯`, `🤣`, `⚡`, `🍌`, `🏆`, `💔`, `🤨`, `😐`, `🍓`, `🍾`, `💋`, `🖕`, `😈`, `😴`, `😭`, `🤓`, `👻`, `👨‍💻`, `👀`, `🎃`, `🙈`, `😇`, `😨`, `🤝`, `✍`, `🤗`, `🫡`, `🎅`, `🎄`, `☃`, `💅`, `🤪`, `🗿`, `🆒`, `💘`, `🙉`, `🦄`, `😘`, `💊`, `🙊`, `😎`, `👾`, `🤷‍♂`, `🤷`, `🤷‍♀`, `😡`
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(emoji: T0) -> Self {
        Self {
            emoji: emoji.into(),
        }
    }

    /// Reaction emoji. Currently, it can be one of `❤`, `👍`, `👎`, `🔥`, `🥰`, `👏`, `😁`, `🤔`, `🤯`, `😱`, `🤬`, `😢`, `🎉`, `🤩`, `🤮`, `💩`, `🙏`, `👌`, `🕊`, `🤡`, `🥱`, `🥴`, `😍`, `🐳`, `❤‍🔥`, `🌚`, `🌭`, `💯`, `🤣`, `⚡`, `🍌`, `🏆`, `💔`, `🤨`, `😐`, `🍓`, `🍾`, `💋`, `🖕`, `😈`, `😴`, `😭`, `🤓`, `👻`, `👨‍💻`, `👀`, `🎃`, `🙈`, `😇`, `😨`, `🤝`, `✍`, `🤗`, `🫡`, `🎅`, `🎄`, `☃`, `💅`, `🤪`, `🗿`, `🆒`, `💘`, `🙉`, `🦄`, `😘`, `💊`, `🙊`, `😎`, `👾`, `🤷‍♂`, `🤷`, `🤷‍♀`, `😡`
    #[must_use]
    pub fn emoji<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.emoji = val.into();
        self
    }
}
