#[cfg(not(feature = "deser"))]
use crate::deserialization::message_origin as decoder;

/// The message was originally sent by an unknown user.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginhiddenuser>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct MessageOriginHiddenUser {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Name of the user that sent the message originally
    pub sender_user_name: Box<str>,
}

impl MessageOriginHiddenUser {
    /// Creates a new `MessageOriginHiddenUser`.
    ///
    /// # Arguments
    /// * `date` - Date the message was sent originally in Unix time
    /// * `sender_user_name` - Name of the user that sent the message originally
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>>(date: T0, sender_user_name: T1) -> Self {
        Self {
            date: date.into(),
            sender_user_name: sender_user_name.into(),
        }
    }

    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// Name of the user that sent the message originally
    #[must_use]
    pub fn sender_user_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sender_user_name = val.into();
        self
    }
}

#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for MessageOriginHiddenUser {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let decoded = decoder::deserialize_content(deserializer, &decoder::HIDDEN_USER)?;
        Ok(Self {
            date: decoded.common.date,
            sender_user_name: decoded.content,
        })
    }
}
