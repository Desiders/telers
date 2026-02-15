use crate::{
    file::camel_to_filename,
    generator::helpers::{
        camel_to_snake, format_attr_description, format_description, get_singular_and_plural_forms,
        sanitize_field_name,
    },
    parser::api::{
        BooleanKind, IntegerKind, NormalizedField, NormalizedSchema, NormalizedSubtypeVariant,
        NormalizedType, SubtypeKind, TypeKindInField,
    },
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::{punctuated::Punctuated, Path, PathSegment};

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
            TypeKindInField::Boolean(kind) => match kind {
                BooleanKind::Any => quote! { bool },
                BooleanKind::True => quote! { bool },
            },
            TypeKindInField::Telegram(name) => {
                let ident = format_ident!("{name}");
                quote! { #ident }
            }
            TypeKindInField::InputFile => quote! { InputFile },
            TypeKindInField::ChatId => quote! { ChatIdKind },
            TypeKindInField::Array(inner) => quote! { Box<[#inner]> },
        };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedSubtypeVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant = format_ident!("{}", self.variant);
        let name = format_ident!("{}", self.name);
        let ts = quote! { #variant(#name), };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = sanitize_field_name(&self.name);
        let ty = &self.r#type;
        let doc = format_attr_description(&self.description);

        let ts = if self.required {
            if self.is_recursive || self.is_boxed {
                quote! {
                    #[doc = #doc]
                    pub #name: Box<#ty>,
                }
            } else {
                quote! {
                    #[doc = #doc]
                    pub #name: #ty,
                }
            }
        } else {
            let ty = if self.is_recursive || self.is_boxed {
                quote! { Box<#ty> }
            } else {
                quote! { #ty }
            };
            quote! {
                #[doc = #doc]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #name: Option<#ty>,
            }
        };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.as_str());
        let doc_lines = format_description(&self.description, &self.href);

        let ts = if self.subtypes.is_empty() {
            let fields = self.fields.iter().filter(|field| {
                if let Some(SubtypeKind::Tagged { ref tag_field }) = self.subtype_kind {
                    field.name != *tag_field
                } else {
                    true
                }
            });

            quote! {
                #( #[doc = #doc_lines] )*
                #[derive(Clone, Debug, Serialize)]
                pub struct #name {
                    #( #fields )*
                }
            }
        } else {
            let serde_attr = match &self.subtype_kind {
                Some(SubtypeKind::Tagged { tag_field }) => quote! {
                    #[serde(tag = #tag_field)]
                },
                Some(SubtypeKind::Untagged) => quote! {
                    #[serde(untagged)]
                },
                None => quote! {},
            };
            let subtypes = self.subtypes.iter();

            quote! {
                #( #[doc = #doc_lines] )*
                #[derive(Clone, Debug, Serialize)]
                #serde_attr
                pub enum #name {
                    #( #subtypes )*
                }
            }
        };

        tokens.extend(ts);
    }
}

pub fn get_from_impls_for_subtypes(type_quote: &NormalizedType) -> Vec<TokenStream> {
    type_quote
        .subtypes
        .iter()
        .map(|subtype| {
            let name = format_ident!("{}", type_quote.name);
            let subtype_name = format_ident!("{}", subtype.name);
            let subtype_variant = format_ident!("{}", subtype.variant);
            quote! {
                impl From<#subtype_name> for #name {
                    fn from(val: #subtype_name) -> Self {
                        Self::#subtype_variant(val)
                    }
                }
            }
        })
        .collect()
}

pub fn builder_impl_for_type(type_quote: &NormalizedType) -> TokenStream {
    if !type_quote.subtypes.is_empty() {
        return quote! {};
    }

    let type_name = format_ident!("{}", type_quote.name);

    let fields: Vec<_> = type_quote
        .fields
        .iter()
        .filter(|field| {
            if let Some(SubtypeKind::Tagged { ref tag_field }) = type_quote.subtype_kind {
                field.name != *tag_field
            } else {
                true
            }
        })
        .collect();

    let required_fields: Box<[_]> = fields.iter().filter(|f| f.required).copied().collect();
    let optional_fields: Box<[_]> = fields.iter().filter(|f| !f.required).copied().collect();

    let new_method_ts = {
        let mut doc_lines = TokenStream::new();

        let doc = format_attr_description(&format!("Creates a new [`{}`].", type_quote.name));
        doc_lines.extend(quote! { #[doc = #doc] });

        if !required_fields.is_empty() {
            let doc = format_attr_description("# Arguments");
            doc_lines.extend([quote! { #[doc = ""] }, quote! { #[doc = #doc] }]);
            for &field in &required_fields {
                let doc =
                    format_attr_description(&format!("* `{}` - {}", field.name, field.description));
                doc_lines.extend(quote! { #[doc = #doc] });
            }
        }
        if !optional_fields.is_empty() {
            let doc = format_attr_description("Use builder methods to set optional fields.");
            doc_lines.extend([quote! { #[doc = ""] }, quote! { #[doc = #doc] }]);
        }
        if !fields.is_empty() {
            let new_args = required_fields.iter().map(|field| {
                let name = sanitize_field_name(&field.name);
                let ty = &field.r#type;
                quote! { #name: impl Into<#ty> }
            });
            let new_init = fields.iter().map(|field| {
                let name = sanitize_field_name(&field.name);
                if field.required {
                    if field.is_recursive || field.is_boxed {
                        quote! { #name: Box::new(#name.into()) }
                    } else {
                        quote! { #name: #name.into() }
                    }
                } else {
                    quote! { #name: None }
                }
            });

            quote! {
                #doc_lines
                #[must_use]
                pub fn new(#( #new_args ),*) -> Self {
                    Self {
                        #( #new_init, )*
                    }
                }
            }
        } else {
            quote! {
                #doc_lines
                #[must_use]
                pub const fn new() -> Self {
                    Self {}
                }
            }
        }
    };
    let default_impl_ts = if required_fields.is_empty() {
        quote! {
            impl Default for #type_name {
                fn default() -> Self {
                    Self::new()
                }
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

                let singular_method_name = sanitize_field_name(&singular);
                let plural_method_name = sanitize_field_name(&plural);

                let mut doc_lines = TokenStream::new();
                let doc = format_attr_description(&field.description);
                doc_lines.extend(quote! { #[doc = #doc] });
                let doc = format_attr_description("# Notes");
                doc_lines.extend([quote! { #[doc = ""] }, quote! { #[doc = #doc] }]);
                let doc = format_attr_description("Adds multiple elements.");
                doc_lines.extend(quote! { #[doc = #doc] });

                let body = if field.required {
                    quote! {
                        let mut #name = std::mem::take(&mut self.#name).into_vec();
                        #name.extend(val.into());
                        let #name = #name.into_boxed_slice();
                        Self { #name, ..self }
                    }
                } else {
                    quote! {
                        let mut #name = self.#name.take().unwrap_or_default().into_vec();
                        #name.extend(val.into());
                        let #name = #name.into_boxed_slice();
                        Self { #name: Some(#name), ..self }
                    }
                };
                methods.push(quote! {
                    #doc_lines
                    #[must_use]
                    pub fn #plural_method_name(mut self, val: impl Into<#ty>) -> Self {
                        #body
                    }
                });

                if singular_method_name != plural_method_name {
                    let mut doc_lines = TokenStream::new();
                    let doc = format_attr_description(&field.description);
                    doc_lines.extend(quote! { #[doc = #doc] });
                    let doc = format_attr_description("# Notes");
                    doc_lines.extend([quote! { #[doc = ""] }, quote! { #[doc = #doc] }]);
                    let doc = format_attr_description("Adds a single element.");
                    doc_lines.extend(quote! { #[doc = #doc] });

                    let body = if field.required {
                        quote! {
                            let mut #name = std::mem::take(&mut self.#name).into_vec();
                            #name.push(val.into());
                            let #name = #name.into_boxed_slice();
                            Self { #name, ..self }
                        }
                    } else {
                        quote! {
                            let mut #name = self.#name.take().unwrap_or_default().into_vec();
                            #name.push(val.into());
                            let #name = #name.into_boxed_slice();
                            Self { #name: Some(#name), ..self }
                        }
                    };

                    methods.push(quote! {
                        #doc_lines
                        #[must_use]
                        pub fn #singular_method_name(mut self, val: impl Into<#inner>) -> Self {
                            #body
                        }
                    });
                }

                if !field.required {
                    let mut doc_lines = TokenStream::new();
                    let doc = format_attr_description(&field.description);
                    doc_lines.extend(quote! { #[doc = #doc] });
                    let doc = format_attr_description("# Notes");
                    doc_lines.extend([quote! { #[doc = ""] }, quote! { #[doc = #doc] }]);
                    let doc = format_attr_description("Adds a single element.");
                    doc_lines.extend(quote! { #[doc = #doc] });

                    let method_name = format_ident!("{}_option", field.name);
                    methods.push(quote! {
                        #doc_lines
                        #[must_use]
                        pub fn #method_name(self, val: Option<impl Into<#ty>>) -> Self {
                            Self { #name: val.map(Into::into), ..self }
                        }
                    });
                }
            } else {
                let doc = format_attr_description(&field.description);
                let value = if field.is_recursive || field.is_boxed {
                    quote! { Box::new(val.into()) }
                } else {
                    quote! { val.into() }
                };
                let value = if field.required {
                    value
                } else {
                    quote! { Some(#value) }
                };

                let method_name = format_ident!("{name}");
                methods.push(quote! {
                    #[doc = #doc]
                    #[must_use]
                    pub fn #method_name(self, val: impl Into<#ty>) -> Self {
                        Self { #name: #value, ..self }
                    }
                });

                let method_name = format_ident!("{}_option", field.name);
                let value = if field.is_recursive || field.is_boxed {
                    quote! { val.map(|val| Box::new(val.into())) }
                } else {
                    quote! { val.map(Into::into) }
                };

                if !field.required {
                    methods.push(quote! {
                        #[doc = #doc]
                        #[must_use]
                        pub fn #method_name(self, val: Option<impl Into<#ty>>) -> Self {
                            Self { #name: #value, ..self }
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

pub fn get_impl_for_type(type_quote: &NormalizedType, schema: &NormalizedSchema) -> TokenStream {
    if type_quote.subtypes.is_empty() {
        return quote! {};
    }

    let type_name = format_ident!("{}", type_quote.name);

    let mut methods = vec![];

    for subtype in &type_quote.subtypes {
        let variant = format_ident!("{}", subtype.variant);
        let name = format_ident!("{}", subtype.name);
        let variant_snake_case = camel_to_snake(&subtype.variant);
        let is_method_name = format_ident!("is_{variant_snake_case}");
        let into_method_name = format_ident!("into_{variant_snake_case}");

        methods.push(quote! {
            #[must_use]
            pub const fn #is_method_name(&self) -> bool {
                matches!(self, Self::#variant(_))
            }
        });
        methods.push(quote! {
            #[must_use]
            pub fn #into_method_name(self) -> Option<#name> {
                if let Self::#variant(val) = self {
                    Some(val)
                } else {
                    None
                }
            }
        });
    }

    let mut field_to_variants = HashMap::new();

    for subtype_variant in &type_quote.subtypes {
        if let Some(subtype) = schema.types.get(&subtype_variant.name) {
            for field in &subtype.fields {
                if let Some(SubtypeKind::Tagged { ref tag_field }) = type_quote.subtype_kind {
                    if field.name == *tag_field {
                        continue;
                    }
                }

                let entry = field_to_variants
                    .entry(field.name.clone())
                    .or_insert_with(|| (field, vec![]));
                entry.1.push(subtype_variant);
            }
        }
    }

    for (field_name_str, (first_field, variants)) in field_to_variants {
        if !variants.iter().all(|variant| {
            let subtype = schema.types.get(&variant.name).unwrap();
            let field = subtype
                .fields
                .iter()
                .find(|f| f.name == field_name_str)
                .unwrap();
            field.r#type == first_field.r#type && field.required == first_field.required
        }) {
            continue;
        }

        let field_name = sanitize_field_name(&field_name_str);
        let field_type = &first_field.r#type;

        let is_copy = field_type.is_copy();
        let is_string = matches!(field_type, TypeKindInField::String);
        let is_boxed = first_field.is_recursive || first_field.is_boxed;
        let is_array = matches!(field_type, TypeKindInField::Array(_));

        let is_common = variants.len() == type_quote.subtypes.len();

        let (return_type, body, needs_option_wrapper) = if is_common {
            if is_array {
                if let TypeKindInField::Array(inner) = field_type {
                    if first_field.required {
                        (quote! { &[#inner] }, quote! { &*val.#field_name }, false)
                    } else {
                        (
                            quote! { Option<&[#inner]> },
                            quote! { val.#field_name.as_deref() },
                            false,
                        )
                    }
                } else {
                    unreachable!()
                }
            } else {
                match (first_field.required, is_copy, is_string) {
                    (true, true, _) => (quote! { #field_type }, quote! { val.#field_name }, false),
                    (false, true, _) => (
                        quote! { Option<#field_type> },
                        quote! { val.#field_name },
                        false,
                    ),
                    (true, false, true) => (quote! { &str }, quote! { &*val.#field_name }, false),
                    (false, false, true) => (
                        quote! { Option<&str> },
                        quote! { val.#field_name.as_deref() },
                        false,
                    ),
                    (true, false, false) => {
                        (quote! { &#field_type }, quote! { &val.#field_name }, false)
                    }
                    (false, false, false) => {
                        let body = if is_boxed {
                            quote! { val.#field_name.as_deref() }
                        } else {
                            quote! { val.#field_name.as_ref() }
                        };
                        (quote! { Option<&#field_type> }, body, false)
                    }
                }
            }
        } else {
            if is_array {
                if let TypeKindInField::Array(inner) = field_type {
                    if first_field.required {
                        (
                            quote! { Option<&[#inner]> },
                            quote! { &*val.#field_name },
                            true,
                        )
                    } else {
                        (
                            quote! { Option<&[#inner]> },
                            quote! { val.#field_name.as_deref() },
                            false,
                        )
                    }
                } else {
                    unreachable!()
                }
            } else {
                match (first_field.required, is_copy, is_string) {
                    (true, true, _) => (
                        quote! { Option<#field_type> },
                        quote! { val.#field_name },
                        true,
                    ),
                    (false, true, _) => (
                        quote! { Option<#field_type> },
                        quote! { val.#field_name },
                        false,
                    ),
                    (true, false, true) => {
                        (quote! { Option<&str> }, quote! { &*val.#field_name }, true)
                    }
                    (false, false, true) => (
                        quote! { Option<&str> },
                        quote! { val.#field_name.as_deref() },
                        false,
                    ),
                    (true, false, false) => (
                        quote! { Option<&#field_type> },
                        quote! { &val.#field_name },
                        true,
                    ),
                    (false, false, false) => {
                        let body = if is_boxed {
                            quote! { val.#field_name.as_deref() }
                        } else {
                            quote! { val.#field_name.as_ref() }
                        };
                        (quote! { Option<&#field_type> }, body, false)
                    }
                }
            }
        };

        let match_arms = variants.iter().map(|subtype| {
            let variant_name = format_ident!("{}", subtype.variant);
            let arm_body = if needs_option_wrapper {
                quote! { Some(#body) }
            } else {
                body.clone()
            };

            quote! {
                Self::#variant_name(val) => #arm_body
            }
        });

        let method = if is_common {
            quote! {
                #[must_use]
                pub fn #field_name(&self) -> #return_type {
                    match self {
                        #( #match_arms ),*
                    }
                }
            }
        } else {
            quote! {
                #[must_use]
                pub fn #field_name(&self) -> #return_type {
                    match self {
                        #( #match_arms, )*
                        _ => None,
                    }
                }
            }
        };
        methods.push(method);
    }

    quote! {
        impl #type_name {
            #( #methods )*
        }
    }
}

pub fn tokenize_type(type_quote: &NormalizedType, schema: &NormalizedSchema) -> TokenStream {
    let imports_quote = if type_quote.get_paths_count() == 0 {
        quote! {
            use serde::Serialize;
        }
    } else {
        quote! {
            use super::*;
            use serde::Serialize;
        }
    };

    let impls_for_subtypes = get_from_impls_for_subtypes(type_quote);
    let impls_for_subtypes_quote = quote! { #(#impls_for_subtypes)* };
    let builder_impl = builder_impl_for_type(type_quote);
    let get_impl = get_impl_for_type(type_quote, schema);

    quote! {
        #imports_quote
        #type_quote
        #builder_impl
        #get_impl
        #impls_for_subtypes_quote
    }
}

pub fn tokenize_types_mod(type_names: &[&String]) -> TokenStream {
    let mods_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        quote! {
            pub mod #mod_name;
        }
    });
    let mod_name = format_ident!("non_telegram");
    let non_telegram_mods_quote = quote! {
        pub(crate) mod #mod_name;
    };
    let uses_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        let type_name = format_ident!("{name}");
        quote! {
            pub use #mod_name::#type_name;
        }
    });

    let mut segments = Punctuated::new();
    segments.push(PathSegment::from(format_ident!("non_telegram")));

    let chat_id_kind_mod_name = Path {
        leading_colon: None,
        segments,
    };
    let chat_id_kind_type_name = format_ident!("ChatIdKind");
    let non_telegram_uses_quote = quote! {
        pub use #chat_id_kind_mod_name::#chat_id_kind_type_name;
    };

    quote! {
        #non_telegram_mods_quote
        #non_telegram_uses_quote
        #( #mods_quote )*
        #( #uses_quote )*
    }
}

#[cfg(test)]
mod tests {
    use super::{
        tokenize_type, IntegerKind, NormalizedField, NormalizedSchema, NormalizedSubtypeVariant,
        NormalizedType, SubtypeKind, TypeKindInField,
    };

    #[test]
    fn test_tokenize_type_1() {
        let ty = NormalizedType {
            name: "Message".into(),
            href: "https://core.telegram.org/bots/api#message".into(),
            description: vec!["Line1.".into(), "Line2.".into()],
            fields: vec![
                NormalizedField {
                    name: "message_id".into(),
                    required: true,
                    description: "Unique message identifier".into(),
                    r#type: TypeKindInField::Integer(IntegerKind::Int64),
                    is_recursive: false,
                    is_boxed: false,
                },
                NormalizedField {
                    name: "text".into(),
                    required: false,
                    description: "Message text".into(),
                    r#type: TypeKindInField::String,
                    is_recursive: false,
                    is_boxed: false,
                },
            ],

            subtype_kind: None,
            subtypes: vec![],
            subtype_of: vec![],
        };
        let schema = NormalizedSchema::default();
        let result = tokenize_type(&ty, &schema).to_string();

        assert!(result.contains("Line1"));
        assert!(result.contains("Line2"));
        assert!(result.contains("<https://core.telegram.org/bots/api#message>"));
        assert!(result.contains("struct Message"));
        assert!(result.contains("Unique message identifier"));
        assert!(result.contains("message_id : i64"));
        assert!(result.contains("text : Option < Box < str > >"));
    }

    #[test]
    fn test_tokenize_type_enum_tagged() {
        let ty = NormalizedType {
            name: "MessageOrigin".into(),
            href: "https://core.telegram.org/bots/api#messageorigin".into(),
            description: vec!["Describes the origin of a message.".into()],
            fields: vec![],
            subtype_kind: Some(SubtypeKind::Tagged {
                tag_field: "type".into(),
            }),
            subtypes: vec![
                NormalizedSubtypeVariant {
                    variant: "MessageOriginUser".into(),
                    name: "MessageOriginUser".into(),
                },
                NormalizedSubtypeVariant {
                    variant: "MessageOriginHiddenUser".into(),
                    name: "MessageOriginHiddenUser".into(),
                },
            ],
            subtype_of: vec![],
        };
        let schema = NormalizedSchema::default();
        let result = tokenize_type(&ty, &schema).to_string();

        assert!(result.contains("enum MessageOrigin"));
        assert!(result.contains("serde (tag = \"type\")"));
        assert!(result.contains("MessageOriginUser (MessageOriginUser)"));
        assert!(result.contains("MessageOriginHiddenUser (MessageOriginHiddenUser)"));
    }

    #[test]
    fn test_tokenize_type_enum_untagged() {
        let ty = NormalizedType {
            name: "MaybeInaccessibleMessage".into(),
            href: "https://core.telegram.org/bots/api#maybeinaccessiblemessage".into(),
            description: vec!["Describes a message that may be inaccessible.".into()],
            fields: vec![],
            subtype_kind: Some(SubtypeKind::Untagged),
            subtypes: vec![
                NormalizedSubtypeVariant {
                    variant: "Message".into(),
                    name: "Message".into(),
                },
                NormalizedSubtypeVariant {
                    variant: "InaccessibleMessage".into(),
                    name: "InaccessibleMessage".into(),
                },
            ],
            subtype_of: vec![],
        };
        let schema = NormalizedSchema::default();
        let result = tokenize_type(&ty, &schema).to_string();

        assert!(result.contains("enum MaybeInaccessibleMessage"));
        assert!(result.contains("serde (untagged)"));
        assert!(result.contains("Message (Message)"));
        assert!(result.contains("InaccessibleMessage (InaccessibleMessage)"));
    }
}
