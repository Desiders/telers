use serde::{Deserialize, Serialize};
/// The message was originally sent by a known user.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginuser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageOriginUser {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// User that sent the message originally
    pub sender_user: Box<crate::types::User>,
}
impl MessageOriginUser {
    /// Creates a new `MessageOriginUser`.
    ///
    /// # Arguments
    /// * `date` - Date the message was sent originally in Unix time
    /// * `sender_user` - User that sent the message originally
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::User>>(date: T0, sender_user: T1) -> Self {
        Self {
            date: date.into(),
            sender_user: Box::new(sender_user.into()),
        }
    }

    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }

    /// User that sent the message originally
    #[must_use]
    pub fn sender_user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.sender_user = Box::new(val.into());
        this
    }
}
