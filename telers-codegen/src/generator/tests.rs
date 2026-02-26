use crate::{
    generator::helpers::camel_to_snake,
    parser::api::{IntegerKind, NormalizedSchema, NormalizedType, TypeKindInField},
};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::{LitStr, Path};

pub fn tokenize_tests(schema: &NormalizedSchema, types_path: &str) -> TokenStream {
    let types_path: Path =
        syn::parse_str(types_path).expect("invalid types path, expected Rust path");
    let mut types = schema.types.values().collect::<Vec<_>>();
    types.sort_by(|a, b| a.name.cmp(&b.name));
    let mut generated_for = HashSet::new();
    let mut tests = vec![];
    for ty in types {
        let subtypes = get_subtypes_from_description(ty, schema);
        if subtypes.is_empty() {
            if generated_for.insert(ty.name.clone()) {
                tests.push(tokenize_type_test(ty, schema));
            }
            continue;
        }

        for subtype_name in subtypes {
            if !generated_for.insert(subtype_name.clone()) {
                continue;
            }
            let Some(subtype) = schema.types.get(&subtype_name) else {
                continue;
            };
            tests.push(tokenize_type_test(subtype, schema));
        }
    }

    quote! {
        use #types_path::types::*;
        use serde::{Serialize, de::DeserializeOwned};

        fn must_parse<T: DeserializeOwned>(type_name: &str, value: &serde_json::Value) -> T {
            serde_json::from_value(value.clone()).unwrap_or_else(|err| {
                panic!("failed to deserialize {type_name} from JSON: {err}; json: {value}")
            })
        }

        fn must_to_value<T: Serialize>(type_name: &str, value: &T) -> serde_json::Value {
            serde_json::to_value(value).unwrap_or_else(|err| {
                panic!("failed to convert {type_name} to JSON value after deserialize: {err}")
            })
        }

        fn must_roundtrip<T: Serialize + DeserializeOwned>(type_name: &str, value: &T) {
            let serialized = serde_json::to_string(value).unwrap_or_else(|err| {
                panic!("failed to serialize {type_name} after deserialize: {err}")
            });
            serde_json::from_str::<T>(&serialized).unwrap_or_else(|err| {
                panic!(
                    "failed roundtrip deserialize {type_name} from JSON: {err}; json: {serialized}"
                )
            });
        }

        fn assert_json_subset(actual: &serde_json::Value, expected: &serde_json::Value) {
            match (actual, expected) {
                (serde_json::Value::Object(actual_obj), serde_json::Value::Object(expected_obj)) => {
                    for (key, actual_val) in actual_obj {
                        let expected_val = expected_obj.get(key).unwrap_or_else(|| {
                            panic!("missing key in expected json: {key}");
                        });
                        assert_json_subset(actual_val, expected_val);
                    }
                }
                (serde_json::Value::Array(actual_arr), serde_json::Value::Array(expected_arr)) => {
                    assert_eq!(
                        actual_arr.len(),
                        expected_arr.len(),
                        "array length mismatch: actual={}, expected={}",
                        actual_arr.len(),
                        expected_arr.len()
                    );
                    for (actual_item, expected_item) in actual_arr.iter().zip(expected_arr.iter()) {
                        assert_json_subset(actual_item, expected_item);
                    }
                }
                _ => assert_eq!(actual, expected),
            }
        }

        #( #tests )*
    }
}

pub fn tokenize_type_test(ty: &NormalizedType, schema: &NormalizedSchema) -> TokenStream {
    let parse_target = get_parse_target(ty, schema);
    let parse_type_name = parse_target
        .as_ref()
        .map_or_else(|| ty.name.clone(), |target| target.parse_type_name.clone());
    let parse_variant_name = parse_target
        .as_ref()
        .and_then(|target| target.parse_variant_name.clone());
    let type_name = format_ident!("{}", parse_type_name);
    let test_name = format_ident!("test_{}_serialize_deserialize", camel_to_snake(&ty.name));
    let value = generate_example_json_value(
        ty,
        schema,
        parse_target
            .as_ref()
            .and_then(|target| target.forced_tag.as_ref()),
    );
    let variant_assertion = parse_variant_name.map(|variant| {
        let variant_name = format_ident!("{}", variant);
        quote! {
            assert!(
                matches!(&parsed, #type_name::#variant_name(_)),
                "failed to deserialize {} into expected subtype {}; parsed={:?}",
                stringify!(#type_name),
                stringify!(#variant_name),
                parsed
            );
        }
    });

    quote! {
        #[test]
        fn #test_name() {
            let value = serde_json::json!(#value);
            let parsed: #type_name = must_parse(stringify!(#type_name), &value);
            #variant_assertion
            let parsed_value = must_to_value(stringify!(#type_name), &parsed);
            assert_json_subset(&parsed_value, &value);
            must_roundtrip(stringify!(#type_name), &parsed);
        }
    }
}

struct ParseTarget {
    parse_type_name: String,
    parse_variant_name: Option<String>,
    forced_tag: Option<TagField>,
}

struct TagField {
    field_name: String,
    field_value: String,
}

fn get_parse_target(ty: &NormalizedType, schema: &NormalizedSchema) -> Option<ParseTarget> {
    let parent_name = ty.subtype_of.first()?;
    let parent = schema.types.get(parent_name)?;
    let variant = parent
        .subtypes
        .iter()
        .find(|subtype| subtype.ty_name == ty.name)
        .map(|subtype| subtype.variant.clone())?;

    let tag_field = parent
        .subtype_kind
        .as_ref()
        .and_then(|kind| kind.get_tags().0)
        .map(|tag_field| TagField {
            field_name: tag_field.to_owned(),
            field_value: camel_to_snake(&variant),
        });

    Some(ParseTarget {
        parse_type_name: parent.name.clone(),
        parse_variant_name: Some(variant),
        forced_tag: tag_field,
    })
}

fn get_subtypes_from_description(ty: &NormalizedType, schema: &NormalizedSchema) -> Vec<String> {
    let mut subtype_names = vec![];
    let mut read_subtypes = false;

    for line in &ty.description {
        if !read_subtypes {
            if line.contains("Currently, it can be one of") {
                read_subtypes = true;
            }
            continue;
        }

        let trimmed = line.trim();
        if !trimmed.starts_with("- ") {
            break;
        }

        let candidate = trimmed
            .trim_start_matches("- ")
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<String>();

        if candidate.is_empty() || !schema.types.contains_key(&candidate) {
            continue;
        }
        subtype_names.push(candidate);
    }

    subtype_names
}

fn generate_example_json_value(
    ty: &NormalizedType,
    schema: &NormalizedSchema,
    forced_tag: Option<&TagField>,
) -> TokenStream {
    let mut visiting = HashSet::new();
    generate_type_json_value(ty, schema, &mut visiting, forced_tag)
}

fn generate_type_json_value(
    ty: &NormalizedType,
    schema: &NormalizedSchema,
    visiting: &mut HashSet<String>,
    forced_tag: Option<&TagField>,
) -> TokenStream {
    if !visiting.insert(ty.name.clone()) {
        return quote!({});
    }

    if let Some(subtype) = ty.subtypes.first() {
        if let Some(subtype_ty) = schema.types.get(&subtype.ty_name) {
            let value = generate_type_json_value(subtype_ty, schema, visiting, forced_tag);
            visiting.remove(&ty.name);
            return value;
        }
    }

    let mut fields = ty
        .fields
        .iter()
        .filter(|field| field.required)
        .filter_map(|field| {
            if let Some(tag) = forced_tag {
                if field.name == tag.field_name {
                    let name = LitStr::new(tag.field_name.as_str(), Span::call_site());
                    let value = LitStr::new(tag.field_value.as_str(), Span::call_site());
                    return Some(quote! { #name: #value });
                }
            }

            let name = LitStr::new(field.name.as_str(), Span::call_site());
            generate_field_json_value(
                &field.r#type,
                schema,
                visiting,
                field.description.as_str(),
                None,
            )
            .map(|value| quote! { #name: #value })
        })
        .collect::<Vec<_>>();

    if let Some(tag) = forced_tag {
        if !ty.fields.iter().any(|field| field.name == tag.field_name) {
            let name = LitStr::new(tag.field_name.as_str(), Span::call_site());
            let value = LitStr::new(tag.field_value.as_str(), Span::call_site());
            fields.push(quote! { #name: #value });
        }
    }

    visiting.remove(&ty.name);

    if fields.is_empty() {
        quote!({})
    } else {
        quote!({ #(#fields),* })
    }
}

fn generate_field_json_value(
    kind: &TypeKindInField,
    schema: &NormalizedSchema,
    visiting: &mut HashSet<String>,
    description: &str,
    forced_tag: Option<&TagField>,
) -> Option<TokenStream> {
    match kind {
        TypeKindInField::String => {
            let value =
                extract_tagged_string_value(description).unwrap_or_else(|| "test".to_owned());
            let value = LitStr::new(value.as_str(), Span::call_site());
            Some(quote! { #value })
        }
        TypeKindInField::Integer(kind) => match kind {
            IntegerKind::Float32 | IntegerKind::Float64 => Some(quote! { 3.14 }),
            _ => Some(quote! { 1 }),
        },
        TypeKindInField::Boolean(_) => Some(quote! { true }),
        TypeKindInField::ChatId => Some(quote! { "123" }),
        TypeKindInField::InputFile => Some(quote! { "" }),
        TypeKindInField::Array(inner) => {
            let inner_value =
                generate_field_json_value(inner, schema, visiting, description, None)?;
            Some(quote! { [#inner_value] })
        }
        TypeKindInField::Telegram(name) => {
            let ty = schema.types.get(name)?;
            Some(generate_type_json_value(ty, schema, visiting, forced_tag))
        }
        TypeKindInField::Either(left, _right) => {
            let left_value = generate_field_json_value(left, schema, visiting, description, None)?;
            Some(quote! { either::Either::Left(#left_value) })
        }
    }
}

fn extract_tagged_string_value(description: &str) -> Option<String> {
    let description = description.to_lowercase();
    const MARKERS: &[&str] = &[
        "always",
        "must be",
        "can be either",
        "one of",
        "can be",
        "currently one of",
        "currently can be",
        "for example",
    ];

    let search_area = MARKERS.iter().find_map(|marker| {
        description
            .find(marker)
            .map(|idx| &description[idx + marker.len()..])
    })?;

    extract_first_quoted_token(search_area).or_else(|| extract_first_bare_token(search_area))
}

fn extract_first_quoted_token(input: &str) -> Option<String> {
    extract_between(input, '"', '"')
        .or_else(|| extract_between(input, '“', '”'))
        .or_else(|| extract_between(input, '`', '`'))
}

fn extract_between(input: &str, start: char, end: char) -> Option<String> {
    let start_idx = input.find(start)?;
    let rest = &input[start_idx + start.len_utf8()..];
    let end_idx = rest.find(end)?;
    let token = rest[..end_idx].trim();
    if is_identifier_like(token) {
        Some(token.to_owned())
    } else {
        None
    }
}

fn extract_first_bare_token(input: &str) -> Option<String> {
    let words = input.split_whitespace();
    for word in words {
        let token = word
            .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .trim();
        if is_identifier_like(token) {
            return Some(token.to_owned());
        }
    }
    None
}

fn is_identifier_like(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
