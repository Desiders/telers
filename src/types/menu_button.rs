use super::{MenuButtonCommands, MenuButtonDefault, MenuButtonWebApp};

use serde::{Deserialize, Serialize};

/// This object describes the bot's menu button in a private chat. It should be one of
/// - `aiogram_rs.types.menu_button_commands.MenuButtonCommands`
/// - `aiogram_rs.types.menu_button_web_app.MenuButtonWebApp`
/// - `aiogram_rs.types.menu_button_default.MenuButtonDefault`
/// If a menu button other than `aiogram_rs.types.menu_button_default.MenuButtonDefault` is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
/// <https://core.telegram.org/bots/api#menubutton>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    Commands(MenuButtonCommands),
    WebApp(MenuButtonWebApp),
    Default(MenuButtonDefault),
}
