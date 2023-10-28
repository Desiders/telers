use super::WebAppInfo;

use crate::enums::MenuButtonType;

use serde::{Deserialize, Serialize};

/// Represents a menu button, which launches a [`Web App`](https://core.telegram.org/bots/webapps).
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttonwebapp>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MenuButtonWebApp {
    /// Type of the button, must be `web_app`
    #[serde(rename = "type", default = "web_app_type")]
    pub button_type: String,
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [`AnswerWebAppQuery`](crate::methods::AnswerWebAppQuery).
    pub web_app: WebAppInfo,
}

impl MenuButtonWebApp {
    #[must_use]
    pub fn new(text: impl Into<String>, web_app: WebAppInfo) -> Self {
        Self {
            button_type: web_app_type(),
            text: text.into(),
            web_app,
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
            web_app: val,
            ..self
        }
    }
}

impl Default for MenuButtonWebApp {
    #[must_use]
    fn default() -> Self {
        Self {
            button_type: web_app_type(),
            text: String::default(),
            web_app: WebAppInfo::default(),
        }
    }
}

fn web_app_type() -> String {
    MenuButtonType::WebApp.into()
}
