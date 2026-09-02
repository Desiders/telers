use serde::{Deserialize, Serialize};
/// This object represents a disabled button which does nothing. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#disabledbutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisabledButton {}
impl DisabledButton {
    /// Creates a new `DisabledButton`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for DisabledButton {
    fn default() -> Self {
        Self::new()
    }
}
