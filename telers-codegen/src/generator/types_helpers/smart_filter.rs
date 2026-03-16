use crate::{
    generator::{helpers::sanitize_field_name, type_utils::collect_common_fields},
    parser::api::{
        NormalizedField, NormalizedSchema, NormalizedSubtypeVariant, NormalizedType,
        TypeKindInField,
    },
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashSet};

struct MethodSpec {
    name: String,
    target_ty: TokenStream,
    fully_required: bool,
}

type NestedEntry<'a> = (
    &'a NormalizedSubtypeVariant,
    &'a NormalizedField,
    &'a NormalizedField,
    bool,
    bool,
);

#[must_use]
fn target_filter_ty(field_ty: &TypeKindInField) -> TokenStream {
    match field_ty {
        TypeKindInField::String => quote! { str },
        TypeKindInField::Array(inner) => quote! { [#inner] },
        _ => quote! { #field_ty },
    }
}

#[must_use]
fn struct_field_access_expr(field: &NormalizedField) -> TokenStream {
    let field_ident = sanitize_field_name(&field.name);

    match &field.r#type {
        TypeKindInField::Array(_) | TypeKindInField::String => {
            if field.required {
                quote! { value.#field_ident.as_ref() }
            } else {
                quote! { value.#field_ident.as_deref() }
            }
        }
        field_ty if field_ty.is_copy() => {
            if field.required {
                quote! { &value.#field_ident }
            } else {
                quote! { value.#field_ident.as_ref() }
            }
        }
        _ => {
            if field.is_recursive || field.is_boxed {
                if field.required {
                    quote! { value.#field_ident.as_ref() }
                } else {
                    quote! { value.#field_ident.as_deref() }
                }
            } else if field.required {
                quote! { &value.#field_ident }
            } else {
                quote! { value.#field_ident.as_ref() }
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
#[must_use]
fn enum_method_specs(
    type_quote: &NormalizedType,
    schema: &NormalizedSchema,
    include_nested: bool,
) -> Vec<MethodSpec> {
    let (tag_field, parent_tag_field) = type_quote
        .subtype_kind
        .as_ref()
        .map(|kind| kind.get_tags())
        .unwrap_or_default();

    let mut method_infos: BTreeMap<String, (&TypeKindInField, bool)> = BTreeMap::new();

    // First, collect direct fields from all subtypes
    let mut fields_subtypes_map: BTreeMap<
        &str,
        Vec<(&NormalizedSubtypeVariant, &NormalizedField)>,
    > = BTreeMap::new();

    for subtype in &type_quote.subtypes {
        let ty = schema.types.get(&subtype.ty_name).unwrap();
        for field in &ty.fields {
            if field.is_tagged(tag_field, parent_tag_field) {
                continue;
            }
            fields_subtypes_map
                .entry(&field.name)
                .or_default()
                .push((subtype, field));
        }
    }

    // Process direct fields
    for (&field_name, subtypes) in &fields_subtypes_map {
        let field = &subtypes[0].1;

        // Skip copy types as they're handled by eq/ne methods
        if field.r#type.is_copy() {
            continue;
        }

        // All subtypes must have the same field type
        let is_identical_field_type = subtypes.iter().all(|(_, f)| f.r#type == field.r#type);
        if !is_identical_field_type {
            continue;
        }

        let is_common = subtypes.len() == type_quote.subtypes.len();
        let is_required_for_all = is_common && subtypes.iter().all(|(_, f)| f.required);
        method_infos.insert(field_name.to_string(), (&field.r#type, is_required_for_all));
    }

    // Process nested fields if requested (only for Update type)
    if include_nested {
        let mut nested_map: BTreeMap<String, Vec<NestedEntry<'_>>> = BTreeMap::new();

        for outer_subtypes in fields_subtypes_map.values() {
            let first_outer = outer_subtypes[0].1;
            let outer_ty = &first_outer.r#type;

            // Only process Telegram fields
            let TypeKindInField::Telegram(inner_type_name) = outer_ty else {
                continue;
            };

            // All subtypes must have the same outer field type
            if !outer_subtypes.iter().all(|(_, f)| &f.r#type == outer_ty) {
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

            // For each common field in the inner type
            for (inner_field_name, (inner_field, inner_field_fully_required, _)) in &inner_common {
                // Skip copy types
                if inner_field.r#type.is_copy() {
                    continue;
                }

                // Skip tagged fields in non-enum inner types
                if !inner_is_enum && inner_field.is_tagged(inner_tag, inner_parent_tag) {
                    continue;
                }

                // Record this nested field for each outer subtype
                for (subtype, outer_field) in outer_subtypes {
                    nested_map
                        .entry((*inner_field_name).to_string())
                        .or_default()
                        .push((
                            *subtype,
                            outer_field,
                            inner_field,
                            inner_is_enum,
                            *inner_field_fully_required,
                        ));
                }
            }
        }

        // Process collected nested fields
        for (inner_field_name, entries) in nested_map {
            // Don't override direct fields
            if method_infos.contains_key(inner_field_name.as_str()) {
                continue;
            }

            // All entries must have the same inner field type
            let inner_ty = &entries[0].2.r#type;
            if entries.iter().any(|(_, _, f, ..)| &f.r#type != inner_ty) {
                continue;
            }

            // Check that we have entries for all subtypes
            let mut seen = HashSet::new();
            let has_all_subtypes = entries
                .iter()
                .all(|(s, ..)| seen.insert(s.variant.as_str()));

            if !has_all_subtypes {
                continue;
            }

            let is_all_covered = entries.len() == type_quote.subtypes.len();

            // Determine if field is required across all paths
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
                && entries.iter().all(|(_, outer, ..)| outer.required);

            method_infos.insert(inner_field_name, (inner_ty, fully_required));
        }
    }

    method_infos
        .into_iter()
        .map(|(name, (field_ty, fully_required))| MethodSpec {
            name,
            target_ty: target_filter_ty(field_ty),
            fully_required,
        })
        .collect()
}

#[must_use]
fn tokenize_struct_type_methods(type_quote: &NormalizedType) -> TokenStream {
    let type_name = format_ident!("{}", type_quote.name);
    let (tag_field, parent_tag_field) = type_quote
        .subtype_kind
        .as_ref()
        .map(|kind| kind.get_tags())
        .unwrap_or_default();

    let methods = type_quote
        .fields
        .iter()
        .filter(|f| !f.is_tagged(tag_field, parent_tag_field))
        .map(|field| {
            let method_name = sanitize_field_name(&field.name);
            let target_ty = target_filter_ty(&field.r#type);
            let access = struct_field_access_expr(field);

            if field.required {
                quote! {
                    #[must_use]
                    pub fn #method_name(self) -> SmartFilterPath<#target_ty> {
                        self.map(|value| #access)
                    }
                }
            } else {
                quote! {
                    #[must_use]
                    pub fn #method_name(self) -> SmartFilterPath<#target_ty> {
                        self.and_then(|value| #access)
                    }
                }
            }
        });

    quote! {
        impl SmartFilterPath<crate::types::#type_name> {
            #( #methods )*
        }
    }
}

#[must_use]
fn tokenize_enum_type_methods(
    type_quote: &NormalizedType,
    schema: &NormalizedSchema,
) -> TokenStream {
    let type_name = format_ident!("{}", type_quote.name);
    let include_nested = type_quote.name == "Update";
    let methods = enum_method_specs(type_quote, schema, include_nested)
        .into_iter()
        .map(|method| {
            let method_ident = sanitize_field_name(&method.name);
            let target_ty = method.target_ty;

            if method.fully_required {
                quote! {
                    #[must_use]
                    pub fn #method_ident(self) -> SmartFilterPath<#target_ty> {
                        self.map(|value| value.#method_ident())
                    }
                }
            } else {
                quote! {
                    #[must_use]
                    pub fn #method_ident(self) -> SmartFilterPath<#target_ty> {
                        self.and_then(|value| value.#method_ident())
                    }
                }
            }
        });

    quote! {
        impl SmartFilterPath<crate::types::#type_name> {
            #( #methods )*
        }
    }
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn tokenize_smart_filter(schema: &NormalizedSchema) -> TokenStream {
    let update = schema
        .types
        .get("Update")
        .expect("schema must contain Update type");
    let update_constructors = enum_method_specs(update, schema, true)
        .into_iter()
        .map(|method| {
            let method_ident = sanitize_field_name(&method.name);
            let target_ty = method.target_ty;
            quote! {
                #[must_use]
                pub fn #method_ident() -> SmartFilterPath<#target_ty> {
                    Self::update().#method_ident()
                }
            }
        });

    let mut type_names: Vec<_> = schema.types.keys().collect();
    type_names.sort_unstable();

    let type_impls = type_names.into_iter().map(|name| {
        let ty = schema.types.get(name).unwrap();
        if ty.subtypes.is_empty() {
            tokenize_struct_type_methods(ty)
        } else {
            tokenize_enum_type_methods(ty, schema)
        }
    });

    quote! {
        #![allow(clippy::wrong_self_convention)]
        #![allow(clippy::redundant_closure_for_method_calls)]

        mod core;

        use crate::types::Update;

        use core::SmartFilterPath;
        use std::sync::Arc;

        pub struct SmartFilter;

        impl SmartFilter {
            #[must_use]
            pub fn update() -> SmartFilterPath<Update> {
                SmartFilterPath {
                    accessor: Arc::new(|update| Some(update)),
                }
            }

            #( #update_constructors )*
        }

        #( #type_impls )*
    }
}
