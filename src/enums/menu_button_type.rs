use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the menu button
/// # Documentation
/// <https://core.telegram.org/bots/api#menubutton>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
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
    pub const fn all() -> [MenuButtonType; 3] {
        [
            MenuButtonType::Commands,
            MenuButtonType::WebApp,
            MenuButtonType::Default,
        ]
    }
}

impl From<MenuButtonType> for Box<str> {
    fn from(button_type: MenuButtonType) -> Self {
        Into::<&'static str>::into(button_type).into()
    }
}

impl<'a> PartialEq<&'a str> for MenuButtonType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
