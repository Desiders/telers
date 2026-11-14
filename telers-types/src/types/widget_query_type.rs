/// This object describes the type of a query from a widget. Currently, it can be one of
/// - [`crate::types::WidgetQueryTypeData`]
/// - [`crate::types::WidgetQueryTypeSignData`]
/// - [`crate::types::WidgetQueryTypeSentTransaction`]
/// # Documentation
/// <https://core.telegram.org/bots/api#widgetquerytype>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(not(feature = "deser"), serde(tag = "type", rename_all = "snake_case"))]
#[cfg_attr(feature = "deser", deser(tag = "type", rename_all = "snake_case"))]
pub enum WidgetQueryType {
    Data(crate::types::WidgetQueryTypeData),
    SignData(crate::types::WidgetQueryTypeSignData),
    SentTransaction(crate::types::WidgetQueryTypeSentTransaction),
    /// Content unknown to this version of the library
    #[cfg_attr(not(feature = "deser"), serde(untagged))]
    #[cfg_attr(feature = "deser", deser(untagged))]
    Unknown(crate::types::WidgetQueryTypeUnknown),
}
impl WidgetQueryType {
    /// Helper method for field `msg_hash`.
    ///
    /// Value of `msg_hash` of the transaction
    #[must_use]
    pub fn msg_hash(&self) -> Option<&str> {
        match self {
            Self::SentTransaction(val) => Some(val.msg_hash.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `sign_data`.
    ///
    /// JSON-serialized signed data
    #[must_use]
    pub fn sign_data(&self) -> Option<&str> {
        match self {
            Self::SignData(val) => Some(val.sign_data.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `wallet`.
    ///
    /// # Variants
    /// - `WidgetQueryTypeSignData`. The wallet that signed the data
    /// - `WidgetQueryTypeSentTransaction`. The wallet from which the transaction was sent
    #[must_use]
    pub fn wallet(&self) -> Option<&crate::types::WidgetQueryWallet> {
        match self {
            Self::SignData(val) => Some(&val.wallet),
            Self::SentTransaction(val) => Some(&val.wallet),
            _ => None,
        }
    }

    /// Helper method for field `wallet_card`.
    ///
    /// TON wallet card of the user who sent the query, if it was sent to the bot along with the query
    #[must_use]
    pub fn wallet_card(&self) -> Option<&crate::types::TonWalletCard> {
        match self {
            Self::Data(val) => val.wallet_card.as_ref(),
            _ => None,
        }
    }

    /// Helper method for nested field `address`.
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::SignData(val) => {
                let inner = &val.wallet;
                Some(inner.address.as_ref())
            }
            Self::SentTransaction(val) => {
                let inner = &val.wallet;
                Some(inner.address.as_ref())
            }
            Self::Data(val) => val.wallet_card.as_ref().map(|inner| inner.address.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `amount`.
    #[must_use]
    pub fn amount(&self) -> Option<&str> {
        match self {
            Self::SignData(val) => {
                let inner = &val.wallet;
                Some(inner.amount.as_ref())
            }
            Self::SentTransaction(val) => {
                let inner = &val.wallet;
                Some(inner.amount.as_ref())
            }
            Self::Data(val) => val.wallet_card.as_ref().map(|inner| inner.amount.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `background_nft_address`.
    #[must_use]
    pub fn background_nft_address(&self) -> Option<&str> {
        match self {
            Self::Data(val) => val
                .wallet_card
                .as_ref()
                .and_then(|inner| inner.background_nft_address.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `public_key`.
    #[must_use]
    pub fn public_key(&self) -> Option<&str> {
        match self {
            Self::SignData(val) => {
                let inner = &val.wallet;
                inner.public_key.as_deref()
            }
            Self::SentTransaction(val) => {
                let inner = &val.wallet;
                inner.public_key.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `state_init`.
    #[must_use]
    pub fn state_init(&self) -> Option<&str> {
        match self {
            Self::SignData(val) => {
                let inner = &val.wallet;
                inner.state_init.as_deref()
            }
            Self::SentTransaction(val) => {
                let inner = &val.wallet;
                inner.state_init.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `stickers`.
    #[must_use]
    pub fn stickers(&self) -> Option<&[crate::types::TonWalletCardSticker]> {
        match self {
            Self::Data(val) => val
                .wallet_card
                .as_ref()
                .map(|inner| inner.stickers.as_ref()),
            _ => None,
        }
    }
}
impl From<crate::types::WidgetQueryTypeData> for WidgetQueryType {
    fn from(val: crate::types::WidgetQueryTypeData) -> Self {
        Self::Data(val)
    }
}
impl TryFrom<WidgetQueryType> for crate::types::WidgetQueryTypeData {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: WidgetQueryType) -> Result<Self, Self::Error> {
        if let WidgetQueryType::Data(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(WidgetQueryType),
                stringify!(WidgetQueryTypeData),
            ))
        }
    }
}
impl From<crate::types::WidgetQueryTypeSignData> for WidgetQueryType {
    fn from(val: crate::types::WidgetQueryTypeSignData) -> Self {
        Self::SignData(val)
    }
}
impl TryFrom<WidgetQueryType> for crate::types::WidgetQueryTypeSignData {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: WidgetQueryType) -> Result<Self, Self::Error> {
        if let WidgetQueryType::SignData(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(WidgetQueryType),
                stringify!(WidgetQueryTypeSignData),
            ))
        }
    }
}
impl From<crate::types::WidgetQueryTypeSentTransaction> for WidgetQueryType {
    fn from(val: crate::types::WidgetQueryTypeSentTransaction) -> Self {
        Self::SentTransaction(val)
    }
}
impl TryFrom<WidgetQueryType> for crate::types::WidgetQueryTypeSentTransaction {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: WidgetQueryType) -> Result<Self, Self::Error> {
        if let WidgetQueryType::SentTransaction(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(WidgetQueryType),
                stringify!(WidgetQueryTypeSentTransaction),
            ))
        }
    }
}
impl From<crate::types::WidgetQueryTypeUnknown> for WidgetQueryType {
    fn from(val: crate::types::WidgetQueryTypeUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<WidgetQueryType> for crate::types::WidgetQueryTypeUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: WidgetQueryType) -> Result<Self, Self::Error> {
        if let WidgetQueryType::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(WidgetQueryType),
                stringify!(WidgetQueryTypeUnknown),
            ))
        }
    }
}
