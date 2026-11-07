/// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
/// # Notes
/// This object represents an update from original update field `purchased_paid_media`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct UpdatePurchasedPaidMedia {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    pub purchased_paid_media: crate::types::PaidMediaPurchased,
}
impl UpdatePurchasedPaidMedia {
    /// Creates a new `UpdatePurchasedPaidMedia`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `purchased_paid_media` - A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::PaidMediaPurchased>>(
        update_id: T0,
        purchased_paid_media: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            purchased_paid_media: purchased_paid_media.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    #[must_use]
    pub fn purchased_paid_media<T: Into<crate::types::PaidMediaPurchased>>(
        mut self,
        val: T,
    ) -> Self {
        self.purchased_paid_media = val.into();
        self
    }
}
impl From<UpdatePurchasedPaidMedia> for crate::types::PaidMediaPurchased {
    fn from(val: UpdatePurchasedPaidMedia) -> Self {
        val.purchased_paid_media
    }
}
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for UpdatePurchasedPaidMedia {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "UpdatePurchasedPaidMedia",
            fields: &["update_id", "purchased_paid_media"],
            specific: &["purchased_paid_media"],
            sequence: "struct UpdatePurchasedPaidMedia with 2 elements",
        };
        let (common, (value_purchased_paid_media,)) = crate::deserialization::update::read_0::<
            D,
            crate::types::PaidMediaPurchased,
        >(deserializer, meta)?;
        Ok(Self {
            update_id: common.update_id,
            purchased_paid_media: value_purchased_paid_media,
        })
    }
}
