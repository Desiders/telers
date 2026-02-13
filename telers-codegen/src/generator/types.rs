use crate::{
    file::camel_to_filename,
    generator::helpers::{
        format_attr_description, format_description, get_singular_and_plural_forms,
        sanitize_field_name,
    },
    parser::api::{
        BooleanKind, IntegerKind, NormalizedField, NormalizedSchema, NormalizedType, SubtypeKind,
        TypeKindInField,
    },
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

impl ToTokens for TypeKindInField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = match self {
            TypeKindInField::String => quote! { String },
            TypeKindInField::Integer(kind) => match kind {
                IntegerKind::Int64 => quote! { i64 },
                IntegerKind::Int32 => quote! { i32 },
                IntegerKind::Int16 => quote! { i16 },
                IntegerKind::Float32 => quote! { f32 },
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
            TypeKindInField::Array(inner) => quote! { Vec<#inner> },
        };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = sanitize_field_name(&self.name);
        let ty = &self.r#type;
        let doc = format_attr_description(&self.description);

        let ts = if self.required {
            quote! {
                #[doc = #doc]
                pub #name: #ty,
            }
        } else {
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
            let fields: Box<[_]> = self
                .fields
                .iter()
                .filter(|field| {
                    if let Some(SubtypeKind::Tagged { ref tag_field }) = self.subtype_kind {
                        field.name != *tag_field
                    } else {
                        true
                    }
                })
                .collect();
            quote! {
                #( #[doc = #doc_lines] )*
                #[derive(Clone, Debug, Serialize)]
                pub struct #name {
                    #( #fields )*
                }
            }
        } else {
            let variants: Box<[_]> = self
                .subtypes
                .iter()
                .map(|name| {
                    let variant_name = format_ident!("{name}");
                    quote! { #variant_name(#variant_name) }
                })
                .collect();

            let serde_attr = match &self.subtype_kind {
                Some(SubtypeKind::Tagged { tag_field }) => quote! {
                    #[serde(tag = #tag_field)]
                },
                Some(SubtypeKind::Untagged) => quote! {
                    #[serde(untagged)]
                },
                None => quote! {},
            };

            quote! {
                #( #[doc = #doc_lines] )*
                #[derive(Clone, Debug, Serialize)]
                #serde_attr
                pub enum #name {
                    #( #variants, )*
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
            let subtype_name = format_ident!("{subtype}");
            quote! {
                impl From<#subtype_name> for #name {
                    fn from(subtype: #subtype_name) -> Self {
                        #name::#subtype_name(subtype)
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

    let required_fields: Vec<_> = fields.iter().filter(|f| f.required).copied().collect();
    let optional_fields: Vec<_> = fields.iter().filter(|f| !f.required).copied().collect();

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
                    quote! { #name: #name.into() }
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
                pub fn new() -> Self {
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
                        let mut #name = self.#name;
                        #name.extend(val.into());
                        Self { #name, ..self }
                    }
                } else {
                    quote! {
                        let mut #name = self.#name.unwrap_or_default();
                        #name.extend(val.into());
                        Self { #name: Some(#name), ..self }
                    }
                };
                methods.push(quote! {
                    #doc_lines
                    #[must_use]
                    pub fn #plural_method_name(self, val: impl Into<Vec<#inner>>) -> Self {
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
                            let mut #name = self.#name;
                            #name.push(val.into());
                            Self { #name, ..self }
                        }
                    } else {
                        quote! {
                            let mut #name = self.#name.unwrap_or_default();
                            #name.push(val.into());
                            Self { #name: Some(#name), ..self }
                        }
                    };

                    methods.push(quote! {
                        #doc_lines
                        #[must_use]
                        pub fn #singular_method_name(self, val: impl Into<#inner>) -> Self {
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
                        pub fn #method_name(self, val: Option<impl Into<Vec<#inner>>>) -> Self {
                            Self { #name: val.map(Into::into), ..self }
                        }
                    });
                }
            } else {
                let doc = format_attr_description(&field.description);
                let value = if field.required {
                    quote! { val.into() }
                } else {
                    quote! { Some(val.into()) }
                };
                methods.push(quote! {
                    #[doc = #doc]
                    #[must_use]
                    pub fn #name(self, val: impl Into<#ty>) -> Self {
                        Self { #name: #value, ..self }
                    }
                });

                if !field.required {
                    let method_name = format_ident!("{}_option", field.name);
                    methods.push(quote! {
                        #[doc = #doc]
                        #[must_use]
                        pub fn #method_name(self, val: Option<impl Into<#ty>>) -> Self {
                            Self { #name: val.map(Into::into), ..self }
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

pub fn tokenize_type(type_quote: &NormalizedType, _schema: &NormalizedSchema) -> TokenStream {
    let paths = type_quote.get_paths();

    let imports_quote = quote! { #(use #paths;)* };
    let impls_for_subtypes = get_from_impls_for_subtypes(type_quote);
    let impls_for_subtypes_quote = quote! { #(#impls_for_subtypes)* };
    let builder_impl = builder_impl_for_type(type_quote);

    quote! {
        use serde::Serialize;

        #imports_quote
        #type_quote
        #impls_for_subtypes_quote
        #builder_impl
    }
}

pub fn tokenize_types_mod(type_names: &[&String]) -> TokenStream {
    let mods_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        quote! {
            pub mod #mod_name;
        }
    });
    let uses_quote = type_names.iter().map(|&name| {
        let mod_name = format_ident!("{}", camel_to_filename(name, None));
        let type_name = format_ident!("{name}");
        quote! {
            pub use #mod_name::#type_name;
        }
    });

    quote! {
        #( #mods_quote )*
        #( #uses_quote )*
    }
}

#[cfg(test)]
mod tests {
    use super::{
        tokenize_type, IntegerKind, NormalizedField, NormalizedSchema, NormalizedType, SubtypeKind,
        TypeKindInField,
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
                },
                NormalizedField {
                    name: "text".into(),
                    required: false,
                    description: "Message text".into(),
                    r#type: TypeKindInField::String,
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
        assert!(result.contains("text : Option < String >"));
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
            subtypes: vec!["MessageOriginUser".into(), "MessageOriginHiddenUser".into()],
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
            subtypes: vec!["Message".into(), "InaccessibleMessage".into()],
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
