use crate::{
    attrs_parsing::{parse_attr, set_once},
    extractor::{extractor_generics, tokenize_extractor_impl},
};

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use std::collections::HashSet;
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DeriveInput, Fields,
    Ident, LitChar, LitStr, Token,
};

/// Rename rule for command names
#[derive(Clone, Copy, Default)]
enum RenameRule {
    /// `UserName` -> `username`
    #[default]
    Lower,
    /// `UserName` -> `user_name`
    SnakeCase,
}

impl RenameRule {
    fn parse(value: &LitStr) -> syn::Result<Self> {
        match value.value().as_str() {
            "lowercase" => Ok(Self::Lower),
            "snake_case" => Ok(Self::SnakeCase),
            _ => Err(syn::Error::new_spanned(
                value,
                "expected one of: `lowercase`, `snake_case`",
            )),
        }
    }

    fn apply(self, ident: &Ident) -> String {
        match self {
            Self::Lower => ident.to_string().to_lowercase(),
            Self::SnakeCase => ident.to_string().to_snake_case(),
        }
    }
}

/// Enum-level `#[command(...)]` attributes
#[derive(Default)]
struct CommandAttrs {
    rename_rule: RenameRule,
    prefix: Option<char>,
    split: Option<char>,
}

impl CommandAttrs {
    /// Parses `#[command(rename_rule = "snake_case", prefix = '!')]`, the defaults if the enum has no such attribute
    fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let (mut rename_rule, mut prefix, mut split) = (None, None, None);
        parse_attr("command", attrs, |meta| {
            if meta.path.is_ident("rename_rule") {
                let val = RenameRule::parse(&meta.value()?.parse()?)?;
                set_once(&mut rename_rule, &meta.path, val)
            } else if meta.path.is_ident("prefix") {
                let val = meta.value()?.parse::<LitChar>()?.value();
                set_once(&mut prefix, &meta.path, val)
            } else if meta.path.is_ident("split") {
                let val = meta.value()?.parse::<LitChar>()?.value();
                set_once(&mut split, &meta.path, val)
            } else {
                Err(meta.error("expected `rename_rule`, `prefix` or `split` attribute"))
            }
        })?;

        Ok(Self {
            rename_rule: rename_rule.unwrap_or_default(),
            prefix,
            split,
        })
    }
}

/// Variant-level `#[command(...)]` attributes
#[derive(Default)]
struct VariantAttrs {
    description: Option<String>,
    hidden: bool,
    aliases: Vec<String>,
    rename: Option<String>,
    prefix: Option<char>,
    split: Option<char>,
}

impl VariantAttrs {
    /// Parses `#[command(description = "...", hidden, aliases = ["a"])]`, the defaults if the variant has no such attribute
    fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let (mut description, mut hidden, mut aliases, mut rename, mut prefix, mut split) =
            (None, None, None, None, None, None);
        parse_attr("command", attrs, |meta| {
            if meta.path.is_ident("description") {
                let val = meta.value()?.parse::<LitStr>()?.value();
                set_once(&mut description, &meta.path, val)
            } else if meta.path.is_ident("hidden") {
                set_once(&mut hidden, &meta.path, ())
            } else if meta.path.is_ident("aliases") {
                let input = meta.value()?;
                let content;
                syn::bracketed!(content in input);
                let val = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?
                    .into_iter()
                    .map(|alias| alias.value())
                    .collect();
                set_once(&mut aliases, &meta.path, val)
            } else if meta.path.is_ident("rename") {
                let val = meta.value()?.parse::<LitStr>()?.value();
                set_once(&mut rename, &meta.path, val)
            } else if meta.path.is_ident("prefix") {
                let val = meta.value()?.parse::<LitChar>()?.value();
                set_once(&mut prefix, &meta.path, val)
            } else if meta.path.is_ident("split") {
                let val = meta.value()?.parse::<LitChar>()?.value();
                set_once(&mut split, &meta.path, val)
            } else {
                Err(meta.error(
                    "expected `description`, `hidden`, `aliases`, `rename`, `prefix` or `split` \
                     attribute",
                ))
            }
        })?;

        Ok(Self {
            description,
            hidden: hidden.is_some(),
            aliases: aliases.unwrap_or_default(),
            rename,
            prefix,
            split,
        })
    }
}

/// Code generated for a single command variant
struct VariantCodegen {
    extractor_arm: TokenStream,
    descriptions_entry: TokenStream,
    bot_commands_entry: TokenStream,
    match_names: Vec<(char, String)>,
}

fn expand_variant(attrs: &CommandAttrs, variant: &syn::Variant) -> syn::Result<VariantCodegen> {
    let variant_attrs = VariantAttrs::parse(&variant.attrs)?;

    let name = variant_attrs
        .rename
        .unwrap_or_else(|| attrs.rename_rule.apply(&variant.ident));

    // The prefix is a part of the command, so `!start` and `/start` are different commands
    let prefix = variant_attrs.prefix.or(attrs.prefix).unwrap_or('/');

    let mut match_names = vec![(prefix, name.to_lowercase())];
    match_names.extend(
        variant_attrs
            .aliases
            .iter()
            .map(|alias| (prefix, alias.to_lowercase())),
    );

    let description = variant_attrs.description.as_deref();
    let hidden = variant_attrs.hidden;

    let descriptions_entry = if hidden {
        TokenStream::new()
    } else if let Some(description) = description {
        quote_spanned! { variant.span() =>
            concat!(#prefix, #name, " - ", #description),
        }
    } else {
        quote_spanned! { variant.span() =>
            concat!(#prefix, #name),
        }
    };
    // `setMyCommands` supports only commands with the `/` prefix
    let bot_commands_entry = if hidden || prefix != '/' {
        TokenStream::new()
    } else {
        let description = description.unwrap_or_default();

        quote_spanned! { variant.span() =>
            ::telers::types::BotCommand::new(#name, #description),
        }
    };

    let variant_ident = &variant.ident;
    let fields = &variant.fields;

    let local_idents = (0..fields.len())
        .map(|index| format_ident!("__field{index}"))
        .collect::<Vec<_>>();
    let field_idents = fields.iter().filter_map(|field| field.ident.as_ref());
    let field_tys = fields.iter().map(|field| &field.ty);

    let args_binding = quote_spanned! { variant.span() =>
        let (#(#local_idents,)*) = ::telers::utils::command_args::parse_args::<(#(#field_tys,)*)>(__cursor)
    };

    let construct = match fields {
        Fields::Unit => quote_spanned! { variant.span() => Self::#variant_ident },
        Fields::Unnamed(_) => {
            quote_spanned! { variant.span() => Self::#variant_ident(#(#local_idents),*) }
        }
        Fields::Named(_) => {
            let field_bindings = fields
                .iter()
                .zip(&local_idents)
                .map(|(field, local_ident)| {
                    let field_ident = field.ident.as_ref().expect("named fields are always named");
                    quote_spanned! { field_ident.span() => #field_ident: #local_ident }
                });
            quote_spanned! { variant.span() => Self::#variant_ident { #(#field_bindings),* } }
        }
    };

    let split = match variant_attrs.split.or(attrs.split) {
        None | Some(' ') => quote_spanned! { variant.span() =>
            ::telers::utils::command_args::SplitType::Whitespace
        },
        Some(split) => quote_spanned! { variant.span() =>
            ::telers::utils::command_args::SplitType::Char(#split)
        },
    };

    let body = quote_spanned! { variant.span() =>
        let __cursor = ::telers::utils::command_args::ArgsCursor::new(&__command.raw_args, #split);
        #args_binding.map_err(|err| Error::new_with_source(err.describe(#name, &[#(stringify!(#field_idents)),*]), err))?;
        ::std::result::Result::Ok(#construct)
    };

    let match_patterns = match_names
        .iter()
        .map(|(prefix, name)| quote_spanned! { variant.span() => (#prefix, #name) });

    let extractor_arm = quote_spanned! { variant.span() =>
        #(#match_patterns)|* => {
            #body
        }
    };

    Ok(VariantCodegen {
        extractor_arm,
        descriptions_entry,
        bot_commands_entry,
        match_names,
    })
}

fn expand_enum(item: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident,
        generics,
        attrs,
        data,
        ..
    } = item;

    if !generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            generics,
            "generic commands are not supported",
        ));
    }

    let command_attrs = CommandAttrs::parse(&attrs)?;

    let Data::Enum(data) = data else {
        return Err(syn::Error::new_spanned(
            &ident,
            "expected `enum` with `Command` derive",
        ));
    };

    let mut extractor_arms = Vec::new();
    let mut descriptions_entries = Vec::new();
    let mut bot_commands_entries = Vec::new();

    let mut seen_names = HashSet::new();
    for variant in &data.variants {
        let codegen = expand_variant(&command_attrs, variant)?;

        for name in codegen.match_names {
            if let Some((prefix, name)) = seen_names.replace(name) {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    format!("duplicate command name `{prefix}{name}`"),
                ));
            }
        }

        extractor_arms.push(codegen.extractor_arm);
        descriptions_entries.push(codegen.descriptions_entry);
        bot_commands_entries.push(codegen.bot_commands_entry);
    }

    let body = quote_spanned! { ident.span() =>
        use ::telers::errors::ExtractionError as Error;

        let __command = request.context
            .get::<::telers::filters::CommandObject>("command");

        async move {
            let __command = __command
                .ok_or_else(|| Error::new(
                    "No `command` in context: the `Command` filter must be used to parse the command. \
                     You didn't forget to add it to the handler?",
                ))?;
            match (__command.prefix, __command.command.to_lowercase().as_str()) {
                #(#extractor_arms)*
                _ => ::std::result::Result::Err(Error::new(format!(
                    "Unknown command `{}{}`",
                    __command.prefix, __command.command
                ))),
            }
        }
    };
    let self_ty = quote_spanned! { ident.span() => #ident };
    let generics = extractor_generics(
        &generics,
        [parse_quote! { #ident: ::std::marker::Send + 'static }],
    );
    let error = quote_spanned! { ident.span() => ::telers::errors::ExtractionError };
    let extractor_impl = tokenize_extractor_impl(ident.span(), &self_ty, &generics, &error, &body);

    let helpers_impl = quote_spanned! { ident.span() =>
        impl #ident {
            /// Returns the descriptions of the commands in the format `/command - description` (with the prefix of the command), separated by newlines
            #[must_use]
            pub fn descriptions() -> String {
                let descriptions: ::std::vec::Vec<&'static str> = ::std::vec![
                    #(#descriptions_entries)*
                ];

                descriptions.join("\n")
            }

            /// Returns the commands in the format required by the Telegram Bot API (`setMyCommands`).
            /// Only commands with the `/` prefix are included, because the method supports no other prefix
            #[must_use]
            pub fn bot_commands() -> ::std::vec::Vec<::telers::types::BotCommand> {
                ::std::vec![
                    #(#bot_commands_entries)*
                ]
            }
        }
    };

    Ok(quote_spanned! { ident.span() =>
        #extractor_impl
        #helpers_impl
    })
}

pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    expand_enum(input)
}
