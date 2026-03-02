use serde::{Deserialize, Serialize};
/// This object describes a message that can be inaccessible to the bot. It can be one of
/// - Message
/// - [`InaccessibleMessage`]
/// # Documentation
/// <https://core.telegram.org/bots/api#maybeinaccessiblemessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    InaccessibleMessage(crate::types::InaccessibleMessage),
    Message(crate::types::Message),
}
impl MaybeInaccessibleMessage {
    /// Helper method for field `chat`.
    ///
    /// Chat the message belonged to
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::InaccessibleMessage(val) => Some(val.chat.as_ref()),
            Self::Message(_) => None,
        }
    }

    /// Helper method for field `date`.
    ///
    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    #[must_use]
    pub fn date(&self) -> Option<i64> {
        match self {
            Self::InaccessibleMessage(val) => Some(val.date),
            Self::Message(_) => None,
        }
    }

    /// Helper method for field `message_id`.
    ///
    /// Unique message identifier inside the chat
    #[must_use]
    pub fn message_id(&self) -> Option<i64> {
        match self {
            Self::InaccessibleMessage(val) => Some(val.message_id),
            Self::Message(_) => None,
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::InaccessibleMessage(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::first_name(inner)
            }
            Self::Message(_) => None,
        }
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> Option<i64> {
        match self {
            Self::InaccessibleMessage(val) => {
                let inner = val.chat.as_ref();
                Some(crate::types::Chat::id(inner))
            }
            Self::Message(_) => None,
        }
    }

    /// Helper method for nested field `is_direct_messages`.
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        match self {
            Self::InaccessibleMessage(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::is_direct_messages(inner)
            }
            Self::Message(_) => None,
        }
    }

    /// Helper method for nested field `is_forum`.
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        match self {
            Self::InaccessibleMessage(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::is_forum(inner)
            }
            Self::Message(_) => None,
        }
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::InaccessibleMessage(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::last_name(inner)
            }
            Self::Message(_) => None,
        }
    }

    /// Helper method for nested field `title`.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::InaccessibleMessage(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::title(inner)
            }
            Self::Message(_) => None,
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::InaccessibleMessage(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::username(inner)
            }
            Self::Message(_) => None,
        }
    }
}
impl From<crate::types::InaccessibleMessage> for MaybeInaccessibleMessage {
    fn from(val: crate::types::InaccessibleMessage) -> Self {
        Self::InaccessibleMessage(val)
    }
}
impl TryFrom<MaybeInaccessibleMessage> for crate::types::InaccessibleMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MaybeInaccessibleMessage) -> Result<Self, Self::Error> {
        match val {
            MaybeInaccessibleMessage::InaccessibleMessage(inner) => Ok(inner),
            MaybeInaccessibleMessage::Message(_) => Err(Self::Error::new(
                stringify!(MaybeInaccessibleMessage),
                stringify!(InaccessibleMessage),
            )),
        }
    }
}
impl From<crate::types::Message> for MaybeInaccessibleMessage {
    fn from(val: crate::types::Message) -> Self {
        Self::Message(val)
    }
}
impl TryFrom<MaybeInaccessibleMessage> for crate::types::Message {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MaybeInaccessibleMessage) -> Result<Self, Self::Error> {
        match val {
            MaybeInaccessibleMessage::Message(inner) => Ok(inner),
            MaybeInaccessibleMessage::InaccessibleMessage(_) => Err(Self::Error::new(
                stringify!(MaybeInaccessibleMessage),
                stringify!(Message),
            )),
        }
    }
}
