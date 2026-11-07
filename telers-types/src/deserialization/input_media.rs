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
    pub(crate) media: crate::types::InputFile,
    pub(crate) parse_mode: Option<Box<str>>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    caption: Option<Option<Box<str>>>,
    caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
    media: Option<crate::types::InputFile>,
    parse_mode: Option<Option<Box<str>>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 4;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "caption" => Some(0),
            "caption_entities" => Some(1),
            "media" => Some(2),
            "parse_mode" => Some(3),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"caption" => Some(0),
            b"caption_entities" => Some(1),
            b"media" => Some(2),
            b"parse_mode" => Some(3),
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
            2 => read_map_field(&mut self.media, map, "media"),
            3 => read_map_field(&mut self.parse_mode, map, "parse_mode"),
            _ => unreachable!(),
        }
    }
}

const fn animation_field_names(variant_fields: &[&'static str; 6]) -> [&'static str; 10] {
    [
        "media",
        variant_fields[0],
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
    ]
}

pub(crate) struct AnimationFields {
    pub(crate) common: CommonFields,
    pub(crate) thumbnail: Option<crate::types::InputFile>,
    pub(crate) show_caption_above_media: Option<bool>,
    pub(crate) width: Option<i64>,
    pub(crate) height: Option<i64>,
    pub(crate) duration: Option<i64>,
    pub(crate) has_spoiler: Option<bool>,
}

struct AnimationVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for AnimationVisitor {
    type Value = AnimationFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        let variant_thumbnail = read_seq_field(&mut seq, 1, &expected)?;
        let common_caption = read_seq_field(&mut seq, 2, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 3, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 4, &expected)?;
        let variant_show_caption_above_media = read_seq_field(&mut seq, 5, &expected)?;
        let variant_width = read_seq_field(&mut seq, 6, &expected)?;
        let variant_height = read_seq_field(&mut seq, 7, &expected)?;
        let variant_duration = read_seq_field(&mut seq, 8, &expected)?;
        let variant_has_spoiler = read_seq_field(&mut seq, 9, &expected)?;
        Ok(AnimationFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            show_caption_above_media: variant_show_caption_above_media,
            width: variant_width,
            height: variant_height,
            duration: variant_duration,
            has_spoiler: variant_has_spoiler,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_thumbnail: Option<Option<crate::types::InputFile>> = None;
        let mut variant_show_caption_above_media: Option<Option<bool>> = None;
        let mut variant_width: Option<Option<i64>> = None;
        let mut variant_height: Option<Option<i64>> = None;
        let mut variant_duration: Option<Option<i64>> = None;
        let mut variant_has_spoiler: Option<Option<bool>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(
                        &mut variant_thumbnail,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_show_caption_above_media,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_width,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_height,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_duration,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_has_spoiler,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        let variant_thumbnail = variant_thumbnail.unwrap_or_default();
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_show_caption_above_media = variant_show_caption_above_media.unwrap_or_default();
        let variant_width = variant_width.unwrap_or_default();
        let variant_height = variant_height.unwrap_or_default();
        let variant_duration = variant_duration.unwrap_or_default();
        let variant_has_spoiler = variant_has_spoiler.unwrap_or_default();
        Ok(AnimationFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            show_caption_above_media: variant_show_caption_above_media,
            width: variant_width,
            height: variant_height,
            duration: variant_duration,
            has_spoiler: variant_has_spoiler,
        })
    }
}

pub(crate) fn deserialize_animation<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<AnimationFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        AnimationVisitor {
            metadata,
        },
    )
}

const fn audio_field_names(variant_fields: &[&'static str; 4]) -> [&'static str; 8] {
    [
        "media",
        variant_fields[0],
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
    ]
}

pub(crate) struct AudioFields {
    pub(crate) common: CommonFields,
    pub(crate) thumbnail: Option<crate::types::InputFile>,
    pub(crate) duration: Option<i64>,
    pub(crate) performer: Option<Box<str>>,
    pub(crate) title: Option<Box<str>>,
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
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        let variant_thumbnail = read_seq_field(&mut seq, 1, &expected)?;
        let common_caption = read_seq_field(&mut seq, 2, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 3, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 4, &expected)?;
        let variant_duration = read_seq_field(&mut seq, 5, &expected)?;
        let variant_performer = read_seq_field(&mut seq, 6, &expected)?;
        let variant_title = read_seq_field(&mut seq, 7, &expected)?;
        Ok(AudioFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            duration: variant_duration,
            performer: variant_performer,
            title: variant_title,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_thumbnail: Option<Option<crate::types::InputFile>> = None;
        let mut variant_duration: Option<Option<i64>> = None;
        let mut variant_performer: Option<Option<Box<str>>> = None;
        let mut variant_title: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(
                        &mut variant_thumbnail,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_duration,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_performer,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        let variant_thumbnail = variant_thumbnail.unwrap_or_default();
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_duration = variant_duration.unwrap_or_default();
        let variant_performer = variant_performer.unwrap_or_default();
        let variant_title = variant_title.unwrap_or_default();
        Ok(AudioFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            duration: variant_duration,
            performer: variant_performer,
            title: variant_title,
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

const fn document_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 6] {
    [
        "media",
        variant_fields[0],
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[1],
    ]
}

pub(crate) struct DocumentFields {
    pub(crate) common: CommonFields,
    pub(crate) thumbnail: Option<crate::types::InputFile>,
    pub(crate) disable_content_type_detection: Option<bool>,
}

struct DocumentVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = DocumentFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        let variant_thumbnail = read_seq_field(&mut seq, 1, &expected)?;
        let common_caption = read_seq_field(&mut seq, 2, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 3, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 4, &expected)?;
        let variant_disable_content_type_detection = read_seq_field(&mut seq, 5, &expected)?;
        Ok(DocumentFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            disable_content_type_detection: variant_disable_content_type_detection,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_thumbnail: Option<Option<crate::types::InputFile>> = None;
        let mut variant_disable_content_type_detection: Option<Option<bool>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(
                        &mut variant_thumbnail,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_disable_content_type_detection,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        let variant_thumbnail = variant_thumbnail.unwrap_or_default();
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_disable_content_type_detection =
            variant_disable_content_type_detection.unwrap_or_default();
        Ok(DocumentFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            disable_content_type_detection: variant_disable_content_type_detection,
        })
    }
}

pub(crate) fn deserialize_document<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<DocumentFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        DocumentVisitor {
            metadata,
        },
    )
}

const fn live_photo_field_names(variant_fields: &[&'static str; 3]) -> [&'static str; 7] {
    [
        "media",
        variant_fields[0],
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[1],
        variant_fields[2],
    ]
}

pub(crate) struct LivePhotoFields {
    pub(crate) common: CommonFields,
    pub(crate) photo: crate::types::InputFile,
    pub(crate) show_caption_above_media: Option<bool>,
    pub(crate) has_spoiler: Option<bool>,
}

struct LivePhotoVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for LivePhotoVisitor {
    type Value = LivePhotoFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        let variant_photo = read_seq_field(&mut seq, 1, &expected)?;
        let common_caption = read_seq_field(&mut seq, 2, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 3, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 4, &expected)?;
        let variant_show_caption_above_media = read_seq_field(&mut seq, 5, &expected)?;
        let variant_has_spoiler = read_seq_field(&mut seq, 6, &expected)?;
        Ok(LivePhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            photo: variant_photo,
            show_caption_above_media: variant_show_caption_above_media,
            has_spoiler: variant_has_spoiler,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_photo: Option<crate::types::InputFile> = None;
        let mut variant_show_caption_above_media: Option<Option<bool>> = None;
        let mut variant_has_spoiler: Option<Option<bool>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(
                        &mut variant_photo,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_show_caption_above_media,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_has_spoiler,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        let variant_photo = variant_photo
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_show_caption_above_media = variant_show_caption_above_media.unwrap_or_default();
        let variant_has_spoiler = variant_has_spoiler.unwrap_or_default();
        Ok(LivePhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            photo: variant_photo,
            show_caption_above_media: variant_show_caption_above_media,
            has_spoiler: variant_has_spoiler,
        })
    }
}

pub(crate) fn deserialize_live_photo<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<LivePhotoFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        LivePhotoVisitor {
            metadata,
        },
    )
}

const fn photo_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 6] {
    [
        "media",
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[0],
        variant_fields[1],
    ]
}

pub(crate) struct PhotoFields {
    pub(crate) common: CommonFields,
    pub(crate) show_caption_above_media: Option<bool>,
    pub(crate) has_spoiler: Option<bool>,
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
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        let common_caption = read_seq_field(&mut seq, 1, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 2, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 3, &expected)?;
        let variant_show_caption_above_media = read_seq_field(&mut seq, 4, &expected)?;
        let variant_has_spoiler = read_seq_field(&mut seq, 5, &expected)?;
        Ok(PhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            show_caption_above_media: variant_show_caption_above_media,
            has_spoiler: variant_has_spoiler,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_show_caption_above_media: Option<Option<bool>> = None;
        let mut variant_has_spoiler: Option<Option<bool>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(
                        &mut variant_show_caption_above_media,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_has_spoiler,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_show_caption_above_media = variant_show_caption_above_media.unwrap_or_default();
        let variant_has_spoiler = variant_has_spoiler.unwrap_or_default();
        Ok(PhotoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            show_caption_above_media: variant_show_caption_above_media,
            has_spoiler: variant_has_spoiler,
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

const fn video_field_names(variant_fields: &[&'static str; 9]) -> [&'static str; 13] {
    [
        "media",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
        variant_fields[8],
    ]
}

pub(crate) struct VideoFields {
    pub(crate) common: CommonFields,
    pub(crate) thumbnail: Option<crate::types::InputFile>,
    pub(crate) cover: Option<crate::types::InputFile>,
    pub(crate) start_timestamp: Option<i64>,
    pub(crate) show_caption_above_media: Option<bool>,
    pub(crate) width: Option<i64>,
    pub(crate) height: Option<i64>,
    pub(crate) duration: Option<i64>,
    pub(crate) supports_streaming: Option<bool>,
    pub(crate) has_spoiler: Option<bool>,
}

struct VideoVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for VideoVisitor {
    type Value = VideoFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        let variant_thumbnail = read_seq_field(&mut seq, 1, &expected)?;
        let variant_cover = read_seq_field(&mut seq, 2, &expected)?;
        let variant_start_timestamp = read_seq_field(&mut seq, 3, &expected)?;
        let common_caption = read_seq_field(&mut seq, 4, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 5, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 6, &expected)?;
        let variant_show_caption_above_media = read_seq_field(&mut seq, 7, &expected)?;
        let variant_width = read_seq_field(&mut seq, 8, &expected)?;
        let variant_height = read_seq_field(&mut seq, 9, &expected)?;
        let variant_duration = read_seq_field(&mut seq, 10, &expected)?;
        let variant_supports_streaming = read_seq_field(&mut seq, 11, &expected)?;
        let variant_has_spoiler = read_seq_field(&mut seq, 12, &expected)?;
        Ok(VideoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            cover: variant_cover,
            start_timestamp: variant_start_timestamp,
            show_caption_above_media: variant_show_caption_above_media,
            width: variant_width,
            height: variant_height,
            duration: variant_duration,
            supports_streaming: variant_supports_streaming,
            has_spoiler: variant_has_spoiler,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_thumbnail: Option<Option<crate::types::InputFile>> = None;
        let mut variant_cover: Option<Option<crate::types::InputFile>> = None;
        let mut variant_start_timestamp: Option<Option<i64>> = None;
        let mut variant_show_caption_above_media: Option<Option<bool>> = None;
        let mut variant_width: Option<Option<i64>> = None;
        let mut variant_height: Option<Option<i64>> = None;
        let mut variant_duration: Option<Option<i64>> = None;
        let mut variant_supports_streaming: Option<Option<bool>> = None;
        let mut variant_has_spoiler: Option<Option<bool>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(
                        &mut variant_thumbnail,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_cover,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_start_timestamp,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_show_caption_above_media,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_width,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_height,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_duration,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_supports_streaming,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_has_spoiler,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        let variant_thumbnail = variant_thumbnail.unwrap_or_default();
        let variant_cover = variant_cover.unwrap_or_default();
        let variant_start_timestamp = variant_start_timestamp.unwrap_or_default();
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_show_caption_above_media = variant_show_caption_above_media.unwrap_or_default();
        let variant_width = variant_width.unwrap_or_default();
        let variant_height = variant_height.unwrap_or_default();
        let variant_duration = variant_duration.unwrap_or_default();
        let variant_supports_streaming = variant_supports_streaming.unwrap_or_default();
        let variant_has_spoiler = variant_has_spoiler.unwrap_or_default();
        Ok(VideoFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            thumbnail: variant_thumbnail,
            cover: variant_cover,
            start_timestamp: variant_start_timestamp,
            show_caption_above_media: variant_show_caption_above_media,
            width: variant_width,
            height: variant_height,
            duration: variant_duration,
            supports_streaming: variant_supports_streaming,
            has_spoiler: variant_has_spoiler,
        })
    }
}

pub(crate) fn deserialize_video<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<VideoFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        VideoVisitor {
            metadata,
        },
    )
}

const fn voice_note_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 5] {
    [
        "media",
        "caption",
        "parse_mode",
        "caption_entities",
        variant_fields[0],
    ]
}

pub(crate) struct VoiceNoteFields {
    pub(crate) common: CommonFields,
    pub(crate) duration: Option<i64>,
}

struct VoiceNoteVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for VoiceNoteVisitor {
    type Value = VoiceNoteFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        let common_caption = read_seq_field(&mut seq, 1, &expected)?;
        let common_parse_mode = read_seq_field(&mut seq, 2, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 3, &expected)?;
        let variant_duration = read_seq_field(&mut seq, 4, &expected)?;
        Ok(VoiceNoteFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            duration: variant_duration,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_duration: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(
                        &mut variant_duration,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        let common_caption = shared.caption.unwrap_or(None);
        let common_parse_mode = shared.parse_mode.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let variant_duration = variant_duration.unwrap_or_default();
        Ok(VoiceNoteFields {
            common: CommonFields {
                caption: common_caption,
                caption_entities: common_caption_entities,
                media: common_media,
                parse_mode: common_parse_mode,
            },
            duration: variant_duration,
        })
    }
}

pub(crate) fn deserialize_voice_note<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<VoiceNoteFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        VoiceNoteVisitor {
            metadata,
        },
    )
}

pub(crate) static ANIMATION: Metadata = Metadata {
    name: "InputMediaAnimation",
    fields: &animation_field_names(&[
        "thumbnail",
        "show_caption_above_media",
        "width",
        "height",
        "duration",
        "has_spoiler",
    ]),
    variant_fields: &[
        "thumbnail",
        "show_caption_above_media",
        "width",
        "height",
        "duration",
        "has_spoiler",
    ],
};

pub(crate) static AUDIO: Metadata = Metadata {
    name: "InputMediaAudio",
    fields: &audio_field_names(&["thumbnail", "duration", "performer", "title"]),
    variant_fields: &["thumbnail", "duration", "performer", "title"],
};

pub(crate) static DOCUMENT: Metadata = Metadata {
    name: "InputMediaDocument",
    fields: &document_field_names(&["thumbnail", "disable_content_type_detection"]),
    variant_fields: &["thumbnail", "disable_content_type_detection"],
};

pub(crate) static LIVE_PHOTO: Metadata = Metadata {
    name: "InputMediaLivePhoto",
    fields: &live_photo_field_names(&["photo", "show_caption_above_media", "has_spoiler"]),
    variant_fields: &["photo", "show_caption_above_media", "has_spoiler"],
};

pub(crate) static PHOTO: Metadata = Metadata {
    name: "InputMediaPhoto",
    fields: &photo_field_names(&["show_caption_above_media", "has_spoiler"]),
    variant_fields: &["show_caption_above_media", "has_spoiler"],
};

pub(crate) static VIDEO: Metadata = Metadata {
    name: "InputMediaVideo",
    fields: &video_field_names(&[
        "thumbnail",
        "cover",
        "start_timestamp",
        "show_caption_above_media",
        "width",
        "height",
        "duration",
        "supports_streaming",
        "has_spoiler",
    ]),
    variant_fields: &[
        "thumbnail",
        "cover",
        "start_timestamp",
        "show_caption_above_media",
        "width",
        "height",
        "duration",
        "supports_streaming",
        "has_spoiler",
    ],
};

pub(crate) static VOICE_NOTE: Metadata = Metadata {
    name: "InputMediaVoiceNote",
    fields: &voice_note_field_names(&["duration"]),
    variant_fields: &["duration"],
};
