use crate::types::MaybeInaccessibleMessage;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes a message that can be inaccessible to the bot. It can be one of
/// - Message
/// - [`crate::types::InaccessibleMessage`]
/// # Documentation
/// <https://core.telegram.org/bots/api#maybeinaccessiblemessage>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum MaybeInaccessibleMessageType {
    #[strum(serialize = "inaccessible_message")]
    InaccessibleMessage,
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl MaybeInaccessibleMessageType {
    #[must_use]
    pub const fn all() -> [MaybeInaccessibleMessageType; 3usize] {
        [
            MaybeInaccessibleMessageType::InaccessibleMessage,
            MaybeInaccessibleMessageType::Message,
            MaybeInaccessibleMessageType::Unknown,
        ]
    }
}
impl From<MaybeInaccessibleMessageType> for Box<str> {
    fn from(val: MaybeInaccessibleMessageType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MaybeInaccessibleMessageType> for String {
    fn from(val: MaybeInaccessibleMessageType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MaybeInaccessibleMessageType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MaybeInaccessibleMessage> for MaybeInaccessibleMessageType {
    fn from(val: &'a MaybeInaccessibleMessage) -> Self {
        match val {
            MaybeInaccessibleMessage::InaccessibleMessage(_) => {
                MaybeInaccessibleMessageType::InaccessibleMessage
            }
            MaybeInaccessibleMessage::Message(_) => MaybeInaccessibleMessageType::Message,
            MaybeInaccessibleMessage::Unknown(_) => MaybeInaccessibleMessageType::Unknown,
        }
    }
}
impl From<MaybeInaccessibleMessage> for MaybeInaccessibleMessageType {
    fn from(val: MaybeInaccessibleMessage) -> Self {
        MaybeInaccessibleMessageType::from(&val)
    }
}
