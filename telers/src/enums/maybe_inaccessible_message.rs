use crate::types::MaybeInaccessibleMessage;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes a message that can be inaccessible to the bot. It can be one of
/// - Message
/// - [`InaccessibleMessage`]
/// # Documentation
/// <https://core.telegram.org/bots/api#maybeinaccessiblemessage>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum MaybeInaccessibleMessageType {
    #[strum(serialize = "inaccessible_message")]
    InaccessibleMessage,
    #[strum(serialize = "message")]
    Message,
}
impl MaybeInaccessibleMessageType {
    #[must_use]
    pub const fn all() -> [MaybeInaccessibleMessageType; 2usize] {
        [
            MaybeInaccessibleMessageType::InaccessibleMessage,
            MaybeInaccessibleMessageType::Message,
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
        }
    }
}
impl From<MaybeInaccessibleMessage> for MaybeInaccessibleMessageType {
    fn from(val: MaybeInaccessibleMessage) -> Self {
        MaybeInaccessibleMessageType::from(&val)
    }
}
