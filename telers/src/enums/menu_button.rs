use crate::types::MenuButton;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the bot's menu button in a private chat. It should be one of
/// - [`crate::types::MenuButtonCommands`]
/// - [`crate::types::MenuButtonWebApp`]
/// - [`crate::types::MenuButtonDefault`]
///
/// If a menu button other than [`crate::types::MenuButtonDefault`] is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubutton>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum MenuButtonType {
    #[strum(serialize = "commands")]
    Commands,
    #[strum(serialize = "web_app")]
    WebApp,
    #[strum(serialize = "default")]
    Default,
}
impl MenuButtonType {
    #[must_use]
    pub const fn all() -> [MenuButtonType; 3usize] {
        [
            MenuButtonType::Commands,
            MenuButtonType::WebApp,
            MenuButtonType::Default,
        ]
    }
}
impl From<MenuButtonType> for Box<str> {
    fn from(val: MenuButtonType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MenuButtonType> for String {
    fn from(val: MenuButtonType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MenuButtonType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MenuButton> for MenuButtonType {
    fn from(val: &'a MenuButton) -> Self {
        match val {
            MenuButton::Commands(_) => MenuButtonType::Commands,
            MenuButton::WebApp(_) => MenuButtonType::WebApp,
            MenuButton::Default(_) => MenuButtonType::Default,
        }
    }
}
impl From<MenuButton> for MenuButtonType {
    fn from(val: MenuButton) -> Self {
        MenuButtonType::from(&val)
    }
}
