use serde::{Deserialize, Serialize};
/// This object describes a unique gift that was upgraded from a regular gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegift>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniqueGift {
    /// Identifier of the regular gift from which the gift was upgraded
    pub gift_id: Box<str>,
    /// Human-readable name of the regular gift from which this unique gift was upgraded
    pub base_name: Box<str>,
    /// Unique name of the gift. This name can be used in <https://t.me/nft/>... links and story areas
    pub name: Box<str>,
    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift
    pub number: i64,
    /// Model of the gift
    pub model: crate::types::UniqueGiftModel,
    /// Symbol of the gift
    pub symbol: crate::types::UniqueGiftSymbol,
    /// Backdrop of the gift
    pub backdrop: crate::types::UniqueGiftBackdrop,
    /// `true`, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// `true`, if the gift was used to craft another gift and isn't available anymore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_burned: Option<bool>,
    /// `true`, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_blockchain: Option<bool>,
    /// The color scheme that can be used by the gift's owner for the chat's name, replies to messages and link previews; for business account gifts and gifts that are currently on sale only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<crate::types::UniqueGiftColors>,
    /// Information about the chat that published the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Box<crate::types::Chat>>,
}
impl UniqueGift {
    /// Creates a new `UniqueGift`.
    ///
    /// # Arguments
    /// * `gift_id` - Identifier of the regular gift from which the gift was upgraded
    /// * `base_name` - Human-readable name of the regular gift from which this unique gift was upgraded
    /// * `name` - Unique name of the gift. This name can be used in <https://t.me/nft/>... links and story areas
    /// * `number` - Unique number of the upgraded gift among gifts upgraded from the same regular gift
    /// * `model` - Model of the gift
    /// * `symbol` - Symbol of the gift
    /// * `backdrop` - Backdrop of the gift
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<i64>,
        T4: Into<crate::types::UniqueGiftModel>,
        T5: Into<crate::types::UniqueGiftSymbol>,
        T6: Into<crate::types::UniqueGiftBackdrop>,
    >(
        gift_id: T0,
        base_name: T1,
        name: T2,
        number: T3,
        model: T4,
        symbol: T5,
        backdrop: T6,
    ) -> Self {
        Self {
            gift_id: gift_id.into(),
            base_name: base_name.into(),
            name: name.into(),
            number: number.into(),
            model: model.into(),
            symbol: symbol.into(),
            backdrop: backdrop.into(),
            is_premium: None,
            is_burned: None,
            is_from_blockchain: None,
            colors: None,
            publisher_chat: None,
        }
    }

    /// Identifier of the regular gift from which the gift was upgraded
    #[must_use]
    pub fn gift_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.gift_id = val.into();
        this
    }

    /// Human-readable name of the regular gift from which this unique gift was upgraded
    #[must_use]
    pub fn base_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.base_name = val.into();
        this
    }

    /// Unique name of the gift. This name can be used in <https://t.me/nft/>... links and story areas
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }

    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift
    #[must_use]
    pub fn number<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.number = val.into();
        this
    }

    /// Model of the gift
    #[must_use]
    pub fn model<T: Into<crate::types::UniqueGiftModel>>(self, val: T) -> Self {
        let mut this = self;
        this.model = val.into();
        this
    }

    /// Symbol of the gift
    #[must_use]
    pub fn symbol<T: Into<crate::types::UniqueGiftSymbol>>(self, val: T) -> Self {
        let mut this = self;
        this.symbol = val.into();
        this
    }

    /// Backdrop of the gift
    #[must_use]
    pub fn backdrop<T: Into<crate::types::UniqueGiftBackdrop>>(self, val: T) -> Self {
        let mut this = self;
        this.backdrop = val.into();
        this
    }

    /// `true`, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
    #[must_use]
    pub fn is_premium<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_premium = Some(val.into());
        this
    }

    /// `true`, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
    #[must_use]
    pub fn is_premium_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_premium = val.map(Into::into);
        this
    }

    /// `true`, if the gift was used to craft another gift and isn't available anymore
    #[must_use]
    pub fn is_burned<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_burned = Some(val.into());
        this
    }

    /// `true`, if the gift was used to craft another gift and isn't available anymore
    #[must_use]
    pub fn is_burned_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_burned = val.map(Into::into);
        this
    }

    /// `true`, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[must_use]
    pub fn is_from_blockchain<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_from_blockchain = Some(val.into());
        this
    }

    /// `true`, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[must_use]
    pub fn is_from_blockchain_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_from_blockchain = val.map(Into::into);
        this
    }

    /// The color scheme that can be used by the gift's owner for the chat's name, replies to messages and link previews; for business account gifts and gifts that are currently on sale only
    #[must_use]
    pub fn colors<T: Into<crate::types::UniqueGiftColors>>(self, val: T) -> Self {
        let mut this = self;
        this.colors = Some(val.into());
        this
    }

    /// The color scheme that can be used by the gift's owner for the chat's name, replies to messages and link previews; for business account gifts and gifts that are currently on sale only
    #[must_use]
    pub fn colors_option<T: Into<crate::types::UniqueGiftColors>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.colors = val.map(Into::into);
        this
    }

    /// Information about the chat that published the gift
    #[must_use]
    pub fn publisher_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.publisher_chat = Some(Box::new(val.into()));
        this
    }

    /// Information about the chat that published the gift
    #[must_use]
    pub fn publisher_chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.publisher_chat = val.map(|val| Box::new(val.into()));
        this
    }
}
