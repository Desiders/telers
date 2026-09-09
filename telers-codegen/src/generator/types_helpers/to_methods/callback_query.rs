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
