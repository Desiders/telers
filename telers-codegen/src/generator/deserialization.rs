#![allow(clippy::too_many_lines)]

use crate::{
    file::camel_to_filename,
    generator::{
        helpers::{sanitize_field_name, snake_to_upper_camel},
        types::tokenize_field_type,
    },
    parser::api::{
        NormalizedField, NormalizedSchema, NormalizedType, SubtypeKind, TypeKindInField,
    },
};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap, HashSet},
};

pub struct DeserializeImplementation {
    pub import: TokenStream,
    pub body: TokenStream,
}

pub struct Deserializers {
    pub root: TokenStream,
    pub modules: BTreeMap<String, TokenStream>,
    pub implementations: HashMap<String, DeserializeImplementation>,
}

fn fields(ty: &NormalizedType) -> Vec<&NormalizedField> {
    let (tag, parent_tag) = ty
        .subtype_kind
        .as_ref()
        .map(SubtypeKind::get_tags)
        .unwrap_or_default();
    ty.fields
        .iter()
        .filter(|field| !field.is_tagged(tag, parent_tag))
        .collect()
}

fn same_field(left: &NormalizedField, right: &NormalizedField) -> bool {
    left.name == right.name
        && left.r#type == right.r#type
        && left.required == right.required
        && left.force_default == right.force_default
        && (left.is_recursive || left.is_boxed) == (right.is_recursive || right.is_boxed)
}

struct Group<'a> {
    name: &'a str,
    common: Vec<&'a NormalizedField>,
    members: BTreeMap<&'a str, &'a NormalizedType>,
}

fn groups(schema: &NormalizedSchema) -> Vec<Group<'_>> {
    let mut parents = schema.types.values().collect::<Vec<_>>();
    parents.sort_by(|left, right| left.name.cmp(&right.name));
    let mut groups: Vec<Group<'_>> = Vec::new();
    for parent in parents {
        let members = parent
            .subtypes
            .iter()
            .filter_map(|subtype| schema.types.get(&subtype.ty_name))
            .filter(|ty| ty.subtypes.is_empty() && !ty.has_extra_fields)
            .map(|ty| (ty.name.as_str(), ty))
            .collect::<BTreeMap<_, _>>();
        if members.len() < 2 {
            continue;
        }
        let mut common = fields(members.values().next().unwrap());
        common.retain(|field| {
            members
                .values()
                .all(|ty| fields(ty).iter().any(|other| same_field(field, other)))
        });
        if common.is_empty() {
            continue;
        }
        common.sort_by(|left, right| left.name.cmp(&right.name));
        if let Some(group) = groups.iter_mut().find(|group| {
            group.members.keys().any(|name| members.contains_key(name))
                && group.common.len() == common.len()
                && group
                    .common
                    .iter()
                    .zip(&common)
                    .all(|(left, right)| same_field(left, right))
        }) {
            group.members.extend(members);
        } else {
            groups.push(Group {
                name: &parent.name,
                common,
                members,
            });
        }
    }
    // Prefer the largest reusable part when a type belongs to several unions.
    groups.sort_by_key(|group| Reverse(group.common.len() * (group.members.len() - 1)));
    let mut assigned = HashSet::new();
    groups.retain_mut(|group| {
        group.members.retain(|name, _| !assigned.contains(name));
        if group.members.len() < 2 {
            return false;
        }
        assigned.extend(group.members.keys().copied());
        true
    });
    groups.sort_by_key(|group| group.name);
    groups
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Common(usize),
    Variant {
        default: bool,
        sequence_default: bool,
    },
}

fn field_layout(ty: &NormalizedType, common: &[&NormalizedField]) -> Vec<Field> {
    fields(ty)
        .iter()
        .map(|field| {
            common
                .iter()
                .position(|other| same_field(field, other))
                .map_or(
                    Field::Variant {
                        default: !field.required || field.force_default,
                        sequence_default: field.required && field.force_default,
                    },
                    Field::Common,
                )
        })
        .collect()
}

struct Layout<'a> {
    fields: Vec<Field>,
    members: Vec<&'a NormalizedType>,
    deserialize: Ident,
    result: Ident,
    visitor: Ident,
    field_names: Ident,
    variant_fields: Vec<Ident>,
    variant_types: Vec<TokenStream>,
    parameters: Vec<Ident>,
}

fn short_names(group: &Group<'_>) -> HashMap<String, String> {
    let parent = camel_to_filename(group.name, None);
    let mut prefix = parent.split('_').collect::<Vec<_>>();
    let names = group
        .members
        .keys()
        .map(|name| (*name, camel_to_filename(name, None)))
        .collect::<Vec<_>>();
    for (_, name) in &names {
        let count = prefix
            .iter()
            .zip(name.split('_'))
            .take_while(|(left, right)| **left == *right)
            .count();
        prefix.truncate(count);
    }
    names
        .into_iter()
        .map(|(name, words)| {
            let short = words
                .split('_')
                .skip(prefix.len())
                .collect::<Vec<_>>()
                .join("_");
            (
                name.to_owned(),
                if short.is_empty() {
                    name.to_owned()
                } else {
                    snake_to_upper_camel(&short)
                },
            )
        })
        .collect()
}

fn layouts<'a>(group: &Group<'a>, names: &HashMap<String, String>) -> Vec<Layout<'a>> {
    let mut layouts: Vec<Layout<'_>> = Vec::new();
    for ty in group.members.values() {
        let fields = field_layout(ty, &group.common);
        if let Some(layout) = layouts.iter_mut().find(|layout| layout.fields == fields) {
            layout.members.push(ty);
        } else {
            layouts.push(Layout {
                fields,
                members: vec![ty],
                deserialize: format_ident!("deserialize"),
                result: format_ident!("DecodedFields"),
                visitor: format_ident!("FieldsVisitor"),
                field_names: format_ident!("field_names"),
                variant_fields: vec![],
                variant_types: vec![],
                parameters: vec![],
            });
        }
    }
    let multiple = layouts.len() > 1;
    let mut used_stems = HashSet::new();
    for layout in &mut layouts {
        let variant_types = layout
            .members
            .iter()
            .map(|ty| {
                fields(ty)
                    .into_iter()
                    .zip(&layout.fields)
                    .filter(|(_, kind)| matches!(kind, Field::Variant { .. }))
                    .map(|(field, _)| tokenize_field_type(field))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let shared_types = variant_types.iter().all(|types| {
            types
                .iter()
                .zip(&variant_types[0])
                .all(|(left, right)| left.to_string() == right.to_string())
        });
        let variants = layout
            .members
            .iter()
            .map(|ty| {
                fields(ty)
                    .into_iter()
                    .zip(&layout.fields)
                    .filter(|(_, field)| matches!(field, Field::Variant { .. }))
                    .map(|(field, _)| field.name.as_str())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let count = variants[0].len();
        let mut used_fields = HashSet::from(["common".to_owned()]);
        for index in 0..count {
            let mut name = if variants
                .iter()
                .all(|fields| fields[index] == variants[0][index])
            {
                variants[0][index].to_owned()
            } else if count == 1 {
                "content".to_owned()
            } else {
                format!("field_{}", index + 1)
            };
            while !used_fields.insert(name.clone()) {
                name = format!("variant_{name}");
            }
            layout.variant_fields.push(sanitize_field_name(&name));
            if shared_types {
                layout.variant_types.push(variant_types[0][index].clone());
            } else {
                let parameter = format_ident!("T{}", snake_to_upper_camel(&name));
                layout.variant_types.push(quote! { #parameter });
                layout.parameters.push(parameter);
            }
        }
        let mut stem = if !multiple {
            String::new()
        } else if count == 0 {
            "Common".to_owned()
        } else if layout.members.len() == 1 {
            names[&layout.members[0].name].clone()
        } else if count == 1 {
            "Content".to_owned()
        } else if layout.members.len() == 2 {
            layout
                .members
                .iter()
                .map(|ty| names[&ty.name].as_str())
                .collect::<Vec<_>>()
                .join("Or")
        } else {
            snake_to_upper_camel(variants[0][0])
        };
        while !used_stems.insert(stem.clone()) {
            stem.push_str("Alternative");
        }
        if !stem.is_empty() {
            let snake = camel_to_filename(&stem, None);
            layout.deserialize = format_ident!("deserialize_{snake}");
            layout.result = format_ident!("{stem}Fields");
            layout.visitor = format_ident!("{stem}Visitor");
            layout.field_names = format_ident!("{snake}_field_names");
        }
        if count == 0 {
            layout.field_names = format_ident!("{}", layout.field_names.to_string().to_uppercase());
        }
    }
    layouts
}

fn metadata_name(ty: &NormalizedType, names: &HashMap<String, String>) -> Ident {
    format_ident!(
        "{}",
        camel_to_filename(&names[&ty.name], None).to_uppercase()
    )
}

fn finish_field(field: &NormalizedField, slot: &TokenStream) -> TokenStream {
    let name = &field.name;
    if !field.required {
        quote! { #slot.unwrap_or(None) }
    } else if field.force_default {
        quote! { #slot.unwrap_or_default() }
    } else {
        quote! { #slot.ok_or_else(|| A::Error::missing_field(#name))? }
    }
}

fn tokenize_layout(common: &[&NormalizedField], layout: &Layout<'_>) -> TokenStream {
    let Layout {
        deserialize,
        result,
        visitor,
        field_names,
        variant_fields,
        variant_types,
        parameters,
        ..
    } = layout;
    let generic_args = (!parameters.is_empty()).then(|| quote! { <#(#parameters),*> });
    let result_type = if variant_fields.is_empty() {
        quote! { CommonFields }
    } else {
        quote! { #result #generic_args }
    };
    let mut bounds = Vec::new();
    let mut declarations = Vec::new();
    let mut map_arms = Vec::new();
    let mut sequence = Vec::new();
    let mut finish = Vec::new();
    let mut wire_names = Vec::new();
    let mut variant_index = 0;
    for (field_index, field) in layout.fields.iter().enumerate() {
        let sequence_index = Literal::usize_unsuffixed(field_index);
        match *field {
            Field::Common(index) => {
                let field = common[index];
                let name = sanitize_field_name(&field.name);
                let value = format_ident!("common_{}", field.name);
                let read = if field.required && field.force_default {
                    quote! { seq.next_element()?.unwrap_or_default() }
                } else {
                    quote! { read_seq_field(&mut seq, #sequence_index, &expected)? }
                };
                sequence.push(quote! { let #value = #read; });
                let read = finish_field(field, &quote! { shared.#name });
                finish.push(quote! { let #value = #read; });
                let wire_name = &field.name;
                wire_names.push(quote! { #wire_name });
            }
            Field::Variant {
                default,
                sequence_default,
            } => {
                let ty = &variant_types[variant_index];
                let name = &variant_fields[variant_index];
                let value = format_ident!("variant_{}", name.to_string().trim_start_matches("r#"));
                let index = Literal::usize_unsuffixed(variant_index);
                let key = Literal::usize_unsuffixed(common.len() + variant_index);
                if let Some(parameter) = parameters.get(variant_index) {
                    bounds.push(if default {
                        quote! { #parameter: Deserialize<'de> + Default }
                    } else {
                        quote! { #parameter: Deserialize<'de> }
                    });
                }
                declarations.push(quote! { let mut #value: Option<#ty> = None; });
                map_arms.push(quote! {
                    #key => {
                        read_map_field(&mut #value, &mut map, self.metadata.variant_fields[#index])?;
                    }
                });
                let read = if sequence_default {
                    quote! { seq.next_element()?.unwrap_or_default() }
                } else {
                    quote! { read_seq_field(&mut seq, #sequence_index, &expected)? }
                };
                sequence.push(quote! { let #value = #read; });
                finish.push(if default {
                    quote! { let #value = #value.unwrap_or_default(); }
                } else {
                    quote! {
                        let #value = #value.ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[#index]))?;
                    }
                });
                wire_names.push(quote! { variant_fields[#index] });
                variant_index += 1;
            }
        }
    }
    let common_fields = common
        .iter()
        .map(|field| {
            let name = sanitize_field_name(&field.name);
            let value = format_ident!("common_{}", field.name);
            quote! { #name: #value }
        })
        .collect::<Vec<_>>();
    let common_value = quote! { CommonFields { #(#common_fields,)* } };
    let variant_values = variant_fields
        .iter()
        .map(|name| {
            let value = format_ident!("variant_{}", name.to_string().trim_start_matches("r#"));
            quote! { #name: #value }
        })
        .collect::<Vec<_>>();
    let value = if variant_fields.is_empty() {
        common_value
    } else {
        quote! { #result { common: #common_value, #(#variant_values,)* } }
    };
    let allow_bools = (variant_types
        .iter()
        .filter(|ty| ty.to_string() == "bool")
        .count()
        > 3)
    .then(|| quote! { #[allow(clippy::struct_excessive_bools)] });
    let result_definition = (!variant_fields.is_empty()).then(|| {
        quote! {
            #allow_bools
            pub(crate) struct #result #generic_args {
                pub(crate) common: CommonFields,
                #(pub(crate) #variant_fields: #variant_types,)*
            }
        }
    });
    let marker_field =
        (!parameters.is_empty()).then(|| quote! { marker: PhantomData<#result_type>, });
    let marker_value = (!parameters.is_empty()).then(|| quote! { marker: PhantomData, });
    let count = Literal::usize_unsuffixed(layout.fields.len());
    let variant_count = Literal::usize_unsuffixed(variant_fields.len());
    let names_definition = if variant_fields.is_empty() {
        quote! { const #field_names: [&str; #count] = [#(#wire_names),*]; }
    } else {
        quote! {
            const fn #field_names(variant_fields: &[&'static str; #variant_count]) -> [&'static str; #count] {
                [#(#wire_names),*]
            }
        }
    };
    let visitor_bounds = (!bounds.is_empty()).then(|| quote! { where #(#bounds,)* });
    quote! {
        #names_definition
        #result_definition
        struct #visitor #generic_args {
            metadata: &'static Metadata,
            #marker_field
        }
        impl<'de, #(#parameters),*> Visitor<'de> for #visitor #generic_args
        #visitor_bounds
        {
            type Value = #result_type;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct ")?;
                formatter.write_str(self.metadata.name)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let expected = SequenceExpected(self.metadata);
                #(#sequence)*
                Ok(#value)
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut shared = CommonSlots::default();
                #(#declarations)*
                while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
                    match field {
                        index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                        #(#map_arms)*
                        _ => { map.next_value::<IgnoredAny>()?; }
                    }
                }
                #(#finish)*
                Ok(#value)
            }
        }
        pub(crate) fn #deserialize<'de, D, #(#parameters),*>(
            deserializer: D,
            metadata: &'static Metadata,
        ) -> Result<#result_type, D::Error>
        where
            D: Deserializer<'de>,
            #(#bounds,)*
        {
            deserializer.deserialize_struct(metadata.name, metadata.fields, #visitor {
                metadata,
                #marker_value
            })
        }
    }
}

fn tokenize_metadata(
    ty: &NormalizedType,
    layout: &Layout<'_>,
    names: &HashMap<String, String>,
) -> TokenStream {
    let name = metadata_name(ty, names);
    let type_name = &ty.name;
    let field_names = &layout.field_names;
    let variants = fields(ty)
        .into_iter()
        .zip(&layout.fields)
        .filter(|(_, field)| matches!(field, Field::Variant { .. }))
        .map(|(field, _)| &field.name)
        .collect::<Vec<_>>();
    let fields = if variants.is_empty() {
        quote! { &#field_names }
    } else {
        quote! { &#field_names(&[#(#variants),*]) }
    };
    quote! {
        pub(crate) static #name: Metadata = Metadata {
            name: #type_name,
            fields: #fields,
            variant_fields: &[#(#variants),*],
        };
    }
}

fn tokenize_implementation(
    ty: &NormalizedType,
    module: &Ident,
    layout: &Layout<'_>,
    names: &HashMap<String, String>,
) -> DeserializeImplementation {
    let name = format_ident!("{}", ty.name);
    let metadata = metadata_name(ty, names);
    let deserialize = &layout.deserialize;
    let mut variant_index = 0;
    let construction = fields(ty)
        .into_iter()
        .zip(&layout.fields)
        .map(|(field, kind)| {
            let name = sanitize_field_name(&field.name);
            match kind {
                Field::Common(_) if layout.variant_fields.is_empty() => {
                    quote! { #name: decoded.#name }
                }
                Field::Common(_) => quote! { #name: decoded.common.#name },
                Field::Variant {
                    ..
                } => {
                    let variant = &layout.variant_fields[variant_index];
                    variant_index += 1;
                    quote! { #name: decoded.#variant }
                }
            }
        })
        .collect::<Vec<_>>();
    DeserializeImplementation {
        import: quote! {
            #[cfg(not(feature = "deser"))]
            use crate::deserialization::#module as decoder;
        },
        body: quote! {
            #[cfg(not(feature = "deser"))]
            impl<'de> serde::Deserialize<'de> for #name {
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let decoded = decoder::#deserialize(deserializer, &decoder::#metadata)?;
                    Ok(Self { #(#construction,)* })
                }
            }
        },
    }
}

fn tokenize_group(
    group: &Group<'_>,
    layouts: &[Layout<'_>],
    names: &HashMap<String, String>,
) -> TokenStream {
    let field_names = group
        .common
        .iter()
        .map(|field| sanitize_field_name(&field.name))
        .collect::<Vec<_>>();
    let types = group
        .common
        .iter()
        .map(|field| tokenize_field_type(field))
        .collect::<Vec<_>>();
    let wire_names = group
        .common
        .iter()
        .map(|field| field.name.as_str())
        .collect::<Vec<_>>();
    let bytes = wire_names
        .iter()
        .map(|name| Literal::byte_string(name.as_bytes()))
        .collect::<Vec<_>>();
    let indices = (0..field_names.len())
        .map(Literal::usize_unsuffixed)
        .collect::<Vec<_>>();
    let count = Literal::usize_unsuffixed(field_names.len());
    let bools = group
        .common
        .iter()
        .filter(|field| field.required && matches!(field.r#type, TypeKindInField::Boolean(_)))
        .count();
    let allow_bools = (bools > 3).then(|| quote! { #[allow(clippy::struct_excessive_bools)] });
    let decoded_layouts = layouts
        .iter()
        .map(|layout| tokenize_layout(&group.common, layout))
        .collect::<Vec<_>>();
    let metadata = layouts
        .iter()
        .flat_map(|layout| {
            layout
                .members
                .iter()
                .map(|ty| tokenize_metadata(ty, layout, names))
        })
        .collect::<Vec<_>>();
    let imports = if layouts.iter().any(|layout| !layout.parameters.is_empty()) {
        quote! {
            use serde::{
                de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
                Deserialize, Deserializer,
            };
            use std::{fmt, marker::PhantomData};
        }
    } else {
        quote! {
            use serde::{
                de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
                Deserializer,
            };
            use std::fmt;
        }
    };
    quote! {
        #![allow(clippy::too_many_lines)]
        use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
        #imports

        #allow_bools
        pub(crate) struct CommonFields {
            #(pub(crate) #field_names: #types,)*
        }

        #[derive(Default)]
        #[allow(clippy::option_option)]
        struct CommonSlots {
            #(#field_names: Option<#types>,)*
        }

        impl FieldLookup for CommonSlots {
            const COUNT: usize = #count;

            #[inline]
            fn field(value: &str) -> Option<usize> {
                match value {
                    #(#wire_names => Some(#indices),)*
                    _ => None,
                }
            }

            #[inline]
            fn field_bytes(value: &[u8]) -> Option<usize> {
                match value {
                    #(#bytes => Some(#indices),)*
                    _ => None,
                }
            }
        }

        impl CommonSlots {
            fn read_field<'de, A: MapAccess<'de>>(&mut self, index: usize, map: &mut A) -> Result<(), A::Error> {
                match index {
                    #(#indices => read_map_field(&mut self.#field_names, map, #wire_names),)*
                    _ => unreachable!(),
                }
            }
        }
        #(#decoded_layouts)*
        #(#metadata)*
    }
}

#[must_use]
pub fn tokenize_deserializers(schema: &NormalizedSchema) -> Deserializers {
    let mut implementations = HashMap::new();
    let mut modules = BTreeMap::new();
    for group in groups(schema) {
        let names = short_names(&group);
        let layouts = layouts(&group, &names);
        let module_name = camel_to_filename(group.name, None);
        let module = format_ident!("{module_name}");
        for layout in &layouts {
            for ty in &layout.members {
                implementations.insert(
                    ty.name.clone(),
                    tokenize_implementation(ty, &module, layout, &names),
                );
            }
        }
        modules.insert(module_name, tokenize_group(&group, &layouts, &names));
    }
    let declarations = modules
        .keys()
        .map(|name| {
            let name = format_ident!("{name}");
            quote! { pub(crate) mod #name; }
        })
        .collect::<Vec<_>>();
    let root = quote! {
        use serde::{
            de::{DeserializeSeed, Error, Expected, MapAccess, SeqAccess, Visitor},
            Deserialize, Deserializer,
        };
        use std::{fmt, marker::PhantomData};
        #(#declarations)*

        #[inline]
        fn read_map_field<'de, A: MapAccess<'de>, T: Deserialize<'de>>(
            slot: &mut Option<T>,
            map: &mut A,
            name: &'static str,
        ) -> Result<(), A::Error> {
            if slot.is_some() {
                return Err(A::Error::duplicate_field(name));
            }
            *slot = Some(map.next_value()?);
            Ok(())
        }

        #[inline]
        fn read_seq_field<'de, A: SeqAccess<'de>, T: Deserialize<'de>>(
            seq: &mut A,
            index: usize,
            expected: &dyn Expected,
        ) -> Result<T, A::Error> {
            seq.next_element()?.ok_or_else(|| A::Error::invalid_length(index, expected))
        }

        pub(crate) struct Metadata {
            name: &'static str,
            fields: &'static [&'static str],
            variant_fields: &'static [&'static str],
        }

        struct SequenceExpected(&'static Metadata);

        impl Expected for SequenceExpected {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                let count = self.0.fields.len();
                let suffix = if count == 1 { "element" } else { "elements" };
                write!(formatter, "struct {} with {count} {suffix}", self.0.name)
            }
        }

        trait FieldLookup {
            const COUNT: usize;
            fn field(value: &str) -> Option<usize>;
            fn field_bytes(value: &[u8]) -> Option<usize>;
        }

        const UNKNOWN_FIELD: usize = usize::MAX;

        struct FieldKey<Fields> {
            metadata: &'static Metadata,
            marker: PhantomData<Fields>,
        }

        impl<Fields> FieldKey<Fields> {
            fn new(metadata: &'static Metadata) -> Self {
                Self { metadata, marker: PhantomData }
            }
        }

        impl<'de, Fields: FieldLookup> DeserializeSeed<'de> for FieldKey<Fields> {
            type Value = usize;
            fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<usize, D::Error> {
                deserializer.deserialize_identifier(self)
            }
        }

        impl<Fields: FieldLookup> Visitor<'_> for FieldKey<Fields> {
            type Value = usize;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("field identifier")
            }
            fn visit_u64<E: Error>(self, value: u64) -> Result<usize, E> {
                if let Some(name) = usize::try_from(value).ok().and_then(|index| self.metadata.fields.get(index)) {
                    self.visit_str(name)
                } else {
                    Ok(UNKNOWN_FIELD)
                }
            }
            fn visit_str<E: Error>(self, value: &str) -> Result<usize, E> {
                Ok(if let Some(index) = Fields::field(value) {
                    index
                } else if let Some(index) = self.metadata.variant_fields.iter().position(|name| *name == value) {
                    Fields::COUNT + index
                } else {
                    UNKNOWN_FIELD
                })
            }
            fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<usize, E> {
                Ok(if let Some(index) = Fields::field_bytes(value) {
                    index
                } else if let Some(index) = self.metadata.variant_fields.iter().position(|name| name.as_bytes() == value) {
                    Fields::COUNT + index
                } else {
                    UNKNOWN_FIELD
                })
            }
        }
    };
    Deserializers {
        root,
        modules,
        implementations,
    }
}
