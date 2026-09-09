use crate::{methods::AnswerCallbackQuery, types::CallbackQuery};
impl CallbackQuery {
    /// Creates [`AnswerCallbackQuery`] for this callback query.
    #[must_use]
    pub fn answer(&self) -> AnswerCallbackQuery {
        AnswerCallbackQuery::new(&*self.id)
    }
}
