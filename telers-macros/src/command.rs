use crate::attrs_parsing::parse_attr;

use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens};
use std::collections::HashSet;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Data, DeriveInput, Fields, Ident, Item, LitChar, LitStr, Token, Type,
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

/// Enum-level `#[command(...)]` attributes
struct CommandAttrs {
    rename_rule: RenameRule,
    parse_with: Option<syn::Path>,
    prefix: Option<char>,
}

impl Parse for CommandAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut rename_rule = None;
        let mut parse_with = None;
        let mut prefix = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(Token![,]) {
                input.parse::<Token![,]>()?;

                continue;
            }

            if lookahead.peek(keywords::rename_rule) {
                let input_rename_rule: keywords::rename_rule = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitStr = input.parse()?;
                let rule = RenameRule::parse(&value)?;

                if rename_rule.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_rename_rule,
                        "duplicate `rename_rule` attribute",
                    ));
                }

                rename_rule = Some(rule);

                continue;
            }

            if lookahead.peek(keywords::parse_with) {
                let input_parse_with: keywords::parse_with = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: syn::Path = input.parse()?;

                if parse_with.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_parse_with,
                        "duplicate `parse_with` attribute",
                    ));
                }

                parse_with = Some(value);

                continue;
            }

            if lookahead.peek(keywords::prefix) {
                let input_prefix: keywords::prefix = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitChar = input.parse()?;

                if prefix.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_prefix,
                        "duplicate `prefix` attribute",
                    ));
                }

                prefix = Some(value.value());

                continue;
            }

            // If we found unknown attribute, then we need to return error
            return Err(syn::Error::new(
                input.span(),
                "expected `rename_rule`, `parse_with` or `prefix` attribute",
            ));
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
    parse_with: Option<syn::Path>,
    prefix: Option<char>,
}

#[allow(clippy::too_many_lines)]
impl Parse for VariantAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut description = None;
        let mut hidden = None;
        let mut aliases = None;
        let mut rename = None;
        let mut parse_with = None;
        let mut prefix = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(Token![,]) {
                input.parse::<Token![,]>()?;

                continue;
            }

            if lookahead.peek(keywords::description) {
                let input_description: keywords::description = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitStr = input.parse()?;

                if description.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_description,
                        "duplicate `description` attribute",
                    ));
                }

                description = Some(value.value());

                continue;
            }

            if lookahead.peek(keywords::hidden) {
                let input_hidden: keywords::hidden = input.parse()?;

                if hidden.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_hidden,
                        "duplicate `hidden` attribute",
                    ));
                }

                hidden = Some(());

                continue;
            }

            if lookahead.peek(keywords::aliases) {
                let input_aliases: keywords::aliases = input.parse()?;

                if aliases.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_aliases,
                        "duplicate `aliases` attribute",
                    ));
                }

                input.parse::<Token![=]>()?;

                let content;
                syn::bracketed!(content in input);

                let mut values = Vec::new();
                while !content.is_empty() {
                    values.push(content.parse::<LitStr>()?.value());

                    if content.is_empty() {
                        break;
                    }

                    content.parse::<Token![,]>()?;
                }

                aliases = Some(values);

                continue;
            }

            if lookahead.peek(keywords::rename) {
                let input_rename: keywords::rename = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitStr = input.parse()?;

                if rename.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_rename,
                        "duplicate `rename` attribute",
                    ));
                }

                rename = Some(value.value());

                continue;
            }

            if lookahead.peek(keywords::parse_with) {
                let input_parse_with: keywords::parse_with = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: syn::Path = input.parse()?;

                if parse_with.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_parse_with,
                        "duplicate `parse_with` attribute",
                    ));
                }

                parse_with = Some(value);

                continue;
            }

            if lookahead.peek(keywords::prefix) {
                let input_prefix: keywords::prefix = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitChar = input.parse()?;

                if prefix.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_prefix,
                        "duplicate `prefix` attribute",
                    ));
                }

                prefix = Some(value.value());

                continue;
            }

            // If we found unknown attribute, then we need to return error
            return Err(syn::Error::new(
                input.span(),
                "expected `description`, `hidden`, `aliases`, `rename`, `parse_with` or `prefix` \
                 attribute",
            ));
        }

        Ok(Self {
            description,
            hidden: hidden.is_some(),
            aliases: aliases.unwrap_or_default(),
            rename,
            parse_with,
            prefix,
        })
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

    let parse_with_path = variant_attrs
        .parse_with
        .as_ref()
        .or(attrs.parse_with.as_ref());

    let prefix = variant_attrs.prefix.or(attrs.prefix);
    let prefix_check = if let Some(prefix) = prefix {
        quote_spanned! { variant.span() =>
            if __command.prefix != #prefix {
                return Err(Error::new(format!(
                    "Unknown command `{}{}`",
                    __command.prefix, __command.command
                )));
            }
        }
    } else {
        TokenStream::new()
    };

    let body = if let Some(path) = parse_with_path {
        quote_spanned! { variant.span() =>
            let __args_str = __command.args.join(" ");
            #path(&__args_str).map_err(|err| Error::new(format!(
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
                let mut __args = __command.args.iter();
            }]
        };

        let parsed_fields = fields.iter().enumerate().map(|(index, field)| {
            let local_ident = format_ident!("__field{index}");
            let field_ty = &field.ty;
            let ty_name = field_ty.to_token_stream().to_string();

            quote_spanned! { field.ty.span() =>
                let #local_ident: #field_ty = __args.next()
                    .ok_or_else(|| Error::new(format!(
                        concat!(
                            "Not enough arguments for `", #name, "` command: expected ",
                            #field_count, ", got {}",
                        ),
                        __command.args.len(),
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
                let local_idents = fields
                    .iter()
                    .enumerate()
                    .map(|(index, _)| format_ident!("__field{index}"));
                quote_spanned! { variant.span() => Ok(Self::#variant_ident(#(#local_idents),*)) }
            }
            Fields::Named(_) => {
                let field_bindings = fields.iter().enumerate().map(|(index, field)| {
                    let field_ident = &field.ident;
                    let local_ident = format_ident!("__field{index}");
                    quote_spanned! { field.ident.span() => #field_ident: #local_ident }
                });
                quote_spanned! { variant.span() => Ok(Self::#variant_ident { #(#field_bindings),* }) }
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
                    let __command = request.context
                        .get::<::telers::filters::CommandObject>("command")
                        .ok_or_else(|| Error::new(
                            "No `command` in context: the `Command` filter must be used to parse the command. \
                             You didn't forget to add it to the handler?",
                        ))?;
                    let __command_name = __command.command.to_lowercase();

                    match __command_name.as_str() {
                        #(#extractor_arms)*
                        _ => Err(Error::new(format!("Unknown command `{}`", __command.command))),
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
                let descriptions: ::std::vec::Vec<&'static str> = ::std::vec![
                    #(#descriptions_entries)*
                ];

                descriptions.join("\n")
            }

            /// Returns the commands in the format required by the Telegram Bot API (`setMyCommands`)
            #[must_use]
            pub fn bot_commands() -> ::std::vec::Vec<::telers::types::BotCommand> {
                let commands: ::std::vec::Vec<::telers::types::BotCommand> = ::std::vec![
                    #(#bot_commands_entries)*
                ];

                commands
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
