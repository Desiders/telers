#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) length: i64,
    pub(crate) offset: i64,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    length: Option<i64>,
    offset: Option<i64>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 2;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "length" => Some(0),
            "offset" => Some(1),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"length" => Some(0),
            b"offset" => Some(1),
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
            0 => read_map_field(&mut self.length, map, "length"),
            1 => read_map_field(&mut self.offset, map, "offset"),
            _ => unreachable!(),
        }
    }
}

const COMMON_FIELD_NAMES: [&str; 2] = ["offset", "length"];

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
        let common_offset = read_seq_field(&mut seq, 0, &expected)?;
        let common_length = read_seq_field(&mut seq, 1, &expected)?;
        Ok(CommonFields {
            length: common_length,
            offset: common_offset,
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
        let common_offset = shared
            .offset
            .ok_or_else(|| A::Error::missing_field("offset"))?;
        let common_length = shared
            .length
            .ok_or_else(|| A::Error::missing_field("length"))?;
        Ok(CommonFields {
            length: common_length,
            offset: common_offset,
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

const fn content_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 3] {
    ["offset", "length", variant_fields[0]]
}

pub(crate) struct ContentFields<TContent> {
    pub(crate) common: CommonFields,
    pub(crate) content: TContent,
}

struct ContentVisitor<TContent> {
    metadata: &'static Metadata,
    marker: PhantomData<ContentFields<TContent>>,
}

impl<'de, TContent> Visitor<'de> for ContentVisitor<TContent>
where
    TContent: Deserialize<'de>,
{
    type Value = ContentFields<TContent>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_offset = read_seq_field(&mut seq, 0, &expected)?;
        let common_length = read_seq_field(&mut seq, 1, &expected)?;
        let variant_content = read_seq_field(&mut seq, 2, &expected)?;
        Ok(ContentFields {
            common: CommonFields {
                length: common_length,
                offset: common_offset,
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
                2 => {
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
        let common_offset = shared
            .offset
            .ok_or_else(|| A::Error::missing_field("offset"))?;
        let common_length = shared
            .length
            .ok_or_else(|| A::Error::missing_field("length"))?;
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(ContentFields {
            common: CommonFields {
                length: common_length,
                offset: common_offset,
            },
            content: variant_content,
        })
    }
}

pub(crate) fn deserialize_content<'de, D, TContent>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ContentFields<TContent>, D::Error>
where
    D: Deserializer<'de>,
    TContent: Deserialize<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ContentVisitor {
            metadata,
            marker: PhantomData,
        },
    )
}

const fn date_time_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 4] {
    ["offset", "length", variant_fields[0], variant_fields[1]]
}

pub(crate) struct DateTimeFields {
    pub(crate) common: CommonFields,
    pub(crate) unix_time: i64,
    pub(crate) date_time_format: Option<Box<str>>,
}

struct DateTimeVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTimeFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_offset = read_seq_field(&mut seq, 0, &expected)?;
        let common_length = read_seq_field(&mut seq, 1, &expected)?;
        let variant_unix_time = read_seq_field(&mut seq, 2, &expected)?;
        let variant_date_time_format = read_seq_field(&mut seq, 3, &expected)?;
        Ok(DateTimeFields {
            common: CommonFields {
                length: common_length,
                offset: common_offset,
            },
            unix_time: variant_unix_time,
            date_time_format: variant_date_time_format,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_unix_time: Option<i64> = None;
        let mut variant_date_time_format: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_unix_time,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_date_time_format,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_offset = shared
            .offset
            .ok_or_else(|| A::Error::missing_field("offset"))?;
        let common_length = shared
            .length
            .ok_or_else(|| A::Error::missing_field("length"))?;
        let variant_unix_time = variant_unix_time
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_date_time_format = variant_date_time_format.unwrap_or_default();
        Ok(DateTimeFields {
            common: CommonFields {
                length: common_length,
                offset: common_offset,
            },
            unix_time: variant_unix_time,
            date_time_format: variant_date_time_format,
        })
    }
}

pub(crate) fn deserialize_date_time<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<DateTimeFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        DateTimeVisitor {
            metadata,
        },
    )
}

const fn pre_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 3] {
    ["offset", "length", variant_fields[0]]
}

pub(crate) struct PreFields {
    pub(crate) common: CommonFields,
    pub(crate) language: Option<Box<str>>,
}

struct PreVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for PreVisitor {
    type Value = PreFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_offset = read_seq_field(&mut seq, 0, &expected)?;
        let common_length = read_seq_field(&mut seq, 1, &expected)?;
        let variant_language = read_seq_field(&mut seq, 2, &expected)?;
        Ok(PreFields {
            common: CommonFields {
                length: common_length,
                offset: common_offset,
            },
            language: variant_language,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_language: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_language,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_offset = shared
            .offset
            .ok_or_else(|| A::Error::missing_field("offset"))?;
        let common_length = shared
            .length
            .ok_or_else(|| A::Error::missing_field("length"))?;
        let variant_language = variant_language.unwrap_or_default();
        Ok(PreFields {
            common: CommonFields {
                length: common_length,
                offset: common_offset,
            },
            language: variant_language,
        })
    }
}

pub(crate) fn deserialize_pre<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<PreFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        PreVisitor {
            metadata,
        },
    )
}

pub(crate) static BLOCKQUOTE: Metadata = Metadata {
    name: "MessageEntityBlockquote",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static BOLD: Metadata = Metadata {
    name: "MessageEntityBold",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static BOT_COMMAND: Metadata = Metadata {
    name: "MessageEntityBotCommand",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static CASHTAG: Metadata = Metadata {
    name: "MessageEntityCashtag",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static CODE: Metadata = Metadata {
    name: "MessageEntityCode",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static EMAIL: Metadata = Metadata {
    name: "MessageEntityEmail",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static EXPANDABLE_BLOCKQUOTE: Metadata = Metadata {
    name: "MessageEntityExpandableBlockquote",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static HASHTAG: Metadata = Metadata {
    name: "MessageEntityHashtag",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static ITALIC: Metadata = Metadata {
    name: "MessageEntityItalic",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static MENTION: Metadata = Metadata {
    name: "MessageEntityMention",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static PHONE_NUMBER: Metadata = Metadata {
    name: "MessageEntityPhoneNumber",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static SPOILER: Metadata = Metadata {
    name: "MessageEntitySpoiler",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static STRIKETHROUGH: Metadata = Metadata {
    name: "MessageEntityStrikethrough",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static UNDERLINE: Metadata = Metadata {
    name: "MessageEntityUnderline",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static URL: Metadata = Metadata {
    name: "MessageEntityUrl",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static CUSTOM_EMOJI: Metadata = Metadata {
    name: "MessageEntityCustomEmoji",
    fields: &content_field_names(&["custom_emoji_id"]),
    variant_fields: &["custom_emoji_id"],
};

pub(crate) static TEXT_LINK: Metadata = Metadata {
    name: "MessageEntityTextLink",
    fields: &content_field_names(&["url"]),
    variant_fields: &["url"],
};

pub(crate) static TEXT_MENTION: Metadata = Metadata {
    name: "MessageEntityTextMention",
    fields: &content_field_names(&["user"]),
    variant_fields: &["user"],
};

pub(crate) static DATE_TIME: Metadata = Metadata {
    name: "MessageEntityDateTime",
    fields: &date_time_field_names(&["unix_time", "date_time_format"]),
    variant_fields: &["unix_time", "date_time_format"],
};

pub(crate) static PRE: Metadata = Metadata {
    name: "MessageEntityPre",
    fields: &pre_field_names(&["language"]),
    variant_fields: &["language"],
};
