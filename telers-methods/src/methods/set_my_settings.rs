/// Use this method to change the settings of the bot. Settings that aren't specified remain unchanged. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmysettings>
/// # Returns
/// - `bool`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct SetMySettings {
    /// Pass `true` to enable inline mode for the bot or `false` to disable it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub supports_inline_queries: Option<bool>,
    /// New placeholder for inline queries, which is shown in the input field after the bot's username. Pass an empty string to use the default placeholder.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub inline_query_placeholder: Option<Box<str>>,
    /// Pass `true` to request the location of the user with every inline query to the bot or `false` to stop requesting it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub needs_inline_query_location: Option<bool>,
    /// Denominator of the share of chosen inline results received by the bot; the bot receives 1 out of `inline_feedback_share_denominator` chosen results. Must be one of 1, 10, 100, or 1000. Pass 0 to stop receiving chosen inline results.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub inline_feedback_share_denominator: Option<i64>,
    /// Pass `true` to allow adding the bot to groups or `false` to disallow it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub can_join_groups: Option<bool>,
    /// Pass `true` to disable privacy mode for the bot or `false` to enable it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub can_read_all_group_messages: Option<bool>,
    /// Pass `true` to allow connecting the bot to user accounts to manage them or `false` to disallow it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub can_connect_to_business: Option<bool>,
    /// Pass `true` to allow creating other bots that will be controlled by the bot or `false` to disallow it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub can_manage_bots: Option<bool>,
    /// Pass `true` to allow guest queries to the bot from chats it isn't a member of or `false` to disallow them
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub supports_guest_queries: Option<bool>,
    /// Pass `true` to allow assigning the bot to process join request queries or `false` to disallow it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub supports_join_request_queries: Option<bool>,
    /// Pass `true` to enable forum topic mode in private chats with the bot or `false` to disable it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub has_topics_enabled: Option<bool>,
    /// Pass `true` to allow users to create and delete topics in private chats with the bot or `false` to disallow it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub allows_users_to_create_topics: Option<bool>,
    /// Pass `true` to show topics in private chats with the bot in a sidebar or `false` to stop showing them in a sidebar
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub has_topic_sidebar: Option<bool>,
    /// Pass `true` to allow the bot to communicate with other bots in groups, private chats, and through business accounts or `false` to disallow it
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub can_communicate_with_bots: Option<bool>,
    /// Mode in which the bot receives callback queries from buttons in messages sent by the bot. Must be one of `default` (the field message is always used and contains an [`crate::types::InaccessibleMessage`] if the message is inaccessible to the bot), `auto` (the field `inline_message_id` is used instead of the field message if the message is inaccessible to the bot; the field message can still contain an inaccessible message if the message has just been deleted), or `inline` (the field `inline_message_id` is always used instead of the field message).
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub callback_query_mode: Option<Box<str>>,
}
impl SetMySettings {
    /// Creates a new `SetMySettings`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            supports_inline_queries: None,
            inline_query_placeholder: None,
            needs_inline_query_location: None,
            inline_feedback_share_denominator: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            can_connect_to_business: None,
            can_manage_bots: None,
            supports_guest_queries: None,
            supports_join_request_queries: None,
            has_topics_enabled: None,
            allows_users_to_create_topics: None,
            has_topic_sidebar: None,
            can_communicate_with_bots: None,
            callback_query_mode: None,
        }
    }

    /// Pass `true` to enable inline mode for the bot or `false` to disable it
    #[must_use]
    pub fn supports_inline_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_inline_queries = Some(val.into());
        self
    }

    /// Pass `true` to enable inline mode for the bot or `false` to disable it
    #[must_use]
    pub fn supports_inline_queries_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.supports_inline_queries = val.map(Into::into);
        self
    }

    /// New placeholder for inline queries, which is shown in the input field after the bot's username. Pass an empty string to use the default placeholder.
    #[must_use]
    pub fn inline_query_placeholder<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_query_placeholder = Some(val.into());
        self
    }

    /// New placeholder for inline queries, which is shown in the input field after the bot's username. Pass an empty string to use the default placeholder.
    #[must_use]
    pub fn inline_query_placeholder_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_query_placeholder = val.map(Into::into);
        self
    }

    /// Pass `true` to request the location of the user with every inline query to the bot or `false` to stop requesting it
    #[must_use]
    pub fn needs_inline_query_location<T: Into<bool>>(mut self, val: T) -> Self {
        self.needs_inline_query_location = Some(val.into());
        self
    }

    /// Pass `true` to request the location of the user with every inline query to the bot or `false` to stop requesting it
    #[must_use]
    pub fn needs_inline_query_location_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.needs_inline_query_location = val.map(Into::into);
        self
    }

    /// Denominator of the share of chosen inline results received by the bot; the bot receives 1 out of `inline_feedback_share_denominator` chosen results. Must be one of 1, 10, 100, or 1000. Pass 0 to stop receiving chosen inline results.
    #[must_use]
    pub fn inline_feedback_share_denominator<T: Into<i64>>(mut self, val: T) -> Self {
        self.inline_feedback_share_denominator = Some(val.into());
        self
    }

    /// Denominator of the share of chosen inline results received by the bot; the bot receives 1 out of `inline_feedback_share_denominator` chosen results. Must be one of 1, 10, 100, or 1000. Pass 0 to stop receiving chosen inline results.
    #[must_use]
    pub fn inline_feedback_share_denominator_option<T: Into<i64>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.inline_feedback_share_denominator = val.map(Into::into);
        self
    }

    /// Pass `true` to allow adding the bot to groups or `false` to disallow it
    #[must_use]
    pub fn can_join_groups<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_join_groups = Some(val.into());
        self
    }

    /// Pass `true` to allow adding the bot to groups or `false` to disallow it
    #[must_use]
    pub fn can_join_groups_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_join_groups = val.map(Into::into);
        self
    }

    /// Pass `true` to disable privacy mode for the bot or `false` to enable it
    #[must_use]
    pub fn can_read_all_group_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_read_all_group_messages = Some(val.into());
        self
    }

    /// Pass `true` to disable privacy mode for the bot or `false` to enable it
    #[must_use]
    pub fn can_read_all_group_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_read_all_group_messages = val.map(Into::into);
        self
    }

    /// Pass `true` to allow connecting the bot to user accounts to manage them or `false` to disallow it
    #[must_use]
    pub fn can_connect_to_business<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_connect_to_business = Some(val.into());
        self
    }

    /// Pass `true` to allow connecting the bot to user accounts to manage them or `false` to disallow it
    #[must_use]
    pub fn can_connect_to_business_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_connect_to_business = val.map(Into::into);
        self
    }

    /// Pass `true` to allow creating other bots that will be controlled by the bot or `false` to disallow it
    #[must_use]
    pub fn can_manage_bots<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_bots = Some(val.into());
        self
    }

    /// Pass `true` to allow creating other bots that will be controlled by the bot or `false` to disallow it
    #[must_use]
    pub fn can_manage_bots_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_bots = val.map(Into::into);
        self
    }

    /// Pass `true` to allow guest queries to the bot from chats it isn't a member of or `false` to disallow them
    #[must_use]
    pub fn supports_guest_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_guest_queries = Some(val.into());
        self
    }

    /// Pass `true` to allow guest queries to the bot from chats it isn't a member of or `false` to disallow them
    #[must_use]
    pub fn supports_guest_queries_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.supports_guest_queries = val.map(Into::into);
        self
    }

    /// Pass `true` to allow assigning the bot to process join request queries or `false` to disallow it
    #[must_use]
    pub fn supports_join_request_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_join_request_queries = Some(val.into());
        self
    }

    /// Pass `true` to allow assigning the bot to process join request queries or `false` to disallow it
    #[must_use]
    pub fn supports_join_request_queries_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.supports_join_request_queries = val.map(Into::into);
        self
    }

    /// Pass `true` to enable forum topic mode in private chats with the bot or `false` to disable it
    #[must_use]
    pub fn has_topics_enabled<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_topics_enabled = Some(val.into());
        self
    }

    /// Pass `true` to enable forum topic mode in private chats with the bot or `false` to disable it
    #[must_use]
    pub fn has_topics_enabled_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_topics_enabled = val.map(Into::into);
        self
    }

    /// Pass `true` to allow users to create and delete topics in private chats with the bot or `false` to disallow it
    #[must_use]
    pub fn allows_users_to_create_topics<T: Into<bool>>(mut self, val: T) -> Self {
        self.allows_users_to_create_topics = Some(val.into());
        self
    }

    /// Pass `true` to allow users to create and delete topics in private chats with the bot or `false` to disallow it
    #[must_use]
    pub fn allows_users_to_create_topics_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allows_users_to_create_topics = val.map(Into::into);
        self
    }

    /// Pass `true` to show topics in private chats with the bot in a sidebar or `false` to stop showing them in a sidebar
    #[must_use]
    pub fn has_topic_sidebar<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_topic_sidebar = Some(val.into());
        self
    }

    /// Pass `true` to show topics in private chats with the bot in a sidebar or `false` to stop showing them in a sidebar
    #[must_use]
    pub fn has_topic_sidebar_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_topic_sidebar = val.map(Into::into);
        self
    }

    /// Pass `true` to allow the bot to communicate with other bots in groups, private chats, and through business accounts or `false` to disallow it
    #[must_use]
    pub fn can_communicate_with_bots<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_communicate_with_bots = Some(val.into());
        self
    }

    /// Pass `true` to allow the bot to communicate with other bots in groups, private chats, and through business accounts or `false` to disallow it
    #[must_use]
    pub fn can_communicate_with_bots_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_communicate_with_bots = val.map(Into::into);
        self
    }

    /// Mode in which the bot receives callback queries from buttons in messages sent by the bot. Must be one of `default` (the field message is always used and contains an [`crate::types::InaccessibleMessage`] if the message is inaccessible to the bot), `auto` (the field `inline_message_id` is used instead of the field message if the message is inaccessible to the bot; the field message can still contain an inaccessible message if the message has just been deleted), or `inline` (the field `inline_message_id` is always used instead of the field message).
    #[must_use]
    pub fn callback_query_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.callback_query_mode = Some(val.into());
        self
    }

    /// Mode in which the bot receives callback queries from buttons in messages sent by the bot. Must be one of `default` (the field message is always used and contains an [`crate::types::InaccessibleMessage`] if the message is inaccessible to the bot), `auto` (the field `inline_message_id` is used instead of the field message if the message is inaccessible to the bot; the field message can still contain an inaccessible message if the message has just been deleted), or `inline` (the field `inline_message_id` is always used instead of the field message).
    #[must_use]
    pub fn callback_query_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.callback_query_mode = val.map(Into::into);
        self
    }
}
impl Default for SetMySettings {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for SetMySettings {
    type Method = Self;
    type Return = bool;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("setMySettings", self, None)
    }
}
