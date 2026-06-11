use crate::{
    file::camel_to_filename,
    generator::helpers::{camel_to_snake, format_description},
    parser::api::NormalizedType,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[must_use]
pub fn tokenize_kind_enum(type_quote: &NormalizedType) -> Option<TokenStream> {
    if type_quote.subtypes.is_empty() {
        return None;
    }

    let type_name = format_ident!("{}", type_quote.name);
    let kind_name = format_ident!("{}Type", type_quote.name);
    let mut doc_lines = format_description(&type_quote.description, &type_quote.href);
    for subtype in &type_quote.subtypes {
        let code_name = format!("`{}`", subtype.ty_name);
        let bare_link = format!("[`{}`]", subtype.ty_name);
        let link_name = format!("[`crate::types::{}`]", subtype.ty_name);
        for line in &mut doc_lines {
            if line.contains(&code_name) {
                *line = line.replace(&code_name, &link_name);
            }
            if line.contains(&bare_link) {
                *line = line.replace(&bare_link, &link_name);
            }
        }
    }
    doc_lines = link_prefixed_type_mentions(doc_lines, &type_quote.name);

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

    let try_from_type_arms: Box<[_]> = type_quote
        .subtypes
        .iter()
        .map(|subtype| {
            let variant = format_ident!("{}", subtype.variant);
            quote! { #type_name::#variant(_) => Ok(#kind_name::#variant) }
        })
        .collect();

    // Variants that aren't Telegram object types (e.g. plain text) have no kind,
    // so the conversion to the kind enum is fallible for such types.
    let from_type_impl = if type_quote.extra_variants().is_empty() {
        quote! {
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
        }
    } else {
        quote! {
            impl<'a> TryFrom<&'a #type_name> for #kind_name {
                type Error = crate::errors::ConvertToTypeError;
                fn try_from(val: &'a #type_name) -> Result<Self, Self::Error> {
                    match val {
                        #( #try_from_type_arms, )*
                        _ => Err(Self::Error::new(stringify!(#type_name), stringify!(#kind_name))),
                    }
                }
            }

            impl TryFrom<#type_name> for #kind_name {
                type Error = crate::errors::ConvertToTypeError;
                fn try_from(val: #type_name) -> Result<Self, Self::Error> {
                    #kind_name::try_from(&val)
                }
            }
        }
    };

    let (strum_derives, string_impls) = (
        quote! { #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr, Deserialize, Serialize)] },
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
        use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
        use serde::{Deserialize, Serialize};

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

fn link_prefixed_type_mentions(lines: Vec<String>, prefix: &str) -> Vec<String> {
    lines
        .into_iter()
        .map(|line| {
            let mut out = String::with_capacity(line.len() + 32);
            let mut rest = line.as_str();

            while let Some(start) = rest.find('`') {
                out.push_str(&rest[..start]);
                let after_start = &rest[start + 1..];
                if let Some(end_rel) = after_start.find('`') {
                    let token = &after_start[..end_rel];
                    if token.starts_with(prefix)
                        && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        out.push_str("[`crate::types::");
                        out.push_str(token);
                        out.push_str("`]");
                    } else {
                        out.push('`');
                        out.push_str(token);
                        out.push('`');
                    }
                    rest = &after_start[end_rel + 1..];
                } else {
                    out.push_str(&rest[start..]);
                    break;
                }
            }

            if out.is_empty() {
                line
            } else {
                out.push_str(rest);
                out
            }
        })
        .collect()
}

#[must_use]
pub fn tokenize_kind_enum_file(type_quote: &NormalizedType) -> Option<TokenStream> {
    let kind_ts = tokenize_kind_enum(type_quote)?;
    let type_name = format_ident!("{}", type_quote.name);

    Some(quote! {
        use crate::types::#type_name;

        #kind_ts
    })
}

#[must_use]
pub fn tokenize_own_enums() -> Vec<(&'static str, TokenStream)> {
    vec![
        ("ParseMode", tokenize_enum_parse_mode()),
        (
            "TelegramObserverType",
            tokenize_enum_telegram_observer_type(),
        ),
    ]
}

#[must_use]
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
        use serde::{Deserialize, Serialize};

        /// This enum represents all possible types of the parse mode
        /// # Documentation
        /// <https://core.telegram.org/bots/api#formatting-options>
        #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr, Deserialize, Serialize)]
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

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn tokenize_enum_telegram_observer_type() -> TokenStream {
    let variants = [
        ("BusinessConnection", "business_connection"),
        ("BusinessMessage", "business_message"),
        ("CallbackQuery", "callback_query"),
        ("ChannelPost", "channel_post"),
        ("ChatBoost", "chat_boost"),
        ("ChatJoinRequest", "chat_join_request"),
        ("ChatMember", "chat_member"),
        ("ChosenInlineResult", "chosen_inline_result"),
        ("DeletedBusinessMessages", "deleted_business_messages"),
        ("EditedBusinessMessage", "edited_business_message"),
        ("EditedChannelPost", "edited_channel_post"),
        ("EditedMessage", "edited_message"),
        ("InlineQuery", "inline_query"),
        ("ManagedBot", "managed_bot"),
        ("Message", "message"),
        ("GuestMessage", "guest_message"),
        ("MessageReaction", "message_reaction"),
        ("MessageReactionCount", "message_reaction_count"),
        ("MyChatMember", "my_chat_member"),
        ("Poll", "poll"),
        ("PollAnswer", "poll_answer"),
        ("PreCheckoutQuery", "pre_checkout_query"),
        ("PurchasedPaidMedia", "purchased_paid_media"),
        ("RemovedChatBoost", "removed_chat_boost"),
        ("ShippingQuery", "shipping_query"),
        ("Update", "update"),
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
            quote! { TelegramObserverType::#variant }
        })
        .collect();

    let observer_mappings: Box<[_]> = variants
        .iter()
        .map(|(name, serialize)| {
            let variant = format_ident!("{name}");
            let observer = format_ident!("{serialize}");
            quote! { (#variant, #observer), }
        })
        .collect();

    let from_update_type_arms: Box<[_]> = variants
        .iter()
        .filter(|(name, _)| *name != "Update")
        .map(|(name, _)| {
            let variant = format_ident!("{name}");
            quote! { UpdateType::#variant => TelegramObserverType::#variant }
        })
        .collect();

    quote! {
        use crate::{enums::UpdateType, types::Update};
        use serde::{Deserialize, Serialize};
        use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

        /// This enum represents all possible telegram observer types.
        /// It contains all [`UpdateType`] variants plus `Update`.
        #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr, Deserialize, Serialize)]
        pub enum TelegramObserverType {
            #( #enum_variants )*
        }

        macro_rules! with_telegram_observer_variants {
            ($callback:ident $(, $args:tt)*) => {
                $callback! {
                    $($args,)*
                    #( #observer_mappings )*
                }
            };
        }
        pub(crate) use with_telegram_observer_variants;

        impl TelegramObserverType {
            #[must_use]
            pub const fn all() -> [TelegramObserverType; #variant_count] {
                [ #( #all_variants, )* ]
            }
        }

        impl From<TelegramObserverType> for Box<str> {
            fn from(val: TelegramObserverType) -> Self {
                Into::<&'static str>::into(val).into()
            }
        }

        impl From<TelegramObserverType> for String {
            fn from(val: TelegramObserverType) -> Self {
                val.as_ref().to_owned()
            }
        }

        impl<'a> PartialEq<&'a str> for TelegramObserverType {
            fn eq(&self, other: &&'a str) -> bool {
                self.as_ref() == *other
            }
        }

        impl From<UpdateType> for TelegramObserverType {
            fn from(val: UpdateType) -> Self {
                match val {
                    #( #from_update_type_arms, )*
                }
            }
        }

        impl<'a> From<&'a UpdateType> for TelegramObserverType {
            fn from(val: &'a UpdateType) -> Self {
                TelegramObserverType::from(*val)
            }
        }

        impl<'a> From<&'a Update> for TelegramObserverType {
            fn from(val: &'a Update) -> Self {
                TelegramObserverType::from(UpdateType::from(val))
            }
        }

        impl From<Update> for TelegramObserverType {
            fn from(val: Update) -> Self {
                TelegramObserverType::from(&val)
            }
        }
    }
}

#[must_use]
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
        //! Enum helpers and discriminator types for Telegram objects.
        //!
        //! This module contains:
        //! - generated `*Type` enums for polymorphic Telegram objects (for example `MessageType`)
        //! - hand-authored enums like [`ParseMode`]
        //!
        //! # Examples
        //! ```rust
        //! use telers::{enums::ParseMode, methods::SendMessage};
        //!
        //! let request = SendMessage::new(1_i64, "*Hello world!*")
        //!     .parse_mode(ParseMode::Markdown);
        //!
        //! assert_eq!(request.parse_mode.as_deref(), Some("Markdown"));
        //! ```

        #( #mods_quote )*
        #( #uses_quote )*
        #( #own_uses_quote )*
    }
}
