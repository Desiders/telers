#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) media: crate::types::InputFile,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    media: Option<crate::types::InputFile>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 1;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "media" => Some(0),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"media" => Some(0),
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
            0 => read_map_field(&mut self.media, map, "media"),
            _ => unreachable!(),
        }
    }
}

const fn live_photo_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 2] {
    ["media", variant_fields[0]]
}

pub(crate) struct LivePhotoFields {
    pub(crate) common: CommonFields,
    pub(crate) photo: crate::types::InputFile,
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
        Ok(LivePhotoFields {
            common: CommonFields {
                media: common_media,
            },
            photo: variant_photo,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_photo: Option<crate::types::InputFile> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_photo,
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
        let variant_photo = variant_photo
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(LivePhotoFields {
            common: CommonFields {
                media: common_media,
            },
            photo: variant_photo,
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

const COMMON_FIELD_NAMES: [&str; 1] = ["media"];

struct CommonVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for CommonVisitor {
    type Value = CommonFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_media = read_seq_field(&mut seq, 0, &expected)?;
        Ok(CommonFields {
            media: common_media,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_media = shared
            .media
            .ok_or_else(|| A::Error::missing_field("media"))?;
        Ok(CommonFields {
            media: common_media,
        })
    }
}

pub(crate) fn deserialize_common<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<CommonFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        CommonVisitor {
            metadata,
        },
    )
}

const fn video_field_names(variant_fields: &[&'static str; 7]) -> [&'static str; 8] {
    [
        "media",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
    ]
}

pub(crate) struct VideoFields {
    pub(crate) common: CommonFields,
    pub(crate) thumbnail: Option<crate::types::InputFile>,
    pub(crate) cover: Option<crate::types::InputFile>,
    pub(crate) start_timestamp: Option<i64>,
    pub(crate) width: Option<i64>,
    pub(crate) height: Option<i64>,
    pub(crate) duration: Option<i64>,
    pub(crate) supports_streaming: Option<bool>,
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
        let variant_width = read_seq_field(&mut seq, 4, &expected)?;
        let variant_height = read_seq_field(&mut seq, 5, &expected)?;
        let variant_duration = read_seq_field(&mut seq, 6, &expected)?;
        let variant_supports_streaming = read_seq_field(&mut seq, 7, &expected)?;
        Ok(VideoFields {
            common: CommonFields {
                media: common_media,
            },
            thumbnail: variant_thumbnail,
            cover: variant_cover,
            start_timestamp: variant_start_timestamp,
            width: variant_width,
            height: variant_height,
            duration: variant_duration,
            supports_streaming: variant_supports_streaming,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_thumbnail: Option<Option<crate::types::InputFile>> = None;
        let mut variant_cover: Option<Option<crate::types::InputFile>> = None;
        let mut variant_start_timestamp: Option<Option<i64>> = None;
        let mut variant_width: Option<Option<i64>> = None;
        let mut variant_height: Option<Option<i64>> = None;
        let mut variant_duration: Option<Option<i64>> = None;
        let mut variant_supports_streaming: Option<Option<bool>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_thumbnail,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                2 => {
                    read_map_field(
                        &mut variant_cover,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_start_timestamp,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_width,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_height,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_duration,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_supports_streaming,
                        &mut map,
                        self.metadata.variant_fields[6],
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
        let variant_width = variant_width.unwrap_or_default();
        let variant_height = variant_height.unwrap_or_default();
        let variant_duration = variant_duration.unwrap_or_default();
        let variant_supports_streaming = variant_supports_streaming.unwrap_or_default();
        Ok(VideoFields {
            common: CommonFields {
                media: common_media,
            },
            thumbnail: variant_thumbnail,
            cover: variant_cover,
            start_timestamp: variant_start_timestamp,
            width: variant_width,
            height: variant_height,
            duration: variant_duration,
            supports_streaming: variant_supports_streaming,
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

pub(crate) static LIVE_PHOTO: Metadata = Metadata {
    name: "InputPaidMediaLivePhoto",
    fields: &live_photo_field_names(&["photo"]),
    variant_fields: &["photo"],
};

pub(crate) static PHOTO: Metadata = Metadata {
    name: "InputPaidMediaPhoto",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static VIDEO: Metadata = Metadata {
    name: "InputPaidMediaVideo",
    fields: &video_field_names(&[
        "thumbnail",
        "cover",
        "start_timestamp",
        "width",
        "height",
        "duration",
        "supports_streaming",
    ]),
    variant_fields: &[
        "thumbnail",
        "cover",
        "start_timestamp",
        "width",
        "height",
        "duration",
        "supports_streaming",
    ],
};
