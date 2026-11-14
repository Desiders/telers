/// This object represents an NFT in the TON blockchain.
/// # Documentation
/// <https://core.telegram.org/bots/api#tonnft>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct TonNft {
    /// Address of the NFT
    pub address: Box<str>,
    /// Address of the collection to which the NFT belongs; omitted if unknown
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub collection_address: Option<Box<str>>,
    /// Index of the NFT among all NFTs from the collection as a decimal number
    pub index: Box<str>,
    /// Name of the NFT
    pub name: Box<str>,
    /// Description of the NFT
    pub description: Box<str>,
    /// Attributes of the NFT
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub attributes: Option<Box<[crate::types::TonNftAttribute]>>,
    /// The unique gift corresponding to the NFT
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub gift: Option<Box<crate::types::UniqueGift>>,
}
impl TonNft {
    /// Creates a new `TonNft`.
    ///
    /// # Arguments
    /// * `address` - Address of the NFT
    /// * `index` - Index of the NFT among all NFTs from the collection as a decimal number
    /// * `name` - Name of the NFT
    /// * `description` - Description of the NFT
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        address: T0,
        index: T1,
        name: T2,
        description: T3,
    ) -> Self {
        Self {
            address: address.into(),
            collection_address: None,
            index: index.into(),
            name: name.into(),
            description: description.into(),
            attributes: None,
            gift: None,
        }
    }

    /// Address of the NFT
    #[must_use]
    pub fn address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }

    /// Address of the collection to which the NFT belongs; omitted if unknown
    #[must_use]
    pub fn collection_address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.collection_address = Some(val.into());
        self
    }

    /// Address of the collection to which the NFT belongs; omitted if unknown
    #[must_use]
    pub fn collection_address_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.collection_address = val.map(Into::into);
        self
    }

    /// Index of the NFT among all NFTs from the collection as a decimal number
    #[must_use]
    pub fn index<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.index = val.into();
        self
    }

    /// Name of the NFT
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    /// Description of the NFT
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = val.into();
        self
    }

    /// Attributes of the NFT
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn attributes<T: Into<Box<[crate::types::TonNftAttribute]>>>(mut self, val: T) -> Self {
        self.attributes = Some(
            self.attributes
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Attributes of the NFT
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn attribute<T: Into<crate::types::TonNftAttribute>>(mut self, val: T) -> Self {
        self.attributes = Some(
            self.attributes
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Attributes of the NFT
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn attributes_option<T: Into<Box<[crate::types::TonNftAttribute]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.attributes = val.map(Into::into);
        self
    }

    /// The unique gift corresponding to the NFT
    #[must_use]
    pub fn gift<T: Into<crate::types::UniqueGift>>(mut self, val: T) -> Self {
        self.gift = Some(Box::new(val.into()));
        self
    }

    /// The unique gift corresponding to the NFT
    #[must_use]
    pub fn gift_option<T: Into<crate::types::UniqueGift>>(mut self, val: Option<T>) -> Self {
        self.gift = val.map(|val| Box::new(val.into()));
        self
    }
}
