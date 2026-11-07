/// The message was originally sent to a channel chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginchannel>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct MessageOriginChannel {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Channel chat to which the message was originally sent
    pub chat: Box<crate::types::Chat>,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Signature of the original post author
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub author_signature: Option<Box<str>>,
}
impl MessageOriginChannel {
    /// Creates a new `MessageOriginChannel`.
    ///
    /// # Arguments
    /// * `date` - Date the message was sent originally in Unix time
    /// * `chat` - Channel chat to which the message was originally sent
    /// * `message_id` - Unique message identifier inside the chat
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Chat>, T2: Into<i64>>(
        date: T0,
        chat: T1,
        message_id: T2,
    ) -> Self {
        Self {
            date: date.into(),
            chat: Box::new(chat.into()),
            message_id: message_id.into(),
            author_signature: None,
        }
    }

    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// Channel chat to which the message was originally sent
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// Unique message identifier inside the chat
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// Signature of the original post author
    #[must_use]
    pub fn author_signature<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.author_signature = Some(val.into());
        self
    }

    /// Signature of the original post author
    #[must_use]
    pub fn author_signature_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.author_signature = val.map(Into::into);
        self
    }
}
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for MessageOriginChannel {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "MessageOriginChannel",
            fields: &["date", "chat", "message_id", "author_signature"],
            specific: &["chat", "message_id", "author_signature"],
            sequence: "struct MessageOriginChannel with 4 elements",
        };
        let (common, (value_chat, value_message_id, value_author_signature)) =
            crate::deserialization::message_origin::read_0::<
                D,
                Box<crate::types::Chat>,
                i64,
                Option<Box<str>>,
            >(deserializer, meta)?;
        Ok(Self {
            date: common.date,
            chat: value_chat,
            message_id: value_message_id,
            author_signature: value_author_signature,
        })
    }
}
