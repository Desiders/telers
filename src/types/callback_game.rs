use serde::{Deserialize, Serialize};

/// A placeholder, currently holds no information. Use [`BotFather`](https://t.me/botfather) to set up your game.
/// # Documentation
/// <https://core.telegram.org/bots/api#callbackgame>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct CallbackGame {}

impl CallbackGame {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
