use serde::{Deserialize, Serialize};
/// A placeholder, currently holds no information. Use `BotFather` to set up your game.
/// # Documentation
/// <https://core.telegram.org/bots/api#callbackgame>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallbackGame {}
impl CallbackGame {
    /// Creates a new `CallbackGame`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for CallbackGame {
    fn default() -> Self {
        Self::new()
    }
}
