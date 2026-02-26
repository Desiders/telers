use crate::{
    file::camel_to_filename,
    generator::helpers::{camel_to_snake, format_description},
    parser::api::NormalizedType,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn tokenize_kind_enum(type_quote: &NormalizedType) -> Option<TokenStream> {
    if type_quote.subtypes.is_empty() {
        return None;
    }

    let type_name = format_ident!("{}", type_quote.name);
    let kind_name = format_ident!("{}Type", type_quote.name);
    let doc_lines = format_description(&type_quote.description, &type_quote.href);

    let variant_count = type_quote.subtypes.len();

    let enum_variants: Box<[_]> = type_quote
        .subtypes
        .iter()
        .map(|subtype| {
            let variant = format_ident!("{}", subtype.variant);
            let serialize = camel_to_snake(&subtype.variant);
            quote! {
                #[strum(serialize = #serialize)]
                #variant,
            }
        })
        .collect();

    let all_variants: Box<[_]> = type_quote
        .subtypes
        .iter()
        .map(|subtype| {
            let variant = format_ident!("{}", subtype.variant);
            quote! { #kind_name::#variant }
        })
        .collect();

    let from_type_arms: Box<[_]> = type_quote
        .subtypes
        .iter()
        .map(|subtype| {
            let variant = format_ident!("{}", subtype.variant);
            quote! { #type_name::#variant(_) => #kind_name::#variant }
        })
        .collect();

    let from_type_impl = quote! {
        impl<'a> From<&'a #type_name> for #kind_name {
            fn from(val: &'a #type_name) -> Self {
                match val {
                    #( #from_type_arms, )*
                }
            }
        }

        impl From<#type_name> for #kind_name {
            fn from(val: #type_name) -> Self {
                #kind_name::from(&val)
            }
        }
    };

    let (strum_import, strum_derives, string_impls) = (
        quote! { use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr}; },
        quote! { #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)] },
        quote! {
            impl From<#kind_name> for Box<str> {
                fn from(val: #kind_name) -> Self {
                    Into::<&'static str>::into(val).into()
                }
            }

            impl From<#kind_name> for String {
                fn from(val: #kind_name) -> Self {
                    val.as_ref().to_owned()
                }
            }

            impl<'a> PartialEq<&'a str> for #kind_name {
                fn eq(&self, other: &&'a str) -> bool {
                    self.as_ref() == *other
                }
            }
        },
    );

    Some(quote! {
        #strum_import

        #( #[doc = #doc_lines] )*
        #strum_derives
        pub enum #kind_name {
            #( #enum_variants )*
        }

        impl #kind_name {
            #[must_use]
            pub const fn all() -> [#kind_name; #variant_count] {
                [ #( #all_variants, )* ]
            }
        }

        #string_impls

        #from_type_impl
    })
}

pub fn tokenize_kind_enum_file(type_quote: &NormalizedType) -> Option<TokenStream> {
    let kind_ts = tokenize_kind_enum(type_quote)?;

    Some(quote! {
        use crate::types::*;

        #kind_ts
    })
}

pub fn tokenize_own_enums() -> Vec<(&'static str, TokenStream)> {
    vec![("ParseMode", tokenize_enum_parse_mode())]
}

pub fn tokenize_enum_parse_mode() -> TokenStream {
    let variants = [
        ("Markdown", "Markdown"),
        ("MarkdownV2", "MarkdownV2"),
        ("HTML", "HTML"),
    ];

    let variant_count = variants.len();

    let enum_variants: Box<[_]> = variants
        .iter()
        .map(|(name, serialize)| {
            let variant = format_ident!("{name}");
            quote! {
                #[strum(serialize = #serialize)]
                #variant,
            }
        })
        .collect();

    let all_variants: Box<[_]> = variants
        .iter()
        .map(|(name, _)| {
            let variant = format_ident!("{name}");
            quote! { ParseMode::#variant }
        })
        .collect();

    quote! {
        use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

        /// This enum represents all possible types of the parse mode
        /// # Documentation
        /// <https://core.telegram.org/bots/api#formatting-options>
        #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
        pub enum ParseMode {
            #( #enum_variants )*
        }

        impl ParseMode {
            #[must_use]
            pub const fn all() -> [ParseMode; #variant_count] {
                [ #( #all_variants, )* ]
            }
        }

        impl From<ParseMode> for Box<str> {
            fn from(parse_mode: ParseMode) -> Self {
                Into::<&'static str>::into(parse_mode).into()
            }
        }

        impl From<ParseMode> for String {
            fn from(parse_mode: ParseMode) -> Self {
                parse_mode.as_ref().to_owned()
            }
        }

        impl<'a> PartialEq<&'a str> for ParseMode {
            fn eq(&self, other: &&'a str) -> bool {
                self.as_ref().to_lowercase() == other.to_lowercase()
            }
        }
    }
}

pub fn tokenize_kind_enums_mod(type_names: &[&str], own_type_names: &[&str]) -> TokenStream {
    let all_module_names: Vec<_> = type_names
        .iter()
        .chain(own_type_names.iter())
        .copied()
        .collect();
    let mods_quote = all_module_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        quote! { pub mod #mod_name; }
    });
    let uses_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        let kind_name = format_ident!("{name}Type");
        quote! { pub use #mod_name::#kind_name; }
    });
    let own_uses_quote = own_type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        let kind_name = format_ident!("{name}");
        quote! { pub use #mod_name::#kind_name; }
    });

    quote! {
        #( #mods_quote )*
        #( #uses_quote )*
        #( #own_uses_quote )*
    }
}
