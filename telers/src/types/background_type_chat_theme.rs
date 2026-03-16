use serde::{Deserialize, Serialize};
/// The background is taken directly from a built-in chat theme.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypechattheme>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundTypeChatTheme {
    /// Name of the chat theme, which is usually an emoji
    pub theme_name: Box<str>,
}
impl BackgroundTypeChatTheme {
    /// Creates a new `BackgroundTypeChatTheme`.
    ///
    /// # Arguments
    /// * `theme_name` - Name of the chat theme, which is usually an emoji
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(theme_name: T0) -> Self {
        Self {
            theme_name: theme_name.into(),
        }
    }

    /// Name of the chat theme, which is usually an emoji
    #[must_use]
    pub fn theme_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.theme_name = val.into();
        this
    }
}
