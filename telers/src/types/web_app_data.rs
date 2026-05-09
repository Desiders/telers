use serde::{Deserialize, Serialize};
/// Describes data sent from a Web App to the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#webappdata>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: Box<str>,
    /// Text of the `web_app` keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    pub button_text: Box<str>,
}
impl WebAppData {
    /// Creates a new `WebAppData`.
    ///
    /// # Arguments
    /// * `data` - The data. Be aware that a bad client can send arbitrary data in this field.
    /// * `button_text` - Text of the `web_app` keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(data: T0, button_text: T1) -> Self {
        Self {
            data: data.into(),
            button_text: button_text.into(),
        }
    }

    /// The data. Be aware that a bad client can send arbitrary data in this field.
    #[must_use]
    pub fn data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.data = val.into();
        self
    }

    /// Text of the `web_app` keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    #[must_use]
    pub fn button_text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.button_text = val.into();
        self
    }
}
