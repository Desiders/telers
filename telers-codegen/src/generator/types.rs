#![allow(clippy::too_many_lines, clippy::missing_panics_doc)]

use crate::{
    file::camel_to_filename,
    generator::helpers::{
        format_attr_description, format_description, get_singular_and_plural_forms,
        sanitize_field_name,
    },
    parser::api::{
        IntegerKind, NormalizedField, NormalizedSchema, NormalizedSubtypeVariant, NormalizedType,
        SubtypeKind, TypeKindInField,
    },
};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::{BTreeMap, HashSet};

struct TypeDocContext<'a> {
    schema_type_names: &'a HashSet<String>,
}

enum AccessExpr {
    Plain(TokenStream),
    Optional(TokenStream),
    WrapInSome(TokenStream),
    EnumMethod {
        method: Ident,
        returns_option: bool,
        wrap_in_some: bool,
    },
}

fn collect_telegram_type_names(kind: &TypeKindInField, out: &mut HashSet<String>) {
    match kind {
        TypeKindInField::Telegram(name) => {
            out.insert(name.clone());
        }
        TypeKindInField::Array(inner) => collect_telegram_type_names(inner, out),
        TypeKindInField::Either(left, right) => {
            collect_telegram_type_names(left, out);
            collect_telegram_type_names(right, out);
        }
        _ => {}
    }
}

#[must_use]
fn link_known_type_mentions(doc: &str, names: &HashSet<String>) -> String {
    let mut out = String::with_capacity(doc.len() + 32);
    let mut rest = doc;

    while let Some(pos) = rest.find('`') {
        out.push_str(&rest[..pos]);

        // Handle bracketed form: [`Type`]
        if pos > 0 && rest.as_bytes()[pos - 1] == b'[' {
            out.pop(); // remove '[' that was already pushed
            let after_tick = &rest[pos + 1..];
            if let Some(end_tick) = after_tick.find('`') {
                let token = &after_tick[..end_tick];
                let after_end_tick = &after_tick[end_tick + 1..];
                if let Some(after_bracket) = after_end_tick.strip_prefix(']') {
                    if names.contains(token) {
                        out.push_str("[`crate::types::");
                        out.push_str(token);
                        out.push_str("`]");
                    } else {
                        out.push_str("[`");
                        out.push_str(token);
                        out.push_str("`]");
                    }
                    rest = after_bracket;
                    continue;
                }
            }
            out.push('[');
            out.push('`');
            rest = &rest[pos + 1..];
            continue;
        }

        // Handle plain backticked form: `Type`
        let after_tick = &rest[pos + 1..];
        if let Some(end_tick) = after_tick.find('`') {
            let token = &after_tick[..end_tick];
            if names.contains(token) {
                out.push_str("[`crate::types::");
                out.push_str(token);
                out.push_str("`]");
            } else {
                out.push('`');
                out.push_str(token);
                out.push('`');
            }
            rest = &after_tick[end_tick + 1..];
        } else {
            out.push('`');
            out.push_str(after_tick);
            rest = "";
            break;
        }
    }

    out.push_str(rest);
    out
}

#[must_use]
fn format_field_doc(description: &str, kind: &TypeKindInField, ctx: &TypeDocContext<'_>) -> String {
    let mut names = HashSet::new();
    collect_telegram_type_names(kind, &mut names);
    let doc = format_attr_description(description);
    let doc = link_known_type_mentions(&doc, &names);
    normalize_doc_line_prefix(&link_schema_type_mentions(&doc, ctx))
}

#[must_use]
fn format_field_arg_doc(field: &NormalizedField, ctx: &TypeDocContext<'_>) -> String {
    let doc = format_attr_description(&format!("* `{}` - {}", field.name, field.description));
    let mut names = HashSet::new();
    collect_telegram_type_names(&field.r#type, &mut names);
    let doc = link_known_type_mentions(&doc, &names);
    normalize_doc_line_prefix(&link_schema_type_mentions(&doc, ctx))
}

#[must_use]
fn link_schema_type_mentions(doc: &str, ctx: &TypeDocContext<'_>) -> String {
    link_known_type_mentions(doc, ctx.schema_type_names)
}

#[must_use]
fn normalize_doc_line_prefix(doc: &str) -> String {
    format!(" {}", doc.trim_start())
}

impl ToTokens for TypeKindInField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = match self {
            TypeKindInField::String => quote! { Box<str> },
            TypeKindInField::Integer(kind) => match kind {
                IntegerKind::Int8 => quote! { i8 },
                IntegerKind::Int16 => quote! { i16 },
                IntegerKind::Int32 => quote! { i32 },
                IntegerKind::Int64 => quote! { i64 },
                IntegerKind::UInt8 => quote! { u8 },
                IntegerKind::UInt16 => quote! { u16 },
                IntegerKind::UInt32 => quote! { u32 },
                IntegerKind::UInt64 => quote! { u64 },
                IntegerKind::Float32 => quote! { f32 },
                IntegerKind::Float64 => quote! { f64 },
            },
            TypeKindInField::Boolean(_) => quote! { bool },
            TypeKindInField::Telegram(name) => {
                let ident = format_ident!("{name}");
                quote! { crate::types::#ident }
            }
            TypeKindInField::InputFile => quote! { crate::types::InputFile },
            TypeKindInField::ChatId => quote! { crate::types::ChatIdKind },
            TypeKindInField::Array(inner) => quote! { Box<[#inner]> },
            TypeKindInField::Either(left, right) => {
                quote! { crate::Either<#left, #right> }
            }
        };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedSubtypeVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = format_ident!("{}", self.variant);
        let name = format_ident!("{}", self.ty_name);
        tokens.extend(quote! { #variant(crate::types::#name), });
    }
}

fn tokenize_field(field: &NormalizedField, ctx: &TypeDocContext<'_>) -> TokenStream {
    let name = sanitize_field_name(&field.name);
    let doc = format_field_doc(&field.description, &field.r#type, ctx);
    let raw_ty = &field.r#type;
    let ty = if field.is_recursive || field.is_boxed {
        quote! { Box<#raw_ty> }
    } else {
        quote! { #raw_ty }
    };

    if field.required {
        quote! { #[doc = #doc] pub #name: #ty, }
    } else {
        quote! {
            #[doc = #doc]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #name: Option<#ty>,
        }
    }
}

fn tokenize_type_definition(type_quote: &NormalizedType, ctx: &TypeDocContext<'_>) -> TokenStream {
    let name = format_ident!("{}", type_quote.name.as_str());
    let mut doc_lines = format_description(&type_quote.description, &type_quote.href);
    doc_lines = doc_lines
        .into_iter()
        .map(|line| link_schema_type_mentions(&line, ctx))
        .collect();
    doc_lines = link_prefixed_type_mentions(doc_lines, &type_quote.name);
    for subtype in &type_quote.subtypes {
        let type_name = &subtype.ty_name;
        let code = format!("`{type_name}`");
        let bare_link = format!("[`{type_name}`]");
        let path_link = format!("[`crate::types::{type_name}`]");
        for line in &mut doc_lines {
            if line.contains(&code) {
                *line = line.replace(&code, &path_link);
            }
            if line.contains(&bare_link) {
                *line = line.replace(&bare_link, &path_link);
            }
        }
    }

    let (tag_field, parent_tag_field) = type_quote
        .subtype_kind
        .as_ref()
        .map(|kind| kind.get_tags())
        .unwrap_or_default();

    let derive_quotes = get_derives_for_types(type_quote);
    if type_quote.subtypes.is_empty() {
        let fields = type_quote
            .fields
            .iter()
            .filter(|f| !f.is_tagged(tag_field, parent_tag_field))
            .map(|f| tokenize_field(f, ctx));
        let extra_field = if type_quote.has_extra_fields {
            quote! {
                #[serde(flatten)]
                pub extra: BTreeMap<Box<str>, serde_json::Value>,
            }
        } else {
            quote! {}
        };
        quote! {
            #( #[doc = #doc_lines] )*
            #( #derive_quotes )*
            pub struct #name {
                #( #fields )*
                #extra_field
            }
        }
    } else {
        let serde_attr = match &type_quote.subtype_kind {
            Some(SubtypeKind::Tagged {
                tag_field, ..
            }) => {
                quote! { #[serde(tag = #tag_field, rename_all = "snake_case")] }
            }
            Some(
                SubtypeKind::Untagged
                | SubtypeKind::UntaggedInTagged {
                    ..
                },
            ) => {
                quote! { #[serde(untagged)] }
            }
            None => quote! {},
        };
        let subtypes = type_quote.subtypes.iter();
        quote! {
            #( #[doc = #doc_lines] )*
            #( #derive_quotes )*
            #serde_attr
            pub enum #name {
                #( #subtypes )*
            }
        }
    }
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
pub fn get_from_impls_for_subtypes(type_quote: &NormalizedType) -> Vec<TokenStream> {
    let name = format_ident!("{}", type_quote.name);
    let variant_count = type_quote.subtypes.len();

    let mut impl_quotes = vec![];
    for subtype in &type_quote.subtypes {
        let subtype_name = format_ident!("{}", subtype.ty_name);
        let subtype_path = quote! { crate::types::#subtype_name };
        let subtype_variant = format_ident!("{}", subtype.variant);
        let try_from_body = if variant_count == 1 {
            quote! {
                let #name::#subtype_variant(inner) = val;
                Ok(inner)
            }
        } else if variant_count == 2 {
            let other_variant = type_quote
                .subtypes
                .iter()
                .find(|other| other.variant != subtype.variant)
                .map(|other| format_ident!("{}", other.variant))
                .expect("two-variant enum must contain the second variant");
            quote! {
                match val {
                    #name::#subtype_variant(inner) => Ok(inner),
                    #name::#other_variant(_) => Err(Self::Error::new(stringify!(#name), stringify!(#subtype_name))),
                }
            }
        } else {
            quote! {
                if let #name::#subtype_variant(inner) = val {
                    Ok(inner)
                } else {
                    Err(Self::Error::new(stringify!(#name), stringify!(#subtype_name)))
                }
            }
        };
        impl_quotes.push(quote! {
            impl From<#subtype_path> for #name {
                fn from(val: #subtype_path) -> Self {
                    Self::#subtype_variant(val)
                }
            }
            impl TryFrom<#name> for #subtype_path {
                type Error = crate::errors::ConvertToTypeError;
                fn try_from(val: #name) -> Result<Self, Self::Error> {
                    #try_from_body
                }
            }
        });
    }
    impl_quotes
}

#[must_use]
pub fn get_impls_for_types(
    type_quote: &NormalizedType,
    schema: &NormalizedSchema,
) -> Vec<TokenStream> {
    let name = format_ident!("{}", type_quote.name);

    let mut impl_quotes = vec![];

    if type_quote.is_update_variant() {
        let ty_field = type_quote
            .update_variant_ty_field()
            .expect("Update variant doesn't have type field");
        let field_name = format_ident!("{}", ty_field.name);
        let field_ty = &ty_field.r#type;

        let body = if ty_field.is_boxed || ty_field.is_recursive {
            quote! { *val.#field_name }
        } else {
            quote! { val.#field_name }
        };

        impl_quotes.push(quote! {
            impl From<#name> for #field_ty {
                fn from(val: #name) -> Self {
                    #body
                }
            }
        });
        impl_quotes.push(quote! {
            impl<Client> crate::Extractor<Client> for #name
            {
                type Error = crate::errors::ConvertToTypeError;
                fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                    let val = TryFrom::try_from((*request.update).clone());
                    async move { val }
                }
            }
        });
    }

    if type_quote.is_update() {
        impl_quotes.push(quote! {
            impl<Client> crate::Extractor<Client> for Update
            {
                type Error = std::convert::Infallible;
                fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                    let val = (*request.update).clone();
                    async move { Ok(val) }
                }
            }
        });
        impl_quotes.push(quote! {
            impl<Client> crate::Extractor<Client> for std::sync::Arc<Update>
            {
                type Error = std::convert::Infallible;
                fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                    let val = request.update.clone();
                    async move { Ok(val) }
                }
            }
        });

        // We need to collect all types that can be got from update variants,
        // for example `Message` from `UpdateMessage` and `UpdateBusinessMessage`,
        // so that we can generate `impl From<Update> for Message` for each of variants.
        let mut types_update_variants_with_field: BTreeMap<&str, Vec<_>> = BTreeMap::new();

        for subtype in &type_quote.subtypes {
            let variant_ty = schema.types.get(&subtype.ty_name).unwrap();
            let variant_field_ty = variant_ty
                .update_variant_ty_field()
                .expect("Update variant doesn't have type field");
            let TypeKindInField::Telegram(variant_field_ty_name) = &variant_field_ty.r#type else {
                panic!("Update variant type field must be Telegram type");
            };

            types_update_variants_with_field
                .entry(variant_field_ty_name.as_str())
                .or_default()
                .push((subtype.variant.as_str(), variant_field_ty));
        }

        for (variant_field_ty_name_str, variants_with_field) in types_update_variants_with_field {
            let variant_field_ty_name = format_ident!("{}", variant_field_ty_name_str);
            let variant_field_ty_path = quote! { crate::types::#variant_field_ty_name };

            let mut match_arms = vec![];
            for (variant, ty_field) in variants_with_field {
                let variant = format_ident!("{}", variant);
                let field_name = format_ident!("{}", ty_field.name);

                let body = if ty_field.is_boxed || ty_field.is_recursive {
                    quote! { *val.#field_name }
                } else {
                    quote! { val.#field_name }
                };

                match_arms.push(quote! {
                    Update::#variant(val) => Ok(#body)
                });
            }
            match_arms.push(quote! {
                _ => Err(crate::errors::ConvertToTypeError::new(stringify!(Update), stringify!(#variant_field_ty_name)))
            });

            impl_quotes.push(quote! {
                impl TryFrom<Update> for #variant_field_ty_path {
                    type Error = crate::errors::ConvertToTypeError;
                    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
                        match val {
                            #(#match_arms),*
                        }
                    }
                }
            });
            impl_quotes.push(quote! {
                impl<Client> crate::Extractor<Client> for #variant_field_ty_path
                {
                    type Error = crate::errors::ConvertToTypeError;
                    fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                        let val = (*request.update).clone().try_into();
                        async move { val }
                    }
                }
            });

            // If update inner type is an enum (for example `Message`), also expose
            // extractors for each of its variants (for example `MessageText`).
            let Some(inner_ty) = schema.types.get(variant_field_ty_name_str) else {
                continue;
            };
            if inner_ty.subtypes.is_empty() {
                continue;
            }

            let mut seen_subtype_names = HashSet::new();
            for subtype in &inner_ty.subtypes {
                if !seen_subtype_names.insert(subtype.ty_name.as_str()) {
                    continue;
                }

                let subtype_ty_name = format_ident!("{}", subtype.ty_name);
                let subtype_ty_path = quote! { crate::types::#subtype_ty_name };
                impl_quotes.push(quote! {
                    impl TryFrom<Update> for #subtype_ty_path {
                        type Error = crate::errors::ConvertToTypeError;
                        fn try_from(val: Update) -> Result<Self, Self::Error> {
                            let parent: #variant_field_ty_path = val.try_into()?;
                            parent.try_into()
                        }
                    }
                });
                impl_quotes.push(quote! {
                    impl<Client> crate::Extractor<Client> for #subtype_ty_path
                    {
                        type Error = crate::errors::ConvertToTypeError;
                        fn extract(request: &crate::Request<Client>) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
                            let val = (*request.update).clone().try_into();
                            async move { val }
                        }
                    }
                });
            }
        }
    }

    impl_quotes
}

#[must_use]
pub fn get_derives_for_types(_type_quote: &NormalizedType) -> Vec<TokenStream> {
    let derive_quotes = vec![
        quote! { #[derive(Clone, Debug)] },
        quote! { #[derive(Serialize, Deserialize)] },
    ];

    derive_quotes
}

#[must_use]
#[allow(clippy::too_many_lines)]
fn builder_impl_for_type(type_quote: &NormalizedType, ctx: &TypeDocContext<'_>) -> TokenStream {
    if !type_quote.subtypes.is_empty() {
        return quote! {};
    }

    let type_name = format_ident!("{}", type_quote.name);

    let (tag_field, parent_tag_field) = type_quote
        .subtype_kind
        .as_ref()
        .map(|kind| kind.get_tags())
        .unwrap_or_default();

    let fields: Box<[_]> = type_quote
        .fields
        .iter()
        .filter(|&f| !f.is_tagged(tag_field, parent_tag_field))
        .collect();
    let required_fields: Box<[_]> = fields.iter().filter(|&&f| f.required).copied().collect();
    let optional_fields: Box<[_]> = fields.iter().filter(|&&f| !f.required).copied().collect();

    let new_method_ts = {
        let doc_creates = format_attr_description(&format!("Creates a new `{}`.", type_quote.name));
        let mut doc_lines: Vec<TokenStream> = vec![quote! { #[doc = #doc_creates] }];

        if !required_fields.is_empty() {
            let doc_args = format_attr_description("# Arguments");
            doc_lines.push(quote! { #[doc = ""] #[doc = #doc_args] });
            for &field in &required_fields {
                let doc = format_field_arg_doc(field, ctx);
                doc_lines.push(quote! { #[doc = #doc] });
            }
        }
        if !optional_fields.is_empty() {
            let doc_notes = format_attr_description("# Notes");
            let doc_opt = format_attr_description("Use builder methods to set optional fields.");
            doc_lines.push(quote! { #[doc = ""] #[doc = #doc_notes] #[doc = #doc_opt] });
        }

        if fields.is_empty() {
            let extra_field = if type_quote.has_extra_fields {
                quote! {
                    extra: BTreeMap::new(),
                }
            } else {
                quote! {}
            };
            quote! {
                #( #doc_lines )*
                #[must_use]
                pub const fn new() -> Self {
                    Self {
                        #extra_field
                    }
                }
            }
        } else {
            let new_generics: Vec<_> = required_fields
                .iter()
                .enumerate()
                .flat_map(|(i, field)| {
                    let ty = &field.r#type;
                    let t = format_ident!("T{i}");
                    if let TypeKindInField::Array(inner) = ty {
                        let t_item = format_ident!("T{i}Item");
                        vec![
                            quote! { #t_item: Into<#inner> },
                            quote! { #t: IntoIterator<Item = #t_item> },
                        ]
                    } else {
                        vec![quote! { #t: Into<#ty> }]
                    }
                })
                .collect();
            let new_args: Vec<_> = required_fields
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let name = sanitize_field_name(&field.name);
                    let t = format_ident!("T{i}");
                    quote! { #name: #t }
                })
                .collect();
            let new_init = fields.iter().map(|field| {
                let name = sanitize_field_name(&field.name);
                if field.required {
                    if let TypeKindInField::Array(_) = &field.r#type {
                        quote! { #name: #name.into_iter().map(Into::into).collect() }
                    } else if field.is_recursive || field.is_boxed {
                        quote! { #name: Box::new(#name.into()) }
                    } else {
                        quote! { #name: #name.into() }
                    }
                } else {
                    quote! { #name: None }
                }
            });
            let extra_field = if type_quote.has_extra_fields {
                quote! {
                    extra: BTreeMap::new(),
                }
            } else {
                quote! {}
            };
            quote! {
                #( #doc_lines )*
                #[must_use]
                pub fn new<#( #new_generics ),*>(#( #new_args ),*) -> Self {
                    Self {
                        #( #new_init, )*
                        #extra_field
                    }
                }
            }
        }
    };

    let default_impl_ts = if required_fields.is_empty() {
        quote! {
            impl Default for #type_name {
                fn default() -> Self { Self::new() }
            }
        }
    } else {
        quote! {}
    };

    let builder_methods_ts: Vec<_> = fields
        .iter()
        .flat_map(|field| {
            let mut methods = vec![];
            let name = sanitize_field_name(&field.name);
            let ty = &field.r#type;

            if let TypeKindInField::Array(inner) = ty {
                let (singular, plural) = get_singular_and_plural_forms(&field.name);
                let singular_name = sanitize_field_name(&singular);
                let plural_name = sanitize_field_name(&plural);

                let make_doc = |note: &str| {
                    let desc = format_field_doc(&field.description, &field.r#type, ctx);
                    let notes = format_attr_description("# Notes");
                    let note = format_attr_description(note);
                    quote! { #[doc = #desc] #[doc = ""] #[doc = #notes] #[doc = #note] }
                };
                let make_body = |is_many: bool| -> TokenStream {
                    let op = if is_many {
                        quote! { val.into() }
                    } else {
                        quote! { Some(val.into()) }
                    };
                    if field.required {
                        quote! {
                            let mut this = self;
                            this.#name = this.#name.into_vec().into_iter().chain(#op).collect();
                            this
                        }
                    } else {
                        quote! {
                            let mut this = self;
                            this.#name = Some(this.#name.unwrap_or_default().into_vec().into_iter().chain(#op).collect());
                            this
                        }
                    }
                };

                let (plural_doc, plural_body) = (make_doc("Adds multiple elements."), make_body(true));
                methods.push(quote! {
                    #plural_doc
                    #[must_use]
                    pub fn #plural_name<T: Into<#ty>>(self, val: T) -> Self {
                        #plural_body
                    }
                });

                if singular_name != plural_name {
                    let (singular_doc, singular_body) = (make_doc("Adds a single element."), make_body(false));
                    methods.push(quote! {
                        #singular_doc
                        #[must_use]
                        pub fn #singular_name<T: Into<#inner>>(self, val: T) -> Self {
                            #singular_body
                        }
                    });
                }

                if !field.required {
                    let option_doc = make_doc("Adds a single element.");
                    let option_name = format_ident!("{}_option", field.name);
                    methods.push(quote! {
                        #option_doc
                        #[must_use]
                        pub fn #option_name<T: Into<#ty>>(self, val: Option<T>) -> Self {
                            let mut this = self;
                            this.#name = val.map(Into::into);
                            this
                        }
                    });
                }
            } else {
                let doc = format_field_doc(&field.description, &field.r#type, ctx);
                let boxed = field.is_recursive || field.is_boxed;
                let inner_value = if boxed { quote! { Box::new(val.into()) } } else { quote! { val.into() } };
                let value = if field.required { inner_value.clone() } else { quote! { Some(#inner_value) } };

                methods.push(quote! {
                    #[doc = #doc]
                    #[must_use]
                    pub fn #name<T: Into<#ty>>(self, val: T) -> Self {
                        let mut this = self;
                        this.#name = #value;
                        this
                    }
                });

                if !field.required {
                    let opt_value = if boxed {
                        quote! { val.map(|val| Box::new(val.into())) }
                    } else {
                        quote! { val.map(Into::into) }
                    };
                    let method_name = format_ident!("{}_option", field.name);
                    methods.push(quote! {
                        #[doc = #doc]
                        #[must_use]
                        pub fn #method_name<T: Into<#ty>>(self, val: Option<T>) -> Self {
                            let mut this = self;
                            this.#name = #opt_value;
                            this
                        }
                    });
                }
            }
            methods
        })
        .collect();

    quote! {
        impl #type_name {
            #new_method_ts
            #( #builder_methods_ts )*
        }
        #default_impl_ts
    }
}

#[must_use]
fn collect_common_fields<'a>(
    ty: &'a NormalizedType,
    schema: &'a NormalizedSchema,
) -> BTreeMap<&'a str, (&'a NormalizedField, bool, bool)> {
    let (tag_field, parent_tag_field) = ty
        .subtype_kind
        .as_ref()
        .map(|k| k.get_tags())
        .unwrap_or_default();

    if ty.subtypes.is_empty() {
        ty.fields
            .iter()
            .filter(|f| !f.is_tagged(tag_field, parent_tag_field))
            .map(|f| (f.name.as_str(), (f, f.required, true)))
            .collect()
    } else {
        let mut map: BTreeMap<&str, Vec<&NormalizedField>> = BTreeMap::new();
        for subtype in &ty.subtypes {
            let sub_ty = schema.types.get(&subtype.ty_name).unwrap();
            // ↓ use the subtype's own tag context, not the parent's
            let (sub_tag, sub_parent_tag) = sub_ty
                .subtype_kind
                .as_ref()
                .map(|k| k.get_tags())
                .unwrap_or_default();
            for field in &sub_ty.fields {
                if !field.is_tagged(tag_field, parent_tag_field)
                    && !field.is_tagged(sub_tag, sub_parent_tag)
                {
                    map.entry(field.name.as_str()).or_default().push(field);
                }
            }
        }
        map.into_iter()
            .filter(|(_, fields)| {
                let first_ty = &fields[0].r#type;
                fields.iter().all(|f| &f.r#type == first_ty)
            })
            .map(|(name, fields)| {
                let is_common = fields.len() == ty.subtypes.len();
                let is_fully_required = is_common && fields.iter().all(|f| f.required);
                (name, (fields[0], is_fully_required, is_common))
            })
            .collect()
    }
}

#[must_use]
fn helper_method_return_type(field_ty: &TypeKindInField, fully_required: bool) -> TokenStream {
    match field_ty {
        TypeKindInField::Array(inner) if fully_required => quote! { &[#inner] },
        TypeKindInField::Array(inner) => quote! { Option<&[#inner]> },
        TypeKindInField::String if fully_required => quote! { &str },
        TypeKindInField::String => quote! { Option<&str> },
        _ if field_ty.is_copy() && fully_required => quote! { #field_ty },
        _ if field_ty.is_copy() => quote! { Option<#field_ty> },
        _ if fully_required => quote! { &#field_ty },
        _ => quote! { Option<&#field_ty> },
    }
}

#[derive(Clone, Copy)]
enum HelperFieldSource<'a> {
    Direct(&'a NormalizedField),
    EnumHelper {
        field: &'a NormalizedField,
        fully_required: bool,
    },
}

impl<'a> HelperFieldSource<'a> {
    #[must_use]
    fn field(self) -> &'a NormalizedField {
        match self {
            HelperFieldSource::Direct(field) => field,
            HelperFieldSource::EnumHelper {
                field, ..
            } => field,
        }
    }

    #[must_use]
    fn required(self) -> bool {
        match self {
            HelperFieldSource::Direct(field) => field.required,
            HelperFieldSource::EnumHelper {
                fully_required, ..
            } => fully_required,
        }
    }
}

#[must_use]
fn helper_field_accessor_expr(field: &NormalizedField) -> TokenStream {
    let field_ident = sanitize_field_name(&field.name);
    let field_ty = &field.r#type;
    let is_required = field.required;

    if matches!(field_ty, TypeKindInField::Array(_)) {
        if is_required {
            quote! { val.#field_ident.as_ref() }
        } else {
            quote! { val.#field_ident.as_deref() }
        }
    } else if field_ty.is_copy() {
        quote! { val.#field_ident }
    } else if field.is_recursive || field.is_boxed || matches!(field_ty, TypeKindInField::String) {
        if is_required {
            quote! { val.#field_ident.as_ref() }
        } else {
            quote! { val.#field_ident.as_deref() }
        }
    } else if is_required {
        quote! { &val.#field_ident }
    } else {
        quote! { val.#field_ident.as_ref() }
    }
}

#[must_use]
fn nested_inner_accessor_expr(
    inner_ident: &Ident,
    inner_field: &NormalizedField,
    inner_ty: &TypeKindInField,
    inner_is_enum: bool,
    inner_field_fully_required: bool,
    fully_required: bool,
) -> AccessExpr {
    if inner_is_enum {
        if inner_field_fully_required && !fully_required {
            AccessExpr::EnumMethod {
                method: inner_ident.clone(),
                returns_option: false,
                wrap_in_some: true,
            }
        } else if inner_field_fully_required {
            AccessExpr::EnumMethod {
                method: inner_ident.clone(),
                returns_option: false,
                wrap_in_some: false,
            }
        } else {
            AccessExpr::EnumMethod {
                method: inner_ident.clone(),
                returns_option: true,
                wrap_in_some: false,
            }
        }
    } else if matches!(inner_ty, TypeKindInField::Array(_)) {
        if inner_field.required {
            AccessExpr::WrapInSome(quote! { inner.#inner_ident.as_ref() })
        } else {
            AccessExpr::Optional(quote! { inner.#inner_ident.as_deref() })
        }
    } else {
        let is_inner_copy = inner_ty.is_copy();
        let is_inner_boxed = inner_field.is_recursive
            || inner_field.is_boxed
            || matches!(inner_ty, TypeKindInField::String);

        match (
            inner_field.required,
            is_inner_copy,
            is_inner_boxed,
            fully_required,
        ) {
            (true, true, _, true) => AccessExpr::Plain(quote! { inner.#inner_ident }),
            (true, true, _, false) => AccessExpr::WrapInSome(quote! { inner.#inner_ident }),
            (false, true, ..) => AccessExpr::Optional(quote! { inner.#inner_ident }),
            (true, false, true, true) => AccessExpr::Plain(quote! { inner.#inner_ident.as_ref() }),
            (true, false, true, false) => {
                AccessExpr::WrapInSome(quote! { inner.#inner_ident.as_ref() })
            }
            (false, _, true, _) => AccessExpr::Optional(quote! { inner.#inner_ident.as_deref() }),
            (true, false, false, true) => AccessExpr::Plain(quote! { &inner.#inner_ident }),
            (true, false, false, false) => AccessExpr::WrapInSome(quote! { &inner.#inner_ident }),
            (false, false, false, _) => {
                AccessExpr::Optional(quote! { inner.#inner_ident.as_ref() })
            }
        }
    }
}

#[must_use]
fn nested_outer_accessor_expr(
    outer_ident: &Ident,
    outer_field: &NormalizedField,
    inner_access: AccessExpr,
) -> TokenStream {
    let enum_method_path = |method: &Ident| {
        let TypeKindInField::Telegram(outer_ty_name) = &outer_field.r#type else {
            unreachable!("enum method access requires telegram outer field");
        };
        let outer_ty_ident = format_ident!("{outer_ty_name}");
        quote! { crate::types::#outer_ty_ident::#method }
    };

    let is_outer_boxed = outer_field.is_recursive || outer_field.is_boxed;
    if outer_field.required {
        let get_inner = if is_outer_boxed {
            quote! { val.#outer_ident.as_ref() }
        } else {
            quote! { &val.#outer_ident }
        };
        let body = match inner_access {
            AccessExpr::Plain(tokens) | AccessExpr::Optional(tokens) => tokens,
            AccessExpr::WrapInSome(tokens) => quote! { Some(#tokens) },
            AccessExpr::EnumMethod {
                method,
                returns_option,
                wrap_in_some,
            } => {
                let method_path = enum_method_path(&method);
                if returns_option {
                    quote! { #method_path(inner) }
                } else if wrap_in_some {
                    quote! { Some(#method_path(inner)) }
                } else {
                    quote! { #method_path(inner) }
                }
            }
        };
        quote! { { let inner = #get_inner; #body } }
    } else {
        let as_opt = if is_outer_boxed {
            quote! { val.#outer_ident.as_deref() }
        } else {
            quote! { val.#outer_ident.as_ref() }
        };
        let use_match = match &inner_access {
            AccessExpr::Plain(tokens)
            | AccessExpr::Optional(tokens)
            | AccessExpr::WrapInSome(tokens) => {
                let code = tokens.to_string();
                code.contains("if let") || code.contains("match ")
            }
            AccessExpr::EnumMethod {
                ..
            } => false,
        };
        match inner_access {
            AccessExpr::Plain(tokens) | AccessExpr::WrapInSome(tokens) => {
                if use_match {
                    quote! {
                        match #as_opt {
                            Some(inner) => Some(#tokens),
                            None => None,
                        }
                    }
                } else {
                    quote! { #as_opt.map(|inner| #tokens) }
                }
            }
            AccessExpr::Optional(tokens) => {
                if use_match {
                    quote! {
                        match #as_opt {
                            Some(inner) => #tokens,
                            None => None,
                        }
                    }
                } else {
                    quote! { #as_opt.and_then(|inner| #tokens) }
                }
            }
            AccessExpr::EnumMethod {
                method,
                returns_option,
                ..
            } => {
                let method_path = enum_method_path(&method);
                if returns_option {
                    quote! { #as_opt.and_then(#method_path) }
                } else {
                    quote! { #as_opt.map(#method_path) }
                }
            }
        }
    }
}

#[must_use]
fn nested_outer_accessor_expr_from_helper(
    outer_access: TokenStream,
    outer_fully_required: bool,
    outer_field_ty: &TypeKindInField,
    inner_access: AccessExpr,
) -> TokenStream {
    let enum_method_path = |method: &Ident| {
        let TypeKindInField::Telegram(outer_ty_name) = outer_field_ty else {
            unreachable!("enum method access requires telegram outer field");
        };
        let outer_ty_ident = format_ident!("{outer_ty_name}");
        quote! { crate::types::#outer_ty_ident::#method }
    };
    if outer_fully_required {
        let body = match inner_access {
            AccessExpr::Plain(tokens) | AccessExpr::Optional(tokens) => tokens,
            AccessExpr::WrapInSome(tokens) => quote! { Some(#tokens) },
            AccessExpr::EnumMethod {
                method,
                returns_option,
                wrap_in_some,
            } => {
                let method_path = enum_method_path(&method);
                if returns_option {
                    quote! { #method_path(inner) }
                } else if wrap_in_some {
                    quote! { Some(#method_path(inner)) }
                } else {
                    quote! { #method_path(inner) }
                }
            }
        };
        quote! { { let inner = #outer_access; #body } }
    } else {
        let use_match = match &inner_access {
            AccessExpr::Plain(tokens)
            | AccessExpr::Optional(tokens)
            | AccessExpr::WrapInSome(tokens) => {
                let code = tokens.to_string();
                code.contains("if let") || code.contains("match ")
            }
            AccessExpr::EnumMethod {
                ..
            } => false,
        };
        match inner_access {
            AccessExpr::Plain(tokens) | AccessExpr::WrapInSome(tokens) => {
                if use_match {
                    quote! {
                        match #outer_access {
                            Some(inner) => Some(#tokens),
                            None => None,
                        }
                    }
                } else {
                    quote! { #outer_access.map(|inner| #tokens) }
                }
            }
            AccessExpr::Optional(tokens) => {
                if use_match {
                    quote! {
                        match #outer_access {
                            Some(inner) => #tokens,
                            None => None,
                        }
                    }
                } else {
                    quote! { #outer_access.and_then(|inner| #tokens) }
                }
            }
            AccessExpr::EnumMethod {
                method,
                returns_option,
                ..
            } => {
                let method_path = enum_method_path(&method);
                if returns_option {
                    quote! { #outer_access.and_then(#method_path) }
                } else {
                    quote! { #outer_access.map(#method_path) }
                }
            }
        }
    }
}

#[must_use]
fn get_helper_impls_for_type(
    type_quote: &NormalizedType,
    schema: &NormalizedSchema,
    ctx: &TypeDocContext<'_>,
) -> TokenStream {
    if type_quote.subtypes.is_empty() {
        return quote! {};
    }

    let type_name = format_ident!("{}", type_quote.name);
    let mut methods = vec![];

    let (tag_field, parent_tag_field) = type_quote
        .subtype_kind
        .as_ref()
        .map(|kind| kind.get_tags())
        .unwrap_or_default();
    let mut fields_subtypes_map: BTreeMap<
        &str,
        Vec<(&NormalizedSubtypeVariant, HelperFieldSource<'_>)>,
    > = BTreeMap::new();
    for subtype in &type_quote.subtypes {
        let ty = schema.types.get(&subtype.ty_name).unwrap();
        if ty.subtypes.is_empty() {
            for field in &ty.fields {
                if field.is_tagged(tag_field, parent_tag_field) {
                    continue;
                }
                fields_subtypes_map
                    .entry(&field.name)
                    .or_default()
                    .push((subtype, HelperFieldSource::Direct(field)));
            }
        } else {
            let common = collect_common_fields(ty, schema);
            for (name, (field, fully_required, _)) in common {
                if field.is_tagged(tag_field, parent_tag_field) {
                    continue;
                }
                fields_subtypes_map.entry(name).or_default().push((
                    subtype,
                    HelperFieldSource::EnumHelper {
                        field,
                        fully_required,
                    },
                ));
            }
        }
    }

    for (&field_name, subtypes) in &fields_subtypes_map {
        let method_name = sanitize_field_name(field_name);
        let field = subtypes[0].1.field();
        let field_ty = &field.r#type;

        let is_identical_field_type = subtypes.iter().all(|(_, f)| f.field().r#type == *field_ty);
        if !is_identical_field_type {
            continue;
        }
        let is_common = subtypes.len() == type_quote.subtypes.len();
        let is_required_for_all = is_common && subtypes.iter().all(|(_, f)| f.required());
        let return_ty = helper_method_return_type(field_ty, is_required_for_all);

        let doc_helper =
            format_attr_description(&format!("Helper method for field `{field_name}`."));
        let mut doc_lines: Vec<TokenStream> =
            vec![quote! { #[doc = #doc_helper] }, quote! { #[doc = ""] }];

        let mut desc_groups: Vec<(&str, Vec<&str>)> = vec![];
        for (subtype, source) in subtypes {
            let desc = source.field().description.as_str();
            if let Some(group) = desc_groups.iter_mut().find(|(d, _)| *d == desc) {
                group.1.push(subtype.ty_name.as_str());
            } else {
                desc_groups.push((desc, vec![subtype.ty_name.as_str()]));
            }
        }

        let all_same_description = desc_groups.len() == 1;
        if !all_same_description {
            let doc_variants = format_attr_description("# Variants");
            doc_lines.push(quote! { #[doc = #doc_variants] });
        }

        for (description, ty_names) in &desc_groups {
            let doc_field = if all_same_description {
                format_field_doc(description, field_ty, ctx)
            } else {
                let label = if ty_names.len() == 1 {
                    format!("- `{}`", ty_names[0])
                } else {
                    let joined = ty_names
                        .iter()
                        .map(|n| format!("`{n}`"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("- {joined}")
                };
                let linked_description = format_field_doc(description, field_ty, ctx);
                format_attr_description(&format!("{label}. {}", linked_description.trim_start()))
            };
            doc_lines.push(quote! { #[doc = #doc_field] });
        }

        let mut match_arms = vec![];
        for (subtype, field) in subtypes {
            let variant = format_ident!("{}", subtype.variant);

            let body = match field {
                HelperFieldSource::Direct(field) => {
                    let body = helper_field_accessor_expr(field);
                    if field.required && !is_required_for_all {
                        quote! { Some(#body) }
                    } else {
                        quote! { #body }
                    }
                }
                HelperFieldSource::EnumHelper {
                    field,
                    fully_required,
                } => {
                    let inner_ident = sanitize_field_name(&field.name);
                    let inner_ty_ident = format_ident!("{}", subtype.ty_name);
                    let method_path = quote! { crate::types::#inner_ty_ident::#inner_ident };
                    if *fully_required && !is_required_for_all {
                        quote! { Some(#method_path(val)) }
                    } else {
                        quote! { #method_path(val) }
                    }
                }
            };

            match_arms.push(quote! {
                Self::#variant(val) => #body
            });
        }
        if !is_common {
            let present: HashSet<&str> = subtypes.iter().map(|(s, _)| s.variant.as_str()).collect();
            let missing: Vec<_> = type_quote
                .subtypes
                .iter()
                .filter(|s| !present.contains(s.variant.as_str()))
                .collect();
            if missing.len() == 1 {
                let missing_variant = format_ident!("{}", missing[0].variant);
                match_arms.push(quote! { Self::#missing_variant(_) => None });
            } else {
                match_arms.push(quote! { _ => None });
            }
        }

        let method = quote! {
            #( #doc_lines )*
            #[must_use]
            pub fn #method_name(&self) -> #return_ty {
                match self { #( #match_arms ),* }
            }
        };

        methods.push(method);
    }

    #[allow(clippy::type_complexity)]
    let mut nested_map: BTreeMap<
        String,
        Vec<(
            &NormalizedSubtypeVariant,
            HelperFieldSource<'_>, // outer field
            &NormalizedField,      // inner field (representative)
            bool,                  // inner_is_enum
            bool,                  // inner_field_fully_required (within inner type)
        )>,
    > = BTreeMap::new();

    for outer_subtypes in fields_subtypes_map.values() {
        let first_outer = outer_subtypes[0].1.field();
        let outer_ty = &first_outer.r#type;

        let TypeKindInField::Telegram(inner_type_name) = outer_ty else {
            continue;
        };
        if !outer_subtypes
            .iter()
            .all(|(_, f)| &f.field().r#type == outer_ty)
        {
            continue;
        }

        let Some(inner_ty) = schema.types.get(inner_type_name.as_str()) else {
            continue;
        };

        let inner_is_enum = !inner_ty.subtypes.is_empty();
        let inner_common = collect_common_fields(inner_ty, schema);

        let (inner_tag, inner_parent_tag) = inner_ty
            .subtype_kind
            .as_ref()
            .map(|k| k.get_tags())
            .unwrap_or_default();

        for (inner_field_name, (inner_field, inner_field_fully_required, _)) in &inner_common {
            if !inner_is_enum && inner_field.is_tagged(inner_tag, inner_parent_tag) {
                continue;
            }
            for (subtype, outer_field) in outer_subtypes {
                nested_map
                    .entry((*inner_field_name).to_string())
                    .or_default()
                    .push((
                        *subtype,
                        *outer_field,
                        inner_field,
                        inner_is_enum,
                        *inner_field_fully_required,
                    ));
            }
        }
    }

    for (inner_field_name, entries) in nested_map {
        let inner_ty = &entries[0].2.r#type;
        if !entries.iter().all(|(_, _, f, ..)| &f.r#type == inner_ty) {
            continue;
        }

        let mut seen = HashSet::new();
        if entries
            .iter()
            .any(|(s, ..)| !seen.insert(s.variant.as_str()))
        {
            continue;
        }

        if fields_subtypes_map.contains_key(inner_field_name.as_str()) {
            continue;
        }

        let method_ident = sanitize_field_name(&inner_field_name);

        let is_all_covered = entries.len() == type_quote.subtypes.len();
        let is_inner_req_all =
            entries
                .iter()
                .all(|(_, _, f, inner_is_enum, inner_field_fully_required)| {
                    if *inner_is_enum {
                        *inner_field_fully_required
                    } else {
                        f.required
                    }
                });
        let fully_required = is_all_covered
            && is_inner_req_all
            && entries.iter().all(|(_, outer, ..)| outer.required());

        let return_ty = helper_method_return_type(inner_ty, fully_required);
        let outer_field_name = entries[0].1.field().name.as_str();
        let can_delegate_via_outer_helper = is_all_covered
            && entries
                .iter()
                .all(|(_, outer, ..)| outer.field().name == outer_field_name);
        let outer_fully_required = entries.iter().all(|(_, outer, ..)| outer.required());

        if can_delegate_via_outer_helper {
            let outer_ident = sanitize_field_name(outer_field_name);
            let (_, outer_field, inner_field, inner_is_enum, inner_field_fully_required) =
                entries[0];
            let inner_ident = sanitize_field_name(&inner_field.name);
            let delegated = if inner_is_enum {
                let TypeKindInField::Telegram(inner_ty_name) = &outer_field.field().r#type else {
                    unreachable!("enum nested helper must have telegram type");
                };
                let inner_ty_ident = format_ident!("{inner_ty_name}");
                let method_path = quote! { crate::types::#inner_ty_ident::#inner_ident };

                if outer_fully_required {
                    if inner_field_fully_required {
                        if fully_required {
                            quote! { #method_path(self.#outer_ident()) }
                        } else {
                            quote! { Some(#method_path(self.#outer_ident())) }
                        }
                    } else {
                        quote! { #method_path(self.#outer_ident()) }
                    }
                } else if inner_field_fully_required {
                    quote! { self.#outer_ident().map(#method_path) }
                } else {
                    quote! { self.#outer_ident().and_then(#method_path) }
                }
            } else {
                let inner_access = nested_inner_accessor_expr(
                    &inner_ident,
                    inner_field,
                    inner_ty,
                    inner_is_enum,
                    inner_field_fully_required,
                    fully_required,
                );
                if outer_fully_required {
                    match inner_access {
                        AccessExpr::Plain(tokens) | AccessExpr::Optional(tokens) => {
                            quote! { { let inner = self.#outer_ident(); #tokens } }
                        }
                        AccessExpr::WrapInSome(tokens) => {
                            quote! { { let inner = self.#outer_ident(); Some(#tokens) } }
                        }
                        AccessExpr::EnumMethod {
                            ..
                        } => {
                            unreachable!("enum method access handled in the enum delegation branch",)
                        }
                    }
                } else {
                    match inner_access {
                        AccessExpr::Plain(tokens) | AccessExpr::WrapInSome(tokens) => {
                            quote! { self.#outer_ident().map(|inner| #tokens) }
                        }
                        AccessExpr::Optional(tokens) => {
                            quote! { self.#outer_ident().and_then(|inner| #tokens) }
                        }
                        AccessExpr::EnumMethod {
                            ..
                        } => {
                            unreachable!("enum method access handled in the enum delegation branch",)
                        }
                    }
                }
            };

            let doc = format_attr_description(&format!(
                "Helper method for nested field `{inner_field_name}`."
            ));
            methods.push(quote! {
                #[doc = #doc]
                #[must_use]
                pub fn #method_ident(&self) -> #return_ty {
                    #delegated
                }
            });
            continue;
        }

        let mut match_arms = vec![];
        for (subtype, outer_field, inner_field, inner_is_enum, inner_field_fully_required) in
            &entries
        {
            let variant = format_ident!("{}", subtype.variant);
            let outer_ident = sanitize_field_name(&outer_field.field().name);
            let inner_ident = sanitize_field_name(&inner_field.name);
            let inner_access = nested_inner_accessor_expr(
                &inner_ident,
                inner_field,
                inner_ty,
                *inner_is_enum,
                *inner_field_fully_required,
                fully_required,
            );
            let body = match outer_field {
                HelperFieldSource::Direct(field) => {
                    nested_outer_accessor_expr(&outer_ident, field, inner_access)
                }
                HelperFieldSource::EnumHelper {
                    field,
                    fully_required,
                } => {
                    let outer_ty_ident = format_ident!("{}", subtype.ty_name);
                    let method_path = quote! { crate::types::#outer_ty_ident::#outer_ident };
                    nested_outer_accessor_expr_from_helper(
                        quote! { #method_path(val) },
                        *fully_required,
                        &field.r#type,
                        inner_access,
                    )
                }
            };

            match_arms.push(quote! { Self::#variant(val) => #body });
        }
        if !is_all_covered {
            let present: HashSet<&str> = entries.iter().map(|(s, ..)| s.variant.as_str()).collect();
            let missing: Vec<_> = type_quote
                .subtypes
                .iter()
                .filter(|s| !present.contains(s.variant.as_str()))
                .collect();
            if missing.len() == 1 {
                let missing_variant = format_ident!("{}", missing[0].variant);
                match_arms.push(quote! { Self::#missing_variant(_) => None });
            } else {
                match_arms.push(quote! { _ => None });
            }
        }

        let doc = format_attr_description(&format!(
            "Helper method for nested field `{inner_field_name}`."
        ));
        methods.push(quote! {
            #[doc = #doc]
            #[must_use]
            pub fn #method_ident(&self) -> #return_ty {
                match self { #( #match_arms ),* }
            }
        });
    }

    quote! {
        impl #type_name {
            #( #methods )*
        }
    }
}

#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn tokenize_type(
    type_quote: &NormalizedType,
    schema: &NormalizedSchema,
    known_schema_type_names: &HashSet<String>,
) -> TokenStream {
    let ctx = TypeDocContext {
        schema_type_names: known_schema_type_names,
    };
    let mut import_quotes = vec![];
    if type_quote.has_extra_fields && type_quote.subtypes.is_empty() {
        import_quotes.push(quote! { use std::collections::BTreeMap; });
    }
    import_quotes.push(quote! { use serde::{Serialize, Deserialize}; });

    let type_impls = get_impls_for_types(type_quote, schema);
    let subtype_impls = get_from_impls_for_subtypes(type_quote);
    let type_definition = tokenize_type_definition(type_quote, &ctx);
    let builder_impls = builder_impl_for_type(type_quote, &ctx);
    let helper_impls = get_helper_impls_for_type(type_quote, schema, &ctx);

    quote! {
        #( #import_quotes )*
        #type_definition
        #builder_impls
        #helper_impls
        #( #type_impls )*
        #( #subtype_impls )*
    }
}

#[must_use]
pub fn tokenize_types_mod(type_names: &[&String]) -> TokenStream {
    let mods_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        quote! { pub mod #mod_name; }
    });
    let uses_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        let type_name = format_ident!("{name}");
        quote! { pub use #mod_name::#type_name; }
    });

    quote! {
        //! Telegram Bot API data types and helper models.
        //!
        //! This module re-exports all generated types from the `crate::types` module, including
        //! shared helper types from `non_telegram`.
        //! Generated type builders follow the same conventions as method builders:
        //! - optional fields can be set with normal chainable builder methods
        //! - optional fields also expose `_option(...)` variants to pass `Option<T>` directly
        //!   (including `None` to clear/unset a field)
        //!
        //! For polymorphic API objects (for example [`crate::types::Message`]), telers uses enums with
        //! split subtypes. In these cases, use generated helper methods like `message.chat()`
        //! and `message.text()` instead of field access like `message.chat`.
        //!
        //! # Examples
        //! ```rust
        //! use telers::types::{ChatIdKind, InlineKeyboardButton, InlineKeyboardMarkup};
        //!
        //! let chat_id = ChatIdKind::id(1);
        //! let keyboard = InlineKeyboardMarkup::new([[
        //!     InlineKeyboardButton::new("Open Telegram API docs")
        //!         // Regular builder setter.
        //!         .url("https://core.telegram.org/bots/api")
        //!         // `_option(...)` variant for Option<T> values.
        //!         .url_option(Some("https://core.telegram.org/bots/api")),
        //! ]]);
        //!
        //! assert!(matches!(chat_id, ChatIdKind::Id(_)));
        //! assert_eq!(keyboard.inline_keyboard.len(), 1);
        //! ```
        //!
        //! ```rust
        //! use telers::types::Message;
        //!
        //! fn inspect_message(message: &Message) {
        //!     // `Message` is an enum, so helper methods provide unified access.
        //!     let _chat = message.chat();
        //!     let _message_id = message.message_id();
        //!     let _maybe_text = message.text();
        //! }
        //! ```

        #![allow(clippy::too_many_arguments)]
        #![allow(clippy::struct_excessive_bools)]
        #![allow(clippy::large_enum_variant)]

        pub(crate) mod non_telegram;
        pub(crate) mod to_methods;
        pub use non_telegram::*;
        #( #mods_quote )*
        #( #uses_quote )*
    }
}
