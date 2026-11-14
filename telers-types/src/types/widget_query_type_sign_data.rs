/// The query contains data signed by a TON wallet of the user who sent the query. The data is signed for the domain bot<`bot_id`>, where <`bot_id`> is the identifier of the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#widgetquerytypesigndata>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct WidgetQueryTypeSignData {
    /// The wallet that signed the data
    pub wallet: crate::types::WidgetQueryWallet,
    /// JSON-serialized signed data
    pub sign_data: Box<str>,
}
impl WidgetQueryTypeSignData {
    /// Creates a new `WidgetQueryTypeSignData`.
    ///
    /// # Arguments
    /// * `wallet` - The wallet that signed the data
    /// * `sign_data` - JSON-serialized signed data
    #[must_use]
    pub fn new<T0: Into<crate::types::WidgetQueryWallet>, T1: Into<Box<str>>>(
        wallet: T0,
        sign_data: T1,
    ) -> Self {
        Self {
            wallet: wallet.into(),
            sign_data: sign_data.into(),
        }
    }

    /// The wallet that signed the data
    #[must_use]
    pub fn wallet<T: Into<crate::types::WidgetQueryWallet>>(mut self, val: T) -> Self {
        self.wallet = val.into();
        self
    }

    /// JSON-serialized signed data
    #[must_use]
    pub fn sign_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sign_data = val.into();
        self
    }
}
