/// The theme with emoji placed on a background.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatsharethemeemoji>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ChatShareThemeEmoji {
    /// The emoji; 0-6 regular or custom emoji
    pub emojis: Box<[crate::types::EmojiType]>,
    /// Identifier of the backdrop used as the background; omitted if the default background is used. Use [`telers_methods::methods::GetChatShareThemeBackdrops`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_chat_share_theme_backdrops/struct.GetChatShareThemeBackdrops.html) to get details about the backdrop.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub background_backdrop_id: Option<i64>,
}
impl ChatShareThemeEmoji {
    /// Creates a new `ChatShareThemeEmoji`.
    ///
    /// # Arguments
    /// * `emojis` - The emoji; 0-6 regular or custom emoji
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::EmojiType>, T0: IntoIterator<Item = T0Item>>(
        emojis: T0,
    ) -> Self {
        Self {
            emojis: emojis.into_iter().map(Into::into).collect(),
            background_backdrop_id: None,
        }
    }

    /// The emoji; 0-6 regular or custom emoji
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn emojis<T: Into<Box<[crate::types::EmojiType]>>>(mut self, val: T) -> Self {
        self.emojis = self
            .emojis
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The emoji; 0-6 regular or custom emoji
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn emoji<T: Into<crate::types::EmojiType>>(mut self, val: T) -> Self {
        self.emojis = self
            .emojis
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Identifier of the backdrop used as the background; omitted if the default background is used. Use [`telers_methods::methods::GetChatShareThemeBackdrops`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_chat_share_theme_backdrops/struct.GetChatShareThemeBackdrops.html) to get details about the backdrop.
    #[must_use]
    pub fn background_backdrop_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.background_backdrop_id = Some(val.into());
        self
    }

    /// Identifier of the backdrop used as the background; omitted if the default background is used. Use [`telers_methods::methods::GetChatShareThemeBackdrops`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_chat_share_theme_backdrops/struct.GetChatShareThemeBackdrops.html) to get details about the backdrop.
    #[must_use]
    pub fn background_backdrop_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.background_backdrop_id = val.map(Into::into);
        self
    }
}
