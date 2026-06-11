use crate::types::MessageOrigin;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the origin of a message. It can be one of
/// - [`crate::types::MessageOriginUser`]
/// - [`crate::types::MessageOriginHiddenUser`]
/// - [`crate::types::MessageOriginChat`]
/// - [`crate::types::MessageOriginChannel`]
/// # Documentation
/// <https://core.telegram.org/bots/api#messageorigin>
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
pub enum MessageOriginType {
    #[strum(serialize = "user")]
    User,
    #[strum(serialize = "hidden_user")]
    HiddenUser,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "channel")]
    Channel,
}
impl MessageOriginType {
    #[must_use]
    pub const fn all() -> [MessageOriginType; 4usize] {
        [
            MessageOriginType::User,
            MessageOriginType::HiddenUser,
            MessageOriginType::Chat,
            MessageOriginType::Channel,
        ]
    }
}
impl From<MessageOriginType> for Box<str> {
    fn from(val: MessageOriginType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MessageOriginType> for String {
    fn from(val: MessageOriginType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MessageOriginType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MessageOrigin> for MessageOriginType {
    fn from(val: &'a MessageOrigin) -> Self {
        match val {
            MessageOrigin::User(_) => MessageOriginType::User,
            MessageOrigin::HiddenUser(_) => MessageOriginType::HiddenUser,
            MessageOrigin::Chat(_) => MessageOriginType::Chat,
            MessageOrigin::Channel(_) => MessageOriginType::Channel,
        }
    }
}
impl From<MessageOrigin> for MessageOriginType {
    fn from(val: MessageOrigin) -> Self {
        MessageOriginType::from(&val)
    }
}
