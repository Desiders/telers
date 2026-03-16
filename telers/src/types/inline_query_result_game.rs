use serde::{Deserialize, Serialize};
/// Represents a Game.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultgame>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// Short name of the game
    pub game_short_name: Box<str>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl InlineQueryResultGame {
    /// Creates a new `InlineQueryResultGame`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `game_short_name` - Short name of the game
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(id: T0, game_short_name: T1) -> Self {
        Self {
            id: id.into(),
            game_short_name: game_short_name.into(),
            reply_markup: None,
        }
    }

    /// Unique identifier for this result, 1-64 bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Short name of the game
    #[must_use]
    pub fn game_short_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.game_short_name = val.into();
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }
}
