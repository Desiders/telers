use serde::{Deserialize, Serialize};
/// This object represents one row of the high scores table for a game.
/// # Documentation
/// <https://core.telegram.org/bots/api#gamehighscore>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: Box<crate::types::User>,
    /// Score
    pub score: i64,
}
impl GameHighScore {
    /// Creates a new `GameHighScore`.
    ///
    /// # Arguments
    /// * `position` - Position in high score table for the game
    /// * `user` - User
    /// * `score` - Score
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::User>, T2: Into<i64>>(
        position: T0,
        user: T1,
        score: T2,
    ) -> Self {
        Self {
            position: position.into(),
            user: Box::new(user.into()),
            score: score.into(),
        }
    }

    /// Position in high score table for the game
    #[must_use]
    pub fn position<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.position = val.into();
        this
    }

    /// User
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// Score
    #[must_use]
    pub fn score<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.score = val.into();
        this
    }
}
