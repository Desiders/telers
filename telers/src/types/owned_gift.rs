use serde::{Deserialize, Serialize};
/// This object describes a gift received and owned by a user or a chat. Currently, it can be one of
/// - [`crate::types::OwnedGiftRegular`]
/// - [`crate::types::OwnedGiftUnique`]
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgift>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OwnedGift {
    Regular(crate::types::OwnedGiftRegular),
    Unique(crate::types::OwnedGiftUnique),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::OwnedGiftUnknown),
}
impl OwnedGift {
    /// Helper method for field `can_be_transferred`.
    ///
    /// `true`, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn can_be_transferred(&self) -> Option<bool> {
        match self {
            Self::Unique(val) => val.can_be_transferred,
            _ => None,
        }
    }

    /// Helper method for field `can_be_upgraded`.
    ///
    /// `true`, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn can_be_upgraded(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val.can_be_upgraded,
            _ => None,
        }
    }

    /// Helper method for field `convert_star_count`.
    ///
    /// Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn convert_star_count(&self) -> Option<i64> {
        match self {
            Self::Regular(val) => val.convert_star_count,
            _ => None,
        }
    }

    /// Helper method for field `entities`.
    ///
    /// Special entities that appear in the text
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Regular(val) => val.entities.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `is_private`.
    ///
    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[must_use]
    pub fn is_private(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val.is_private,
            _ => None,
        }
    }

    /// Helper method for field `is_saved`.
    ///
    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_saved(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val.is_saved,
            Self::Unique(val) => val.is_saved,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `is_upgrade_separate`.
    ///
    /// `true`, if the gift's upgrade was purchased after the gift was sent; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_upgrade_separate(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val.is_upgrade_separate,
            _ => None,
        }
    }

    /// Helper method for field `next_transfer_date`.
    ///
    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now.
    #[must_use]
    pub fn next_transfer_date(&self) -> Option<i64> {
        match self {
            Self::Unique(val) => val.next_transfer_date,
            _ => None,
        }
    }

    /// Helper method for field `owned_gift_id`.
    ///
    /// # Variants
    /// - `OwnedGiftRegular`. Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    /// - `OwnedGiftUnique`. Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn owned_gift_id(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val.owned_gift_id.as_deref(),
            Self::Unique(val) => val.owned_gift_id.as_deref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `prepaid_upgrade_star_count`.
    ///
    /// Number of Telegram Stars that were paid for the ability to upgrade the gift
    #[must_use]
    pub fn prepaid_upgrade_star_count(&self) -> Option<i64> {
        match self {
            Self::Regular(val) => val.prepaid_upgrade_star_count,
            _ => None,
        }
    }

    /// Helper method for field `send_date`.
    ///
    /// Date the gift was sent in Unix time
    #[must_use]
    pub fn send_date(&self) -> i64 {
        match self {
            Self::Regular(val) => val.send_date,
            Self::Unique(val) => val.send_date,
            Self::Unknown(val) => val.send_date,
        }
    }

    /// Helper method for field `sender_user`.
    ///
    /// Sender of the gift if it is a known user
    #[must_use]
    pub fn sender_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Regular(val) => val.sender_user.as_deref(),
            Self::Unique(val) => val.sender_user.as_deref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `text`.
    ///
    /// Text of the message that was added to the gift
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val.text.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `transfer_star_count`.
    ///
    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    #[must_use]
    pub fn transfer_star_count(&self) -> Option<i64> {
        match self {
            Self::Unique(val) => val.transfer_star_count,
            _ => None,
        }
    }

    /// Helper method for field `unique_gift_number`.
    ///
    /// Unique number reserved for this gift when upgraded. See the number field in [`crate::types::UniqueGift`].
    #[must_use]
    pub fn unique_gift_number(&self) -> Option<i64> {
        match self {
            Self::Regular(val) => val.unique_gift_number,
            _ => None,
        }
    }

    /// Helper method for field `was_refunded`.
    ///
    /// `true`, if the gift was refunded and isn't available anymore
    #[must_use]
    pub fn was_refunded(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val.was_refunded,
            _ => None,
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.added_to_attachment_menu),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.added_to_attachment_menu),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.allows_users_to_create_topics),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.allows_users_to_create_topics),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_connect_to_business),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_connect_to_business),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_join_groups),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_join_groups),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_manage_bots`.
    #[must_use]
    pub fn can_manage_bots(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_manage_bots),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_manage_bots),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_read_all_group_messages),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.can_read_all_group_messages),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .map(|inner| inner.first_name.as_ref()),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .map(|inner| inner.first_name.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.has_main_web_app),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.has_main_web_app),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.has_topics_enabled),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.has_topics_enabled),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> Option<i64> {
        match self {
            Self::Regular(val) => val.sender_user.as_deref().map(|inner| inner.id),
            Self::Unique(val) => val.sender_user.as_deref().map(|inner| inner.id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val.sender_user.as_deref().map(|inner| inner.is_bot),
            Self::Unique(val) => val.sender_user.as_deref().map(|inner| inner.is_bot),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.is_premium),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.is_premium),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.language_code.as_deref()),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.language_code.as_deref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.last_name.as_deref()),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.last_name.as_deref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `supports_guest_queries`.
    #[must_use]
    pub fn supports_guest_queries(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.supports_guest_queries),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.supports_guest_queries),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.supports_inline_queries),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.supports_inline_queries),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `supports_join_request_queries`.
    #[must_use]
    pub fn supports_join_request_queries(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.supports_join_request_queries),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.supports_join_request_queries),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.username.as_deref()),
            Self::Unique(val) => val
                .sender_user
                .as_deref()
                .and_then(|inner| inner.username.as_deref()),
            Self::Unknown(_) => None,
        }
    }
}
impl From<crate::types::OwnedGiftRegular> for OwnedGift {
    fn from(val: crate::types::OwnedGiftRegular) -> Self {
        Self::Regular(val)
    }
}
impl TryFrom<OwnedGift> for crate::types::OwnedGiftRegular {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: OwnedGift) -> Result<Self, Self::Error> {
        if let OwnedGift::Regular(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(OwnedGift),
                stringify!(OwnedGiftRegular),
            ))
        }
    }
}
impl From<crate::types::OwnedGiftUnique> for OwnedGift {
    fn from(val: crate::types::OwnedGiftUnique) -> Self {
        Self::Unique(val)
    }
}
impl TryFrom<OwnedGift> for crate::types::OwnedGiftUnique {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: OwnedGift) -> Result<Self, Self::Error> {
        if let OwnedGift::Unique(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(OwnedGift),
                stringify!(OwnedGiftUnique),
            ))
        }
    }
}
impl From<crate::types::OwnedGiftUnknown> for OwnedGift {
    fn from(val: crate::types::OwnedGiftUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<OwnedGift> for crate::types::OwnedGiftUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: OwnedGift) -> Result<Self, Self::Error> {
        if let OwnedGift::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(OwnedGift),
                stringify!(OwnedGiftUnknown),
            ))
        }
    }
}
