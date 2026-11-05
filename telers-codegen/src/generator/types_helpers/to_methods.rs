pub mod callback_query;
pub mod message;

use crate::parser::api::NormalizedSchema;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn tokenize_to_methods_files(schema: &NormalizedSchema) -> Vec<(&'static str, TokenStream)> {
    vec![
        (
            "callback_query",
            callback_query::tokenize_callback_query_to_methods(),
        ),
        ("message", message::tokenize_message_to_methods(schema)),
    ]
}

/// Generate framework shortcuts implemented for types from an external crate.
#[must_use]
pub fn tokenize_to_methods_files_for_external_types(
    schema: &NormalizedSchema,
) -> Vec<(&'static str, TokenStream)> {
    vec![
        (
            "callback_query",
            callback_query::tokenize_callback_query_to_methods_trait(),
        ),
        ("message", message::tokenize_message_to_methods(schema)),
    ]
}

#[must_use]
pub fn tokenize_to_methods_mod(type_names: &[&str]) -> TokenStream {
    tokenize_to_methods_mod_inner(type_names, false)
}

fn tokenize_to_methods_mod_inner(type_names: &[&str], external_types: bool) -> TokenStream {
    let mut names = type_names.to_vec();
    names.sort_unstable();
    let mods_quote = names.iter().map(|name| {
        let mod_name = format_ident!("{name}");
        quote! { pub(crate) mod #mod_name; }
    });
    let callback_export = external_types.then(|| {
        quote! {
            pub use callback_query::CallbackQueryShortcuts;
        }
    });

    quote! {
        #( #mods_quote )*
        #callback_export
        pub use message::{MessageRenderers, MessageShortcuts};
    }
}

/// Export extension traits for all framework shortcuts on external API types.
#[must_use]
pub fn tokenize_to_methods_mod_for_external_types(type_names: &[&str]) -> TokenStream {
    tokenize_to_methods_mod_inner(type_names, true)
}
