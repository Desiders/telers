use crate::enums::MenuButtonType;

use serde::{Deserialize, Serialize};

/// Describes that no specific value for the menu button was set.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttondefault>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonDefault {
    /// Type of the button, must be *default*
    #[serde(rename = "type", default = "default")]
    pub button_type: String,
}

impl MenuButtonDefault {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MenuButtonDefault {
    #[must_use]
    fn default() -> Self {
        Self {
            button_type: default(),
        }
    }
}

fn default() -> String {
    MenuButtonType::Default.into()
}
