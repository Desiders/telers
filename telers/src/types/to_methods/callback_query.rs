use crate::{methods::AnswerCallbackQuery, types::CallbackQuery};
/// Shortcuts that create methods for a callback query.
pub trait CallbackQueryShortcuts {
    /// Creates [`AnswerCallbackQuery`] for this callback query.
    #[must_use]
    fn answer(&self) -> AnswerCallbackQuery;
}
impl CallbackQueryShortcuts for CallbackQuery {
    fn answer(&self) -> AnswerCallbackQuery {
        AnswerCallbackQuery::new(&*self.id)
    }
}
