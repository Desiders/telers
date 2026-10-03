/// A divider, corresponding to the HTML tag <hr/>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockdivider>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
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
