use serde::{Deserialize, Serialize};

/// Describes that no specific value for the menu button was set.
/// <https://core.telegram.org/bots/api#menubuttondefault>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonDefault {
    /// Type of the button, must be *default*
    #[serde(rename = "type", default = "default")]
    pub button_type: String,
}

impl Default for MenuButtonDefault {
    fn default() -> Self {
        Self {
            button_type: default(),
        }
    }
}

fn default() -> String {
    "default".to_string()
}
