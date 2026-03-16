use serde::{Deserialize, Serialize};
/// This object represents a button to be shown above inline query results. You must use exactly one of the optional fields.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultsbutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultsButton {
    /// Label text on the button
    pub text: Box<str>,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method switchInlineQuery inside the Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<crate::types::WebAppInfo>,
    /// Deep-linking parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only A-Z, a-z, 0-9, `_` and - are allowed. Example: An inline bot that sends `YouTube` videos can ask the user to connect the bot to their `YouTube` account to adapt search results accordingly. To do this, it displays a 'Connect your `YouTube` account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a `switch_inline` button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<Box<str>>,
}
impl InlineQueryResultsButton {
    /// Creates a new `InlineQueryResultsButton`.
    ///
    /// # Arguments
    /// * `text` - Label text on the button
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(text: T0) -> Self {
        Self {
            text: text.into(),
            web_app: None,
            start_parameter: None,
        }
    }

    /// Label text on the button
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = val.into();
        this
    }

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method switchInlineQuery inside the Web App.
    #[must_use]
    pub fn web_app<T: Into<crate::types::WebAppInfo>>(self, val: T) -> Self {
        let mut this = self;
        this.web_app = Some(val.into());
        this
    }

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method switchInlineQuery inside the Web App.
    #[must_use]
    pub fn web_app_option<T: Into<crate::types::WebAppInfo>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.web_app = val.map(Into::into);
        this
    }

    /// Deep-linking parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only A-Z, a-z, 0-9, `_` and - are allowed. Example: An inline bot that sends `YouTube` videos can ask the user to connect the bot to their `YouTube` account to adapt search results accordingly. To do this, it displays a 'Connect your `YouTube` account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a `switch_inline` button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[must_use]
    pub fn start_parameter<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.start_parameter = Some(val.into());
        this
    }

    /// Deep-linking parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only A-Z, a-z, 0-9, `_` and - are allowed. Example: An inline bot that sends `YouTube` videos can ask the user to connect the bot to their `YouTube` account to adapt search results accordingly. To do this, it displays a 'Connect your `YouTube` account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a `switch_inline` button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[must_use]
    pub fn start_parameter_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.start_parameter = val.map(Into::into);
        this
    }
}
