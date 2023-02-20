use super::{MenuButtonCommands, MenuButtonDefault, MenuButtonWebApp};

use serde::{Deserialize, Serialize};

/// This object describes the bot's menu button in a private chat. It should be one of
/// - [`MenuButtonCommands`]
/// - [`MenuButtonWebApp`]
/// - [`MenuButtonDefault`]
/// If a menu button other than [`MenuButtonDefault`] is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubutton>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    Commands(MenuButtonCommands),
    WebApp(MenuButtonWebApp),
    Default(MenuButtonDefault),
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

impl Default for MenuButton {
    #[must_use]
    fn default() -> Self {
        Self::Default(MenuButtonDefault::default())
    }
}
