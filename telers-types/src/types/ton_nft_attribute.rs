/// This object represents an attribute of an NFT in the TON blockchain.
/// # Documentation
/// <https://core.telegram.org/bots/api#tonnftattribute>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct TonNftAttribute {
    /// Trait type of the attribute
    pub trait_type: Box<str>,
    /// Value of the attribute
    pub value: Box<str>,
}
impl TonNftAttribute {
    /// Creates a new `TonNftAttribute`.
    ///
    /// # Arguments
    /// * `trait_type` - Trait type of the attribute
    /// * `value` - Value of the attribute
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(trait_type: T0, value: T1) -> Self {
        Self {
            trait_type: trait_type.into(),
            value: value.into(),
        }
    }

    /// Trait type of the attribute
    #[must_use]
    pub fn trait_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.trait_type = val.into();
        self
    }

    /// Value of the attribute
    #[must_use]
    pub fn value<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.value = val.into();
        self
    }
}
