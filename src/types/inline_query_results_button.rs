use super::WebAppInfo;

use serde::{Deserialize, Serialize};

/// This object represents a button to be shown above inline query results. You **must** use exactly one of the optional fields.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultsbutton>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultsButton {
    /// Label text on the button
    pub text: String,
    /// Description of the [`Web App`](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method [`switchInlineQuery`](https://core.telegram.org/bots/webapps#initializing-web-apps) inside the Web App.
    pub web_app: Option<WebAppInfo>,
    /// [`Deep-linking`](https://core.telegram.org/bots/features#deep-linking) parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    /// Example: An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a [`switch_inline`](https://core.telegram.org/bots/api#inlinekeyboardmarkup) button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    pub start_parameter: Option<String>,
}

impl InlineQueryResultsButton {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            web_app: None,
            start_parameter: None,
        }
    }

    #[must_use]
    pub fn text(self, val: impl Into<String>) -> Self {
        Self {
            text: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn web_app(self, val: WebAppInfo) -> Self {
        Self {
            web_app: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn start_parameter(self, val: impl Into<String>) -> Self {
        Self {
            start_parameter: Some(val.into()),
            ..self
        }
    }
}
