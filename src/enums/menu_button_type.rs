use std::fmt::{self, Debug};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MenuButtonType {
    Commands,
    WebApp,
    Default,
}

impl Debug for MenuButtonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MenuButtonType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            MenuButtonType::Commands => "commands",
            MenuButtonType::WebApp => "web_app",
            MenuButtonType::Default => "default",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [MenuButtonType; 3] {
        &[
            MenuButtonType::Commands,
            MenuButtonType::WebApp,
            MenuButtonType::Default,
        ]
    }
}

impl From<MenuButtonType> for String {
    fn from(button_type: MenuButtonType) -> Self {
        button_type.as_str().to_string()
    }
}
