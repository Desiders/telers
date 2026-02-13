use crate::{
    generator::helpers::{format_attr_description, format_description, sanitize_field_name},
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
                pub #name: Option<#ty>,
            }
        };
        tokens.extend(ts);
    }
}

impl ToTokens for NormalizedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.as_str());
        let description_lines = format_description(
            self.description.iter().map(String::as_str).collect(),
            &self.href,
        );

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
                #( #[doc = #description_lines] )*
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
                #( #[doc = #description_lines] )*
                #serde_attr
                pub enum #name {
                    #( #variants, )*
                }
            }
        };

        tokens.extend(ts);
    }
}

pub fn get_from_impls_for_subtypes(
    type_quote: &NormalizedType,
    _schema: &NormalizedSchema,
) -> Vec<TokenStream> {
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

pub fn tokenize_type(type_quote: &NormalizedType, _schema: &NormalizedSchema) -> TokenStream {
    let paths = type_quote.get_paths();

    let imports_quote = quote! { #(use #paths;)* };
    let impls_for_subtypes = get_from_impls_for_subtypes(type_quote, _schema);
    let impls_for_subtypes_quote = quote! { #(#impls_for_subtypes)* };

    let file_quote = quote! {
        #imports_quote
        #type_quote
        #impls_for_subtypes_quote
    };

    file_quote
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
        assert!(result.contains("Message text"));
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
