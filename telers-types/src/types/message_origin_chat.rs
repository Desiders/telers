#[cfg(not(feature = "deser"))]
use crate::deserialization::message_origin as decoder;

/// The message was originally sent on behalf of a chat to a group chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginchat>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct MessageOriginChat {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Chat that sent the message originally
    pub sender_chat: Box<crate::types::Chat>,
    /// For messages originally sent by an anonymous chat administrator, original message author signature
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub author_signature: Option<Box<str>>,
}

impl MessageOriginChat {
    /// Creates a new `MessageOriginChat`.
    ///
    /// # Arguments
    /// * `date` - Date the message was sent originally in Unix time
    /// * `sender_chat` - Chat that sent the message originally
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Chat>>(date: T0, sender_chat: T1) -> Self {
        Self {
            date: date.into(),
            sender_chat: Box::new(sender_chat.into()),
            author_signature: None,
        }
    }

    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// Chat that sent the message originally
    #[must_use]
    pub fn sender_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.sender_chat = Box::new(val.into());
        self
    }

    /// For messages originally sent by an anonymous chat administrator, original message author signature
    #[must_use]
    pub fn author_signature<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.author_signature = Some(val.into());
        self
    }

    /// For messages originally sent by an anonymous chat administrator, original message author signature
    #[must_use]
    pub fn author_signature_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.author_signature = val.map(Into::into);
        self
    }
}

#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for MessageOriginChat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let decoded = decoder::deserialize_chat(deserializer, &decoder::CHAT)?;
        Ok(Self {
            date: decoded.common.date,
            sender_chat: decoded.sender_chat,
            author_signature: decoded.author_signature,
        })
    }
}
