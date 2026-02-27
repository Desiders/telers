use serde::{Deserialize, Serialize};
/// Describes that no specific value for the menu button was set.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttondefault>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuButtonDefault {}
impl MenuButtonDefault {
    /// Creates a new `MenuButtonDefault`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for MenuButtonDefault {
    fn default() -> Self {
        Self::new()
    }
}
