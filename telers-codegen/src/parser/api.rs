#![allow(
    clippy::missing_panics_doc,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]

use quote::format_ident;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    mem,
};
use syn::{Path, PathSegment, punctuated::Punctuated};
use tracing::warn;

use crate::generator::helpers::{capitalize, snake_to_upper_camel};

pub type TelegramTypeName = String;
pub type TelegramMethodName = String;
pub type FieldName = String;
pub type RawType = String;

const BOXED_TYPES: &[&str] = &[
    "Message",
    "MaybeInaccessibleMessage",
    "Chat",
    "User",
    "ExternalReplyInfo",
    "Gift",
    "UniqueGift",
    "BackgroundType",
    "Audio",
    "Animation",
    "Poll",
    "Sticker",
    "SuccessfulPayment",
    "VideoNote",
    "Venue",
    "Video",
    "Document",
    "RichText",
];

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: FieldName,
    pub required: bool,
    #[serde(default)]
    pub description: String,
    pub types: Vec<RawType>,
}

impl Field {
    /// # Panics
    /// * If the field has multiple types, but it's not a known special case.
    #[must_use]
    pub fn identify_field_type(&self) -> TypeKindInField {
        let types = self.types.as_slice();

        if multi_type_is_input_file(types, &self.description) {
            return TypeKindInField::InputFile;
        }
        if multi_type_is_chat_id(types) {
            return TypeKindInField::ChatId;
        }
        if multi_type_is_reply_markup(types, &self.name) {
            return TypeKindInField::Telegram("ReplyMarkup".to_owned());
        }
        if multi_type_is_input_media(types, &self.name) {
            return TypeKindInField::Array(Box::new(TypeKindInField::Telegram(
                "InputMedia".to_owned(),
            )));
        }
        if multi_type_is_input_rich_message_media(types) {
            return TypeKindInField::Telegram("InputRichMessageMediaContent".to_owned());
        }
        if types.len() > 1 {
            unimplemented!(
                "Unknown case for multi types: field='{name}', types={types:?}, \
                 description='{description}'",
                name = self.name,
                description = self.description,
            );
        }

        let r#type = types.first().unwrap();

        if is_array_of(r#type) {
            let inner_type = Field {
                name: self.name.clone(),
                required: self.required,
                // A numeric range in the array's description (e.g. "list of 1-100 identifiers"
                // or "list of 1-3 colors") bounds the array length, not each element's value.
                // Passing it down would wrongly size integer elements (e.g. `message_ids` as `u8`),
                // so strip only the range; other hints like "RGB format" are kept so element
                // typing (e.g. `i32` for colors) still works.
                description: strip_length_range(&self.description),
                types: vec![r#type.replacen("Array of ", "", 1)],
            }
            .identify_field_type();
            return TypeKindInField::Array(Box::new(inner_type));
        }
        if is_string(r#type) {
            return TypeKindInField::String;
        }
        if let Some(kind) = get_if_integer(r#type, &self.description) {
            return TypeKindInField::Integer(kind);
        }
        if let Some(kind) = get_if_boolean(r#type) {
            return TypeKindInField::Boolean(kind);
        }

        TypeKindInField::Telegram(r#type.to_owned())
    }

    #[must_use]
    pub fn is_tagged(&self) -> bool {
        self.required
            && self.types == ["String"]
            && (self.description.contains("always") || self.description.contains("must be"))
    }

    #[must_use]
    pub fn get_tagged_value(&self) -> Option<String> {
        if !is_string(self.types.first()?) {
            return None;
        }
        let desc = self.description.to_lowercase();
        let patterns = [
            r#"always "([^"]+)""#,
            r"always ([a-z_]+)",
            r#"must be "([^"]+)""#,
            r"must be ([a-z_]+)",
        ];

        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(&desc) {
                    if let Some(m) = caps.get(1) {
                        return Some(m.as_str().to_owned());
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub name: TelegramTypeName,
    pub href: String,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub fields: Vec<Field>,
    #[serde(default)]
    pub subtypes: Vec<TelegramTypeName>,
    #[serde(default)]
    pub subtype_of: Vec<TelegramTypeName>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Method {
    pub name: TelegramMethodName,
    pub href: String,
    #[serde(default)]
    pub description: Vec<String>,
    pub returns: Vec<RawType>,
    #[serde(default)]
    pub fields: Vec<Field>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Schema {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    #[serde(default)]
    pub methods: HashMap<TelegramMethodName, Method>,
    pub types: HashMap<TelegramTypeName, Type>,
}

impl Schema {
    pub fn parse_from_json(content: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content)
    }

    fn detect_subtypes(
        &self,
        subtypes: &[TelegramTypeName],
    ) -> Option<(SubtypeKind, SubtypeTaggedValues)> {
        if subtypes.is_empty() {
            return None;
        }

        let mut tagged_values = SubtypeTaggedValues::default();
        let first_ty = self.types.get(subtypes.first().unwrap()).unwrap();
        let Some(first_ty_tagged_field) = first_ty.fields.first() else {
            return Some((SubtypeKind::Untagged, tagged_values));
        };
        if !first_ty_tagged_field.is_tagged() {
            return Some((SubtypeKind::Untagged, tagged_values));
        }
        tagged_values.insert(
            first_ty.name.clone(),
            first_ty_tagged_field.get_tagged_value().unwrap(),
        );

        for subtype in subtypes.iter().skip(1) {
            let ty = self.types.get(subtype).unwrap();
            let Some(ty_tagged_field) = ty.fields.first() else {
                return Some((SubtypeKind::Untagged, tagged_values));
            };
            if !ty_tagged_field.is_tagged() {
                return Some((SubtypeKind::Untagged, tagged_values));
            }
            tagged_values.insert(ty.name.clone(), ty_tagged_field.get_tagged_value().unwrap());
        }

        Some((
            SubtypeKind::Tagged {
                tag_field: first_ty_tagged_field.name.clone(),
                parent_tag_field: None,
            },
            tagged_values,
        ))
    }

    #[must_use]
    pub fn normalize(self) -> NormalizedSchema {
        let mut subtype_info = HashMap::new();
        for (name, ty) in &self.types {
            // Only object subtypes carry a discriminator; drop non-type entries (plain `String`,
            // `Array of self`) so the lookup in `detect_subtypes` doesn't miss on them.
            let named_subtypes: Vec<TelegramTypeName> = ty
                .subtypes
                .iter()
                .filter(|subtype| classify_extra_subtype(subtype.as_str(), name).is_none())
                .cloned()
                .collect();
            if let Some((kind, tagged_values)) = self.detect_subtypes(&named_subtypes) {
                subtype_info.insert(name.clone(), (kind.clone(), None));
                for subtype in &named_subtypes {
                    subtype_info.insert(
                        subtype.clone(),
                        (kind.clone(), tagged_values.get(subtype).cloned()),
                    );
                }
            }
        }

        let normalized_types = self
            .types
            .into_iter()
            .map(|(name, ty)| {
                let fields = ty
                    .fields
                    .into_iter()
                    .map(|field| {
                        let field_type = field.identify_field_type();
                        let is_recursive =
                            matches!(&field_type, TypeKindInField::Telegram(ty) if *ty == name);
                        let is_boxed = matches!(&field_type,
                            TypeKindInField::Telegram(ty) if BOXED_TYPES.contains(&ty.as_str()));
                        NormalizedField {
                            name: field.name,
                            required: field.required,
                            description: field.description,
                            r#type: field_type,
                            is_recursive,
                            is_boxed,
                            is_update_variant_field: false,
                        }
                    })
                    .collect();
                let mut extra_subtypes = vec![];
                let subtypes = ty
                    .subtypes
                    .into_iter()
                    .filter_map(|subtype_name| {
                        // Peel off non-type variants (plain `String`, `Array of self`) into
                        // `extra_subtypes`; keep only object subtypes as named variants.
                        if let Some(extra) = classify_extra_subtype(&subtype_name, &name) {
                            if !extra_subtypes.contains(&extra) {
                                extra_subtypes.push(extra);
                            }
                            return None;
                        }
                        let subtype_value = subtype_info
                            .get(&subtype_name)
                            .map(|(_, tagged_value)| tagged_value);
                        let variant = match subtype_value {
                            Some(Some(value)) => snake_to_upper_camel(value),
                            _ => subtype_name.clone(),
                        };
                        Some(NormalizedSubtypeVariant {
                            variant,
                            ty_name: subtype_name,
                        })
                    })
                    .collect();
                let subtype_kind = subtype_info.get(&name).map(|(kind, _)| kind).cloned();
                let normalized = NormalizedType {
                    name: ty.name,
                    href: ty.href,
                    description: ty.description,
                    fields,
                    subtype_kind,
                    subtypes,
                    extra_subtypes,
                    subtype_of: ty.subtype_of,
                    has_extra_fields: false,
                };
                (name, normalized)
            })
            .collect();

        let normalized_methods = self
            .methods
            .into_iter()
            .map(|(name, method)| {
                let fields = method
                    .fields
                    .into_iter()
                    .map(|field| {
                        let field_type = field.identify_field_type();
                        NormalizedField {
                            name: field.name,
                            required: field.required,
                            description: field.description,
                            r#type: field_type,
                            is_recursive: false,
                            is_boxed: false,
                            is_update_variant_field: false,
                        }
                    })
                    .collect::<Vec<_>>();
                (
                    name,
                    NormalizedMethod {
                        name: capitalize(&method.name),
                        href: method.href,
                        description: method.description,
                        returns: method
                            .returns
                            .into_iter()
                            .map(|r#type| {
                                Field {
                                    name: "returns".to_owned(),
                                    required: true,
                                    description: String::new(),
                                    types: vec![r#type],
                                }
                                .identify_field_type()
                            })
                            .collect(),
                        fields,
                    },
                )
            })
            .collect();

        NormalizedSchema {
            version: self.version,
            release_date: self.release_date,
            changelog: self.changelog,
            methods: normalized_methods,
            types: normalized_types,
        }
    }

    #[must_use]
    pub fn is_telegram_type(&self, raw_type: &RawType) -> bool {
        self.types.contains_key(raw_type)
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct NormalizedField {
    pub name: FieldName,
    pub required: bool,
    pub description: String,
    pub r#type: TypeKindInField,
    pub is_recursive: bool,
    pub is_boxed: bool,
    pub is_update_variant_field: bool,
}

impl NormalizedField {
    #[must_use]
    pub fn is_tagged(&self, tag_field: Option<&str>, parent_tag_field: Option<&str>) -> bool {
        let Some(tag_name) = tag_field else {
            return false;
        };
        if self.r#type != TypeKindInField::String {
            return false;
        }
        if self.name == tag_name {
            return true;
        }
        if let Some(parent_tag_field) = parent_tag_field {
            if self.name == parent_tag_field {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub enum SubtypeKind {
    Tagged {
        tag_field: String,
        parent_tag_field: Option<String>,
    },
    Untagged,
    UntaggedInTagged {
        tag_field: String,
    },
}

impl SubtypeKind {
    #[must_use]
    pub fn get_tags(&self) -> (Option<&str>, Option<&str>) {
        match self {
            SubtypeKind::Untagged => (None, None),
            SubtypeKind::UntaggedInTagged {
                tag_field,
            } => (Some(tag_field.as_str()), None),
            SubtypeKind::Tagged {
                tag_field,
                parent_tag_field,
            } => (Some(tag_field.as_str()), parent_tag_field.as_deref()),
        }
    }
}

pub type SubtypeTaggedValue = String;
pub type SubtypeTaggedValues = HashMap<String, SubtypeTaggedValue>;

/// Variants of a sum type that aren't Telegram object types,
/// e.g. `RichText` can also be a plain `String` or an `Array of RichText`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraSubtypeVariant {
    PlainText,
    ArrayOfSelf,
}

/// Classifies a raw subtype entry that isn't a Telegram object type: `String` is plain text,
/// `Array of {parent}` is an array of the type itself. Named object subtypes return `None`.
///
/// The schema lists these alongside the object subtypes (e.g. `RichText`), so they're read
/// straight from `subtypes` instead of parsing the type description.
#[must_use]
fn classify_extra_subtype(subtype: &str, parent_name: &str) -> Option<ExtraSubtypeVariant> {
    if subtype == "String" {
        Some(ExtraSubtypeVariant::PlainText)
    } else if subtype == format!("Array of {parent_name}") {
        Some(ExtraSubtypeVariant::ArrayOfSelf)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct NormalizedSubtypeVariant {
    pub variant: TelegramTypeName,
    pub ty_name: TelegramTypeName,
}

#[derive(Debug)]
pub struct NormalizedType {
    pub name: TelegramTypeName,
    pub href: String,
    pub description: Vec<String>,
    pub fields: Vec<NormalizedField>,
    pub subtype_kind: Option<SubtypeKind>,
    pub subtypes: Vec<NormalizedSubtypeVariant>,
    /// Sum-type variants that aren't Telegram object types (plain `String`, `Array of self`),
    /// kept out of [`Self::subtypes`] so they aren't treated as object variants.
    pub extra_subtypes: Vec<ExtraSubtypeVariant>,
    pub subtype_of: Vec<TelegramTypeName>,
    pub has_extra_fields: bool,
}

impl NormalizedType {
    /// Sum-type variants that aren't Telegram object types (plain `String`, `Array of self`),
    /// classified from `subtypes` during [`Schema::normalize`] (see [`classify_extra_subtype`]).
    #[must_use]
    pub fn extra_variants(&self) -> Vec<ExtraSubtypeVariant> {
        self.extra_subtypes.clone()
    }

    #[must_use]
    pub fn get_paths(&self) -> Vec<Path> {
        self.fields
            .iter()
            .filter_map(|f| f.r#type.get_path())
            .chain(self.subtypes.iter().map(|s| {
                let mut segments = Punctuated::new();
                segments.push(PathSegment::from(format_ident!("{}", s.ty_name)));
                Path {
                    leading_colon: None,
                    segments,
                }
            }))
            .collect()
    }

    #[must_use]
    pub fn get_paths_count(&self) -> usize {
        self.get_paths().len()
    }

    #[must_use]
    pub fn is_update(&self) -> bool {
        matches!(&self.subtype_kind, Some(SubtypeKind::Untagged)) && self.name == "Update"
    }

    #[must_use]
    pub fn is_update_variant(&self) -> bool {
        matches!(&self.subtype_kind, Some(SubtypeKind::Untagged))
            && self.subtype_of.contains(&"Update".to_owned())
    }

    #[must_use]
    pub fn update_variant_ty_field(&self) -> Option<&NormalizedField> {
        self.fields.iter().find(|f| f.is_update_variant_field)
    }
}

#[derive(Debug)]
pub struct NormalizedMethod {
    pub name: TelegramMethodName,
    pub href: String,
    pub description: Vec<String>,
    pub returns: Vec<TypeKindInField>,
    pub fields: Vec<NormalizedField>,
}

#[derive(Debug, Default)]
pub struct NormalizedSchema {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub methods: HashMap<TelegramMethodName, NormalizedMethod>,
    pub types: HashMap<TelegramTypeName, NormalizedType>,
}

impl NormalizedSchema {
    /// Finalizes a parent enum type after splitting: sets description, `subtype_kind`, subtypes.
    fn finalize_split(
        parent: &mut NormalizedType,
        subtypes: &[(String, String)],
        subtype_kind: SubtypeKind,
    ) {
        let mut sorted_subtypes = subtypes.to_vec();
        sorted_subtypes.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        parent
            .description
            .push("Currently, it can be one of".to_owned());
        parent
            .description
            .extend(sorted_subtypes.iter().map(|(_, name)| format!("- {name}")));
        parent.subtype_kind = Some(subtype_kind);
        parent.subtypes = subtypes
            .iter()
            .map(|(variant, name)| NormalizedSubtypeVariant {
                variant: variant.clone(),
                ty_name: name.clone(),
            })
            .collect();
    }

    /// Skips types by name
    pub fn skip_types(&mut self, names: &[&str]) {
        for &name in names {
            self.types.remove(name);
        }
    }

    /// Reorders subtypes for untagged enums to place more specific variants first.
    ///
    /// This avoids premature matching of broad variants during serde untagged deserialization.
    pub fn reorder_untagged_subtypes(&mut self) {
        let required_fields_map: HashMap<_, _> = self
            .types
            .iter()
            .map(|(name, ty)| {
                let required: HashSet<String> = ty
                    .fields
                    .iter()
                    .filter(|f| f.required)
                    .map(|f| f.name.clone())
                    .collect();
                (name.clone(), (required, ty.fields.len()))
            })
            .collect();

        for ty in self.types.values_mut() {
            let is_untagged = matches!(
                ty.subtype_kind,
                Some(SubtypeKind::Untagged | SubtypeKind::UntaggedInTagged { .. })
            );
            if !is_untagged || ty.subtypes.len() < 2 {
                continue;
            }

            ty.subtypes.sort_by(|a, b| {
                let Some((a_required, a_fields_len)) = required_fields_map.get(&a.ty_name) else {
                    return Ordering::Equal;
                };
                let Some((b_required, b_fields_len)) = required_fields_map.get(&b.ty_name) else {
                    return Ordering::Equal;
                };

                let a_is_superset = a_required.is_superset(b_required) && a_required != b_required;
                let b_is_superset = b_required.is_superset(a_required) && a_required != b_required;
                if a_is_superset {
                    return Ordering::Less;
                }
                if b_is_superset {
                    return Ordering::Greater;
                }

                b_required
                    .len()
                    .cmp(&a_required.len())
                    .then_with(|| b_fields_len.cmp(a_fields_len))
                    .then_with(|| a.ty_name.cmp(&b.ty_name))
            });
        }
    }

    pub fn split_inline_query_result(&mut self) {
        let inline_query_result = self
            .types
            .get_mut("InlineQueryResult")
            .expect("InlineQueryResult doesn't exist in schema");

        #[allow(clippy::items_after_statements)]
        enum TypeKind {
            CachedAndNotCached,
            Cached,
            NotCached,
        }

        let inline_query_types = [
            ("Audio", TypeKind::CachedAndNotCached),
            ("Document", TypeKind::CachedAndNotCached),
            ("Gif", TypeKind::CachedAndNotCached),
            ("Mpeg4Gif", TypeKind::CachedAndNotCached),
            ("Photo", TypeKind::CachedAndNotCached),
            ("Sticker", TypeKind::Cached),
            ("Video", TypeKind::CachedAndNotCached),
            ("Voice", TypeKind::CachedAndNotCached),
            ("Article", TypeKind::NotCached),
            ("Contact", TypeKind::NotCached),
            ("Game", TypeKind::NotCached),
            ("Location", TypeKind::NotCached),
            ("Venue", TypeKind::NotCached),
        ];

        let mut types = HashMap::new();

        for mut subtype in mem::take(&mut inline_query_result.subtypes) {
            let Some((inline_query_type, kind)) = inline_query_types
                .iter()
                .find(|(variant, _)| subtype.variant == *variant)
            else {
                warn!("Unknown inline query type: {}", subtype.variant);
                continue;
            };

            if let TypeKind::CachedAndNotCached = kind {
                let cached = format!("InlineQueryResultCached{inline_query_type}");
                let not_cached = format!("InlineQueryResult{inline_query_type}");
                let combined = format!("InlineQueryResult{inline_query_type}Kind");

                let name = combined.clone();
                if types.contains_key(&name) {
                    continue;
                }
                subtype.ty_name.clone_from(&name);

                types.insert(
                    name.clone(),
                    NormalizedType {
                        name: name.clone(),
                        href: inline_query_result.href.clone(),
                        description: vec![
                            "# Notes".to_owned(),
                            format!(
                                "This object represents an inline query result kind as combine of \
                                 {cached} and {not_cached}."
                            ),
                        ],
                        fields: vec![],
                        subtype_kind: Some(SubtypeKind::UntaggedInTagged {
                            tag_field: "type".to_owned(),
                        }),
                        subtypes: vec![
                            NormalizedSubtypeVariant {
                                variant: "Cached".to_owned(),
                                ty_name: cached.clone(),
                            },
                            NormalizedSubtypeVariant {
                                variant: "Uncached".to_owned(),
                                ty_name: not_cached.clone(),
                            },
                        ],
                        extra_subtypes: vec![],
                        subtype_of: vec![inline_query_result.name.clone()],
                        has_extra_fields: false,
                    },
                );
            }

            inline_query_result.subtypes.push(subtype);
        }

        self.types.extend(types);
    }

    /// Creates synthetic `ReplyMarkup` type as untagged union of existing reply markup types.
    pub fn compose_reply_markup_type(&mut self) {
        let subtypes = vec![
            (
                "InlineKeyboardMarkup".to_owned(),
                "InlineKeyboardMarkup".to_owned(),
            ),
            (
                "ReplyKeyboardMarkup".to_owned(),
                "ReplyKeyboardMarkup".to_owned(),
            ),
            (
                "ReplyKeyboardRemove".to_owned(),
                "ReplyKeyboardRemove".to_owned(),
            ),
            ("ForceReply".to_owned(), "ForceReply".to_owned()),
        ];

        let mut reply_markup = NormalizedType {
            name: "ReplyMarkup".to_owned(),
            href: "https://core.telegram.org/bots/api".to_owned(),
            description: vec!["This object represents available reply markup variants.".to_owned()],
            fields: vec![],
            subtype_kind: None,
            subtypes: vec![],
            extra_subtypes: vec![],
            subtype_of: vec![],
            has_extra_fields: false,
        };
        Self::finalize_split(&mut reply_markup, &subtypes, SubtypeKind::Untagged);

        for (_, subtype_name) in &subtypes {
            let subtype = self
                .types
                .get_mut(subtype_name)
                .expect("Subtype should exist in schema");
            subtype.subtype_of.push(reply_markup.name.clone());
        }

        self.types.insert(reply_markup.name.clone(), reply_markup);
    }

    /// Composes the enum for the non-array `media` union of [`InputRichMessageMedia`],
    /// which is a distinct set from [`InputMedia`] (it drops `Document`/`LivePhoto` and adds InputMediaVoiceNote`),
    /// so it can't reuse that type.
    /// Like [`InputMedia`] the variants share a `type` discriminator, so the enum is tagged by it.
    pub fn compose_input_rich_message_media_type(&mut self) {
        let subtypes = vec![
            ("Animation".to_owned(), "InputMediaAnimation".to_owned()),
            ("Audio".to_owned(), "InputMediaAudio".to_owned()),
            ("Photo".to_owned(), "InputMediaPhoto".to_owned()),
            ("Video".to_owned(), "InputMediaVideo".to_owned()),
            ("VoiceNote".to_owned(), "InputMediaVoiceNote".to_owned()),
        ];

        let mut media = NormalizedType {
            name: "InputRichMessageMediaContent".to_owned(),
            href: "https://core.telegram.org/bots/api#inputrichmessagemedia".to_owned(),
            description: vec![
                "This object represents the media content of a rich message to be sent.".to_owned(),
            ],
            fields: vec![],
            subtype_kind: None,
            subtypes: vec![],
            extra_subtypes: vec![],
            subtype_of: vec![],
            has_extra_fields: false,
        };
        Self::finalize_split(
            &mut media,
            &subtypes,
            SubtypeKind::Tagged {
                tag_field: "type".to_owned(),
                parent_tag_field: None,
            },
        );

        for (_, subtype_name) in &subtypes {
            let subtype = self
                .types
                .get_mut(subtype_name)
                .expect("Subtype should exist in schema");
            subtype.subtype_of.push(media.name.clone());
            // Members shared with `InputMedia` already got their `type` tag stripped during
            // normalization; ones unique to this union (e.g. `InputMediaVoiceNote`) haven't,
            // so tag them by `type` too — otherwise the tag survives in the struct and serde
            // reports a missing `type` when deserializing through the tagged enum.
            if subtype.subtype_kind.is_none() {
                subtype.subtype_kind = Some(SubtypeKind::Tagged {
                    tag_field: "type".to_owned(),
                    parent_tag_field: None,
                });
            }
        }

        self.types.insert(media.name.clone(), media);
    }

    pub fn split_message_types(&mut self) {
        let mut message = self
            .types
            .remove("Message")
            .expect("Message doesn't exist in schema");

        let content_fields = [
            "text",
            "animation",
            "audio",
            "document",
            "live_photo",
            "paid_media",
            "photo",
            "sticker",
            "story",
            "video",
            "video_note",
            "voice",
            "checklist",
            "contact",
            "dice",
            "game",
            "poll",
            "venue",
            "location",
            "rich_message",
        ];
        let service_fields = [
            "new_chat_members",
            "left_chat_member",
            "chat_owner_left",
            "chat_owner_changed",
            "new_chat_title",
            "new_chat_photo",
            "delete_chat_photo",
            "group_chat_created",
            "supergroup_chat_created",
            "channel_chat_created",
            "message_auto_delete_timer_changed",
            "migrate_to_chat_id",
            "migrate_from_chat_id",
            "pinned_message",
            "invoice",
            "successful_payment",
            "refunded_payment",
            "users_shared",
            "chat_shared",
            "gift",
            "unique_gift",
            "gift_upgrade_sent",
            "connected_website",
            "write_access_allowed",
            "passport_data",
            "proximity_alert_triggered",
            "boost_added",
            "chat_background_set",
            "checklist_tasks_done",
            "checklist_tasks_added",
            "community_chat_added",
            "community_chat_removed",
            "direct_message_price_changed",
            "forum_topic_created",
            "forum_topic_edited",
            "forum_topic_closed",
            "forum_topic_reopened",
            "general_forum_topic_hidden",
            "general_forum_topic_unhidden",
            "giveaway_created",
            "giveaway",
            "giveaway_winners",
            "giveaway_completed",
            "managed_bot_created",
            "paid_message_price_changed",
            "poll_option_added",
            "poll_option_deleted",
            "suggested_post_approved",
            "suggested_post_approval_failed",
            "suggested_post_declined",
            "suggested_post_paid",
            "suggested_post_refunded",
            "video_chat_scheduled",
            "video_chat_started",
            "video_chat_ended",
            "video_chat_participants_invited",
            "web_app_data",
        ];

        let mut common_fields = vec![];
        let mut content_fields_map = HashMap::new();
        let mut service_fields_map = HashMap::new();

        for field in mem::take(&mut message.fields) {
            let name = field.name.as_str();
            if content_fields.contains(&name) {
                content_fields_map.insert(name.to_owned(), field);
            } else if service_fields.contains(&name) {
                service_fields_map.insert(name.to_owned(), field);
            } else {
                common_fields.push(field);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for (kind_label, fields_map) in [
            ("message", content_fields_map),
            ("service message", service_fields_map),
        ] {
            for (field_name, mut field) in fields_map {
                let variant_name = snake_to_upper_camel(&field_name);
                let type_name = format!("{}{variant_name}", message.name);
                field.required = true;

                let mut fields = common_fields.clone();
                fields.push(field.clone());

                let description = vec![
                    field.description.clone(),
                    "# Notes".to_owned(),
                    format!(
                        "This object represents a {kind_label} from original message field \
                         {field_name}."
                    ),
                ];

                types.insert(
                    type_name.clone(),
                    NormalizedType {
                        name: type_name.clone(),
                        href: message.href.clone(),
                        description,
                        fields,
                        subtype_kind: Some(SubtypeKind::Untagged),
                        subtypes: vec![],
                        extra_subtypes: vec![],
                        subtype_of: vec![message.name.clone()],
                        has_extra_fields: false,
                    },
                );
                subtypes.push((variant_name, type_name));
            }
        }

        Self::finalize_split(&mut message, &subtypes, SubtypeKind::Untagged);
        self.types.insert(message.name.clone(), message);
        self.types.extend(types);
    }

    pub fn split_external_reply_info_types(&mut self) {
        let mut info = self
            .types
            .remove("ExternalReplyInfo")
            .expect("ExternalReplyInfo doesn't exist in schema");

        let mut common_fields = vec![];
        let mut content_fields_map = HashMap::new();

        for field in mem::take(&mut info.fields) {
            let name = field.name.clone();
            let desc = field.description.to_lowercase();
            // "message is ..." for most content fields; "message contains ..." for `paid_media`.
            let is_content_field = desc.contains("message is")
                || desc.contains("message contains")
                || name == "giveaway_winners";
            if is_content_field {
                content_fields_map.insert(name, field);
            } else {
                common_fields.push(field);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for (field_name, mut field) in content_fields_map {
            let variant_name = snake_to_upper_camel(&field_name);
            let type_name = format!("{}{variant_name}", info.name);
            field.required = true;

            let mut fields = common_fields.clone();
            fields.push(field.clone());

            let description = vec![
                field.description.clone(),
                "# Notes".to_owned(),
                format!(
                    "This object represents an external reply info from original field \
                     `{field_name}`."
                ),
            ];

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: info.href.clone(),
                    description,
                    fields,
                    subtype_kind: Some(SubtypeKind::Untagged),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![info.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        // Replies to text messages (and to content types unknown to the schema) have no
        // content field in `ExternalReplyInfo`, so a variant with only the common fields is
        // needed. It has the fewest required fields, so `reorder_untagged_subtypes` places it
        // last and it doesn't shadow the content variants.
        let type_name = format!("{}Unknown", info.name);
        types.insert(
            type_name.clone(),
            NormalizedType {
                name: type_name.clone(),
                href: info.href.clone(),
                description: vec![
                    "Message is a text message or a message with content unknown to the library"
                        .to_owned(),
                    "# Notes".to_owned(),
                    "This object represents an external reply info without a content field; the \
                     quoted part of the original message is available in the message's `quote` \
                     field."
                        .to_owned(),
                ],
                fields: common_fields,
                subtype_kind: Some(SubtypeKind::Untagged),
                subtypes: vec![],
                extra_subtypes: vec![],
                subtype_of: vec![info.name.clone()],
                has_extra_fields: false,
            },
        );
        subtypes.push(("Unknown".to_owned(), type_name));

        Self::finalize_split(&mut info, &subtypes, SubtypeKind::Untagged);
        self.types.insert(info.name.clone(), info);
        self.types.extend(types);
    }

    pub fn split_update_types(&mut self) {
        let mut update = self
            .types
            .remove("Update")
            .expect("Update doesn't exist in schema");

        let mut common_fields = vec![];
        let mut variant_fields_map = HashMap::new();

        for field in mem::take(&mut update.fields) {
            if field.name == "update_id" {
                common_fields.push(field);
                continue;
            }

            variant_fields_map.insert(field.name.clone(), field);
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for (field_name, mut field) in variant_fields_map {
            let variant_name = snake_to_upper_camel(&field_name);
            let type_name = format!("{}{variant_name}", update.name);

            field.required = true;
            field.is_update_variant_field = true;

            let description = vec![
                field.description.clone(),
                "# Notes".to_owned(),
                format!(
                    "This object represents an update from original update field `{field_name}`."
                ),
            ];

            let mut fields = common_fields.clone();
            fields.push(field);

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: update.href.clone(),
                    description,
                    fields,
                    subtype_kind: Some(SubtypeKind::Untagged),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![update.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        let mut sorted_subtypes = subtypes.clone();
        sorted_subtypes.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        update
            .description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &sorted_subtypes {
            update.description.push(format!("- {name}"));
        }
        update.subtype_kind = Some(SubtypeKind::Untagged);
        update.subtypes = subtypes
            .into_iter()
            .map(|(variant, ty_name)| NormalizedSubtypeVariant {
                variant,
                ty_name,
            })
            .collect();

        self.types.insert(
            "UpdateUnparsed".to_owned(),
            NormalizedType {
                name: "UpdateUnparsed".to_owned(),
                href: update.href.clone(),
                description: vec![
                    "This object represents an update that can't be parsed.".to_owned(),
                ],
                fields: common_fields,
                subtype_kind: None,
                subtypes: vec![],
                extra_subtypes: vec![],
                subtype_of: vec![],
                has_extra_fields: true,
            },
        );
        self.types.insert(update.name.clone(), update);
        self.types.extend(types);
    }

    pub fn split_chat_types(&mut self) {
        self.split_chat_type("Chat");
        self.split_chat_type("ChatFullInfo");
    }

    pub fn split_chat_type(&mut self, type_name: &str) {
        let mut chat = self
            .types
            .remove(type_name)
            .expect("Chat type doesn't exist in schema");

        let chat_types = ["private", "group", "supergroup", "channel"];
        let mut common_fields = vec![];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for field in mem::take(&mut chat.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }

            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            if desc.contains("private") {
                applicable.push("private");
            }
            if desc.contains("supergroup") {
                applicable.push("supergroup");
            } else if desc.contains("group") {
                applicable.push("group");
            }
            if desc.contains("channel") && field.name != "is_direct_messages" {
                applicable.push("channel");
            }
            if applicable.is_empty() {
                applicable.extend(chat_types);
            }
            for &chat_type in &applicable {
                let f = field.clone();
                type_fields_map.entry(chat_type).or_default().push(f);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for chat_type in chat_types {
            let variant_name = snake_to_upper_camel(chat_type);
            let type_name = format!("{}{variant_name}", chat.name);

            let mut fields = common_fields.clone();
            if let Some(specific) = type_fields_map.get(chat_type) {
                fields.extend(specific.clone());
            }

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: chat.href.clone(),
                    description: vec![
                        format!("This object represents a {chat_type} chat."),
                        "# Notes".to_owned(),
                        format!(
                            "This object represents a chat from original chat type `{chat_type}`."
                        ),
                    ],
                    fields,
                    subtype_kind: Some(SubtypeKind::Tagged {
                        tag_field: "type".to_owned(),
                        parent_tag_field: None,
                    }),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![chat.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        Self::finalize_split(
            &mut chat,
            &subtypes,
            SubtypeKind::Tagged {
                tag_field: "type".to_owned(),
                parent_tag_field: None,
            },
        );
        self.types.insert(chat.name.clone(), chat);
        self.types.extend(types);
    }

    pub fn split_sticker_types(&mut self) {
        let mut sticker = self
            .types
            .remove("Sticker")
            .expect("Sticker doesn't exist in schema");

        let sticker_types = ["regular", "mask", "custom_emoji"];
        let mut common_fields = vec![];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for field in mem::take(&mut sticker.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            if desc.contains("regular") {
                applicable.push("regular");
            }
            if desc.contains("mask") {
                applicable.push("mask");
            }
            if desc.contains("custom_emoji") {
                applicable.push("custom_emoji");
            }
            if applicable.is_empty() {
                applicable.extend(sticker_types);
            }

            for &t in &applicable {
                let f = field.clone();
                type_fields_map.entry(t).or_default().push(f);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for sticker_type in sticker_types {
            let variant_name = snake_to_upper_camel(sticker_type);
            let type_name = format!("{}{variant_name}", sticker.name);

            let mut fields = common_fields.clone();
            if let Some(specific) = type_fields_map.get(sticker_type) {
                fields.extend(specific.clone());
            }

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: sticker.href.clone(),
                    description: vec![
                        format!("This object represents a {} sticker.", sticker_type),
                        "# Notes".to_owned(),
                        format!(
                            "This object represents a sticker from original sticker type \
                             `{sticker_type}`."
                        ),
                    ],
                    fields,
                    subtype_kind: Some(SubtypeKind::Tagged {
                        tag_field: "type".to_owned(),
                        parent_tag_field: None,
                    }),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![sticker.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        Self::finalize_split(
            &mut sticker,
            &subtypes,
            SubtypeKind::Tagged {
                tag_field: "type".to_owned(),
                parent_tag_field: None,
            },
        );
        self.types.insert(sticker.name.clone(), sticker);
        self.types.extend(types);
    }

    pub fn split_poll_types(&mut self) {
        let mut poll = self
            .types
            .remove("Poll")
            .expect("Poll doesn't exist in schema");

        let poll_types = ["regular", "quiz"];
        let mut common_fields = vec![];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for field in mem::take(&mut poll.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            if desc.contains("regular") {
                applicable.push("regular");
            }
            if desc.contains("quiz") {
                applicable.push("quiz");
            }
            if applicable.is_empty() {
                applicable.extend(poll_types);
            }

            for &t in &applicable {
                let f = field.clone();
                type_fields_map.entry(t).or_default().push(f);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for poll_type in poll_types {
            let variant_name = snake_to_upper_camel(poll_type);
            let type_name = format!("{}{variant_name}", poll.name);

            let mut fields = common_fields.clone();
            if let Some(specific) = type_fields_map.get(poll_type) {
                fields.extend(specific.clone());
            }

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: poll.href.clone(),
                    description: vec![
                        format!("This object represents a {} poll.", poll_type),
                        "# Notes".to_owned(),
                        format!(
                            "This object represents a poll from original poll type `{poll_type}`."
                        ),
                    ],
                    fields,
                    subtype_kind: Some(SubtypeKind::Tagged {
                        tag_field: "type".to_owned(),
                        parent_tag_field: None,
                    }),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![poll.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        Self::finalize_split(
            &mut poll,
            &subtypes,
            SubtypeKind::Tagged {
                tag_field: "type".to_owned(),
                parent_tag_field: None,
            },
        );
        self.types.insert(poll.name.clone(), poll);
        self.types.extend(types);
    }

    pub fn split_poll_media_types(&mut self) {
        let mut media = self
            .types
            .remove("PollMedia")
            .expect("PollMedia doesn't exist in schema");

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for mut field in mem::take(&mut media.fields) {
            let field_name = field.name.clone();
            let variant_name = snake_to_upper_camel(&field_name);
            let type_name = format!("{}{variant_name}", media.name);
            field.required = true;

            let description = vec![
                field.description.clone(),
                "# Notes".to_owned(),
                format!("This object represents a poll media from original field `{field_name}`."),
            ];

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: media.href.clone(),
                    description,
                    fields: vec![field],
                    subtype_kind: Some(SubtypeKind::Untagged),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![media.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        Self::finalize_split(&mut media, &subtypes, SubtypeKind::Untagged);
        self.types.insert(media.name.clone(), media);
        self.types.extend(types);
    }

    pub fn split_giveaway_types(&mut self) {
        let mut giveaway = self
            .types
            .remove("Giveaway")
            .expect("Giveaway doesn't exist in schema");
        let giveaway_types = ["star", "premium"];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for mut field in mem::take(&mut giveaway.fields) {
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            if desc.contains("telegram star") {
                applicable.push("star");
            }
            if desc.contains("telegram premium") {
                applicable.push("premium");
            }
            if applicable.is_empty() {
                applicable.extend(giveaway_types);
            } else {
                field.required = true;
            }
            for &t in &applicable {
                let f = field.clone();
                type_fields_map.entry(t).or_default().push(f);
            }
        }

        let (types, subtypes) = Self::build_untagged_subtypes(
            &giveaway.name,
            &giveaway.href,
            &giveaway_types,
            &type_fields_map,
            |t| {
                vec![
                    format!("This object represents a {t} giveaway."),
                    "# Notes".to_owned(),
                    format!("This object represents a giveaway from original giveaway type `{t}`."),
                ]
            },
        );

        Self::finalize_split(&mut giveaway, &subtypes, SubtypeKind::Untagged);
        self.types.insert(giveaway.name.clone(), giveaway);
        self.types.extend(types);
    }

    pub fn split_giveaway_winners_types(&mut self) {
        let mut winners = self
            .types
            .remove("GiveawayWinners")
            .expect("GiveawayWinners doesn't exist in schema");
        let winners_types = ["star", "premium"];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for mut field in mem::take(&mut winners.fields) {
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            if desc.contains("telegram star") {
                applicable.push("star");
            }
            if desc.contains("telegram premium") {
                applicable.push("premium");
            }
            if applicable.is_empty() {
                applicable.extend(winners_types);
            } else {
                field.required = true;
            }
            for &t in &applicable {
                type_fields_map.entry(t).or_default().push(field.clone());
            }
        }

        let (types, subtypes) = Self::build_untagged_subtypes(
            &winners.name,
            &winners.href,
            &winners_types,
            &type_fields_map,
            |t| {
                vec![
                    format!("This object represents a {t} giveaway winners."),
                    "# Notes".to_owned(),
                    format!("This object represents giveaway winners from original field `{t}`."),
                ]
            },
        );

        Self::finalize_split(&mut winners, &subtypes, SubtypeKind::Untagged);
        self.types.insert(winners.name.clone(), winners);
        self.types.extend(types);
    }

    pub fn split_star_transaction_types(&mut self) {
        let mut transaction = self
            .types
            .remove("StarTransaction")
            .expect("StarTransaction doesn't exist in schema");
        let transaction_types = ["incoming", "outgoing"];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for mut field in mem::take(&mut transaction.fields) {
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            if desc.contains("source of an incoming transaction") {
                applicable.push("incoming");
            }
            if desc.contains("receiver of an outgoing transaction") {
                applicable.push("outgoing");
            }
            if applicable.is_empty() {
                applicable.extend(transaction_types);
            } else {
                field.required = true;
            }
            for &t in &applicable {
                let f = field.clone();
                type_fields_map.entry(t).or_default().push(f);
            }
        }

        let (types, subtypes) = Self::build_untagged_subtypes(
            &transaction.name,
            &transaction.href,
            &transaction_types,
            &type_fields_map,
            |t| {
                vec![
                    format!("This object represents an {t} star transaction."),
                    "# Notes".to_owned(),
                    format!("This object represents a star transaction from original field `{t}`."),
                ]
            },
        );

        Self::finalize_split(&mut transaction, &subtypes, SubtypeKind::Untagged);
        self.types.insert(transaction.name.clone(), transaction);
        self.types.extend(types);
    }

    pub fn split_encrypted_passport_element_types(&mut self) {
        let mut element = self
            .types
            .remove("EncryptedPassportElement")
            .expect("EncryptedPassportElement doesn't exist in schema");

        let element_types = [
            "personal_details",
            "passport",
            "driver_license",
            "identity_card",
            "internal_passport",
            "address",
            "utility_bill",
            "bank_statement",
            "rental_agreement",
            "passport_registration",
            "temporary_registration",
            "phone_number",
            "email",
        ];

        let mut common_fields = vec![];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for mut field in mem::take(&mut element.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }

            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            for &t in &element_types {
                let type_key = format!("\"{t}\"");
                if desc.contains(&type_key) {
                    applicable.push(t);
                }
            }
            if applicable.is_empty() {
                applicable.extend(element_types);
            } else {
                field.required = true;
            }

            for &element_type in &applicable {
                let f = field.clone();
                type_fields_map.entry(element_type).or_default().push(f);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for element_type in element_types {
            let variant_name = snake_to_upper_camel(element_type);
            let type_name = format!("{}{variant_name}", element.name);

            let mut fields = common_fields.clone();
            if let Some(specific) = type_fields_map.get(element_type) {
                fields.extend(specific.clone());
            }

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: element.href.clone(),
                    description: vec![
                        format!(
                            "This object represents a/an {} encrypted passport element.",
                            element_type.replace('_', " ")
                        ),
                        "# Notes".to_owned(),
                        format!(
                            "This object represents an encrypted passport element from original \
                             field `{element_type}`."
                        ),
                    ],
                    fields,
                    subtype_kind: Some(SubtypeKind::Tagged {
                        tag_field: "type".to_owned(),
                        parent_tag_field: None,
                    }),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![element.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        Self::finalize_split(
            &mut element,
            &subtypes,
            SubtypeKind::Tagged {
                tag_field: "type".to_owned(),
                parent_tag_field: None,
            },
        );
        self.types.insert(element.name.clone(), element);
        self.types.extend(types);
    }

    pub fn split_message_entity_types(&mut self) {
        let mut entity = self
            .types
            .remove("MessageEntity")
            .expect("MessageEntity doesn't exist in schema");

        let entity_types = [
            "mention",
            "hashtag",
            "cashtag",
            "bot_command",
            "url",
            "email",
            "phone_number",
            "bold",
            "italic",
            "underline",
            "strikethrough",
            "spoiler",
            "blockquote",
            "expandable_blockquote",
            "code",
            "pre",
            "text_link",
            "text_mention",
            "custom_emoji",
            "date_time",
        ];

        let mut common_fields = vec![];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for mut field in mem::take(&mut entity.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }

            let desc = field.description.to_lowercase();
            let mut applicable = vec![];

            for &t in &entity_types {
                let type_key = format!("\"{t}\"");
                if desc.contains(&type_key) {
                    applicable.push(t);
                    break;
                }
            }
            if applicable.is_empty() {
                applicable.extend(entity_types);
            } else if !field.required
                && !["language", "date_time_format"].contains(&field.name.as_str())
            {
                field.required = true;
            }

            for &entity_type in &applicable {
                let f = field.clone();
                type_fields_map.entry(entity_type).or_default().push(f);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for entity_type in entity_types {
            let variant_name = snake_to_upper_camel(entity_type);
            let type_name = format!("{}{variant_name}", entity.name);

            let mut fields = common_fields.clone();
            if let Some(specific) = type_fields_map.get(entity_type) {
                fields.extend(specific.clone());
            }

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: entity.href.clone(),
                    description: vec![
                        format!(
                            "This object represents a/an {} message entity.",
                            entity_type.replace('_', " ")
                        ),
                        "# Notes".to_owned(),
                        format!(
                            "This object represents a message entity from original field \
                             `{entity_type}`."
                        ),
                    ],
                    fields,
                    subtype_kind: Some(SubtypeKind::Tagged {
                        tag_field: "type".to_owned(),
                        parent_tag_field: None,
                    }),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![entity.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        Self::finalize_split(
            &mut entity,
            &subtypes,
            SubtypeKind::Tagged {
                tag_field: "type".to_owned(),
                parent_tag_field: None,
            },
        );
        self.types.insert(entity.name.clone(), entity);
        self.types.extend(types);
    }

    pub fn split_transaction_partner_user_types(&mut self) {
        let mut partner = self
            .types
            .remove("TransactionPartnerUser")
            .expect("TransactionPartnerUser doesn't exist in schema");

        let transaction_types = [
            "invoice_payment",
            "paid_media_payment",
            "gift_purchase",
            "premium_purchase",
            "business_account_transfer",
        ];

        let parent_tag_field = match partner.subtype_kind.as_ref().unwrap() {
            SubtypeKind::Tagged {
                tag_field, ..
            } => tag_field.to_owned(),
            _ => unreachable!(),
        };
        let mut common_fields = vec![];
        let mut type_fields_map: HashMap<&str, Vec<_>> = HashMap::new();

        for mut field in mem::take(&mut partner.fields) {
            if field.name == "transaction_type" {
                common_fields.push(field);
                continue;
            }

            let desc = field.description.to_lowercase();
            let mut applicable = vec![];
            for &t in &transaction_types {
                let type_key = format!("\"{t}\"");
                if desc.contains(&type_key) {
                    applicable.push(t);
                }
            }
            if applicable.is_empty() {
                applicable.extend(transaction_types);
            } else {
                field.required = !desc.contains("can be available");
            }

            for &t in &applicable {
                let f = field.clone();
                type_fields_map.entry(t).or_default().push(f);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for transaction_type in transaction_types {
            let variant_name = snake_to_upper_camel(transaction_type);
            let type_name = format!("{}{variant_name}", partner.name);

            let mut fields = common_fields.clone();
            if let Some(specific) = type_fields_map.get(transaction_type) {
                fields.extend(specific.clone());
            }

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: partner.href.clone(),
                    description: vec![
                        format!(
                            "This object represents a/an {} transaction partner user.",
                            transaction_type.replace('_', " ")
                        ),
                        "# Notes".to_owned(),
                        format!(
                            "This object represents a transaction partner user from original \
                             field `{transaction_type}`."
                        ),
                    ],
                    fields,
                    subtype_kind: Some(SubtypeKind::Tagged {
                        tag_field: "transaction_type".to_owned(),
                        parent_tag_field: Some(parent_tag_field.clone()),
                    }),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![partner.name.clone()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        Self::finalize_split(
            &mut partner,
            &subtypes,
            SubtypeKind::Tagged {
                tag_field: "transaction_type".to_owned(),
                parent_tag_field: Some(parent_tag_field),
            },
        );
        self.types.insert(partner.name.clone(), partner);
        self.types.extend(types);
    }

    pub fn modify_get_updates_returns_method(&mut self) {
        let method = self
            .methods
            .get_mut("getUpdates")
            .expect("getUpdates doesn't exist in schema");

        method.returns.remove(0);
        let either_ty = TypeKindInField::Array(Box::new(TypeKindInField::Either(
            Box::new(TypeKindInField::Telegram("Update".to_owned())),
            Box::new(TypeKindInField::Telegram("UpdateUnparsed".to_owned())),
        )));

        method.returns.insert(0, either_ty);
    }

    /// Shared helper for split methods that produce `Untagged` subtypes with no common fields.
    fn build_untagged_subtypes(
        parent_name: &str,
        parent_href: &str,
        variant_keys: &[&str],
        type_fields_map: &HashMap<&str, Vec<NormalizedField>>,
        description_fn: impl Fn(&str) -> Vec<String>,
    ) -> (HashMap<String, NormalizedType>, Vec<(String, String)>) {
        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for &key in variant_keys {
            let variant_name = snake_to_upper_camel(key);
            let type_name = format!("{parent_name}{variant_name}");

            let fields = type_fields_map.get(key).cloned().unwrap_or_default();

            types.insert(
                type_name.clone(),
                NormalizedType {
                    name: type_name.clone(),
                    href: parent_href.to_owned(),
                    description: description_fn(key),
                    fields,
                    subtype_kind: Some(SubtypeKind::Untagged),
                    subtypes: vec![],
                    extra_subtypes: vec![],
                    subtype_of: vec![parent_name.to_owned()],
                    has_extra_fields: false,
                },
            );
            subtypes.push((variant_name, type_name));
        }

        (types, subtypes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerKind {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
}

/// - `Any` - Any boolean value
/// - `True` - Only possible value is `true`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanKind {
    Any,
    True,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKindInField {
    String,
    InputFile,
    ChatId,
    Integer(IntegerKind),
    Boolean(BooleanKind),
    Telegram(TelegramTypeName),
    Array(Box<TypeKindInField>),
    Either(Box<TypeKindInField>, Box<TypeKindInField>),
}

impl TypeKindInField {
    #[must_use]
    pub fn get_path(&self) -> Option<Path> {
        match self {
            TypeKindInField::InputFile => {
                Some(syn::parse_str("super::InputFile").expect("incorrect path"))
            }
            TypeKindInField::ChatId => {
                Some(syn::parse_str("super::ChatIdKind").expect("incorrect path"))
            }
            TypeKindInField::Telegram(name) => {
                Some(syn::parse_str(&format!("super::{name}")).expect("incorrect path"))
            }
            TypeKindInField::Array(kind) => kind.get_path(),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_copy(&self) -> bool {
        matches!(
            self,
            TypeKindInField::Integer(_) | TypeKindInField::Boolean(_)
        )
    }

    #[must_use]
    pub fn require_import(&self) -> bool {
        match self {
            TypeKindInField::Telegram(_) | TypeKindInField::ChatId | TypeKindInField::InputFile => {
                true
            }
            TypeKindInField::Either(left, right) => left.require_import() || right.require_import(),
            TypeKindInField::Array(inner) => inner.require_import(),
            _ => false,
        }
    }
}

#[must_use]
pub fn is_string(raw_type: &RawType) -> bool {
    raw_type == "String"
}

#[must_use]
pub fn get_if_integer(raw_type: &RawType, description: &str) -> Option<IntegerKind> {
    match raw_type.as_str() {
        "Integer" => {
            // 24-bit colors ("RGB"/"RGB24" format) always fit in `i32`. Detect them before
            // range-based sizing: their descriptions carry no value range, and any numbers
            // present (e.g. example color constants) would otherwise mislead it. The 32-bit
            // variants ("ARGB"/"RGBA") include an alpha byte, can exceed `i32::MAX`, and are
            // excluded so they keep the default `i64`.
            let desc = description.to_lowercase();
            if desc.contains("rgb") && !desc.contains("argb") && !desc.contains("rgba") {
                return Some(IntegerKind::Int32);
            }
            Some(match extract_range(description) {
                Some((min, max)) if min < 0 => {
                    if max <= i64::from(i8::MAX) {
                        IntegerKind::Int8
                    } else if max <= i64::from(i16::MAX) {
                        IntegerKind::Int16
                    } else if max <= i64::from(i32::MAX) {
                        IntegerKind::Int32
                    } else {
                        IntegerKind::Int64
                    }
                }
                Some((_, max)) => {
                    if max <= i64::from(u8::MAX) {
                        IntegerKind::UInt8
                    } else if max <= i64::from(u16::MAX) {
                        IntegerKind::UInt16
                    } else if max <= i64::from(u32::MAX) {
                        IntegerKind::UInt32
                    } else {
                        IntegerKind::UInt64
                    }
                }
                None => IntegerKind::Int64,
            })
        }
        "Float" => Some(IntegerKind::Float64),
        _ => None,
    }
}

fn extract_range(description: &str) -> Option<(i64, i64)> {
    let doc = description.to_lowercase();
    let re =
        Regex::new(r"(?:from|between|must be)?\s*([-]?\d+)\s*(?:-|to|and)\s*([-]?\d+)").ok()?;
    let caps = re.captures(&doc)?;
    // A match followed by an arithmetic operator is part of an expression, not a range:
    // e.g. "0 - 7 * 24 * 60" is the value `7 * 24 * 60`, not the range `0-7`.
    let rest = doc[caps.get(0)?.end()..].trim_start();
    if rest.starts_with(['*', '/', '+']) {
        return None;
    }
    let min: i64 = caps[1].parse().ok()?;
    let max: i64 = caps[2].parse().ok()?;
    (min <= max).then_some((min, max))
}

/// Remove the first numeric range (e.g. "1-100", "from 1 to 3") from a description.
///
/// For array fields such a range describes the array length, not the element values, so it
/// must not influence element integer sizing (see the array branch of [`Field::identify_field_type`]).
/// Non-range text (e.g. "RGB format") is preserved so element typing still works.
fn strip_length_range(description: &str) -> String {
    let Ok(re) = Regex::new(r"(?i)(?:from|between|must be)?\s*[-]?\d+\s*(?:-|to|and)\s*[-]?\d+")
    else {
        return description.to_owned();
    };
    re.replace(description, " ").into_owned()
}

/// # Notes
/// Currently use only [`BooleanKind::Any`] and [`BooleanKind::True`].
/// Type like `False` is not used in the Telegram API.
#[must_use]
pub fn get_if_boolean(raw_type: &RawType) -> Option<BooleanKind> {
    match raw_type.as_str() {
        "Boolean" => Some(BooleanKind::Any),
        "True" => Some(BooleanKind::True),
        _ => None,
    }
}

/// All arrays in the Telegram API start with `Array of` prefix.
#[must_use]
pub fn is_array_of(raw_type: &RawType) -> bool {
    raw_type.starts_with("Array of")
}

/// `InputFile` if types is `[InputFile]` or `[InputFile, String]`.
#[must_use]
pub fn multi_type_is_input_file(types: &[RawType], description: &str) -> bool {
    if description.contains("attach://<file_attach_name>") {
        return true;
    }

    let has_input_file = types.contains(&"InputFile".to_owned());
    match types.len() {
        1 => has_input_file,
        2 => has_input_file && types.contains(&"String".to_owned()),
        _ => false,
    }
}

/// `ChatId` if types is `[Integer, String]`.
#[must_use]
pub fn multi_type_is_chat_id(types: &[RawType]) -> bool {
    types.len() == 2
        && types.contains(&"Integer".to_owned())
        && types.contains(&"String".to_owned())
}

/// `ReplyMarkup` if field name is `reply_markup` and there are multiple types.
#[must_use]
pub fn multi_type_is_reply_markup(types: &[RawType], name: &str) -> bool {
    types.len() > 1 && name == "reply_markup"
}

/// `Array of InputMedia` for field `media` in sendMediaGroup-like methods.
#[must_use]
pub fn multi_type_is_input_media(types: &[RawType], name: &str) -> bool {
    if name != "media" {
        return false;
    }
    let expected = [
        "Array of InputMediaAudio",
        "Array of InputMediaDocument",
        "Array of InputMediaLivePhoto",
        "Array of InputMediaPhoto",
        "Array of InputMediaVideo",
    ];
    types.len() == expected.len()
        && expected
            .iter()
            .all(|&expected_type| types.contains(&expected_type.to_string()))
}

/// `InputRichMessageMediaContent` for the non-array `media` union
/// {`InputMediaAnimation`, `InputMediaAudio`, `InputMediaPhoto`, `InputMediaVideo`,
/// `InputMediaVoiceNote`} of `InputRichMessageMedia`. Matched by the exact type set (the
/// bare variants, not the `Array of ...` form), so it never collides with the array case.
#[must_use]
pub fn multi_type_is_input_rich_message_media(types: &[RawType]) -> bool {
    let expected = [
        "InputMediaAnimation",
        "InputMediaAudio",
        "InputMediaPhoto",
        "InputMediaVideo",
        "InputMediaVoiceNote",
    ];
    types.len() == expected.len()
        && expected
            .iter()
            .all(|&expected_type| types.contains(&expected_type.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_rich_message_media_union_maps_to_composed_enum() {
        let field = Field {
            name: "media".to_owned(),
            required: true,
            description: "The media to be sent.".to_owned(),
            types: vec![
                "InputMediaVideo".to_owned(),
                "InputMediaAnimation".to_owned(),
                "InputMediaVoiceNote".to_owned(),
                "InputMediaAudio".to_owned(),
                "InputMediaPhoto".to_owned(),
            ],
        };
        assert_eq!(
            field.identify_field_type(),
            TypeKindInField::Telegram("InputRichMessageMediaContent".to_owned()),
        );

        assert!(!multi_type_is_input_rich_message_media(&[
            "Array of InputMediaAudio".to_owned(),
            "Array of InputMediaDocument".to_owned(),
            "Array of InputMediaLivePhoto".to_owned(),
            "Array of InputMediaPhoto".to_owned(),
            "Array of InputMediaVideo".to_owned(),
        ]));
    }

    #[test]
    fn test_parse_json_to_schema() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            }
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_json(content).unwrap();

        assert_eq!(schema.version, "1.0");
        assert_eq!(schema.release_date, "2021-01-01");
        assert_eq!(schema.changelog, "Initial release");
        assert_eq!(schema.types.len(), 1);
    }

    #[test]
    fn test_is_telegram_type() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            }
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_json(content).unwrap();

        assert!(schema.is_telegram_type(&"Type1".to_owned()));
        assert!(!schema.is_telegram_type(&"Type2".to_owned()));
    }

    #[test]
    fn test_is_string() {
        assert!(is_string(&"String".to_owned()));
        assert!(!is_string(&"Integer".to_owned()));
    }

    #[test]
    fn test_get_if_integer() {
        assert_eq!(
            get_if_integer(&"Integer".to_owned(), ""),
            Some(IntegerKind::Int64)
        );
        assert_eq!(
            get_if_integer(&"Float".to_owned(), ""),
            Some(IntegerKind::Float64)
        );
        assert_eq!(get_if_integer(&"String".to_owned(), ""), None);
    }

    #[test]
    fn test_get_if_boolean() {
        assert_eq!(
            get_if_boolean(&"Boolean".to_owned()),
            Some(BooleanKind::Any)
        );
        assert_eq!(get_if_boolean(&"True".to_owned()), Some(BooleanKind::True));
        assert_eq!(get_if_boolean(&"String".to_owned()), None);
    }

    #[test]
    fn test_is_array_of() {
        assert!(is_array_of(&"Array of String".to_owned()));
        assert!(is_array_of(&"Array of Array of String".to_owned()));
        assert!(!is_array_of(&"String".to_owned()));
    }

    #[test]
    fn test_multi_type_is_input_file() {
        assert!(multi_type_is_input_file(&["InputFile".to_owned()], ""));
        assert!(multi_type_is_input_file(
            &["InputFile".to_owned(), "String".to_owned(),],
            ""
        ));
        assert!(multi_type_is_input_file(
            &["String".to_owned(), "InputFile".to_owned(),],
            ""
        ));
        assert!(!multi_type_is_input_file(&["String".to_owned()], ""));
        assert!(!multi_type_is_input_file(
            &["String".to_owned(), "Integer".to_owned(),],
            ""
        ));
    }

    #[test]
    fn test_multi_type_is_chat_id() {
        assert!(!multi_type_is_chat_id(&["Integer".to_owned()]));
        assert!(!multi_type_is_chat_id(&["String".to_owned()]));
        assert!(multi_type_is_chat_id(&[
            "String".to_owned(),
            "Integer".to_owned(),
        ]));
        assert!(multi_type_is_chat_id(&[
            "Integer".to_owned(),
            "String".to_owned(),
        ]));
        assert!(!multi_type_is_chat_id(&["InputFile".to_owned()]));
        assert!(!multi_type_is_chat_id(&[
            "InputFile".to_owned(),
            "String".to_owned(),
        ]));
    }

    #[test]
    fn test_multi_type_is_reply_markup() {
        assert!(!multi_type_is_reply_markup(
            &["String".to_owned()],
            "reply_markup"
        ));
        assert!(!multi_type_is_reply_markup(
            &["InlineKeyboardMarkup".to_owned()],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &["Markup1".to_owned(), "Markup2".to_owned()],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &[
                "InlineKeyboardMarkup".to_owned(),
                "ReplyKeyboardMarkup".to_owned(),
            ],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &[
                "ReplyKeyboardMarkup".to_owned(),
                "InlineKeyboardMarkup".to_owned(),
            ],
            "reply_markup"
        ));
    }

    #[test]
    fn test_multi_type_is_input_media() {
        assert!(multi_type_is_input_media(
            &[
                "Array of InputMediaAudio".to_owned(),
                "Array of InputMediaDocument".to_owned(),
                "Array of InputMediaLivePhoto".to_owned(),
                "Array of InputMediaPhoto".to_owned(),
                "Array of InputMediaVideo".to_owned(),
            ],
            "media"
        ));
        assert!(!multi_type_is_input_media(
            &[
                "Array of InputMediaAudio".to_owned(),
                "Array of InputMediaDocument".to_owned(),
            ],
            "media"
        ));
        assert!(!multi_type_is_input_media(
            &[
                "Array of InputMediaAudio".to_owned(),
                "Array of InputMediaDocument".to_owned(),
                "Array of InputMediaLivePhoto".to_owned(),
                "Array of InputMediaPhoto".to_owned(),
                "Array of InputMediaVideo".to_owned(),
            ],
            "results"
        ));
    }

    #[test]
    fn test_identify_field_type() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            },
                            {
                                "name": "field2",
                                "required": true,
                                "description": "Field2 description",
                                "types": ["Integer"]
                            },
                            {
                                "name": "field3",
                                "required": true,
                                "description": "Field3 description",
                                "types": ["Boolean"]
                            },
                            {
                                "name": "field4",
                                "required": true,
                                "description": "Field4 description",
                                "types": ["Float"]
                            },
                            {
                                "name": "field5",
                                "required": true,
                                "description": "Field5 description",
                                "types": ["Array of String"]
                            },
                            {
                                "name": "field6",
                                "required": true,
                                "description": "Field6 description",
                                "types": ["Array of Array of String"]
                            },
                            {
                                "name": "field7",
                                "required": true,
                                "description": "Field7 description",
                                "types": ["Array of Integer"]
                            },
                            {
                                "name": "field8",
                                "required": true,
                                "description": "Field8 description",
                                "types": ["Array of Float"]
                            },
                            {
                                "name": "field9",
                                "required": true,
                                "description": "Field9 description",
                                "types": ["True"]
                            },
                            {
                                "name": "field10",
                                "required": true,
                                "description": "Field10 description",
                                "types": ["Type1"]
                            }
                        ]
                    },
                    "Type2": {
                        "name": "Type2",
                        "href": "https://example.com",
                        "description": ["Type2 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["InputFile", "String"]
                            },
                            {
                                "name": "field2",
                                "required": true,
                                "description": "Field2 description",
                                "types": ["InputFile"]
                            },
                            {
                                "name": "reply_markup",
                                "required": true,
                                "description": "Field description",
                                "types": ["InlineKeyboardMarkup", "ReplyKeyboardMarkup"]
                            },
                            {
                                "name": "reply_markup",
                                "required": true,
                                "description": "Field description",
                                "types": ["ReplyKeyboardMarkup"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["Integer", "String"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["String", "Integer"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["Integer"]
                            }
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_json(content).unwrap();

        let fields = schema.types.get("Type1").unwrap().fields.as_slice();

        assert_eq!(
            fields.get(0).unwrap().identify_field_type(),
            TypeKindInField::String
        );
        assert_eq!(
            fields.get(1).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Int64)
        );
        assert_eq!(
            fields.get(2).unwrap().identify_field_type(),
            TypeKindInField::Boolean(BooleanKind::Any)
        );
        assert_eq!(
            fields.get(3).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Float64)
        );
        assert_eq!(
            fields.get(4).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::String))
        );
        assert_eq!(
            fields.get(5).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Array(Box::new(
                TypeKindInField::String
            ))))
        );
        assert_eq!(
            fields.get(6).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Integer(IntegerKind::Int64))),
        );
        assert_eq!(
            fields.get(7).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Integer(IntegerKind::Float64))),
        );
        assert_eq!(
            fields.get(8).unwrap().identify_field_type(),
            TypeKindInField::Boolean(BooleanKind::True),
        );
        assert_eq!(
            fields.get(9).unwrap().identify_field_type(),
            TypeKindInField::Telegram("Type1".to_owned()),
        );

        let fields = schema.types.get("Type2").unwrap().fields.as_slice();

        assert_eq!(
            fields.get(0).unwrap().identify_field_type(),
            TypeKindInField::InputFile,
        );
        assert_eq!(
            fields.get(1).unwrap().identify_field_type(),
            TypeKindInField::InputFile
        );
        assert_eq!(
            fields.get(2).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ReplyMarkup".to_owned())
        );
        assert_eq!(
            fields.get(3).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ReplyKeyboardMarkup".to_owned()),
        );
        assert_eq!(
            fields.get(4).unwrap().identify_field_type(),
            TypeKindInField::ChatId,
        );
        assert_eq!(
            fields.get(5).unwrap().identify_field_type(),
            TypeKindInField::ChatId,
        );
        assert_eq!(
            fields.get(6).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Int64),
        );
    }

    #[test]
    fn test_split_message_types_creates_new_types() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Message": {
                        "name": "Message",
                        "href": "https://core.telegram.org/bots/api#message",
                        "description": [
                            "This object represents a message."
                        ],
                        "fields": [
                            {
                            "name": "message_id",
                            "types": [
                                "Integer"
                            ],
                            "required": true,
                            "description": "Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent"
                            },
                            {
                            "name": "message_thread_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only"
                            },
                            {
                            "name": "direct_messages_topic",
                            "types": [
                                "DirectMessagesTopic"
                            ],
                            "required": false,
                            "description": "Optional. Information about the direct messages chat topic that contains the message"
                            },
                            {
                            "name": "from",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats"
                            },
                            {
                            "name": "sender_chat",
                            "types": [
                                "Chat"
                            ],
                            "required": false,
                            "description": "Optional. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats."
                            },
                            {
                            "name": "sender_boost_count",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. If the sender of the message boosted the chat, the number of boosts added by the user"
                            },
                            {
                            "name": "sender_business_bot",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account."
                            },
                            {
                            "name": "date",
                            "types": [
                                "Integer"
                            ],
                            "required": true,
                            "description": "Date the message was sent in Unix time. It is always a positive number, representing a valid date."
                            },
                            {
                            "name": "business_connection_id",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier."
                            },
                            {
                            "name": "chat",
                            "types": [
                                "Chat"
                            ],
                            "required": true,
                            "description": "Chat the message belongs to"
                            },
                            {
                            "name": "forward_origin",
                            "types": [
                                "MessageOrigin"
                            ],
                            "required": false,
                            "description": "Optional. Information about the original message for forwarded messages"
                            },
                            {
                            "name": "is_topic_message",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message is sent to a topic in a forum supergroup or a private chat with the bot"
                            },
                            {
                            "name": "is_automatic_forward",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message is a channel post that was automatically forwarded to the connected discussion group"
                            },
                            {
                            "name": "reply_to_message",
                            "types": [
                                "Message"
                            ],
                            "required": false,
                            "description": "Optional. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply."
                            },
                            {
                            "name": "external_reply",
                            "types": [
                                "ExternalReplyInfo"
                            ],
                            "required": false,
                            "description": "Optional. Information about the message that is being replied to, which may come from another chat or forum topic"
                            },
                            {
                            "name": "quote",
                            "types": [
                                "TextQuote"
                            ],
                            "required": false,
                            "description": "Optional. For replies that quote part of the original message, the quoted part of the message"
                            },
                            {
                            "name": "reply_to_story",
                            "types": [
                                "Story"
                            ],
                            "required": false,
                            "description": "Optional. For replies to a story, the original story"
                            },
                            {
                            "name": "reply_to_checklist_task_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. Identifier of the specific checklist task that is being replied to"
                            },
                            {
                            "name": "via_bot",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. Bot through which the message was sent"
                            },
                            {
                            "name": "edit_date",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. Date the message was last edited in Unix time"
                            },
                            {
                            "name": "has_protected_content",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message can't be forwarded"
                            },
                            {
                            "name": "is_from_offline",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message"
                            },
                            {
                            "name": "is_paid_post",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited."
                            },
                            {
                            "name": "media_group_id",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. The unique identifier of a media message group this message belongs to"
                            },
                            {
                            "name": "author_signature",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator"
                            },
                            {
                            "name": "paid_star_count",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. The number of Telegram Stars that were paid by the sender of the message to send it"
                            },
                            {
                            "name": "text",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. For text messages, the actual UTF-8 text of the message"
                            },
                            {
                            "name": "entities",
                            "types": [
                                "Array of MessageEntity"
                            ],
                            "required": false,
                            "description": "Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text"
                            },
                            {
                            "name": "link_preview_options",
                            "types": [
                                "LinkPreviewOptions"
                            ],
                            "required": false,
                            "description": "Optional. Options used for link preview generation for the message, if it is a text message and link preview options were changed"
                            },
                            {
                            "name": "suggested_post_info",
                            "types": [
                                "SuggestedPostInfo"
                            ],
                            "required": false,
                            "description": "Optional. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited."
                            },
                            {
                            "name": "effect_id",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Unique identifier of the message effect added to the message"
                            },
                            {
                            "name": "animation",
                            "types": [
                                "Animation"
                            ],
                            "required": false,
                            "description": "Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set"
                            },
                            {
                            "name": "audio",
                            "types": [
                                "Audio"
                            ],
                            "required": false,
                            "description": "Optional. Message is an audio file, information about the file"
                            },
                            {
                            "name": "document",
                            "types": [
                                "Document"
                            ],
                            "required": false,
                            "description": "Optional. Message is a general file, information about the file"
                            },
                            {
                            "name": "paid_media",
                            "types": [
                                "PaidMediaInfo"
                            ],
                            "required": false,
                            "description": "Optional. Message contains paid media; information about the paid media"
                            },
                            {
                            "name": "photo",
                            "types": [
                                "Array of PhotoSize"
                            ],
                            "required": false,
                            "description": "Optional. Message is a photo, available sizes of the photo"
                            },
                            {
                            "name": "sticker",
                            "types": [
                                "Sticker"
                            ],
                            "required": false,
                            "description": "Optional. Message is a sticker, information about the sticker"
                            },
                            {
                            "name": "story",
                            "types": [
                                "Story"
                            ],
                            "required": false,
                            "description": "Optional. Message is a forwarded story"
                            },
                            {
                            "name": "video",
                            "types": [
                                "Video"
                            ],
                            "required": false,
                            "description": "Optional. Message is a video, information about the video"
                            },
                            {
                            "name": "video_note",
                            "types": [
                                "VideoNote"
                            ],
                            "required": false,
                            "description": "Optional. Message is a video note, information about the video message"
                            },
                            {
                            "name": "voice",
                            "types": [
                                "Voice"
                            ],
                            "required": false,
                            "description": "Optional. Message is a voice message, information about the file"
                            },
                            {
                            "name": "caption",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Caption for the animation, audio, document, paid media, photo, video or voice"
                            },
                            {
                            "name": "caption_entities",
                            "types": [
                                "Array of MessageEntity"
                            ],
                            "required": false,
                            "description": "Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption"
                            },
                            {
                            "name": "show_caption_above_media",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the caption must be shown above the message media"
                            },
                            {
                            "name": "has_media_spoiler",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message media is covered by a spoiler animation"
                            },
                            {
                            "name": "checklist",
                            "types": [
                                "Checklist"
                            ],
                            "required": false,
                            "description": "Optional. Message is a checklist"
                            },
                            {
                            "name": "contact",
                            "types": [
                                "Contact"
                            ],
                            "required": false,
                            "description": "Optional. Message is a shared contact, information about the contact"
                            },
                            {
                            "name": "dice",
                            "types": [
                                "Dice"
                            ],
                            "required": false,
                            "description": "Optional. Message is a dice with random value"
                            },
                            {
                            "name": "game",
                            "types": [
                                "Game"
                            ],
                            "required": false,
                            "description": "Optional. Message is a game, information about the game. More about games: https://core.telegram.org/bots/api#games"
                            },
                            {
                            "name": "poll",
                            "types": [
                                "Poll"
                            ],
                            "required": false,
                            "description": "Optional. Message is a native poll, information about the poll"
                            },
                            {
                            "name": "venue",
                            "types": [
                                "Venue"
                            ],
                            "required": false,
                            "description": "Optional. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set"
                            },
                            {
                            "name": "location",
                            "types": [
                                "Location"
                            ],
                            "required": false,
                            "description": "Optional. Message is a shared location, information about the location"
                            },
                            {
                            "name": "new_chat_members",
                            "types": [
                                "Array of User"
                            ],
                            "required": false,
                            "description": "Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)"
                            },
                            {
                            "name": "left_chat_member",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. A member was removed from the group, information about them (this member may be the bot itself)"
                            },
                            {
                            "name": "chat_owner_left",
                            "types": [
                                "ChatOwnerLeft"
                            ],
                            "required": false,
                            "description": "Optional. Service message: chat owner has left"
                            },
                            {
                            "name": "chat_owner_changed",
                            "types": [
                                "ChatOwnerChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: chat owner has changed"
                            },
                            {
                            "name": "new_chat_title",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. A chat title was changed to this value"
                            },
                            {
                            "name": "new_chat_photo",
                            "types": [
                                "Array of PhotoSize"
                            ],
                            "required": false,
                            "description": "Optional. A chat photo was change to this value"
                            },
                            {
                            "name": "delete_chat_photo",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the chat photo was deleted"
                            },
                            {
                            "name": "group_chat_created",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the group has been created"
                            },
                            {
                            "name": "supergroup_chat_created",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup."
                            },
                            {
                            "name": "channel_chat_created",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel."
                            },
                            {
                            "name": "message_auto_delete_timer_changed",
                            "types": [
                                "MessageAutoDeleteTimerChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: auto-delete timer settings changed in the chat"
                            },
                            {
                            "name": "migrate_to_chat_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier."
                            },
                            {
                            "name": "migrate_from_chat_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier."
                            },
                            {
                            "name": "pinned_message",
                            "types": [
                                "MaybeInaccessibleMessage"
                            ],
                            "required": false,
                            "description": "Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply."
                            },
                            {
                            "name": "invoice",
                            "types": [
                                "Invoice"
                            ],
                            "required": false,
                            "description": "Optional. Message is an invoice for a payment, information about the invoice. More about payments: https://core.telegram.org/bots/api#payments"
                            },
                            {
                            "name": "successful_payment",
                            "types": [
                                "SuccessfulPayment"
                            ],
                            "required": false,
                            "description": "Optional. Message is a service message about a successful payment, information about the payment. More about payments: https://core.telegram.org/bots/api#payments"
                            },
                            {
                            "name": "refunded_payment",
                            "types": [
                                "RefundedPayment"
                            ],
                            "required": false,
                            "description": "Optional. Message is a service message about a refunded payment, information about the payment. More about payments: https://core.telegram.org/bots/api#payments"
                            },
                            {
                            "name": "users_shared",
                            "types": [
                                "UsersShared"
                            ],
                            "required": false,
                            "description": "Optional. Service message: users were shared with the bot"
                            },
                            {
                            "name": "chat_shared",
                            "types": [
                                "ChatShared"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a chat was shared with the bot"
                            },
                            {
                            "name": "gift",
                            "types": [
                                "GiftInfo"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a regular gift was sent or received"
                            },
                            {
                            "name": "unique_gift",
                            "types": [
                                "UniqueGiftInfo"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a unique gift was sent or received"
                            },
                            {
                            "name": "gift_upgrade_sent",
                            "types": [
                                "GiftInfo"
                            ],
                            "required": false,
                            "description": "Optional. Service message: upgrade of a gift was purchased after the gift was sent"
                            },
                            {
                            "name": "connected_website",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. The domain name of the website on which the user has logged in. More about Telegram Login: https://core.telegram.org/widgets/login"
                            },
                            {
                            "name": "write_access_allowed",
                            "types": [
                                "WriteAccessAllowed"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess"
                            },
                            {
                            "name": "passport_data",
                            "types": [
                                "PassportData"
                            ],
                            "required": false,
                            "description": "Optional. Telegram Passport data"
                            },
                            {
                            "name": "proximity_alert_triggered",
                            "types": [
                                "ProximityAlertTriggered"
                            ],
                            "required": false,
                            "description": "Optional. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location."
                            },
                            {
                            "name": "boost_added",
                            "types": [
                                "ChatBoostAdded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: user boosted the chat"
                            },
                            {
                            "name": "chat_background_set",
                            "types": [
                                "ChatBackground"
                            ],
                            "required": false,
                            "description": "Optional. Service message: chat background set"
                            },
                            {
                            "name": "checklist_tasks_done",
                            "types": [
                                "ChecklistTasksDone"
                            ],
                            "required": false,
                            "description": "Optional. Service message: some tasks in a checklist were marked as done or not done"
                            },
                            {
                            "name": "checklist_tasks_added",
                            "types": [
                                "ChecklistTasksAdded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: tasks were added to a checklist"
                            },
                            {
                            "name": "direct_message_price_changed",
                            "types": [
                                "DirectMessagePriceChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed"
                            },
                            {
                            "name": "forum_topic_created",
                            "types": [
                                "ForumTopicCreated"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic created"
                            },
                            {
                            "name": "forum_topic_edited",
                            "types": [
                                "ForumTopicEdited"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic edited"
                            },
                            {
                            "name": "forum_topic_closed",
                            "types": [
                                "ForumTopicClosed"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic closed"
                            },
                            {
                            "name": "forum_topic_reopened",
                            "types": [
                                "ForumTopicReopened"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic reopened"
                            },
                            {
                            "name": "general_forum_topic_hidden",
                            "types": [
                                "GeneralForumTopicHidden"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the 'General' forum topic hidden"
                            },
                            {
                            "name": "general_forum_topic_unhidden",
                            "types": [
                                "GeneralForumTopicUnhidden"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the 'General' forum topic unhidden"
                            },
                            {
                            "name": "giveaway_created",
                            "types": [
                                "GiveawayCreated"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a scheduled giveaway was created"
                            },
                            {
                            "name": "giveaway",
                            "types": [
                                "Giveaway"
                            ],
                            "required": false,
                            "description": "Optional. The message is a scheduled giveaway message"
                            },
                            {
                            "name": "giveaway_winners",
                            "types": [
                                "GiveawayWinners"
                            ],
                            "required": false,
                            "description": "Optional. A giveaway with public winners was completed"
                            },
                            {
                            "name": "giveaway_completed",
                            "types": [
                                "GiveawayCompleted"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a giveaway without public winners was completed"
                            },
                            {
                            "name": "paid_message_price_changed",
                            "types": [
                                "PaidMessagePriceChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the price for paid messages has changed in the chat"
                            },
                            {
                            "name": "suggested_post_approved",
                            "types": [
                                "SuggestedPostApproved"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a suggested post was approved"
                            },
                            {
                            "name": "suggested_post_approval_failed",
                            "types": [
                                "SuggestedPostApprovalFailed"
                            ],
                            "required": false,
                            "description": "Optional. Service message: approval of a suggested post has failed"
                            },
                            {
                            "name": "suggested_post_declined",
                            "types": [
                                "SuggestedPostDeclined"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a suggested post was declined"
                            },
                            {
                            "name": "suggested_post_paid",
                            "types": [
                                "SuggestedPostPaid"
                            ],
                            "required": false,
                            "description": "Optional. Service message: payment for a suggested post was received"
                            },
                            {
                            "name": "suggested_post_refunded",
                            "types": [
                                "SuggestedPostRefunded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: payment for a suggested post was refunded"
                            },
                            {
                            "name": "video_chat_scheduled",
                            "types": [
                                "VideoChatScheduled"
                            ],
                            "required": false,
                            "description": "Optional. Service message: video chat scheduled"
                            },
                            {
                            "name": "video_chat_started",
                            "types": [
                                "VideoChatStarted"
                            ],
                            "required": false,
                            "description": "Optional. Service message: video chat started"
                            },
                            {
                            "name": "video_chat_ended",
                            "types": [
                                "VideoChatEnded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: video chat ended"
                            },
                            {
                            "name": "video_chat_participants_invited",
                            "types": [
                                "VideoChatParticipantsInvited"
                            ],
                            "required": false,
                            "description": "Optional. Service message: new participants invited to a video chat"
                            },
                            {
                            "name": "web_app_data",
                            "types": [
                                "WebAppData"
                            ],
                            "required": false,
                            "description": "Optional. Service message: data sent by a Web App"
                            },
                            {
                            "name": "reply_markup",
                            "types": [
                                "InlineKeyboardMarkup"
                            ],
                            "required": false,
                            "description": "Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons."
                            }
                        ],
                        "subtype_of": [
                            "MaybeInaccessibleMessage"
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_json(content).unwrap();
        let mut normalized = schema.normalize();

        normalized.split_message_types();

        let message = normalized.types.get("Message").unwrap();
        assert!(message.fields.is_empty());
        assert!(message.subtype_kind.is_some());
        assert_eq!(message.subtypes.len(), 70);

        let expected_major_types = [
            "MessageText",
            "MessagePhoto",
            "MessageVideo",
            "MessageAnimation",
            "MessageAudio",
            "MessageDocument",
            "MessageVoice",
            "MessageSticker",
            "MessagePoll",
            "MessageLocation",
            "MessageVenue",
            "MessageContact",
            "MessageGame",
            "MessageDice",
            "MessageInvoice",
            "MessageSuccessfulPayment",
            "MessageNewChatMembers",
            "MessageLeftChatMember",
            "MessagePinnedMessage",
        ];

        for type_name in expected_major_types.iter() {
            assert!(
                normalized.types.contains_key(*type_name),
                "Major type {} should exist",
                type_name
            );
        }

        for subtype in &message.subtypes {
            let type_name = &subtype.ty_name;
            let ty = normalized.types.get(type_name).unwrap();

            assert!(
                normalized.types.contains_key(type_name),
                "Subtype {type_name} should exist"
            );
            assert_eq!(
                ty.subtype_of,
                vec!["Message"],
                "{type_name} should be subtype of Message",
            );
            assert!(!ty.fields.is_empty(), "{} should have fields", type_name);
        }

        let content_type_main_fields = [
            ("MessageText", "text"),
            ("MessagePhoto", "photo"),
            ("MessageVideo", "video"),
            ("MessageAnimation", "animation"),
            ("MessageAudio", "audio"),
            ("MessageDocument", "document"),
            ("MessageVoice", "voice"),
            ("MessageSticker", "sticker"),
            ("MessagePoll", "poll"),
            ("MessageLocation", "location"),
            ("MessageVenue", "venue"),
            ("MessageContact", "contact"),
            ("MessageGame", "game"),
            ("MessageDice", "dice"),
            ("MessageInvoice", "invoice"),
            ("MessageSuccessfulPayment", "successful_payment"),
        ];

        for (type_name, main_field) in content_type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                let field = ty.fields.iter().find(|f| f.name == *main_field);

                assert!(
                    field.is_some(),
                    "{type_name} should have {main_field} field",
                );
                assert!(
                    field.unwrap().required,
                    "{main_field} field should be required in {type_name}",
                );
            }
        }

        let service_type_main_fields = [
            ("MessageNewChatMembers", "new_chat_members"),
            ("MessageLeftChatMember", "left_chat_member"),
            ("MessagePinnedMessage", "pinned_message"),
        ];

        for (type_name, main_field) in service_type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                let field = ty.fields.iter().find(|f| f.name == *main_field);

                assert!(
                    field.is_some(),
                    "{type_name} should have {main_field} field",
                );
                assert!(
                    field.unwrap().required,
                    "{main_field} field should be required in {type_name}",
                );
            }
        }

        let all_main_fields: Vec<&str> = content_type_main_fields.iter().map(|(_, f)| *f).collect();
        for (type_name, main_field) in content_type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                for other_field in all_main_fields.iter() {
                    if *other_field != *main_field {
                        assert!(
                            !ty.fields.iter().any(|f| f.name == *other_field),
                            "{type_name} should not contain {other_field} field",
                        );
                    }
                }
            }
        }

        let common_fields = ["message_id", "date", "chat", "from"];
        let types_to_check = ["MessageText", "MessagePhoto", "MessageNewChatMembers"];

        for type_name in types_to_check.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                for field_name in common_fields.iter() {
                    assert!(
                        ty.fields.iter().any(|f| f.name == *field_name),
                        "{type_name} should have common field {field_name}",
                    );
                }
            }
        }
    }

    #[test]
    fn test_split_chat_types_creates_new_types() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Chat": {
                        "name": "Chat",
                        "href": "https://core.telegram.org/bots/api#chat",
                        "description": [
                            "This object represents a chat."
                        ],
                        "fields": [
                            {
                            "name": "id",
                            "types": [
                                "Integer"
                            ],
                            "required": true,
                            "description": "Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier."
                            },
                            {
                            "name": "type",
                            "types": [
                                "String"
                            ],
                            "required": true,
                            "description": "Type of the chat, can be either \"private\", \"group\", \"supergroup\" or \"channel\""
                            },
                            {
                            "name": "title",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Title, for supergroups, channels and group chats"
                            },
                            {
                            "name": "username",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Username, for private chats, supergroups and channels if available"
                            },
                            {
                            "name": "first_name",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. First name of the other party in a private chat"
                            },
                            {
                            "name": "last_name",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Last name of the other party in a private chat"
                            },
                            {
                            "name": "is_forum",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the supergroup chat is a forum (has topics enabled)"
                            },
                            {
                            "name": "is_direct_messages",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the chat is the direct messages chat of a channel"
                            }
                        ]
                        }
                }
            }
        "#;

        let schema = Schema::parse_from_json(content).unwrap();
        let mut normalized = schema.normalize();

        normalized.split_chat_type("Chat");

        let chat = normalized.types.get("Chat").unwrap();
        assert!(chat.fields.is_empty());
        assert!(chat.subtype_kind.is_some());
        assert_eq!(chat.subtypes.len(), 4);

        let expected_types = ["ChatPrivate", "ChatGroup", "ChatSupergroup", "ChatChannel"];

        for type_name in expected_types.iter() {
            assert!(
                normalized.types.contains_key(*type_name),
                "Type {} should exist",
                type_name
            );
        }

        for subtype in &chat.subtypes {
            let type_name = &subtype.ty_name;
            let ty = normalized.types.get(type_name).unwrap();

            assert!(
                normalized.types.contains_key(type_name),
                "Subtype {type_name} should exist"
            );
            assert_eq!(
                ty.subtype_of,
                vec!["Chat"],
                "{type_name} should be subtype of Chat",
            );
            assert!(!ty.fields.is_empty(), "{} should have fields", type_name);
        }

        let type_main_fields = [
            ("ChatPrivate", "type", "private"),
            ("ChatGroup", "type", "group"),
            ("ChatSupergroup", "type", "supergroup"),
            ("ChatChannel", "type", "channel"),
        ];

        for (type_name, main_field, _) in type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                let field = ty.fields.iter().find(|f| f.name == *main_field);

                assert!(
                    field.is_some(),
                    "{type_name} should have {main_field} field",
                );
                assert!(
                    field.unwrap().required,
                    "{main_field} field should be required in {type_name}",
                );
            }
        }

        let common_fields = ["id", "type"];
        let types_to_check = ["ChatPrivate", "ChatGroup", "ChatSupergroup", "ChatChannel"];

        for type_name in types_to_check.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                for field_name in common_fields.iter() {
                    assert!(
                        ty.fields.iter().any(|f| f.name == *field_name),
                        "{type_name} should have common field {field_name}",
                    );
                }
            }
        }
    }

    #[test]
    fn test_split_poll_media_types_creates_new_types() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "PollMedia": {
                        "name": "PollMedia",
                        "href": "https://core.telegram.org/bots/api#pollmedia",
                        "description": [
                            "At most one of the optional fields can be present in any given object."
                        ],
                        "fields": [
                            {
                            "name": "animation",
                            "types": ["Animation"],
                            "required": false,
                            "description": "Optional. Media is an animation, information about the animation"
                            },
                            {
                            "name": "live_photo",
                            "types": ["LivePhoto"],
                            "required": false,
                            "description": "Optional. Media is a live photo, information about the live photo"
                            },
                            {
                            "name": "photo",
                            "types": ["Array of PhotoSize"],
                            "required": false,
                            "description": "Optional. Media is a photo, available sizes of the photo"
                            }
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_json(content).unwrap();
        let mut normalized = schema.normalize();

        normalized.split_poll_media_types();

        let media = normalized.types.get("PollMedia").unwrap();
        assert!(media.fields.is_empty());
        assert!(matches!(media.subtype_kind, Some(SubtypeKind::Untagged)));
        assert_eq!(media.subtypes.len(), 3);

        let expected = [
            ("PollMediaAnimation", "animation"),
            ("PollMediaLivePhoto", "live_photo"),
            ("PollMediaPhoto", "photo"),
        ];

        for (type_name, field_name) in expected {
            let ty = normalized
                .types
                .get(type_name)
                .unwrap_or_else(|| panic!("Subtype {type_name} should exist"));
            assert_eq!(ty.subtype_of, vec!["PollMedia"]);
            let field = ty
                .fields
                .iter()
                .find(|f| f.name == field_name)
                .unwrap_or_else(|| panic!("{type_name} should have {field_name} field"));
            assert!(
                field.required,
                "{field_name} should be required in {type_name}"
            );
            assert_eq!(
                ty.fields.len(),
                1,
                "{type_name} should have only its own field"
            );
        }
    }

    // #[test]
    // fn test_create_enums_from_fields() {
    //     let content = r#"
    //         {
    //             "version": "1.0",
    //             "release_date": "2021-01-01",
    //             "changelog": "Initial release",
    //             "types": {

    //             }
    //         }
    //     "#;

    //     let schema = Schema::parse_from_json(content).unwrap();
    //     let mut normalized = schema.normalize();

    //     normalized.create_enums_from_fields();
    // }
}
