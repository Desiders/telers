use serde::{Deserialize, Serialize};
/// A divider, corresponding to the HTML tag <hr/>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockdivider>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockDivider {}
impl RichBlockDivider {
    /// Creates a new `RichBlockDivider`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for RichBlockDivider {
    fn default() -> Self {
        Self::new()
    }
}
