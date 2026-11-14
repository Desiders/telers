/// The query contains data for the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#widgetquerytypedata>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct WidgetQueryTypeData {
    /// TON wallet card of the user who sent the query, if it was sent to the bot along with the query
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub wallet_card: Option<crate::types::TonWalletCard>,
}
impl WidgetQueryTypeData {
    /// Creates a new `WidgetQueryTypeData`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            wallet_card: None,
        }
    }

    /// TON wallet card of the user who sent the query, if it was sent to the bot along with the query
    #[must_use]
    pub fn wallet_card<T: Into<crate::types::TonWalletCard>>(mut self, val: T) -> Self {
        self.wallet_card = Some(val.into());
        self
    }

    /// TON wallet card of the user who sent the query, if it was sent to the bot along with the query
    #[must_use]
    pub fn wallet_card_option<T: Into<crate::types::TonWalletCard>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.wallet_card = val.map(Into::into);
        self
    }
}
impl Default for WidgetQueryTypeData {
    fn default() -> Self {
        Self::new()
    }
}
