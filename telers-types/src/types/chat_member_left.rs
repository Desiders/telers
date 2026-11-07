/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberleft>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct ChatMemberLeft {
    /// Information about the user
    pub user: Box<crate::types::User>,
}
impl ChatMemberLeft {
    /// Creates a new `ChatMemberLeft`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(user: T0) -> Self {
        Self {
            user: Box::new(user.into()),
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for ChatMemberLeft {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "ChatMemberLeft",
            fields: &["user"],
            specific: &[],
            sequence: "struct ChatMemberLeft with 1 element",
        };
        let (common, ()) = crate::deserialization::chat_member::read_2::<D>(deserializer, meta)?;
        Ok(Self {
            user: common.user,
        })
    }
}
