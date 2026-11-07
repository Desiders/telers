#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) message: Box<str>,
    pub(crate) r#type: Box<str>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    message: Option<Box<str>>,
    r#type: Option<Box<str>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 2;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "message" => Some(0),
            "type" => Some(1),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"message" => Some(0),
            b"type" => Some(1),
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
            0 => read_map_field(&mut self.message, map, "message"),
            1 => read_map_field(&mut self.r#type, map, "type"),
            _ => unreachable!(),
        }
    }
}

const fn data_field_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 4] {
    ["type", variant_fields[0], variant_fields[1], "message"]
}

pub(crate) struct DataFieldFields {
    pub(crate) common: CommonFields,
    pub(crate) field_name: Box<str>,
    pub(crate) data_hash: Box<str>,
}

struct DataFieldVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for DataFieldVisitor {
    type Value = DataFieldFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_type = read_seq_field(&mut seq, 0, &expected)?;
        let variant_field_name = read_seq_field(&mut seq, 1, &expected)?;
        let variant_data_hash = read_seq_field(&mut seq, 2, &expected)?;
        let common_message = read_seq_field(&mut seq, 3, &expected)?;
        Ok(DataFieldFields {
            common: CommonFields {
                message: common_message,
                r#type: common_type,
            },
            field_name: variant_field_name,
            data_hash: variant_data_hash,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_field_name: Option<Box<str>> = None;
        let mut variant_data_hash: Option<Box<str>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_field_name,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_data_hash,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_type = shared
            .r#type
            .ok_or_else(|| A::Error::missing_field("type"))?;
        let variant_field_name = variant_field_name
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_data_hash = variant_data_hash
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let common_message = shared
            .message
            .ok_or_else(|| A::Error::missing_field("message"))?;
        Ok(DataFieldFields {
            common: CommonFields {
                message: common_message,
                r#type: common_type,
            },
            field_name: variant_field_name,
            data_hash: variant_data_hash,
        })
    }
}

pub(crate) fn deserialize_data_field<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<DataFieldFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        DataFieldVisitor {
            metadata,
        },
    )
}

const fn content_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 3] {
    ["type", variant_fields[0], "message"]
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
        let common_type = read_seq_field(&mut seq, 0, &expected)?;
        let variant_content = read_seq_field(&mut seq, 1, &expected)?;
        let common_message = read_seq_field(&mut seq, 2, &expected)?;
        Ok(ContentFields {
            common: CommonFields {
                message: common_message,
                r#type: common_type,
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
        let common_type = shared
            .r#type
            .ok_or_else(|| A::Error::missing_field("type"))?;
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_message = shared
            .message
            .ok_or_else(|| A::Error::missing_field("message"))?;
        Ok(ContentFields {
            common: CommonFields {
                message: common_message,
                r#type: common_type,
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

pub(crate) static DATA_FIELD: Metadata = Metadata {
    name: "PassportElementErrorDataField",
    fields: &data_field_field_names(&["field_name", "data_hash"]),
    variant_fields: &["field_name", "data_hash"],
};

pub(crate) static FILE: Metadata = Metadata {
    name: "PassportElementErrorFile",
    fields: &content_field_names(&["file_hash"]),
    variant_fields: &["file_hash"],
};

pub(crate) static FILES: Metadata = Metadata {
    name: "PassportElementErrorFiles",
    fields: &content_field_names(&["file_hashes"]),
    variant_fields: &["file_hashes"],
};

pub(crate) static FRONT_SIDE: Metadata = Metadata {
    name: "PassportElementErrorFrontSide",
    fields: &content_field_names(&["file_hash"]),
    variant_fields: &["file_hash"],
};

pub(crate) static REVERSE_SIDE: Metadata = Metadata {
    name: "PassportElementErrorReverseSide",
    fields: &content_field_names(&["file_hash"]),
    variant_fields: &["file_hash"],
};

pub(crate) static SELFIE: Metadata = Metadata {
    name: "PassportElementErrorSelfie",
    fields: &content_field_names(&["file_hash"]),
    variant_fields: &["file_hash"],
};

pub(crate) static TRANSLATION_FILE: Metadata = Metadata {
    name: "PassportElementErrorTranslationFile",
    fields: &content_field_names(&["file_hash"]),
    variant_fields: &["file_hash"],
};

pub(crate) static TRANSLATION_FILES: Metadata = Metadata {
    name: "PassportElementErrorTranslationFiles",
    fields: &content_field_names(&["file_hashes"]),
    variant_fields: &["file_hashes"],
};

pub(crate) static UNSPECIFIED: Metadata = Metadata {
    name: "PassportElementErrorUnspecified",
    fields: &content_field_names(&["element_hash"]),
    variant_fields: &["element_hash"],
};
