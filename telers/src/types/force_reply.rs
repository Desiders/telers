use serde::{Deserialize, Serialize};
/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode. Not supported in channels and for messages sent on behalf of a user account.
/// # Documentation
/// <https://core.telegram.org/bots/api#forcereply>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: bool,
    /// The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<Box<str>>,
    /// Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}
impl ForceReply {
    /// Creates a new `ForceReply`.
    ///
    /// # Arguments
    /// * `force_reply` - Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<bool>>(force_reply: T0) -> Self {
        Self {
            force_reply: force_reply.into(),
            input_field_placeholder: None,
            selective: None,
        }
    }

    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    #[must_use]
    pub fn force_reply<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.force_reply = val.into();
        this
    }

    /// The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[must_use]
    pub fn input_field_placeholder<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.input_field_placeholder = Some(val.into());
        this
    }

    /// The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[must_use]
    pub fn input_field_placeholder_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.input_field_placeholder = val.map(Into::into);
        this
    }

    /// Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
    #[must_use]
    pub fn selective<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.selective = Some(val.into());
        this
    }

    /// Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
    #[must_use]
    pub fn selective_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.selective = val.map(Into::into);
        this
    }
}
