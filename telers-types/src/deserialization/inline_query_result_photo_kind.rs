#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) caption: Option<Box<str>>,
    pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    pub(crate) description: Option<Box<str>>,
    pub(crate) id: Box<str>,
    pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
    pub(crate) parse_mode: Option<Box<str>>,
    pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    pub(crate) show_caption_above_media: Option<bool>,
    pub(crate) title: Option<Box<str>>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    caption: Option<Option<Box<str>>>,
    caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
    description: Option<Option<Box<str>>>,
    id: Option<Box<str>>,
    input_message_content: Option<Option<crate::types::InputMessageContent>>,
    parse_mode: Option<Option<Box<str>>>,
    reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
    show_caption_above_media: Option<Option<bool>>,
    title: Option<Option<Box<str>>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 9;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "caption" => Some(0),
            "caption_entities" => Some(1),
            "description" => Some(2),
            "id" => Some(3),
            "input_message_content" => Some(4),
            "parse_mode" => Some(5),
            "reply_markup" => Some(6),
            "show_caption_above_media" => Some(7),
            "title" => Some(8),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"caption" => Some(0),
            b"caption_entities" => Some(1),
            b"description" => Some(2),
            b"id" => Some(3),
            b"input_message_content" => Some(4),
            b"parse_mode" => Some(5),
            b"reply_markup" => Some(6),
            b"show_caption_above_media" => Some(7),
            b"title" => Some(8),
            _ => None,
        }
    }
}

impl CommonSlots {
    fn read_field<'de, A: MapAccess<'de>>(
        &mut self,
        index: usize,
        map: &mut A,
    ) -> Result<(), A::Error> {
        match index {
            0 => read_map_field(&mut self.caption, map, "caption"),
            1 => read_map_field(&mut self.caption_entities, map, "caption_entities"),
            2 => read_map_field(&mut self.description, map, "description"),
            3 => read_map_field(&mut self.id, map, "id"),
            4 => read_map_field(
                &mut self.input_message_content,
                map,
                "input_message_content",
            ),
            5 => read_map_field(&mut self.parse_mode, map, "parse_mode"),
            6 => read_map_field(&mut self.reply_markup, map, "reply_markup"),
            7 => read_map_field(
                &mut self.show_caption_above_media,
                map,
                "show_caption_above_media",
            ),
            8 => read_map_field(&mut self.title, map, "title"),
            _ => unreachable!(),
        }
    }
}

const fn cached_photo_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 10] {
    [
        "id",
        variant_fields[0],
        "title",
        "description",
        "caption",
        "parse_mode",
        "caption_entities",
        "show_caption_above_media",
        "reply_markup",
        "input_message_content",
    ]
}

pub(crate) struct CachedPhotoFields {
    pub(crate) common: CommonFields,
    pub(crate) photo_file_id: Box<str>,
}

struct CachedPhotoVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for CachedPhotoVisitor {
    type Value = CachedPhotoFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_photo_file_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_title = read_seq_field(&mut seq, 2, &expected)?;
        let common_description = read_seq_field(&mut seq, 3, &expected)?;
        let common_caption = read_seq_field(&mut seq, 4, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 5, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 6, &expected)?;
        let common_show_caption_above_media = read_seq_field(&mut seq, 7, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 8, &expected)?;
        let common_input_message_content = read_seq_field(&mut seq, 9, &expected)?;
        Ok(CachedPhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                description: common_description,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            photo_file_id: variant_photo_file_id,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_photo_file_id: Option<Box<str>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                9 => {
                    read_map_field(
                        &mut variant_photo_file_id,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_photo_file_id = variant_photo_file_id
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_title = shared.title.unwrap_or(None);
        let common_description = shared.description.unwrap_or(None);
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let common_show_caption_above_media = shared.show_caption_above_media.unwrap_or(None);
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let common_input_message_content = shared.input_message_content.unwrap_or(None);
        Ok(CachedPhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                description: common_description,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            photo_file_id: variant_photo_file_id,
        })
    }
}

pub(crate) fn deserialize_cached_photo<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<CachedPhotoFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        CachedPhotoVisitor {
            metadata,
        },
    )
}

const fn photo_field_names(variant_fields: &[&'static str; 4]) -> [&'static str; 13] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        "title",
        "description",
        "caption",
        "parse_mode",
        "caption_entities",
        "show_caption_above_media",
        "reply_markup",
        "input_message_content",
    ]
}

pub(crate) struct PhotoFields {
    pub(crate) common: CommonFields,
    pub(crate) photo_url: Box<str>,
    pub(crate) thumbnail_url: Box<str>,
    pub(crate) photo_width: Option<i64>,
    pub(crate) photo_height: Option<i64>,
}

struct PhotoVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for PhotoVisitor {
    type Value = PhotoFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_photo_url = read_seq_field(&mut seq, 1, &expected)?;
        let variant_thumbnail_url = read_seq_field(&mut seq, 2, &expected)?;
        let variant_photo_width = read_seq_field(&mut seq, 3, &expected)?;
        let variant_photo_height = read_seq_field(&mut seq, 4, &expected)?;
        let common_title = read_seq_field(&mut seq, 5, &expected)?;
        let common_description = read_seq_field(&mut seq, 6, &expected)?;
        let common_caption = read_seq_field(&mut seq, 7, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 8, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 9, &expected)?;
        let common_show_caption_above_media = read_seq_field(&mut seq, 10, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 11, &expected)?;
        let common_input_message_content = read_seq_field(&mut seq, 12, &expected)?;
        Ok(PhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                description: common_description,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            photo_url: variant_photo_url,
            thumbnail_url: variant_thumbnail_url,
            photo_width: variant_photo_width,
            photo_height: variant_photo_height,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_photo_url: Option<Box<str>> = None;
        let mut variant_thumbnail_url: Option<Box<str>> = None;
        let mut variant_photo_width: Option<Option<i64>> = None;
        let mut variant_photo_height: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                9 => {
                    read_map_field(
                        &mut variant_photo_url,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_thumbnail_url,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_photo_width,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_photo_height,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_photo_url = variant_photo_url
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_thumbnail_url = variant_thumbnail_url
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_photo_width = variant_photo_width.unwrap_or_default();
        let variant_photo_height = variant_photo_height.unwrap_or_default();
        let common_title = shared.title.unwrap_or(None);
        let common_description = shared.description.unwrap_or(None);
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let common_show_caption_above_media = shared.show_caption_above_media.unwrap_or(None);
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let common_input_message_content = shared.input_message_content.unwrap_or(None);
        Ok(PhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                description: common_description,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            photo_url: variant_photo_url,
            thumbnail_url: variant_thumbnail_url,
            photo_width: variant_photo_width,
            photo_height: variant_photo_height,
        })
    }
}

pub(crate) fn deserialize_photo<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<PhotoFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        PhotoVisitor {
            metadata,
        },
    )
}

pub(crate) static CACHED_PHOTO: Metadata = Metadata {
    name: "InlineQueryResultCachedPhoto",
    fields: &cached_photo_field_names(&["photo_file_id"]),
    variant_fields: &["photo_file_id"],
};

pub(crate) static PHOTO: Metadata = Metadata {
    name: "InlineQueryResultPhoto",
    fields: &photo_field_names(&["photo_url", "thumbnail_url", "photo_width", "photo_height"]),
    variant_fields: &["photo_url", "thumbnail_url", "photo_width", "photo_height"],
};
