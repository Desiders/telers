use super::{MenuButtonCommands, MenuButtonDefault, MenuButtonWebApp, WebAppInfo};

use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// This object describes the bot's menu button in a private chat. It should be one of
/// - [`MenuButtonCommands`]
/// - [`MenuButtonWebApp`]
/// - [`MenuButtonDefault`]
/// If a menu button other than [`MenuButtonDefault`] is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubutton>
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    #[strum(serialize = "commands")]
    Commands(MenuButtonCommands),
    #[strum(serialize = "web_app")]
    WebApp(MenuButtonWebApp),
    #[strum(serialize = "default")]
    Default(MenuButtonDefault),
}

impl MenuButton {
    #[must_use]
    pub fn commands() -> Self {
        Self::Commands(MenuButtonCommands::new())
    }

    #[must_use]
    pub fn web_app(text: impl Into<String>, web_app: WebAppInfo) -> Self {
        Self::WebApp(MenuButtonWebApp::new(text, web_app))
    }

    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn default() -> Self {
        Self::Default(MenuButtonDefault::new())
    }
}

impl Default for MenuButton {
    fn default() -> Self {
        Self::default()
    }
}

impl From<MenuButtonCommands> for MenuButton {
    fn from(commands: MenuButtonCommands) -> Self {
        Self::Commands(commands)
    }
}

impl From<MenuButtonWebApp> for MenuButton {
    fn from(web_app: MenuButtonWebApp) -> Self {
        Self::WebApp(web_app)
    }
}

impl From<MenuButtonDefault> for MenuButton {
    fn from(default: MenuButtonDefault) -> Self {
        Self::Default(default)
    }
}
