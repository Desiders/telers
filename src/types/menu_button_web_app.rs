use super::WebAppInfo;

use serde::{Deserialize, Serialize};

/// Represents a menu button, which launches a `Web App <https://core.telegram.org/bots/webapps>`_.
/// <https://core.telegram.org/bots/api#menubuttonwebapp>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonWebApp {
    /// Type of the button, must be `web_app`
    #[serde(rename = "type", default = "web_app")]
    pub button_type: String,
    /// Text on the button
    pub text: String,
    /// WebAppInfo Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method :class:`aiogram_rs.methods.answer_web_app_query.AnswerWebAppQuery`.
    pub web_app: WebAppInfo,
}

impl Default for MenuButtonWebApp {
    fn default() -> Self {
        Self {
            button_type: web_app(),
            text: String::default(),
            web_app: WebAppInfo::default(),
        }
    }
}

fn web_app() -> String {
    "web_app".to_string()
}
