use crate::attrs_parsing::parse_attr;

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Data, DeriveInput, Fields, Ident, Item, LitStr, Token, Type,
};

mod keywords {
    syn::custom_keyword!(rename_rule);
    syn::custom_keyword!(description);
}

/// Rename rule for command names
#[derive(Clone, Copy)]
enum RenameRule {
    Lower,
    SnakeCase,
    KebabCase,
    CamelCase,
    PascalCase,
    ScreamingSnakeCase,
}

impl RenameRule {
    fn parse(value: &LitStr) -> Result<Self, syn::Error> {
        let rule = value.value();

        let result = match rule.as_str() {
            "lowercase" => Self::Lower,
            "snake_case" => Self::SnakeCase,
            "kebab_case" => Self::KebabCase,
            "camel_case" => Self::CamelCase,
            "pascal_case" => Self::PascalCase,
            "screaming_snake_case" => Self::ScreamingSnakeCase,
            _ => {
                return Err(syn::Error::new_spanned(
                    value,
                    "expected one of: `lowercase`, `snake_case`, `kebab_case`, `camel_case`, \
                     `pascal_case`, `screaming_snake_case`",
                ))
            }
        };

        Ok(result)
    }

    fn apply(self, ident: &Ident) -> String {
        let name = ident.to_string();

        match self {
            Self::Lower => name.to_lowercase(),
            Self::SnakeCase => name.to_snake_case(),
            Self::KebabCase => name.to_kebab_case(),
            Self::CamelCase => name.to_lower_camel_case(),
            Self::PascalCase => name.to_upper_camel_case(),
            Self::ScreamingSnakeCase => name.to_shouty_snake_case(),
        }
    }
}

/// Enum-level `#[command(...)]` attributes
struct CommandAttrs {
    rename_rule: RenameRule,
}

impl Parse for CommandAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let rename_rule = if input.peek(keywords::rename_rule) {
            input.parse::<keywords::rename_rule>()?;
            input.parse::<Token![=]>()?;
            let value: LitStr = input.parse()?;
            Some(RenameRule::parse(&value)?)
        } else {
            None
        };

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        if !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "expected `rename_rule = \"...\"` attribute",
            ));
        }

        Ok(Self {
            rename_rule: rename_rule.unwrap_or(RenameRule::Lower),
        })
    }
}

/// Variant-level `#[command(...)]` attributes
#[derive(Default)]
struct VariantAttrs {
    description: Option<String>,
}

impl Parse for VariantAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let description = if input.peek(keywords::description) {
            input.parse::<keywords::description>()?;
            input.parse::<Token![=]>()?;
            let value: LitStr = input.parse()?;
            Some(value.value())
        } else {
            None
        };

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        if !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "expected `description = \"...\"` attribute",
            ));
        }

        Ok(Self {
            description,
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
}

fn expand_variant(rule: RenameRule, variant: &syn::Variant) -> Result<VariantCodegen, syn::Error> {
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

    let name = rule.apply(&variant.ident);
    let name_lower = name.to_lowercase();
    let description = variant_attrs.description.unwrap_or_default();

    let descriptions_entry = quote_spanned! { variant.span() =>
        concat!("/", #name, " - ", #description),
    };
    let bot_commands_entry = quote_spanned! { variant.span() =>
        ::telers::types::BotCommand::new(#name, #description),
    };

    let fields = fields_of(&variant.fields);
    let field_count = fields.len();
    let field_tys = fields.iter().map(|field| field.ty.clone()).collect();

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

    let extractor_arm = quote_spanned! { variant.span() =>
        #name_lower => {
            #(#args_binding)*
            #(#parsed_fields)*
            #construct
        }
    };

    Ok(VariantCodegen {
        extractor_arm,
        descriptions_entry,
        bot_commands_entry,
        field_tys,
    })
}

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

    for variant in &data.variants {
        let codegen = expand_variant(command_attrs.rename_rule, variant)?;

        extractor_arms.push(codegen.extractor_arm);
        descriptions_entries.push(codegen.descriptions_entry);
        bot_commands_entries.push(codegen.bot_commands_entry);
        all_field_tys.extend(codegen.field_tys);
    }

    let extractor_impl = quote_spanned! { ident.span() =>
        #[automatically_derived]
        impl<__C> ::telers::Extractor<__C> for #ident
        where
            #ident: ::std::clone::Clone + Send + 'static,
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
