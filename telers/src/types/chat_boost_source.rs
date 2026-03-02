use serde::{Deserialize, Serialize};
/// This object describes the source of a chat boost. It can be one of
/// - [`ChatBoostSourcePremium`]
/// - [`ChatBoostSourceGiftCode`]
/// - [`ChatBoostSourceGiveaway`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsource>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ChatBoostSource {
    Premium(crate::types::ChatBoostSourcePremium),
    GiftCode(crate::types::ChatBoostSourceGiftCode),
    Giveaway(crate::types::ChatBoostSourceGiveaway),
}
impl ChatBoostSource {
    /// Helper method for field `giveaway_message_id`.
    ///
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    #[must_use]
    pub fn giveaway_message_id(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => Some(val.giveaway_message_id),
            _ => None,
        }
    }

    /// Helper method for field `is_unclaimed`.
    ///
    /// `true`, if the giveaway was completed, but there was no user to win the prize
    #[must_use]
    pub fn is_unclaimed(&self) -> Option<bool> {
        match self {
            Self::Giveaway(val) => val.is_unclaimed,
            _ => None,
        }
    }

    /// Helper method for field `prize_star_count`.
    ///
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => val.prize_star_count,
            _ => None,
        }
    }

    /// Helper method for field `user`.
    ///
    /// # Variants
    /// - `ChatBoostSourcePremium`. User that boosted the chat
    /// - `ChatBoostSourceGiftCode`. User for which the gift code was created
    /// - `ChatBoostSourceGiveaway`. User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[must_use]
    pub fn user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Premium(val) => Some(val.user.as_ref()),
            Self::GiftCode(val) => Some(val.user.as_ref()),
            Self::Giveaway(val) => val.user.as_deref(),
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        self.user().and_then(|inner| inner.added_to_attachment_menu)
    }

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        self.user()
            .and_then(|inner| inner.allows_users_to_create_topics)
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        self.user().and_then(|inner| inner.can_connect_to_business)
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        self.user().and_then(|inner| inner.can_join_groups)
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        self.user()
            .and_then(|inner| inner.can_read_all_group_messages)
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        self.user().map(|inner| inner.first_name.as_ref())
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        self.user().and_then(|inner| inner.has_main_web_app)
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        self.user().and_then(|inner| inner.has_topics_enabled)
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> Option<i64> {
        self.user().map(|inner| inner.id)
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> Option<bool> {
        self.user().map(|inner| inner.is_bot)
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        self.user().and_then(|inner| inner.is_premium)
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        self.user().and_then(|inner| inner.language_code.as_deref())
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        self.user().and_then(|inner| inner.last_name.as_deref())
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        self.user().and_then(|inner| inner.supports_inline_queries)
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        self.user().and_then(|inner| inner.username.as_deref())
    }
}
impl From<crate::types::ChatBoostSourcePremium> for ChatBoostSource {
    fn from(val: crate::types::ChatBoostSourcePremium) -> Self {
        Self::Premium(val)
    }
}
impl TryFrom<ChatBoostSource> for crate::types::ChatBoostSourcePremium {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatBoostSource) -> Result<Self, Self::Error> {
        if let ChatBoostSource::Premium(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatBoostSource),
                stringify!(ChatBoostSourcePremium),
            ))
        }
    }
}
impl From<crate::types::ChatBoostSourceGiftCode> for ChatBoostSource {
    fn from(val: crate::types::ChatBoostSourceGiftCode) -> Self {
        Self::GiftCode(val)
    }
}
impl TryFrom<ChatBoostSource> for crate::types::ChatBoostSourceGiftCode {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatBoostSource) -> Result<Self, Self::Error> {
        if let ChatBoostSource::GiftCode(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatBoostSource),
                stringify!(ChatBoostSourceGiftCode),
            ))
        }
    }
}
impl From<crate::types::ChatBoostSourceGiveaway> for ChatBoostSource {
    fn from(val: crate::types::ChatBoostSourceGiveaway) -> Self {
        Self::Giveaway(val)
    }
}
impl TryFrom<ChatBoostSource> for crate::types::ChatBoostSourceGiveaway {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatBoostSource) -> Result<Self, Self::Error> {
        if let ChatBoostSource::Giveaway(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatBoostSource),
                stringify!(ChatBoostSourceGiveaway),
            ))
        }
    }
}
