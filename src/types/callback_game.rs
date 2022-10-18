use serde::{Deserialize, Serialize};

/// A placeholder, currently holds no information. Use `BotFather <https://t.me/botfather>`_ to set up your game.
/// <https://core.telegram.org/bots/api#callbackgame>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CallbackGame {}
