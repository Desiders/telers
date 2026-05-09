use serde::{Deserialize, Serialize};
/// A new bot was created to be managed by the bot, or token or owner of a managed bot was changed
/// # Notes
/// This object represents an update from original update field `managed_bot`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateManagedBot {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A new bot was created to be managed by the bot, or token or owner of a managed bot was changed
    pub managed_bot: crate::types::ManagedBotUpdated,
}
impl UpdateManagedBot {
    /// Creates a new `UpdateManagedBot`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `managed_bot` - A new bot was created to be managed by the bot, or token or owner of a managed bot was changed
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ManagedBotUpdated>>(
        update_id: T0,
        managed_bot: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            managed_bot: managed_bot.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// A new bot was created to be managed by the bot, or token or owner of a managed bot was changed
    #[must_use]
    pub fn managed_bot<T: Into<crate::types::ManagedBotUpdated>>(self, val: T) -> Self {
        let mut this = self;
        this.managed_bot = val.into();
        this
    }
}
impl From<UpdateManagedBot> for crate::types::ManagedBotUpdated {
    fn from(val: UpdateManagedBot) -> Self {
        val.managed_bot
    }
}
impl<Client> crate::Extractor<Client> for UpdateManagedBot {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
