use serde::{Deserialize, Serialize};
/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// - [`crate::types::ChatMemberOwner`]
/// - [`crate::types::ChatMemberAdministrator`]
/// - [`crate::types::ChatMemberMember`]
/// - [`crate::types::ChatMemberRestricted`]
/// - [`crate::types::ChatMemberLeft`]
/// - [`crate::types::ChatMemberBanned`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ChatMember {
    Creator(crate::types::ChatMemberOwner),
    Administrator(crate::types::ChatMemberAdministrator),
    Member(crate::types::ChatMemberMember),
    Restricted(crate::types::ChatMemberRestricted),
    Left(crate::types::ChatMemberLeft),
    Kicked(crate::types::ChatMemberBanned),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::ChatMemberUnknown),
}
impl ChatMember {
    /// Helper method for field `can_add_web_page_previews`.
    ///
    /// `true`, if the user is allowed to add web page previews to their messages
    #[must_use]
    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_add_web_page_previews),
            _ => None,
        }
    }

    /// Helper method for field `can_be_edited`.
    ///
    /// `true`, if the bot is allowed to edit administrator privileges of that user
    #[must_use]
    pub fn can_be_edited(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_be_edited),
            _ => None,
        }
    }

    /// Helper method for field `can_change_info`.
    ///
    /// `true`, if the user is allowed to change the chat title, photo and other settings
    #[must_use]
    pub fn can_change_info(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_change_info),
            Self::Restricted(val) => Some(val.can_change_info),
            _ => None,
        }
    }

    /// Helper method for field `can_delete_messages`.
    ///
    /// `true`, if the administrator can delete messages of other users
    #[must_use]
    pub fn can_delete_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_delete_messages),
            _ => None,
        }
    }

    /// Helper method for field `can_delete_stories`.
    ///
    /// `true`, if the administrator can delete stories posted by other users
    #[must_use]
    pub fn can_delete_stories(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_delete_stories),
            _ => None,
        }
    }

    /// Helper method for field `can_edit_messages`.
    ///
    /// `true`, if the administrator can edit messages of other users and can pin messages; for channels only
    #[must_use]
    pub fn can_edit_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => val.can_edit_messages,
            _ => None,
        }
    }

    /// Helper method for field `can_edit_stories`.
    ///
    /// `true`, if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive
    #[must_use]
    pub fn can_edit_stories(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_edit_stories),
            _ => None,
        }
    }

    /// Helper method for field `can_edit_tag`.
    ///
    /// `true`, if the user is allowed to edit their own tag
    #[must_use]
    pub fn can_edit_tag(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_edit_tag),
            _ => None,
        }
    }

    /// Helper method for field `can_invite_users`.
    ///
    /// `true`, if the user is allowed to invite new users to the chat
    #[must_use]
    pub fn can_invite_users(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_invite_users),
            Self::Restricted(val) => Some(val.can_invite_users),
            _ => None,
        }
    }

    /// Helper method for field `can_manage_chat`.
    ///
    /// `true`, if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages, ignore slow mode, and send messages to the chat without paying Telegram Stars. Implied by any other administrator privilege.
    #[must_use]
    pub fn can_manage_chat(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_manage_chat),
            _ => None,
        }
    }

    /// Helper method for field `can_manage_direct_messages`.
    ///
    /// `true`, if the administrator can manage direct messages of the channel and decline suggested posts; for channels only
    #[must_use]
    pub fn can_manage_direct_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => val.can_manage_direct_messages,
            _ => None,
        }
    }

    /// Helper method for field `can_manage_tags`.
    ///
    /// `true`, if the administrator can edit the tags of regular members; for groups and supergroups only
    #[must_use]
    pub fn can_manage_tags(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => val.can_manage_tags,
            _ => None,
        }
    }

    /// Helper method for field `can_manage_topics`.
    ///
    /// # Variants
    /// - `ChatMemberAdministrator`. `true`, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    /// - `ChatMemberRestricted`. `true`, if the user is allowed to create forum topics
    #[must_use]
    pub fn can_manage_topics(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => val.can_manage_topics,
            Self::Restricted(val) => Some(val.can_manage_topics),
            _ => None,
        }
    }

    /// Helper method for field `can_manage_video_chats`.
    ///
    /// `true`, if the administrator can manage video chats
    #[must_use]
    pub fn can_manage_video_chats(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_manage_video_chats),
            _ => None,
        }
    }

    /// Helper method for field `can_pin_messages`.
    ///
    /// # Variants
    /// - `ChatMemberAdministrator`. `true`, if the user is allowed to pin messages; for groups and supergroups only
    /// - `ChatMemberRestricted`. `true`, if the user is allowed to pin messages
    #[must_use]
    pub fn can_pin_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => val.can_pin_messages,
            Self::Restricted(val) => Some(val.can_pin_messages),
            _ => None,
        }
    }

    /// Helper method for field `can_post_messages`.
    ///
    /// `true`, if the administrator can post messages in the channel, approve suggested posts, or access channel statistics; for channels only
    #[must_use]
    pub fn can_post_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => val.can_post_messages,
            _ => None,
        }
    }

    /// Helper method for field `can_post_stories`.
    ///
    /// `true`, if the administrator can post stories to the chat
    #[must_use]
    pub fn can_post_stories(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_post_stories),
            _ => None,
        }
    }

    /// Helper method for field `can_promote_members`.
    ///
    /// `true`, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    #[must_use]
    pub fn can_promote_members(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_promote_members),
            _ => None,
        }
    }

    /// Helper method for field `can_react_to_messages`.
    ///
    /// `true`, if the user is allowed to react to messages
    #[must_use]
    pub fn can_react_to_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_react_to_messages),
            _ => None,
        }
    }

    /// Helper method for field `can_restrict_members`.
    ///
    /// `true`, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    #[must_use]
    pub fn can_restrict_members(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_restrict_members),
            _ => None,
        }
    }

    /// Helper method for field `can_send_audios`.
    ///
    /// `true`, if the user is allowed to send audios
    #[must_use]
    pub fn can_send_audios(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_audios),
            _ => None,
        }
    }

    /// Helper method for field `can_send_documents`.
    ///
    /// `true`, if the user is allowed to send documents
    #[must_use]
    pub fn can_send_documents(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_documents),
            _ => None,
        }
    }

    /// Helper method for field `can_send_messages`.
    ///
    /// `true`, if the user is allowed to send text messages, rich messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[must_use]
    pub fn can_send_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_messages),
            _ => None,
        }
    }

    /// Helper method for field `can_send_other_messages`.
    ///
    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    #[must_use]
    pub fn can_send_other_messages(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_other_messages),
            _ => None,
        }
    }

    /// Helper method for field `can_send_photos`.
    ///
    /// `true`, if the user is allowed to send photos
    #[must_use]
    pub fn can_send_photos(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_photos),
            _ => None,
        }
    }

    /// Helper method for field `can_send_polls`.
    ///
    /// `true`, if the user is allowed to send polls and checklists
    #[must_use]
    pub fn can_send_polls(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_polls),
            _ => None,
        }
    }

    /// Helper method for field `can_send_video_notes`.
    ///
    /// `true`, if the user is allowed to send video notes
    #[must_use]
    pub fn can_send_video_notes(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_video_notes),
            _ => None,
        }
    }

    /// Helper method for field `can_send_videos`.
    ///
    /// `true`, if the user is allowed to send videos
    #[must_use]
    pub fn can_send_videos(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_videos),
            _ => None,
        }
    }

    /// Helper method for field `can_send_voice_notes`.
    ///
    /// `true`, if the user is allowed to send voice notes
    #[must_use]
    pub fn can_send_voice_notes(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.can_send_voice_notes),
            _ => None,
        }
    }

    /// Helper method for field `can_send_welcome_messages`.
    ///
    /// `true`, if the administrator can manage chat welcome messages or directly send them in the case of bots
    #[must_use]
    pub fn can_send_welcome_messages(&self) -> Option<bool> {
        match self {
            Self::Administrator(val) => Some(val.can_send_welcome_messages),
            _ => None,
        }
    }

    /// Helper method for field `custom_title`.
    ///
    /// Custom title for this user
    #[must_use]
    pub fn custom_title(&self) -> Option<&str> {
        match self {
            Self::Creator(val) => val.custom_title.as_deref(),
            Self::Administrator(val) => val.custom_title.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `is_anonymous`.
    ///
    /// `true`, if the user's presence in the chat is hidden
    #[must_use]
    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            Self::Creator(val) => Some(val.is_anonymous),
            Self::Administrator(val) => Some(val.is_anonymous),
            _ => None,
        }
    }

    /// Helper method for field `is_member`.
    ///
    /// `true`, if the user is a member of the chat at the moment of the request
    #[must_use]
    pub fn is_member(&self) -> Option<bool> {
        match self {
            Self::Restricted(val) => Some(val.is_member),
            _ => None,
        }
    }

    /// Helper method for field `tag`.
    ///
    /// Tag of the member
    #[must_use]
    pub fn tag(&self) -> Option<&str> {
        match self {
            Self::Member(val) => val.tag.as_deref(),
            Self::Restricted(val) => val.tag.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `until_date`.
    ///
    /// # Variants
    /// - `ChatMemberMember`. Date when the user's subscription will expire; Unix time
    /// - `ChatMemberRestricted`. Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever.
    /// - `ChatMemberBanned`. Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever.
    #[must_use]
    pub fn until_date(&self) -> Option<i64> {
        match self {
            Self::Member(val) => val.until_date,
            Self::Restricted(val) => Some(val.until_date),
            Self::Kicked(val) => Some(val.until_date),
            _ => None,
        }
    }

    /// Helper method for field `user`.
    ///
    /// Information about the user
    #[must_use]
    pub fn user(&self) -> &crate::types::User {
        match self {
            Self::Creator(val) => val.user.as_ref(),
            Self::Administrator(val) => val.user.as_ref(),
            Self::Member(val) => val.user.as_ref(),
            Self::Restricted(val) => val.user.as_ref(),
            Self::Left(val) => val.user.as_ref(),
            Self::Kicked(val) => val.user.as_ref(),
            Self::Unknown(val) => val.user.as_ref(),
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.added_to_attachment_menu
        }
    }

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.allows_users_to_create_topics
        }
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.can_connect_to_business
        }
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.can_join_groups
        }
    }

    /// Helper method for nested field `can_manage_bots`.
    #[must_use]
    pub fn can_manage_bots(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.can_manage_bots
        }
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.can_read_all_group_messages
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        {
            let inner = self.user();
            inner.first_name.as_ref()
        }
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.has_main_web_app
        }
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.has_topics_enabled
        }
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> i64 {
        {
            let inner = self.user();
            inner.id
        }
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> bool {
        {
            let inner = self.user();
            inner.is_bot
        }
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.is_premium
        }
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        {
            let inner = self.user();
            inner.language_code.as_deref()
        }
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        {
            let inner = self.user();
            inner.last_name.as_deref()
        }
    }

    /// Helper method for nested field `supports_guest_queries`.
    #[must_use]
    pub fn supports_guest_queries(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.supports_guest_queries
        }
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.supports_inline_queries
        }
    }

    /// Helper method for nested field `supports_join_request_queries`.
    #[must_use]
    pub fn supports_join_request_queries(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.supports_join_request_queries
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        {
            let inner = self.user();
            inner.username.as_deref()
        }
    }
}
impl From<crate::types::ChatMemberOwner> for ChatMember {
    fn from(val: crate::types::ChatMemberOwner) -> Self {
        Self::Creator(val)
    }
}
impl TryFrom<ChatMember> for crate::types::ChatMemberOwner {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatMember) -> Result<Self, Self::Error> {
        if let ChatMember::Creator(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatMember),
                stringify!(ChatMemberOwner),
            ))
        }
    }
}
impl From<crate::types::ChatMemberAdministrator> for ChatMember {
    fn from(val: crate::types::ChatMemberAdministrator) -> Self {
        Self::Administrator(val)
    }
}
impl TryFrom<ChatMember> for crate::types::ChatMemberAdministrator {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatMember) -> Result<Self, Self::Error> {
        if let ChatMember::Administrator(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatMember),
                stringify!(ChatMemberAdministrator),
            ))
        }
    }
}
impl From<crate::types::ChatMemberMember> for ChatMember {
    fn from(val: crate::types::ChatMemberMember) -> Self {
        Self::Member(val)
    }
}
impl TryFrom<ChatMember> for crate::types::ChatMemberMember {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatMember) -> Result<Self, Self::Error> {
        if let ChatMember::Member(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatMember),
                stringify!(ChatMemberMember),
            ))
        }
    }
}
impl From<crate::types::ChatMemberRestricted> for ChatMember {
    fn from(val: crate::types::ChatMemberRestricted) -> Self {
        Self::Restricted(val)
    }
}
impl TryFrom<ChatMember> for crate::types::ChatMemberRestricted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatMember) -> Result<Self, Self::Error> {
        if let ChatMember::Restricted(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatMember),
                stringify!(ChatMemberRestricted),
            ))
        }
    }
}
impl From<crate::types::ChatMemberLeft> for ChatMember {
    fn from(val: crate::types::ChatMemberLeft) -> Self {
        Self::Left(val)
    }
}
impl TryFrom<ChatMember> for crate::types::ChatMemberLeft {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatMember) -> Result<Self, Self::Error> {
        if let ChatMember::Left(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatMember),
                stringify!(ChatMemberLeft),
            ))
        }
    }
}
impl From<crate::types::ChatMemberBanned> for ChatMember {
    fn from(val: crate::types::ChatMemberBanned) -> Self {
        Self::Kicked(val)
    }
}
impl TryFrom<ChatMember> for crate::types::ChatMemberBanned {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatMember) -> Result<Self, Self::Error> {
        if let ChatMember::Kicked(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatMember),
                stringify!(ChatMemberBanned),
            ))
        }
    }
}
impl From<crate::types::ChatMemberUnknown> for ChatMember {
    fn from(val: crate::types::ChatMemberUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<ChatMember> for crate::types::ChatMemberUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatMember) -> Result<Self, Self::Error> {
        if let ChatMember::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatMember),
                stringify!(ChatMemberUnknown),
            ))
        }
    }
}
