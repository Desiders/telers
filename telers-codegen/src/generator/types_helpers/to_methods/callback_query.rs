use proc_macro2::TokenStream;
use quote::quote;

#[must_use]
pub fn tokenize_callback_query_to_methods() -> TokenStream {
    quote! {
        use crate::{methods::AnswerCallbackQuery, types::CallbackQuery};

        impl CallbackQuery {
            /// Creates [`AnswerCallbackQuery`] for this callback query.
            #[must_use]
            pub fn answer(&self) -> AnswerCallbackQuery {
                AnswerCallbackQuery::new(&*self.id)
            }
        }
    }
}

/// Method shortcuts belong to the framework, while `CallbackQuery` belongs to the types crate.
#[must_use]
pub fn tokenize_callback_query_to_methods_trait() -> TokenStream {
    quote! {
        use crate::{methods::AnswerCallbackQuery, types::CallbackQuery};

        /// Shortcuts that construct Bot API requests for a callback query.
        ///
        /// Import this trait to call the shortcuts: `use telers::types::CallbackQueryShortcuts as _;`.
        ///
        /// ```
        /// use telers::types::{CallbackQuery, CallbackQueryShortcuts as _, User};
        ///
        /// let query = CallbackQuery::new("query-id", User::new(1, false, "Alice"), "chat-instance");
        /// let answer = query.answer().text("Done");
        /// assert_eq!(&*answer.callback_query_id, "query-id");
        /// ```
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

        #[cfg(test)]
        mod tests {
            use super::CallbackQueryShortcuts as _;
            use crate::types::{CallbackQuery, User};

            #[test]
            fn answer_uses_the_query_identifier() {
                let query = CallbackQuery::new("query-id", User::new(1, false, "Alice"), "chat-instance");
                let answer = query.answer().text("Done");
                assert_eq!(answer.callback_query_id.as_ref(), "query-id");
                assert_eq!(answer.text.as_deref(), Some("Done"));
            }
        }
    }
}
