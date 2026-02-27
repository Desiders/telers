use serde::{Deserialize, Serialize};
/// Represents a menu button, which launches a Web App.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttonwebapp>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuButtonWebApp {
    /// Text on the button
    pub text: Box<str>,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answer[`WebAppQuery`]. Alternatively, a t.me link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
    pub web_app: crate::types::WebAppInfo,
}
impl MenuButtonWebApp {
    /// Creates a new `MenuButtonWebApp`.
    ///
    /// # Arguments
    /// * `text` - Text on the button
    /// * `web_app` - Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answer[`WebAppQuery`]. Alternatively, a t.me link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::WebAppInfo>>(
        text: T0,
        web_app: T1,
    ) -> Self {
        Self {
            text: text.into(),
            web_app: web_app.into(),
        }
    }

    /// Text on the button
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = val.into();
        this
    }

    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answer[`WebAppQuery`]. Alternatively, a t.me link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
    #[must_use]
    pub fn web_app<T: Into<crate::types::WebAppInfo>>(self, val: T) -> Self {
        let mut this = self;
        this.web_app = val.into();
        this
    }
}
