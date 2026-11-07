#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) emoji: Option<Box<str>>,
    pub(crate) file_id: Box<str>,
    pub(crate) file_size: Option<i64>,
    pub(crate) file_unique_id: Box<str>,
    pub(crate) height: i64,
    pub(crate) is_animated: bool,
    pub(crate) is_video: bool,
    pub(crate) needs_repainting: Option<bool>,
    pub(crate) set_name: Option<Box<str>>,
    pub(crate) thumbnail: Option<crate::types::PhotoSize>,
    pub(crate) width: i64,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    emoji: Option<Option<Box<str>>>,
    file_id: Option<Box<str>>,
    file_size: Option<Option<i64>>,
    file_unique_id: Option<Box<str>>,
    height: Option<i64>,
    is_animated: Option<bool>,
    is_video: Option<bool>,
    needs_repainting: Option<Option<bool>>,
    set_name: Option<Option<Box<str>>>,
    thumbnail: Option<Option<crate::types::PhotoSize>>,
    width: Option<i64>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 11;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "emoji" => Some(0),
            "file_id" => Some(1),
            "file_size" => Some(2),
            "file_unique_id" => Some(3),
            "height" => Some(4),
            "is_animated" => Some(5),
            "is_video" => Some(6),
            "needs_repainting" => Some(7),
            "set_name" => Some(8),
            "thumbnail" => Some(9),
            "width" => Some(10),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"emoji" => Some(0),
            b"file_id" => Some(1),
            b"file_size" => Some(2),
            b"file_unique_id" => Some(3),
            b"height" => Some(4),
            b"is_animated" => Some(5),
            b"is_video" => Some(6),
            b"needs_repainting" => Some(7),
            b"set_name" => Some(8),
            b"thumbnail" => Some(9),
            b"width" => Some(10),
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
            0 => read_map_field(&mut self.emoji, map, "emoji"),
            1 => read_map_field(&mut self.file_id, map, "file_id"),
            2 => read_map_field(&mut self.file_size, map, "file_size"),
            3 => read_map_field(&mut self.file_unique_id, map, "file_unique_id"),
            4 => read_map_field(&mut self.height, map, "height"),
            5 => read_map_field(&mut self.is_animated, map, "is_animated"),
            6 => read_map_field(&mut self.is_video, map, "is_video"),
            7 => read_map_field(&mut self.needs_repainting, map, "needs_repainting"),
            8 => read_map_field(&mut self.set_name, map, "set_name"),
            9 => read_map_field(&mut self.thumbnail, map, "thumbnail"),
            10 => read_map_field(&mut self.width, map, "width"),
            _ => unreachable!(),
        }
    }
}

const fn field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 12] {
    [
        "file_id",
        "file_unique_id",
        "width",
        "height",
        "is_animated",
        "is_video",
        "thumbnail",
        "emoji",
        "set_name",
        variant_fields[0],
        "needs_repainting",
        "file_size",
    ]
}

pub(crate) struct DecodedFields<TContent> {
    pub(crate) common: CommonFields,
    pub(crate) content: TContent,
}

struct FieldsVisitor<TContent> {
    metadata: &'static Metadata,
    marker: PhantomData<DecodedFields<TContent>>,
}

impl<'de, TContent> Visitor<'de> for FieldsVisitor<TContent>
where
    TContent: Deserialize<'de> + Default,
{
    type Value = DecodedFields<TContent>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_file_id = read_seq_field(&mut seq, 0, &expected)?;
        let common_file_unique_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_width = read_seq_field(&mut seq, 2, &expected)?;
        let common_height = read_seq_field(&mut seq, 3, &expected)?;
        let common_is_animated = read_seq_field(&mut seq, 4, &expected)?;
        let common_is_video = read_seq_field(&mut seq, 5, &expected)?;
        let common_thumbnail = read_seq_field(&mut seq, 6, &expected)?;
        let common_emoji = read_seq_field(&mut seq, 7, &expected)?;
        let common_set_name = read_seq_field(&mut seq, 8, &expected)?;
        let variant_content = read_seq_field(&mut seq, 9, &expected)?;
        let common_needs_repainting = read_seq_field(&mut seq, 10, &expected)?;
        let common_file_size = read_seq_field(&mut seq, 11, &expected)?;
        Ok(DecodedFields {
            common: CommonFields {
                emoji: common_emoji,
                file_id: common_file_id,
                file_size: common_file_size,
                file_unique_id: common_file_unique_id,
                height: common_height,
                is_animated: common_is_animated,
                is_video: common_is_video,
                needs_repainting: common_needs_repainting,
                set_name: common_set_name,
                thumbnail: common_thumbnail,
                width: common_width,
            },
            content: variant_content,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_content: Option<TContent> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                11 => {
                    read_map_field(
                        &mut variant_content,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_file_id = shared
            .file_id
            .ok_or_else(|| A::Error::missing_field("file_id"))?;
        let common_file_unique_id = shared
            .file_unique_id
            .ok_or_else(|| A::Error::missing_field("file_unique_id"))?;
        let common_width = shared
            .width
            .ok_or_else(|| A::Error::missing_field("width"))?;
        let common_height = shared
            .height
            .ok_or_else(|| A::Error::missing_field("height"))?;
        let common_is_animated = shared
            .is_animated
            .ok_or_else(|| A::Error::missing_field("is_animated"))?;
        let common_is_video = shared
            .is_video
            .ok_or_else(|| A::Error::missing_field("is_video"))?;
        let common_thumbnail = shared.thumbnail.unwrap_or(None);
        let common_emoji = shared.emoji.unwrap_or(None);
        let common_set_name = shared.set_name.unwrap_or(None);
        let variant_content = variant_content.unwrap_or_default();
        let common_needs_repainting = shared.needs_repainting.unwrap_or(None);
        let common_file_size = shared.file_size.unwrap_or(None);
        Ok(DecodedFields {
            common: CommonFields {
                emoji: common_emoji,
                file_id: common_file_id,
                file_size: common_file_size,
                file_unique_id: common_file_unique_id,
                height: common_height,
                is_animated: common_is_animated,
                is_video: common_is_video,
                needs_repainting: common_needs_repainting,
                set_name: common_set_name,
                thumbnail: common_thumbnail,
                width: common_width,
            },
            content: variant_content,
        })
    }
}

pub(crate) fn deserialize<'de, D, TContent>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<DecodedFields<TContent>, D::Error>
where
    D: Deserializer<'de>,
    TContent: Deserialize<'de> + Default,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        FieldsVisitor {
            metadata,
            marker: PhantomData,
        },
    )
}

pub(crate) static CUSTOM_EMOJI: Metadata = Metadata {
    name: "StickerCustomEmoji",
    fields: &field_names(&["custom_emoji_id"]),
    variant_fields: &["custom_emoji_id"],
};

pub(crate) static MASK: Metadata = Metadata {
    name: "StickerMask",
    fields: &field_names(&["mask_position"]),
    variant_fields: &["mask_position"],
};

pub(crate) static REGULAR: Metadata = Metadata {
    name: "StickerRegular",
    fields: &field_names(&["premium_animation"]),
    variant_fields: &["premium_animation"],
};
