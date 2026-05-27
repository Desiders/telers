use serde::{Deserialize, Serialize};
/// Represents the rights of a business bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessbotrights>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessBotRights {
    /// `true`, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_reply: Option<bool>,
    /// `true`, if the bot can mark incoming private messages as read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_messages: Option<bool>,
    /// `true`, if the bot can delete messages sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_sent_messages: Option<bool>,
    /// `true`, if the bot can delete all private messages in managed chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_all_messages: Option<bool>,
    /// `true`, if the bot can edit the first and last name of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_name: Option<bool>,
    /// `true`, if the bot can edit the bio of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_bio: Option<bool>,
    /// `true`, if the bot can edit the profile photo of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_profile_photo: Option<bool>,
    /// `true`, if the bot can edit the username of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_username: Option<bool>,
    /// `true`, if the bot can change the privacy settings pertaining to gifts for the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_gift_settings: Option<bool>,
    /// `true`, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_gifts_and_stars: Option<bool>,
    /// `true`, if the bot can convert regular gifts owned by the business account to Telegram Stars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_convert_gifts_to_stars: Option<bool>,
    /// `true`, if the bot can transfer and upgrade gifts owned by the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    /// `true`, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_transfer_stars: Option<bool>,
    /// `true`, if the bot can post, edit and delete stories on behalf of the business account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_stories: Option<bool>,
}
impl BusinessBotRights {
    /// Creates a new `BusinessBotRights`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            can_reply: None,
            can_read_messages: None,
            can_delete_sent_messages: None,
            can_delete_all_messages: None,
            can_edit_name: None,
            can_edit_bio: None,
            can_edit_profile_photo: None,
            can_edit_username: None,
            can_change_gift_settings: None,
            can_view_gifts_and_stars: None,
            can_convert_gifts_to_stars: None,
            can_transfer_and_upgrade_gifts: None,
            can_transfer_stars: None,
            can_manage_stories: None,
        }
    }

    /// `true`, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    #[must_use]
    pub fn can_reply<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_reply = Some(val.into());
        self
    }

    /// `true`, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
    #[must_use]
    pub fn can_reply_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_reply = val.map(Into::into);
        self
    }

    /// `true`, if the bot can mark incoming private messages as read
    #[must_use]
    pub fn can_read_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_read_messages = Some(val.into());
        self
    }

    /// `true`, if the bot can mark incoming private messages as read
    #[must_use]
    pub fn can_read_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_read_messages = val.map(Into::into);
        self
    }

    /// `true`, if the bot can delete messages sent by the bot
    #[must_use]
    pub fn can_delete_sent_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_delete_sent_messages = Some(val.into());
        self
    }

    /// `true`, if the bot can delete messages sent by the bot
    #[must_use]
    pub fn can_delete_sent_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_delete_sent_messages = val.map(Into::into);
        self
    }

    /// `true`, if the bot can delete all private messages in managed chats
    #[must_use]
    pub fn can_delete_all_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_delete_all_messages = Some(val.into());
        self
    }

    /// `true`, if the bot can delete all private messages in managed chats
    #[must_use]
    pub fn can_delete_all_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_delete_all_messages = val.map(Into::into);
        self
    }

    /// `true`, if the bot can edit the first and last name of the business account
    #[must_use]
    pub fn can_edit_name<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_name = Some(val.into());
        self
    }

    /// `true`, if the bot can edit the first and last name of the business account
    #[must_use]
    pub fn can_edit_name_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_edit_name = val.map(Into::into);
        self
    }

    /// `true`, if the bot can edit the bio of the business account
    #[must_use]
    pub fn can_edit_bio<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_bio = Some(val.into());
        self
    }

    /// `true`, if the bot can edit the bio of the business account
    #[must_use]
    pub fn can_edit_bio_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_edit_bio = val.map(Into::into);
        self
    }

    /// `true`, if the bot can edit the profile photo of the business account
    #[must_use]
    pub fn can_edit_profile_photo<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_profile_photo = Some(val.into());
        self
    }

    /// `true`, if the bot can edit the profile photo of the business account
    #[must_use]
    pub fn can_edit_profile_photo_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_edit_profile_photo = val.map(Into::into);
        self
    }

    /// `true`, if the bot can edit the username of the business account
    #[must_use]
    pub fn can_edit_username<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_edit_username = Some(val.into());
        self
    }

    /// `true`, if the bot can edit the username of the business account
    #[must_use]
    pub fn can_edit_username_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_edit_username = val.map(Into::into);
        self
    }

    /// `true`, if the bot can change the privacy settings pertaining to gifts for the business account
    #[must_use]
    pub fn can_change_gift_settings<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_change_gift_settings = Some(val.into());
        self
    }

    /// `true`, if the bot can change the privacy settings pertaining to gifts for the business account
    #[must_use]
    pub fn can_change_gift_settings_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_change_gift_settings = val.map(Into::into);
        self
    }

    /// `true`, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    #[must_use]
    pub fn can_view_gifts_and_stars<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_view_gifts_and_stars = Some(val.into());
        self
    }

    /// `true`, if the bot can view gifts and the amount of Telegram Stars owned by the business account
    #[must_use]
    pub fn can_view_gifts_and_stars_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_view_gifts_and_stars = val.map(Into::into);
        self
    }

    /// `true`, if the bot can convert regular gifts owned by the business account to Telegram Stars
    #[must_use]
    pub fn can_convert_gifts_to_stars<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_convert_gifts_to_stars = Some(val.into());
        self
    }

    /// `true`, if the bot can convert regular gifts owned by the business account to Telegram Stars
    #[must_use]
    pub fn can_convert_gifts_to_stars_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_convert_gifts_to_stars = val.map(Into::into);
        self
    }

    /// `true`, if the bot can transfer and upgrade gifts owned by the business account
    #[must_use]
    pub fn can_transfer_and_upgrade_gifts<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_transfer_and_upgrade_gifts = Some(val.into());
        self
    }

    /// `true`, if the bot can transfer and upgrade gifts owned by the business account
    #[must_use]
    pub fn can_transfer_and_upgrade_gifts_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_transfer_and_upgrade_gifts = val.map(Into::into);
        self
    }

    /// `true`, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    #[must_use]
    pub fn can_transfer_stars<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_transfer_stars = Some(val.into());
        self
    }

    /// `true`, if the bot can transfer Telegram Stars received by the business account to its own account, or use them to upgrade and transfer gifts
    #[must_use]
    pub fn can_transfer_stars_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_transfer_stars = val.map(Into::into);
        self
    }

    /// `true`, if the bot can post, edit and delete stories on behalf of the business account
    #[must_use]
    pub fn can_manage_stories<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_stories = Some(val.into());
        self
    }

    /// `true`, if the bot can post, edit and delete stories on behalf of the business account
    #[must_use]
    pub fn can_manage_stories_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_stories = val.map(Into::into);
        self
    }
}
impl Default for BusinessBotRights {
    fn default() -> Self {
        Self::new()
    }
}
