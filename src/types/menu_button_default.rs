use serde::{Deserialize, Serialize};

/// Describes that no specific value for the menu button was set.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttondefault>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MenuButtonDefault {}

impl MenuButtonDefault {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
