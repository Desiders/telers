use crate::parser::api::{
    NormalizedField, NormalizedSchema, NormalizedSubtypeVariant, NormalizedType,
};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub enum HelperFieldSource<'a> {
    Direct(&'a NormalizedField),
    EnumHelper {
        field: &'a NormalizedField,
        fully_required: bool,
    },
}

impl<'a> HelperFieldSource<'a> {
    #[must_use]
    pub fn field(self) -> &'a NormalizedField {
        match self {
            HelperFieldSource::Direct(field)
            | HelperFieldSource::EnumHelper {
                field, ..
            } => field,
        }
    }

    #[must_use]
    pub fn required(self) -> bool {
        match self {
            HelperFieldSource::Direct(field) => field.required,
            HelperFieldSource::EnumHelper {
                fully_required, ..
            } => fully_required,
        }
    }
}

#[must_use]
pub fn build_fields_subtypes_map<'a>(
    type_quote: &'a NormalizedType,
    schema: &'a NormalizedSchema,
) -> BTreeMap<&'a str, Vec<(&'a NormalizedSubtypeVariant, HelperFieldSource<'a>)>> {
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
    fields_subtypes_map
}

#[must_use]
pub fn collect_common_fields<'a>(
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
