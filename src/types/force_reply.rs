use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice `privacy mode <https://core.telegram.org/bots/features/#privacy-mode>`.
/// **Example:** A `poll bot <https://t.me/PollBot>` for groups runs in privacy mode (only receives commands, replies to its messages and mentions). There could be two ways to create a new poll:
/// - Explain the user how to send a command with parameters (e.g. /newpoll question answer1 answer2). May be appealing for hardcore users but lacks modern day polish.
/// - Guide the user through a step-by-step process. 'Please send me your question', 'Cool, now let's add the first answer option', 'Great. Keep adding answer options, then send /done when you're ready'.
/// The last option is definitely more attractive. And if you use `aiogram_rs.types.force_reply.ForceReply` in your bot's questions, it will receive the user's answers even if it only receives replies, commands and mentions - without any extra work for the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#forcereply>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: bool,
    /// *Optional*. The placeholder to be shown in the input field when the reply is active; 1-64 characters
    pub input_field_placeholder: Option<String>,
    /// *Optional*. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the *text* of the `aiogram_rs.types.message.Message` object; 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    pub selective: Option<bool>,
}

impl ForceReply {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn input_field_placeholder<T: Into<String>>(mut self, val: T) -> Self {
        self.input_field_placeholder = Some(val.into());
        self
    }

    #[must_use]
    pub fn selective(mut self, val: bool) -> Self {
        self.selective = Some(val);
        self
    }
}

impl Default for ForceReply {
    #[must_use]
    fn default() -> Self {
        Self {
            force_reply: true,
            input_field_placeholder: None,
            selective: None,
        }
    }
}
