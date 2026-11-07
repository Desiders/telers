/// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
/// # Notes
/// This object represents an update from original update field `chosen_inline_result`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
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
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for UpdateChosenInlineResult {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "UpdateChosenInlineResult",
            fields: &["update_id", "chosen_inline_result"],
            specific: &["chosen_inline_result"],
            sequence: "struct UpdateChosenInlineResult with 2 elements",
        };
        let (common, (value_chosen_inline_result,)) = crate::deserialization::update::read_0::<
            D,
            crate::types::ChosenInlineResult,
        >(deserializer, meta)?;
        Ok(Self {
            update_id: common.update_id,
            chosen_inline_result: value_chosen_inline_result,
        })
    }
}
