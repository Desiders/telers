pub mod message;

use crate::parser::api::NormalizedSchema;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[must_use]
pub fn tokenize_to_methods_files(schema: &NormalizedSchema) -> Vec<(&'static str, TokenStream)> {
    vec![(
        "message",
        message::tokenize_message_to_methods(schema.types.get("Message").unwrap()),
    )]
}

#[must_use]
pub fn tokenize_to_methods_mod(type_names: &[&str]) -> TokenStream {
    let mods_quote = type_names.iter().map(|name| {
        let mod_name = format_ident!("{name}");
        quote! { pub(crate) mod #mod_name; }
    });

    quote! {
        #( #mods_quote )*
    }
}
