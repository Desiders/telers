use serde::{Deserialize, Serialize};
/// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
/// # Notes
/// This object represents an update from original update field `chosen_inline_result`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChosenInlineResult {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    pub chosen_inline_result: crate::types::ChosenInlineResult,
}
impl UpdateChosenInlineResult {
    /// Creates a new `UpdateChosenInlineResult`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `chosen_inline_result` - The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ChosenInlineResult>>(
        update_id: T0,
        chosen_inline_result: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            chosen_inline_result: chosen_inline_result.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[must_use]
    pub fn chosen_inline_result<T: Into<crate::types::ChosenInlineResult>>(
        mut self,
        val: T,
    ) -> Self {
        self.chosen_inline_result = val.into();
        self
    }
}
impl From<UpdateChosenInlineResult> for crate::types::ChosenInlineResult {
    fn from(val: UpdateChosenInlineResult) -> Self {
        val.chosen_inline_result
    }
}
impl<Client> crate::Extractor<Client> for UpdateChosenInlineResult {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
