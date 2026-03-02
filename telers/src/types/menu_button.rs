use serde::{Deserialize, Serialize};
/// This object describes the bot's menu button in a private chat. It should be one of
/// - [`MenuButtonCommands`]
/// - [`MenuButtonWebApp`]
/// - [`MenuButtonDefault`]
///
/// If a menu button other than [`MenuButtonDefault`] is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    Commands(crate::types::MenuButtonCommands),
    WebApp(crate::types::MenuButtonWebApp),
    Default(crate::types::MenuButtonDefault),
}
impl MenuButton {
    /// Helper method for field `text`.
    ///
    /// Text on the button
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::WebApp(val) => Some(val.text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `web_app`.
    ///
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answer[`WebAppQuery`]. Alternatively, a t.me link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.
    #[must_use]
    pub fn web_app(&self) -> Option<&crate::types::WebAppInfo> {
        match self {
            Self::WebApp(val) => Some(&val.web_app),
            _ => None,
        }
    }

    /// Helper method for nested field `url`.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::WebApp(val) => {
                let inner = &val.web_app;
                Some(inner.url.as_ref())
            }
            _ => None,
        }
    }
}
impl From<crate::types::MenuButtonCommands> for MenuButton {
    fn from(val: crate::types::MenuButtonCommands) -> Self {
        Self::Commands(val)
    }
}
impl TryFrom<MenuButton> for crate::types::MenuButtonCommands {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MenuButton) -> Result<Self, Self::Error> {
        if let MenuButton::Commands(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MenuButton),
                stringify!(MenuButtonCommands),
            ))
        }
    }
}
impl From<crate::types::MenuButtonWebApp> for MenuButton {
    fn from(val: crate::types::MenuButtonWebApp) -> Self {
        Self::WebApp(val)
    }
}
impl TryFrom<MenuButton> for crate::types::MenuButtonWebApp {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MenuButton) -> Result<Self, Self::Error> {
        if let MenuButton::WebApp(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MenuButton),
                stringify!(MenuButtonWebApp),
            ))
        }
    }
}
impl From<crate::types::MenuButtonDefault> for MenuButton {
    fn from(val: crate::types::MenuButtonDefault) -> Self {
        Self::Default(val)
    }
}
impl TryFrom<MenuButton> for crate::types::MenuButtonDefault {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MenuButton) -> Result<Self, Self::Error> {
        if let MenuButton::Default(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MenuButton),
                stringify!(MenuButtonDefault),
            ))
        }
    }
}
