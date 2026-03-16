use serde::{Deserialize, Serialize};
/// This object represents a list of boosts added to a chat by a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#userchatboosts>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user
    pub boosts: Box<[crate::types::ChatBoost]>,
}
impl UserChatBoosts {
    /// Creates a new `UserChatBoosts`.
    ///
    /// # Arguments
    /// * `boosts` - The list of boosts added to the chat by the user
    #[must_use]
    pub fn new<T0Item: Into<crate::types::ChatBoost>, T0: IntoIterator<Item = T0Item>>(
        boosts: T0,
    ) -> Self {
        Self {
            boosts: boosts.into_iter().map(Into::into).collect(),
        }
    }

    /// The list of boosts added to the chat by the user
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn boosts<T: Into<Box<[crate::types::ChatBoost]>>>(self, val: T) -> Self {
        let mut this = self;
        this.boosts = this
            .boosts
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// The list of boosts added to the chat by the user
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn boost<T: Into<crate::types::ChatBoost>>(self, val: T) -> Self {
        let mut this = self;
        this.boosts = this
            .boosts
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
