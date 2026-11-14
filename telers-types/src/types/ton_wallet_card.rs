/// This object describes a TON wallet card, which is shown on the profile page of a user to everyone.
/// # Documentation
/// <https://core.telegram.org/bots/api#tonwalletcard>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct TonWalletCard {
    /// Address of the wallet
    pub address: Box<str>,
    /// Balance of the wallet, in the smallest units of the cryptocurrency, as a decimal number
    pub amount: Box<str>,
    /// Stickers placed on the card
    pub stickers: Box<[crate::types::TonWalletCardSticker]>,
    /// Address of the NFT of one of the stickers, whose colors must be used for the card background; omitted if the default background must be used
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub background_nft_address: Option<Box<str>>,
}
impl TonWalletCard {
    /// Creates a new `TonWalletCard`.
    ///
    /// # Arguments
    /// * `address` - Address of the wallet
    /// * `amount` - Balance of the wallet, in the smallest units of the cryptocurrency, as a decimal number
    /// * `stickers` - Stickers placed on the card
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2Item: Into<crate::types::TonWalletCardSticker>,
        T2: IntoIterator<Item = T2Item>,
    >(
        address: T0,
        amount: T1,
        stickers: T2,
    ) -> Self {
        Self {
            address: address.into(),
            amount: amount.into(),
            stickers: stickers.into_iter().map(Into::into).collect(),
            background_nft_address: None,
        }
    }

    /// Address of the wallet
    #[must_use]
    pub fn address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }

    /// Balance of the wallet, in the smallest units of the cryptocurrency, as a decimal number
    #[must_use]
    pub fn amount<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }

    /// Stickers placed on the card
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn stickers<T: Into<Box<[crate::types::TonWalletCardSticker]>>>(mut self, val: T) -> Self {
        self.stickers = self
            .stickers
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Stickers placed on the card
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn sticker<T: Into<crate::types::TonWalletCardSticker>>(mut self, val: T) -> Self {
        self.stickers = self
            .stickers
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Address of the NFT of one of the stickers, whose colors must be used for the card background; omitted if the default background must be used
    #[must_use]
    pub fn background_nft_address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.background_nft_address = Some(val.into());
        self
    }

    /// Address of the NFT of one of the stickers, whose colors must be used for the card background; omitted if the default background must be used
    #[must_use]
    pub fn background_nft_address_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.background_nft_address = val.map(Into::into);
        self
    }
}
