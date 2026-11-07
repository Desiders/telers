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
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 6;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "caption" => Some(0),
            "caption_entities" => Some(1),
            "id" => Some(2),
            "input_message_content" => Some(3),
            "parse_mode" => Some(4),
            "reply_markup" => Some(5),
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
            _ => unreachable!(),
        }
    }
}

const fn audio_field_names(variant_fields: &[&'static str; 4]) -> [&'static str; 10] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[2],
        variant_fields[3],
        "reply_markup",
        "input_message_content",
    ]
}

pub(crate) struct AudioFields {
    pub(crate) common: CommonFields,
    pub(crate) audio_url: Box<str>,
    pub(crate) title: Box<str>,
    pub(crate) performer: Option<Box<str>>,
    pub(crate) audio_duration: Option<i64>,
}

struct AudioVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for AudioVisitor {
    type Value = AudioFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_audio_url = read_seq_field(&mut seq, 1, &expected)?;
        let variant_title = read_seq_field(&mut seq, 2, &expected)?;
        let common_caption = read_seq_field(&mut seq, 3, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 4, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 5, &expected)?;
        let variant_performer = read_seq_field(&mut seq, 6, &expected)?;
        let variant_audio_duration = read_seq_field(&mut seq, 7, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 8, &expected)?;
        let common_input_message_content = read_seq_field(&mut seq, 9, &expected)?;
        Ok(AudioFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
            },
            audio_url: variant_audio_url,
            title: variant_title,
            performer: variant_performer,
            audio_duration: variant_audio_duration,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_audio_url: Option<Box<str>> = None;
        let mut variant_title: Option<Box<str>> = None;
        let mut variant_performer: Option<Option<Box<str>>> = None;
        let mut variant_audio_duration: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                6 => {
                    read_map_field(
                        &mut variant_audio_url,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_performer,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_audio_duration,
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
        let variant_audio_url = variant_audio_url
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_title = variant_title
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_performer = variant_performer.unwrap_or_default();
        let variant_audio_duration = variant_audio_duration.unwrap_or_default();
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let common_input_message_content = shared.input_message_content.unwrap_or(None);
        Ok(AudioFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
            },
            audio_url: variant_audio_url,
            title: variant_title,
            performer: variant_performer,
            audio_duration: variant_audio_duration,
        })
    }
}

pub(crate) fn deserialize_audio<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<AudioFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        AudioVisitor {
            metadata,
        },
    )
}

const fn cached_audio_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 7] {
    [
        "id",
        variant_fields[0],
        "caption",
        "parse_mode",
        "caption_entities",
        "reply_markup",
        "input_message_content",
    ]
}

pub(crate) struct CachedAudioFields {
    pub(crate) common: CommonFields,
    pub(crate) audio_file_id: Box<str>,
}

struct CachedAudioVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for CachedAudioVisitor {
    type Value = CachedAudioFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_audio_file_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_caption = read_seq_field(&mut seq, 2, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 3, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 4, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 5, &expected)?;
        let common_input_message_content = read_seq_field(&mut seq, 6, &expected)?;
        Ok(CachedAudioFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
            },
            audio_file_id: variant_audio_file_id,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_audio_file_id: Option<Box<str>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                6 => {
                    read_map_field(
                        &mut variant_audio_file_id,
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
        let variant_audio_file_id = variant_audio_file_id
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let common_input_message_content = shared.input_message_content.unwrap_or(None);
        Ok(CachedAudioFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                id: common_id,
                input_message_content: common_input_message_content,
                parse_mode: common_parse_mode,
                reply_markup: common_reply_markup,
            },
            audio_file_id: variant_audio_file_id,
        })
    }
}

pub(crate) fn deserialize_cached_audio<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<CachedAudioFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        CachedAudioVisitor {
            metadata,
        },
    )
}

pub(crate) static AUDIO: Metadata = Metadata {
    name: "InlineQueryResultAudio",
    fields: &audio_field_names(&["audio_url", "title", "performer", "audio_duration"]),
    variant_fields: &["audio_url", "title", "performer", "audio_duration"],
};

pub(crate) static CACHED_AUDIO: Metadata = Metadata {
    name: "InlineQueryResultCachedAudio",
    fields: &cached_audio_field_names(&["audio_file_id"]),
    variant_fields: &["audio_file_id"],
};
