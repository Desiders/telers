use super::WebAppInfo;

use serde::{Deserialize, Serialize};

/// Represents a menu button, which launches a `Web App <https://core.telegram.org/bots/webapps>`.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttonwebapp>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonWebApp {
    /// Type of the button, must be `web_app`
    #[serde(rename = "type")]
    pub button_type: String,
    /// Text on the button
    pub text: String,
    /// WebAppInfo Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method `aiogram_rs.methods.answer_web_app_query.AnswerWebAppQuery`.
    pub web_app: WebAppInfo,
}

impl MenuButtonWebApp {
    #[must_use]
    pub fn new<T: Into<String>>(text: T, web_app: WebAppInfo) -> Self {
        Self {
            button_type: "web_app".to_string(),
            text: text.into(),
            web_app,
        }
    }

    #[must_use]
    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = text.into();
        self
    }

    #[must_use]
    pub fn web_app(mut self, web_app: WebAppInfo) -> Self {
        self.web_app = web_app;
        self
    }
}

impl Default for MenuButtonWebApp {
    #[must_use]
    fn default() -> Self {
        Self {
            button_type: "web_app".to_string(),
            text: String::default(),
            web_app: WebAppInfo::default(),
        }
    }
}
