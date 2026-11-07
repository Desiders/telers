use crate::{
    file::camel_to_filename,
    generator::{helpers::sanitize_field_name, types::tokenize_field_type},
    parser::api::{NormalizedField, NormalizedSchema, NormalizedType, SubtypeKind},
};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap, HashSet},
};

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
            group.common.len() == common.len()
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
    Specific {
        default: bool,
        sequence_default: bool,
    },
}

fn layout(ty: &NormalizedType, common: &[&NormalizedField]) -> Vec<Field> {
    fields(ty)
        .iter()
        .map(|field| {
            common
                .iter()
                .position(|other| same_field(field, other))
                .map_or(
                    Field::Specific {
                        default: !field.required || field.force_default,
                        sequence_default: field.required && field.force_default,
                    },
                    Field::Common,
                )
        })
        .collect()
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

#[allow(clippy::too_many_lines)]
fn tokenize_layout(common: &[&NormalizedField], layout: &[Field], index: usize) -> TokenStream {
    let visitor = format_ident!("Visitor{index}");
    let deserialize = format_ident!("read_{index}");
    let mut parameters = Vec::new();
    let mut bounds = Vec::new();
    let mut specific = Vec::new();
    let mut declarations = Vec::new();
    let mut map_arms = Vec::new();
    let mut sequence = Vec::new();
    let mut finish = Vec::new();
    let common_count = common.len();
    for (field_index, field) in layout.iter().enumerate() {
        match *field {
            Field::Common(index) => {
                let field = common[index];
                let name = sanitize_field_name(&field.name);
                let ty = tokenize_field_type(field);
                let value = if field.required && field.force_default {
                    quote! { seq.next_element::<#ty>()?.unwrap_or_default() }
                } else {
                    quote! { seq_field::<A, #ty>(&mut seq, #field_index, &self.meta.sequence)? }
                };
                sequence.push(quote! {
                    let #name = #value;
                });
                let value = finish_field(field, &quote! { common.#name });
                finish.push(quote! { let #name = #value; });
            }
            Field::Specific {
                default,
                sequence_default,
            } => {
                let index = specific.len();
                let key_index = common_count + index;
                let parameter = format_ident!("P{index}");
                let value = format_ident!("value_{index}");
                parameters.push(quote! { #parameter });
                bounds.push(if default {
                    quote! { #parameter: Deserialize<'de> + Default }
                } else {
                    quote! { #parameter: Deserialize<'de> }
                });
                specific.push(quote! { #value });
                declarations.push(quote! { let mut #value = None; });
                map_arms.push(quote! {
                    #key_index => { map_field(&mut #value, &mut map, self.meta.specific[#index])?; }
                });
                let sequence_value = if sequence_default {
                    quote! { seq.next_element::<#parameter>()?.unwrap_or_default() }
                } else {
                    quote! { seq_field::<A, #parameter>(&mut seq, #field_index, &self.meta.sequence)? }
                };
                sequence.push(quote! {
                    let #value = #sequence_value;
                });
                finish.push(if default {
                    quote! { let #value = #value.unwrap_or_default(); }
                } else {
                    quote! {
                        let #value = #value.ok_or_else(|| A::Error::missing_field(self.meta.specific[#index]))?;
                    }
                });
            }
        }
    }
    let common_names = common
        .iter()
        .map(|field| sanitize_field_name(&field.name))
        .collect::<Vec<_>>();
    let generic_args = (!parameters.is_empty()).then(|| quote! { <#(#parameters),*> });
    quote! {
        struct #visitor #generic_args {
            meta: Meta,
            marker: PhantomData<(#(#parameters,)*)>,
        }
        impl<'de, #(#bounds),*> Visitor<'de> for #visitor #generic_args {
            type Value = (Common, (#(#parameters,)*));

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct ")?;
                formatter.write_str(self.meta.name)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                #(#sequence)*
                Ok((Common { #(#common_names,)* }, (#(#specific,)*)))
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut common = Slots::default();
                #(#declarations)*
                while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                    match field {
                        index if index < #common_count => common.read(index, &mut map)?,
                        #(#map_arms)*
                        _ => { map.next_value::<IgnoredAny>()?; }
                    }
                }
                #(#finish)*
                Ok((Common { #(#common_names,)* }, (#(#specific,)*)))
            }
        }

        pub(crate) fn #deserialize<'de, D: Deserializer<'de>, #(#bounds),*>(
            deserializer: D, meta: Meta,
        ) -> Result<(Common, (#(#parameters,)*)), D::Error> {
            deserializer.deserialize_struct(meta.name, meta.fields, #visitor {
                meta, marker: PhantomData,
            })
        }

    }
}

fn tokenize_implementation(
    ty: &NormalizedType,
    module: &Ident,
    layout: &[Field],
    index: usize,
) -> TokenStream {
    let name = format_ident!("{}", ty.name);
    let deserialize = format_ident!("read_{index}");
    let fields = fields(ty);
    let field_names = fields.iter().map(|field| &field.name).collect::<Vec<_>>();
    let mut specific_types = Vec::new();
    let mut specific_names = Vec::new();
    let mut specific_values = Vec::new();
    let construction = fields
        .iter()
        .zip(layout)
        .map(|(field, kind)| {
            let name = sanitize_field_name(&field.name);
            match kind {
                Field::Common(_) => quote! { #name: common.#name },
                Field::Specific {
                    ..
                } => {
                    let value = format_ident!("value_{}", field.name);
                    specific_types.push(tokenize_field_type(field));
                    specific_names.push(&field.name);
                    specific_values.push(value.clone());
                    quote! { #name: #value }
                }
            }
        })
        .collect::<Vec<_>>();
    let count = fields.len();
    let suffix = if count == 1 { "element" } else { "elements" };
    let expected = format!("struct {} with {count} {suffix}", ty.name);
    let type_name = &ty.name;
    quote! {
        #[cfg(not(feature = "deser"))]
        impl<'de> serde::Deserialize<'de> for #name {
            #[allow(clippy::too_many_lines)]
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let meta = crate::deserialization::Meta {
                    name: #type_name,
                    fields: &[#(#field_names),*],
                    specific: &[#(#specific_names),*],
                    sequence: #expected,
                };
                let (common, (#(#specific_values,)*)) = crate::deserialization::#module::#deserialize::<D, #(#specific_types),*>(
                    deserializer, meta,
                )?;
                Ok(Self { #(#construction,)* })
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn tokenize_deserializers(
    schema: &NormalizedSchema,
) -> (TokenStream, HashMap<String, TokenStream>) {
    let mut implementations = HashMap::new();
    let modules = groups(schema)
        .into_iter()
        .map(|group| {
            let module = format_ident!("{}", camel_to_filename(group.name, None));
            let mut layouts: Vec<Vec<Field>> = Vec::new();
            for ty in group.members.values() {
                let layout = layout(ty, &group.common);
                let index = layouts.iter().position(|other| *other == layout).unwrap_or_else(|| {
                    layouts.push(layout.clone());
                    layouts.len() - 1
                });
                implementations.insert(ty.name.clone(), tokenize_implementation(ty, &module, &layout, index));
            }
            let names = group.common.iter().map(|field| sanitize_field_name(&field.name)).collect::<Vec<_>>();
            let types = group.common.iter().map(|field| tokenize_field_type(field)).collect::<Vec<_>>();
            let wire_names = group.common.iter().map(|field| field.name.as_str()).collect::<Vec<_>>();
            let bytes = wire_names.iter().map(|name| Literal::byte_string(name.as_bytes())).collect::<Vec<_>>();
            let indices = (0..names.len()).collect::<Vec<_>>();
            let count = names.len();
            let layouts = layouts.iter().enumerate().map(|(index, layout)| {
                tokenize_layout(&group.common, layout, index)
            }).collect::<Vec<_>>();
            quote! {
                pub(crate) mod #module {
                    use super::{
                        fmt, map_field, seq_field, CommonFields, Deserialize,
                        Deserializer, Error, IgnoredAny, Key, MapAccess,
                        Meta, PhantomData, SeqAccess, Visitor,
                    };

                    pub(crate) struct Common { #(pub(crate) #names: #types,)* }
                    #[derive(Default)]
                    struct Slots { #(#names: Option<#types>,)* }

                    impl CommonFields for Slots {
                        const COUNT: usize = #count;

                        #[inline]
                        fn field(value: &str) -> Option<usize> {
                            match value { #(#wire_names => Some(#indices),)* _ => None }
                        }
                        #[inline]
                        fn field_bytes(value: &[u8]) -> Option<usize> {
                            match value { #(#bytes => Some(#indices),)* _ => None }
                        }
                    }
                    impl Slots {
                        fn read<'de, A: MapAccess<'de>>(&mut self, index: usize, map: &mut A) -> Result<(), A::Error> {
                            match index {
                                #(#indices => map_field(&mut self.#names, map, #wire_names),)*
                                _ => unreachable!(),
                            }
                        }
                    }
                    #(#layouts)*
                }
            }
        })
        .collect::<Vec<_>>();
    let tokens = quote! {
        #![allow(
            clippy::option_option,
            clippy::similar_names,
            clippy::struct_excessive_bools,
            clippy::too_many_lines,
            clippy::type_complexity
        )]

        use serde::{
            de::{DeserializeSeed, Error, Expected, IgnoredAny, MapAccess, SeqAccess, Visitor},
            Deserialize, Deserializer,
        };
        use std::{fmt, marker::PhantomData};

        #[inline]
        fn map_field<'de, A: MapAccess<'de>, T: Deserialize<'de>>(
            slot: &mut Option<T>, map: &mut A, name: &'static str,
        ) -> Result<(), A::Error> {
            if slot.is_some() {
                return Err(A::Error::duplicate_field(name));
            }
            *slot = Some(map.next_value()?);
            Ok(())
        }

        #[inline]
        fn seq_field<'de, A: SeqAccess<'de>, T: Deserialize<'de>>(
            seq: &mut A, index: usize, expected: &dyn Expected,
        ) -> Result<T, A::Error> {
            seq.next_element()?.ok_or_else(|| A::Error::invalid_length(index, expected))
        }

        #[derive(Clone, Copy)]
        pub(crate) struct Meta {
            pub(crate) name: &'static str,
            pub(crate) fields: &'static [&'static str],
            pub(crate) specific: &'static [&'static str],
            pub(crate) sequence: &'static str,
        }

        trait CommonFields {
            const COUNT: usize;
            fn field(value: &str) -> Option<usize>;
            fn field_bytes(value: &[u8]) -> Option<usize>;
        }

        struct Key<C> { meta: Meta, marker: PhantomData<C> }
        impl<C> Key<C> {
            fn new(meta: Meta) -> Self { Self { meta, marker: PhantomData } }
        }
        impl<'de, C: CommonFields> DeserializeSeed<'de> for Key<C> {
            type Value = usize;
            fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<usize, D::Error> {
                deserializer.deserialize_identifier(self)
            }
        }
        impl<C: CommonFields> Visitor<'_> for Key<C> {
            type Value = usize;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("field identifier")
            }
            fn visit_u64<E: Error>(self, value: u64) -> Result<usize, E> {
                if let Some(name) = usize::try_from(value).ok().and_then(|index| self.meta.fields.get(index)) {
                    self.visit_str(name)
                } else { Ok(usize::MAX) }
            }
            fn visit_str<E: Error>(self, value: &str) -> Result<usize, E> {
                Ok(if let Some(index) = C::field(value) {
                    index
                } else if let Some(index) = self.meta.specific.iter().position(|name| *name == value) {
                    C::COUNT + index
                } else { usize::MAX })
            }
            fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<usize, E> {
                Ok(if let Some(index) = C::field_bytes(value) {
                    index
                } else if let Some(index) = self.meta.specific.iter().position(|name| name.as_bytes() == value) {
                    C::COUNT + index
                } else { usize::MAX })
            }
        }
        #(#modules)*
    };
    (tokens, implementations)
}
