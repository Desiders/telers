/// Verifies a user on behalf of the organization which is represented by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#verifyuser>
/// # Returns
/// - `bool`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct VerifyUser {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub custom_description: Option<Box<str>>,
    /// Mode for parsing entities in the custom description. See formatting options for more details. Only `text_link` entities are supported.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub custom_description_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the custom description, which can be specified instead of `custom_description_parse_mode`. Only `text_link` entities are supported.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub custom_description_entities: Option<Box<[crate::types::MessageEntity]>>,
}
impl VerifyUser {
    /// Creates a new `VerifyUser`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
            custom_description: None,
            custom_description_parse_mode: None,
            custom_description_entities: None,
        }
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[must_use]
    pub fn custom_description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_description = Some(val.into());
        self
    }

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[must_use]
    pub fn custom_description_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.custom_description = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the custom description. See formatting options for more details. Only `text_link` entities are supported.
    #[must_use]
    pub fn custom_description_parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_description_parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the custom description. See formatting options for more details. Only `text_link` entities are supported.
    #[must_use]
    pub fn custom_description_parse_mode_option<T: Into<Box<str>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.custom_description_parse_mode = val.map(Into::into);
        self
    }

    /// A JSON-serialized list of special entities that appear in the custom description, which can be specified instead of `custom_description_parse_mode`. Only `text_link` entities are supported.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn custom_description_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: T,
    ) -> Self {
        self.custom_description_entities = Some(
            self.custom_description_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the custom description, which can be specified instead of `custom_description_parse_mode`. Only `text_link` entities are supported.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn custom_description_entity<T: Into<crate::types::MessageEntity>>(
        mut self,
        val: T,
    ) -> Self {
        self.custom_description_entities = Some(
            self.custom_description_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the custom description, which can be specified instead of `custom_description_parse_mode`. Only `text_link` entities are supported.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn custom_description_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.custom_description_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }
}
impl super::TelegramMethod for VerifyUser {
    type Method = Self;
    type Return = bool;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("verifyUser", self, None)
    }
}
