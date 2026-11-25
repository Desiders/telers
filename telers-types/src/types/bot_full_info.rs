/// This object contains full information about the bot itself.
/// # Documentation
/// <https://core.telegram.org/bots/api#botfullinfo>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct BotFullInfo {
    /// Unique identifier for this bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type is safe for storing this identifier.
    pub id: i64,
    /// Always `true`
    pub is_bot: bool,
    /// Bot's first name
    pub first_name: Box<str>,
    /// Bot's username
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub username: Option<Box<str>>,
    /// If non-empty, the list of all active usernames of the bot
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// `true`, if the bot can be added to groups
    pub can_join_groups: bool,
    /// `true`, if privacy mode is disabled for the bot
    pub can_read_all_group_messages: bool,
    /// `true`, if the bot supports guest queries from chats it is not a member of
    pub supports_guest_queries: bool,
    /// `true`, if the bot supports inline queries
    pub supports_inline_queries: bool,
    /// Placeholder for inline queries, which is shown in the input field after the bot's username; omitted if the default placeholder is used
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub inline_query_placeholder: Option<Box<str>>,
    /// `true`, if the bot requests the location of the user with every inline query
    pub needs_inline_query_location: bool,
    /// `true`, if the bot can be connected to a user account to manage it
    pub can_connect_to_business: bool,
    /// `true`, if the bot has a main Web App
    pub has_main_web_app: bool,
    /// `true`, if the bot has forum topic mode enabled in private chats
    pub has_topics_enabled: bool,
    /// `true`, if the bot allows users to create and delete topics in private chats
    pub allows_users_to_create_topics: bool,
    /// `true`, if topics in private chats with the bot are shown in a sidebar
    pub has_topic_sidebar: bool,
    /// `true`, if other bots can be created to be controlled by the bot
    pub can_manage_bots: bool,
    /// `true`, if the bot supports join request queries and can be assigned to process them
    pub supports_join_request_queries: bool,
}
impl BotFullInfo {
    /// Creates a new `BotFullInfo`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type is safe for storing this identifier.
    /// * `is_bot` - Always `true`
    /// * `first_name` - Bot's first name
    /// * `can_join_groups` - `true`, if the bot can be added to groups
    /// * `can_read_all_group_messages` - `true`, if privacy mode is disabled for the bot
    /// * `supports_guest_queries` - `true`, if the bot supports guest queries from chats it is not a member of
    /// * `supports_inline_queries` - `true`, if the bot supports inline queries
    /// * `needs_inline_query_location` - `true`, if the bot requests the location of the user with every inline query
    /// * `can_connect_to_business` - `true`, if the bot can be connected to a user account to manage it
    /// * `has_main_web_app` - `true`, if the bot has a main Web App
    /// * `has_topics_enabled` - `true`, if the bot has forum topic mode enabled in private chats
    /// * `allows_users_to_create_topics` - `true`, if the bot allows users to create and delete topics in private chats
    /// * `has_topic_sidebar` - `true`, if topics in private chats with the bot are shown in a sidebar
    /// * `can_manage_bots` - `true`, if other bots can be created to be controlled by the bot
    /// * `supports_join_request_queries` - `true`, if the bot supports join request queries and can be assigned to process them
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1: Into<bool>,
        T2: Into<Box<str>>,
        T3: Into<bool>,
        T4: Into<bool>,
        T5: Into<bool>,
        T6: Into<bool>,
        T7: Into<bool>,
        T8: Into<bool>,
        T9: Into<bool>,
        T10: Into<bool>,
        T11: Into<bool>,
        T12: Into<bool>,
        T13: Into<bool>,
        T14: Into<bool>,
    >(
        id: T0,
        is_bot: T1,
        first_name: T2,
        can_join_groups: T3,
        can_read_all_group_messages: T4,
        supports_guest_queries: T5,
        supports_inline_queries: T6,
        needs_inline_query_location: T7,
        can_connect_to_business: T8,
        has_main_web_app: T9,
        has_topics_enabled: T10,
        allows_users_to_create_topics: T11,
        has_topic_sidebar: T12,
        can_manage_bots: T13,
        supports_join_request_queries: T14,
    ) -> Self {
        Self {
            id: id.into(),
            is_bot: is_bot.into(),
            first_name: first_name.into(),
            username: None,
            active_usernames: None,
            can_join_groups: can_join_groups.into(),
            can_read_all_group_messages: can_read_all_group_messages.into(),
            supports_guest_queries: supports_guest_queries.into(),
            supports_inline_queries: supports_inline_queries.into(),
            inline_query_placeholder: None,
            needs_inline_query_location: needs_inline_query_location.into(),
            can_connect_to_business: can_connect_to_business.into(),
            has_main_web_app: has_main_web_app.into(),
            has_topics_enabled: has_topics_enabled.into(),
            allows_users_to_create_topics: allows_users_to_create_topics.into(),
            has_topic_sidebar: has_topic_sidebar.into(),
            can_manage_bots: can_manage_bots.into(),
            supports_join_request_queries: supports_join_request_queries.into(),
        }
    }

    /// Unique identifier for this bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type is safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Always `true`
    #[must_use]
    pub fn is_bot<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_bot = val.into();
        self
    }

    /// Bot's first name
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.first_name = val.into();
        self
    }

    /// Bot's username
    #[must_use]
    pub fn username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.username = Some(val.into());
        self
    }

    /// Bot's username
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.username = val.map(Into::into);
        self
    }

    /// If non-empty, the list of all active usernames of the bot
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn active_usernames<T: Into<Box<[Box<str>]>>>(mut self, val: T) -> Self {
        self.active_usernames = Some(
            self.active_usernames
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// If non-empty, the list of all active usernames of the bot
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn active_username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.active_usernames = Some(
            self.active_usernames
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// If non-empty, the list of all active usernames of the bot
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn active_usernames_option<T: Into<Box<[Box<str>]>>>(mut self, val: Option<T>) -> Self {
        self.active_usernames = val.map(Into::into);
        self
    }

    /// `true`, if the bot can be added to groups
    #[must_use]
    pub fn can_join_groups<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_join_groups = val.into();
        self
    }

    /// `true`, if privacy mode is disabled for the bot
    #[must_use]
    pub fn can_read_all_group_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_read_all_group_messages = val.into();
        self
    }

    /// `true`, if the bot supports guest queries from chats it is not a member of
    #[must_use]
    pub fn supports_guest_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_guest_queries = val.into();
        self
    }

    /// `true`, if the bot supports inline queries
    #[must_use]
    pub fn supports_inline_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_inline_queries = val.into();
        self
    }

    /// Placeholder for inline queries, which is shown in the input field after the bot's username; omitted if the default placeholder is used
    #[must_use]
    pub fn inline_query_placeholder<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_query_placeholder = Some(val.into());
        self
    }

    /// Placeholder for inline queries, which is shown in the input field after the bot's username; omitted if the default placeholder is used
    #[must_use]
    pub fn inline_query_placeholder_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_query_placeholder = val.map(Into::into);
        self
    }

    /// `true`, if the bot requests the location of the user with every inline query
    #[must_use]
    pub fn needs_inline_query_location<T: Into<bool>>(mut self, val: T) -> Self {
        self.needs_inline_query_location = val.into();
        self
    }

    /// `true`, if the bot can be connected to a user account to manage it
    #[must_use]
    pub fn can_connect_to_business<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_connect_to_business = val.into();
        self
    }

    /// `true`, if the bot has a main Web App
    #[must_use]
    pub fn has_main_web_app<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_main_web_app = val.into();
        self
    }

    /// `true`, if the bot has forum topic mode enabled in private chats
    #[must_use]
    pub fn has_topics_enabled<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_topics_enabled = val.into();
        self
    }

    /// `true`, if the bot allows users to create and delete topics in private chats
    #[must_use]
    pub fn allows_users_to_create_topics<T: Into<bool>>(mut self, val: T) -> Self {
        self.allows_users_to_create_topics = val.into();
        self
    }

    /// `true`, if topics in private chats with the bot are shown in a sidebar
    #[must_use]
    pub fn has_topic_sidebar<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_topic_sidebar = val.into();
        self
    }

    /// `true`, if other bots can be created to be controlled by the bot
    #[must_use]
    pub fn can_manage_bots<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_bots = val.into();
        self
    }

    /// `true`, if the bot supports join request queries and can be assigned to process them
    #[must_use]
    pub fn supports_join_request_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_join_request_queries = val.into();
        self
    }
}
