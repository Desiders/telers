use serde::Deserialize;

/// This object represents one row of the high scores table for a game.
/// And that's about all we've got for now.
/// If you've got any questions, please check out our `https://core.telegram.org/bots/faq <https://core.telegram.org/bots/faq>` **Bot FAQ »**
/// <https://core.telegram.org/bots/api#gamehighscore>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
pub struct GameHighScore;
