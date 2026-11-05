/// The reaction is paid.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontypepaid>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ReactionTypePaid {}
impl ReactionTypePaid {
    /// Creates a new `ReactionTypePaid`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for ReactionTypePaid {
    fn default() -> Self {
        Self::new()
    }
}
