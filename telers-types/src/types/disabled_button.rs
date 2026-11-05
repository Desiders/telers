/// This object represents a disabled button which does nothing. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#disabledbutton>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
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
