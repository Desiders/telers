use serde::{Deserialize, Serialize};

/// This object represents one row of the high scores table for a game.
/// And that's about all we've got for now.
/// If you've got any questions, please check out our `https://core.telegram.org/bots/faq <https://core.telegram.org/bots/faq>`_ **Bot FAQ »**
/// <https://core.telegram.org/bots/api#gamehighscore>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GameHighScore {}

impl Default for GameHighScore {
    fn default() -> Self {
        Self {}
    }
}
