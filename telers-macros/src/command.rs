use crate::attrs_parsing::parse_attr;

use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens};
use std::collections::HashSet;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Data, DeriveInput, Fields, Ident, Item, LitStr, Token, Type,
};

mod keywords {
    syn::custom_keyword!(rename_rule);
    syn::custom_keyword!(description);
    syn::custom_keyword!(hidden);
    syn::custom_keyword!(aliases);
    syn::custom_keyword!(rename);
    syn::custom_keyword!(parse_with);
    syn::custom_keyword!(prefix);
}

/// Rename rule for command names
#[derive(Clone, Copy)]
enum RenameRule {
    /// `UserName` -> `username`
    Lower,
    /// `UserName` -> `user_name`
    SnakeCase,
    /// `UserName` -> `UserName`
    PascalCase,
    /// `UserName` -> `userName`
    CamelCase,
}

impl RenameRule {
    fn parse(value: &LitStr) -> Result<Self, syn::Error> {
        match value.value().as_str() {
            "lowercase" => Ok(Self::Lower),
            "snake_case" => Ok(Self::SnakeCase),
            "pascal_case" => Ok(Self::PascalCase),
            "camel_case" => Ok(Self::CamelCase),
            _ => Err(syn::Error::new_spanned(
                value,
                "expected one of: `lowercase`, `snake_case`, `pascal_case`, `camel_case`",
            )),
        }
    }

    fn apply(self, ident: &Ident) -> String {
        match self {
            Self::Lower => ident.to_string().to_lowercase(),
            Self::SnakeCase => ident.to_string().to_snake_case(),
            Self::PascalCase => ident.to_string().to_upper_camel_case(),
            Self::CamelCase => ident.to_string().to_lower_camel_case(),
        }
    }
}

/// Parse `#[command(keyword = "value")]` and return the `value` of `keyword`
macro_rules! parse_attr_value {
    ($input:ident, $kw:path, $ty:ty, $attr_name:literal) => {{
        let value = if $input.peek($kw) {
            $input.parse::<$kw>()?;
            $input.parse::<Token![=]>()?;
            Some($input.parse::<$ty>()?)
        } else {
            None
        };

        if $input.peek(Token![,]) {
            $input.parse::<Token![,]>()?;
        }

        value
    }};
}

/// Parse `#[command(keyword = ["a", "b"])]` and return the list of values
macro_rules! parse_attr_list {
    ($input:ident, $kw:path, $attr_name:literal) => {{
        let value = if $input.peek($kw) {
            $input.parse::<$kw>()?;
            $input.parse::<Token![=]>()?;

            let content;
            syn::bracketed!(content in $input);

            let mut values = Vec::new();
            while !content.is_empty() {
                values.push(content.parse::<LitStr>()?);
                if content.is_empty() {
                    break;
                }
                content.parse::<Token![,]>()?;
            }

            Some(values)
        } else {
            None
        };

        if $input.peek(Token![,]) {
            $input.parse::<Token![,]>()?;
        }

        value
    }};
}

/// Parse a bare flag `#[command(keyword)]` and return whether it is present
macro_rules! parse_attr_flag {
    ($input:ident, $kw:path) => {{
        let value = if $input.peek($kw) {
            $input.parse::<$kw>()?;
            true
        } else {
            false
        };
        if $input.peek(Token![,]) {
            $input.parse::<Token![,]>()?;
        }
        value
    }};
}

/// Parse a `prefix` literal, which must be a single character
fn parse_prefix(value: &LitStr) -> Result<char, syn::Error> {
    let string = value.value();
    let mut chars = string.chars();
    match (chars.next(), chars.next()) {
        (Some(prefix), None) => Ok(prefix),
        _ => Err(syn::Error::new_spanned(
            value,
            "`prefix` must be a single character",
        )),
    }
}

/// Enum-level `#[command(...)]` attributes
struct CommandAttrs {
    rename_rule: RenameRule,
    parse_with: Option<String>,
    prefix: Option<char>,
}

impl Parse for CommandAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut rename_rule = None;
        let mut parse_with = None;
        let mut prefix = None;

        while !input.is_empty() {
            if input.peek(keywords::rename_rule) {
                let value = parse_attr_value!(input, keywords::rename_rule, LitStr, "rename_rule")
                    .expect("peeked `rename_rule`");
                rename_rule = Some(RenameRule::parse(&value)?);
            } else if input.peek(keywords::parse_with) {
                let value = parse_attr_value!(input, keywords::parse_with, LitStr, "parse_with")
                    .expect("peeked `parse_with`");
                parse_with = Some(value.value());
            } else if input.peek(keywords::prefix) {
                let value = parse_attr_value!(input, keywords::prefix, LitStr, "prefix")
                    .expect("peeked `prefix`");
                prefix = Some(parse_prefix(&value)?);
            } else {
                return Err(
                    input.error("expected `rename_rule`, `parse_with` or `prefix` attribute")
                );
            }
        }

        Ok(Self {
            rename_rule: rename_rule.unwrap_or(RenameRule::Lower),
            parse_with,
            prefix,
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
    parse_with: Option<String>,
    prefix: Option<char>,
}

impl Parse for VariantAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut attrs = VariantAttrs::default();

        while !input.is_empty() {
            if input.peek(keywords::description) {
                let value = parse_attr_value!(input, keywords::description, LitStr, "description")
                    .expect("peeked `description`");
                attrs.description = Some(value.value());
            } else if input.peek(keywords::hidden) {
                parse_attr_flag!(input, keywords::hidden);
                attrs.hidden = true;
            } else if input.peek(keywords::aliases) {
                let values = parse_attr_list!(input, keywords::aliases, "aliases")
                    .expect("peeked `aliases`");
                attrs.aliases = values.into_iter().map(|value| value.value()).collect();
            } else if input.peek(keywords::rename) {
                let value = parse_attr_value!(input, keywords::rename, LitStr, "rename")
                    .expect("peeked `rename`");
                attrs.rename = Some(value.value());
            } else if input.peek(keywords::parse_with) {
                let value = parse_attr_value!(input, keywords::parse_with, LitStr, "parse_with")
                    .expect("peeked `parse_with`");
                attrs.parse_with = Some(value.value());
            } else if input.peek(keywords::prefix) {
                let value = parse_attr_value!(input, keywords::prefix, LitStr, "prefix")
                    .expect("peeked `prefix`");
                attrs.prefix = Some(parse_prefix(&value)?);
            } else {
                return Err(input.error(
                    "expected `description`, `hidden`, `aliases`, `rename`, `parse_with` or \
                     `prefix` attribute",
                ));
            }
        }

        Ok(attrs)
    }
}

/// Field of a command variant: identifier (if named) and type
struct CommandField {
    ident: Ident,
    ty: Type,
}

/// Parse a field of a tuple or named variant
///
/// For tuple variants we generate artificial identifiers (`field0`, `field1`, ...).
fn fields_of(fields: &Fields) -> Vec<CommandField> {
    match fields {
        Fields::Unit => vec![],
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| CommandField {
                ident: format_ident!("field{index}"),
                ty: field.ty.clone(),
            })
            .collect(),
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| CommandField {
                ident: field.ident.clone().expect("named fields are always named"),
                ty: field.ty.clone(),
            })
            .collect(),
    }
}

/// Code generated for a single command variant
struct VariantCodegen {
    extractor_arm: TokenStream,
    descriptions_entry: TokenStream,
    bot_commands_entry: TokenStream,
    field_tys: Vec<Type>,
    match_names: Vec<String>,
}

#[allow(clippy::too_many_lines)]
fn expand_variant(
    attrs: &CommandAttrs,
    variant: &syn::Variant,
) -> Result<VariantCodegen, syn::Error> {
    let variant_attrs = match parse_attr("command", &variant.attrs) {
        Ok(Some(attrs)) => attrs,
        Ok(None) => VariantAttrs::default(),
        Err(err) => {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                format!("failed to parse `#[command(...)]` attributes: {err}"),
            ))
        }
    };

    let name = match &variant_attrs.rename {
        Some(rename) => rename.clone(),
        None => attrs.rename_rule.apply(&variant.ident),
    };
    let name_lower = name.to_lowercase();

    let mut match_names = vec![name_lower.clone()];
    match_names.extend(
        variant_attrs
            .aliases
            .iter()
            .map(|alias| alias.to_lowercase()),
    );

    let description = variant_attrs.description;
    let bot_description = description.clone().unwrap_or_default();

    let hidden = variant_attrs.hidden;

    let descriptions_entry = if hidden {
        TokenStream::new()
    } else if let Some(description) = &description {
        quote_spanned! { variant.span() =>
            concat!("/", #name, " - ", #description),
        }
    } else {
        quote_spanned! { variant.span() =>
            concat!("/", #name),
        }
    };
    let bot_commands_entry = if hidden {
        TokenStream::new()
    } else {
        quote_spanned! { variant.span() =>
            ::telers::types::BotCommand::new(#name, #bot_description),
        }
    };

    let parse_with = variant_attrs
        .parse_with
        .as_ref()
        .or(attrs.parse_with.as_ref());
    let parse_with_path = match parse_with {
        Some(path) => Some(syn::parse_str::<syn::Path>(path).map_err(|err| {
            syn::Error::new_spanned(&variant.ident, format!("invalid `parse_with` path: {err}"))
        })?),
        None => None,
    };

    let prefix = variant_attrs.prefix.or(attrs.prefix);
    let prefix_check = if let Some(prefix) = prefix {
        quote_spanned! { variant.span() =>
            if command.prefix != #prefix {
                return Err(Error::new(format!(
                    "Unknown command `{}{}`",
                    command.prefix, command.command
                )));
            }
        }
    } else {
        TokenStream::new()
    };

    let body = if let Some(path) = &parse_with_path {
        quote_spanned! { variant.span() =>
            let args_str = command.args.join(" ");
            #path(&args_str).map_err(|err| Error::new(format!(
                concat!("Failed to parse arguments for `", #name, "` command: {}"),
                err,
            )))
        }
    } else {
        let fields = fields_of(&variant.fields);
        let field_count = fields.len();

        let variant_ident = &variant.ident;

        let args_binding = if fields.is_empty() {
            Vec::new()
        } else {
            vec![quote_spanned! { variant.span() =>
                let mut args = command.args.iter();
            }]
        };

        let parsed_fields = fields.iter().map(|field| {
            let field_ident = &field.ident;
            let field_ty = &field.ty;
            let ty_name = field_ty.to_token_stream().to_string();

            quote_spanned! { field.ty.span() =>
                let #field_ident: #field_ty = args.next()
                    .ok_or_else(|| Error::new(format!(
                        concat!(
                            "Not enough arguments for `", #name, "` command: expected ",
                            #field_count, ", got {}",
                        ),
                        command.args.len(),
                    )))?
                    .parse()
                    .map_err(|err| Error::new(format!(
                        concat!("Failed to parse `", #ty_name, "` for `", #name, "` command: {}"),
                        err,
                    )))?;
            }
        });

        let construct = match &variant.fields {
            Fields::Unit => quote_spanned! { variant.span() => Ok(Self::#variant_ident) },
            Fields::Unnamed(_) => {
                let field_idents = fields.iter().map(|field| &field.ident);
                quote_spanned! { variant.span() => Ok(Self::#variant_ident(#(#field_idents),*)) }
            }
            Fields::Named(_) => {
                let field_idents = fields.iter().map(|field| &field.ident);
                quote_spanned! { variant.span() => Ok(Self::#variant_ident { #(#field_idents),* }) }
            }
        };

        quote_spanned! { variant.span() =>
            #(#args_binding)*
            #(#parsed_fields)*
            #construct
        }
    };

    let field_tys = if parse_with_path.is_some() {
        Vec::new()
    } else {
        fields_of(&variant.fields)
            .iter()
            .map(|field| field.ty.clone())
            .collect()
    };

    let match_pattern = if match_names.len() == 1 {
        let name = &match_names[0];
        quote_spanned! { variant.span() => #name }
    } else {
        let name = &match_names[0];
        let aliases = &match_names[1..];
        quote_spanned! { variant.span() => #name | #(#aliases)|* }
    };

    let extractor_arm = quote_spanned! { variant.span() =>
        #match_pattern => {
            #prefix_check
            #body
        }
    };

    Ok(VariantCodegen {
        extractor_arm,
        descriptions_entry,
        bot_commands_entry,
        field_tys,
        match_names,
    })
}

#[allow(clippy::too_many_lines)]
fn expand_enum(item: DeriveInput) -> Result<TokenStream, syn::Error> {
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

    let command_attrs = match parse_attr("command", &attrs) {
        Ok(Some(attrs)) => attrs,
        Ok(None) => CommandAttrs {
            rename_rule: RenameRule::Lower,
            parse_with: None,
            prefix: None,
        },
        Err(err) => {
            return Err(syn::Error::new_spanned(
                &ident,
                format!("failed to parse `#[command(...)]` attributes: {err}"),
            ))
        }
    };

    let Data::Enum(data) = data else {
        unreachable!("`expand` checks that item is an enum")
    };

    let mut extractor_arms = Vec::new();
    let mut descriptions_entries = Vec::new();
    let mut bot_commands_entries = Vec::new();
    let mut all_field_tys = Vec::new();

    let mut seen_names = HashSet::new();
    for variant in &data.variants {
        let codegen = expand_variant(&command_attrs, variant)?;

        for name in &codegen.match_names {
            if !seen_names.insert(name.clone()) {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    format!("duplicate command name `{name}`"),
                ));
            }
        }

        extractor_arms.push(codegen.extractor_arm);
        descriptions_entries.push(codegen.descriptions_entry);
        bot_commands_entries.push(codegen.bot_commands_entry);
        all_field_tys.extend(codegen.field_tys);
    }

    let extractor_impl_generics = quote_spanned! { ident.span() =>
        impl<__C> ::telers::Extractor<__C> for #ident
    };

    let extractor_impl = quote_spanned! { ident.span() =>
        #[automatically_derived]
        #extractor_impl_generics
        where
            #ident: Send + 'static,
            #(#all_field_tys: ::std::str::FromStr,)*
            #(<#all_field_tys as ::std::str::FromStr>::Err: ::std::fmt::Display,)*
        {
            type Error = ::telers::errors::ExtractionError;

            #[inline]
            fn extract(request: &::telers::Request<__C>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                use ::telers::errors::ExtractionError as Error;

                let res = (|| -> Result<Self, Error> {
                    let command = request.context
                        .get::<::telers::filters::CommandObject>("command")
                        .ok_or_else(|| Error::new(
                            "No `command` in context: the `Command` filter must be used to parse the command. \
                             You didn't forget to add it to the handler?",
                        ))?;
                    let command_name = command.command.to_lowercase();

                    match command_name.as_str() {
                        #(#extractor_arms)*
                        _ => Err(Error::new(format!("Unknown command `{}`", command.command))),
                    }
                })();

                async move { res }
            }
        }
    };

    let helpers_impl = quote_spanned! { ident.span() =>
        impl #ident {
            /// Returns the descriptions of the commands in the format `/command - description`, separated by newlines
            #[must_use]
            pub fn descriptions() -> String {
                [
                    #(#descriptions_entries)*
                ]
                .join("\n")
            }

            /// Returns the commands in the format required by the Telegram Bot API (`setMyCommands`)
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

pub(crate) fn expand(item: Item) -> Result<TokenStream, syn::Error> {
    let Item::Enum(item) = item else {
        return Err(syn::Error::new_spanned(
            item,
            "expected `enum` with `Command` derive",
        ));
    };

    expand_enum(DeriveInput::from(item))
}
