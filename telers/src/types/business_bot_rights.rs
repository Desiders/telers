use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the rights of a business bot
/// # Documentation
/// <https://core.telegram.org/bots/api#businessbotrights>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BusinessBotRights {
    /// `true`, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    pub can_reply: Option<bool>,
    /// `true`, if the bot can mark incoming private messages as read
    pub can_read_messages: Option<bool>,
    /// `true`, if the bot can delete messages sent by the bot
    pub can_delete_sent_messages: Option<bool>,
    /// `true`, if the bot can delete all private messages in managed chats
    pub can_delete_all_messages: Option<bool>,
    /// `true`, if the bot can edit the first and last name of the business account
    pub can_edit_name: Option<bool>,
    /// `true`, if the bot can edit the bio of the business account
    pub can_edit_bio: Option<bool>,
    /// `true`, if the bot can edit the profile photo of the business account
    pub can_edit_profile_photo: Option<bool>,
    /// `true`, if the bot can edit the username of the business account
    pub can_edit_username: Option<bool>,
    /// `true`, if the bot can change the privacy settings pertaining to gifts for the business account
    pub can_change_gift_settings: Option<bool>,
    /// `true`, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    pub can_view_gifts_and_stars: Option<bool>,
    /// `true`, if the bot can convert regular gifts owned by the business account to Telegram Stars
    pub can_convert_gifts_to_stars: Option<bool>,
    /// `true`, if the bot can transfer and upgrade gifts owned by the business account
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    /// `true`, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    pub can_transfer_stars: Option<bool>,
    /// `true`, if the bot can post, edit and delete stories on behalf of the business account
    pub can_manage_stories: Option<bool>,
}
