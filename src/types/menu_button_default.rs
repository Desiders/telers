use serde::{Deserialize, Serialize};

/// Describes that no specific value for the menu button was set.
/// <https://core.telegram.org/bots/api#menubuttondefault>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonDefault {
    /// Type of the button, must be *default*
    #[serde(rename = "type")]
    pub button_type: String,
}

impl MenuButtonDefault {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MenuButtonDefault {
    fn default() -> Self {
        Self {
            button_type: "default".to_string(),
        }
    }
}
