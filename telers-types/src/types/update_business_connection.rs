/// The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
/// # Notes
/// This object represents an update from original update field `business_connection`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct UpdateBusinessConnection {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    pub business_connection: crate::types::BusinessConnection,
}
impl UpdateBusinessConnection {
    /// Creates a new `UpdateBusinessConnection`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `business_connection` - The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::BusinessConnection>>(
        update_id: T0,
        business_connection: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            business_connection: business_connection.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    #[must_use]
    pub fn business_connection<T: Into<crate::types::BusinessConnection>>(
        mut self,
        val: T,
    ) -> Self {
        self.business_connection = val.into();
        self
    }
}
impl From<UpdateBusinessConnection> for crate::types::BusinessConnection {
    fn from(val: UpdateBusinessConnection) -> Self {
        val.business_connection
    }
}
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for UpdateBusinessConnection {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "UpdateBusinessConnection",
            fields: &["update_id", "business_connection"],
            specific: &["business_connection"],
            sequence: "struct UpdateBusinessConnection with 2 elements",
        };
        let (common, (value_business_connection,)) = crate::deserialization::update::read_0::<
            D,
            crate::types::BusinessConnection,
        >(deserializer, meta)?;
        Ok(Self {
            update_id: common.update_id,
            business_connection: value_business_connection,
        })
    }
}
