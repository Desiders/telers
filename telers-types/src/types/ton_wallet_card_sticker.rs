/// This object describes a sticker on a TON wallet card.
/// # Documentation
/// <https://core.telegram.org/bots/api#tonwalletcardsticker>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct TonWalletCardSticker {
    /// Position of the sticker on the card
    pub position: crate::types::AreaPosition,
    /// The NFT shown as the sticker
    pub nft: crate::types::TonNft,
}
impl TonWalletCardSticker {
    /// Creates a new `TonWalletCardSticker`.
    ///
    /// # Arguments
    /// * `position` - Position of the sticker on the card
    /// * `nft` - The NFT shown as the sticker
    #[must_use]
    pub fn new<T0: Into<crate::types::AreaPosition>, T1: Into<crate::types::TonNft>>(
        position: T0,
        nft: T1,
    ) -> Self {
        Self {
            position: position.into(),
            nft: nft.into(),
        }
    }

    /// Position of the sticker on the card
    #[must_use]
    pub fn position<T: Into<crate::types::AreaPosition>>(mut self, val: T) -> Self {
        self.position = val.into();
        self
    }

    /// The NFT shown as the sticker
    #[must_use]
    pub fn nft<T: Into<crate::types::TonNft>>(mut self, val: T) -> Self {
        self.nft = val.into();
        self
    }
}
