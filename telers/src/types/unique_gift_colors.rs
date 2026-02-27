use serde::{Deserialize, Serialize};
/// This object contains information about the color scheme for a user's name, message replies and link previews based on a unique gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftcolors>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: Box<str>,
    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: Box<str>,
    /// Main color used in light themes; RGB format
    pub light_theme_main_color: i64,
    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Box<[u8]>,
    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: i64,
    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Box<[u8]>,
}
impl UniqueGiftColors {
    /// Creates a new `UniqueGiftColors`.
    ///
    /// # Arguments
    /// * `model_custom_emoji_id` - Custom emoji identifier of the unique gift's model
    /// * `symbol_custom_emoji_id` - Custom emoji identifier of the unique gift's symbol
    /// * `light_theme_main_color` - Main color used in light themes; RGB format
    /// * `light_theme_other_colors` - List of 1-3 additional colors used in light themes; RGB format
    /// * `dark_theme_main_color` - Main color used in dark themes; RGB format
    /// * `dark_theme_other_colors` - List of 1-3 additional colors used in dark themes; RGB format
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<i64>,
        T3Item: Into<u8>,
        T3: IntoIterator<Item = T3Item>,
        T4: Into<i64>,
        T5Item: Into<u8>,
        T5: IntoIterator<Item = T5Item>,
    >(
        model_custom_emoji_id: T0,
        symbol_custom_emoji_id: T1,
        light_theme_main_color: T2,
        light_theme_other_colors: T3,
        dark_theme_main_color: T4,
        dark_theme_other_colors: T5,
    ) -> Self {
        Self {
            model_custom_emoji_id: model_custom_emoji_id.into(),
            symbol_custom_emoji_id: symbol_custom_emoji_id.into(),
            light_theme_main_color: light_theme_main_color.into(),
            light_theme_other_colors: light_theme_other_colors
                .into_iter()
                .map(Into::into)
                .collect(),
            dark_theme_main_color: dark_theme_main_color.into(),
            dark_theme_other_colors: dark_theme_other_colors
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }

    /// Custom emoji identifier of the unique gift's model
    #[must_use]
    pub fn model_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.model_custom_emoji_id = val.into();
        this
    }

    /// Custom emoji identifier of the unique gift's symbol
    #[must_use]
    pub fn symbol_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.symbol_custom_emoji_id = val.into();
        this
    }

    /// Main color used in light themes; RGB format
    #[must_use]
    pub fn light_theme_main_color<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.light_theme_main_color = val.into();
        this
    }

    /// List of 1-3 additional colors used in light themes; RGB format
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn light_theme_other_colors<T: Into<Box<[u8]>>>(self, val: T) -> Self {
        let mut this = self;
        this.light_theme_other_colors = this
            .light_theme_other_colors
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of 1-3 additional colors used in light themes; RGB format
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn light_theme_other_color<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.light_theme_other_colors = this
            .light_theme_other_colors
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Main color used in dark themes; RGB format
    #[must_use]
    pub fn dark_theme_main_color<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.dark_theme_main_color = val.into();
        this
    }

    /// List of 1-3 additional colors used in dark themes; RGB format
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn dark_theme_other_colors<T: Into<Box<[u8]>>>(self, val: T) -> Self {
        let mut this = self;
        this.dark_theme_other_colors = this
            .dark_theme_other_colors
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of 1-3 additional colors used in dark themes; RGB format
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn dark_theme_other_color<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.dark_theme_other_colors = this
            .dark_theme_other_colors
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
