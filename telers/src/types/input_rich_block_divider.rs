use serde::{Deserialize, Serialize};
/// A divider, corresponding to the HTML tag <hr/>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockdivider>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockDivider {}
impl InputRichBlockDivider {
    /// Creates a new `InputRichBlockDivider`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for InputRichBlockDivider {
    fn default() -> Self {
        Self::new()
    }
}
