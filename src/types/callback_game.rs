use serde::{Deserialize, Serialize};

/// A placeholder, currently holds no information. Use `BotFather <https://t.me/botfather>` to set up your game.
/// <https://core.telegram.org/bots/api#callbackgame>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CallbackGame {}
