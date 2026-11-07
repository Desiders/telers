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
    id: Option<Box<str>>,
    input_message_content: Option<Option<crate::types::InputMessageContent>>,
    parse_mode: Option<Option<Box<str>>>,
    reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
    show_caption_above_media: Option<Option<bool>>,
    title: Option<Option<Box<str>>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 8;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "caption" => Some(0),
            "caption_entities" => Some(1),
            "id" => Some(2),
            "input_message_content" => Some(3),
            "parse_mode" => Some(4),
            "reply_markup" => Some(5),
            "show_caption_above_media" => Some(6),
            "title" => Some(7),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"caption" => Some(0),
            b"caption_entities" => Some(1),
            b"id" => Some(2),
            b"input_message_content" => Some(3),
            b"parse_mode" => Some(4),
            b"reply_markup" => Some(5),
            b"show_caption_above_media" => Some(6),
            b"title" => Some(7),
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
            2 => read_map_field(&mut self.id, map, "id"),
            3 => read_map_field(
                &mut self.input_message_content,
                map,
                "input_message_content",
            ),
            4 => read_map_field(&mut self.parse_mode, map, "parse_mode"),
            5 => read_map_field(&mut self.reply_markup, map, "reply_markup"),
            6 => read_map_field(
                &mut self.show_caption_above_media,
                map,
                "show_caption_above_media",
            ),
            7 => read_map_field(&mut self.title, map, "title"),
            _ => unreachable!(),
        }
    }
}

const fn cached_mpeg4_gif_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 9] {
    [
        "id",
        variant_fields[0],
        "title",
        "caption",
        "parse_mode",
        "caption_entities",
        "show_caption_above_media",
        "reply_markup",
        "input_message_content",
    ]
}

pub(crate) struct CachedMpeg4GifFields {
    pub(crate) common: CommonFields,
    pub(crate) mpeg4_file_id: Box<str>,
}

struct CachedMpeg4GifVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for CachedMpeg4GifVisitor {
    type Value = CachedMpeg4GifFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_mpeg4_file_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_title = read_seq_field(&mut seq, 2, &expected)?;
        let common_caption = read_seq_field(&mut seq, 3, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 4, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 5, &expected)?;
        let common_show_caption_above_media = read_seq_field(&mut seq, 6, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 7, &expected)?;
        let common_input_message_content = read_seq_field(&mut seq, 8, &expected)?;
        Ok(CachedMpeg4GifFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            mpeg4_file_id: variant_mpeg4_file_id,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_mpeg4_file_id: Option<Box<str>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                8 => {
                    read_map_field(
                        &mut variant_mpeg4_file_id,
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
        let variant_mpeg4_file_id = variant_mpeg4_file_id
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_title = shared.title.unwrap_or(None);
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let common_show_caption_above_media = shared.show_caption_above_media.unwrap_or(None);
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let common_input_message_content = shared.input_message_content.unwrap_or(None);
        Ok(CachedMpeg4GifFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            mpeg4_file_id: variant_mpeg4_file_id,
        })
    }
}

pub(crate) fn deserialize_cached_mpeg4_gif<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<CachedMpeg4GifFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        CachedMpeg4GifVisitor {
            metadata,
        },
    )
}

const fn mpeg4_gif_field_names(variant_fields: &[&'static str; 6]) -> [&'static str; 14] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        "title",
        "caption",
        "parse_mode",
        "caption_entities",
        "show_caption_above_media",
        "reply_markup",
        "input_message_content",
    ]
}

pub(crate) struct Mpeg4GifFields {
    pub(crate) common: CommonFields,
    pub(crate) mpeg4_url: Box<str>,
    pub(crate) mpeg4_width: Option<i64>,
    pub(crate) mpeg4_height: Option<i64>,
    pub(crate) mpeg4_duration: Option<i64>,
    pub(crate) thumbnail_url: Box<str>,
    pub(crate) thumbnail_mime_type: Option<Box<str>>,
}

struct Mpeg4GifVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for Mpeg4GifVisitor {
    type Value = Mpeg4GifFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_mpeg4_url = read_seq_field(&mut seq, 1, &expected)?;
        let variant_mpeg4_width = read_seq_field(&mut seq, 2, &expected)?;
        let variant_mpeg4_height = read_seq_field(&mut seq, 3, &expected)?;
        let variant_mpeg4_duration = read_seq_field(&mut seq, 4, &expected)?;
        let variant_thumbnail_url = read_seq_field(&mut seq, 5, &expected)?;
        let variant_thumbnail_mime_type = read_seq_field(&mut seq, 6, &expected)?;
        let common_title = read_seq_field(&mut seq, 7, &expected)?;
        let common_caption = read_seq_field(&mut seq, 8, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 9, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 10, &expected)?;
        let common_show_caption_above_media = read_seq_field(&mut seq, 11, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 12, &expected)?;
        let common_input_message_content = read_seq_field(&mut seq, 13, &expected)?;
        Ok(Mpeg4GifFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            mpeg4_url: variant_mpeg4_url,
            mpeg4_width: variant_mpeg4_width,
            mpeg4_height: variant_mpeg4_height,
            mpeg4_duration: variant_mpeg4_duration,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_mime_type: variant_thumbnail_mime_type,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_mpeg4_url: Option<Box<str>> = None;
        let mut variant_mpeg4_width: Option<Option<i64>> = None;
        let mut variant_mpeg4_height: Option<Option<i64>> = None;
        let mut variant_mpeg4_duration: Option<Option<i64>> = None;
        let mut variant_thumbnail_url: Option<Box<str>> = None;
        let mut variant_thumbnail_mime_type: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                8 => {
                    read_map_field(
                        &mut variant_mpeg4_url,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_mpeg4_width,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_mpeg4_height,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_mpeg4_duration,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_thumbnail_url,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                13 => {
                    read_map_field(
                        &mut variant_thumbnail_mime_type,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_mpeg4_url = variant_mpeg4_url
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_mpeg4_width = variant_mpeg4_width.unwrap_or_default();
        let variant_mpeg4_height = variant_mpeg4_height.unwrap_or_default();
        let variant_mpeg4_duration = variant_mpeg4_duration.unwrap_or_default();
        let variant_thumbnail_url = variant_thumbnail_url
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[4]))?;
        let variant_thumbnail_mime_type = variant_thumbnail_mime_type.unwrap_or_default();
        let common_title = shared.title.unwrap_or(None);
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let common_show_caption_above_media = shared.show_caption_above_media.unwrap_or(None);
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let common_input_message_content = shared.input_message_content.unwrap_or(None);
        Ok(Mpeg4GifFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
                show_caption_above_media: common_show_caption_above_media,
                title: common_title,
            },
            mpeg4_url: variant_mpeg4_url,
            mpeg4_width: variant_mpeg4_width,
            mpeg4_height: variant_mpeg4_height,
            mpeg4_duration: variant_mpeg4_duration,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_mime_type: variant_thumbnail_mime_type,
        })
    }
}

pub(crate) fn deserialize_mpeg4_gif<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<Mpeg4GifFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        Mpeg4GifVisitor {
            metadata,
        },
    )
}

pub(crate) static CACHED_MPEG4_GIF: Metadata = Metadata {
    name: "InlineQueryResultCachedMpeg4Gif",
    fields: &cached_mpeg4_gif_field_names(&["mpeg4_file_id"]),
    variant_fields: &["mpeg4_file_id"],
};

pub(crate) static MPEG4_GIF: Metadata = Metadata {
    name: "InlineQueryResultMpeg4Gif",
    fields: &mpeg4_gif_field_names(&[
        "mpeg4_url",
        "mpeg4_width",
        "mpeg4_height",
        "mpeg4_duration",
        "thumbnail_url",
        "thumbnail_mime_type",
    ]),
    variant_fields: &[
        "mpeg4_url",
        "mpeg4_width",
        "mpeg4_height",
        "mpeg4_duration",
        "thumbnail_url",
        "thumbnail_mime_type",
    ],
};
