#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) date: i64,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    date: Option<i64>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 1;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "date" => Some(0),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"date" => Some(0),
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
            0 => read_map_field(&mut self.date, map, "date"),
            _ => unreachable!(),
        }
    }
}

const fn channel_field_names(variant_fields: &[&'static str; 3]) -> [&'static str; 4] {
    [
        "date",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
    ]
}

pub(crate) struct ChannelFields {
    pub(crate) common: CommonFields,
    pub(crate) chat: Box<crate::types::Chat>,
    pub(crate) message_id: i64,
    pub(crate) author_signature: Option<Box<str>>,
}

struct ChannelVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for ChannelVisitor {
    type Value = ChannelFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_date = read_seq_field(&mut seq, 0, &expected)?;
        let variant_chat = read_seq_field(&mut seq, 1, &expected)?;
        let variant_message_id = read_seq_field(&mut seq, 2, &expected)?;
        let variant_author_signature = read_seq_field(&mut seq, 3, &expected)?;
        Ok(ChannelFields {
            common: CommonFields {
                date: common_date,
            },
            chat: variant_chat,
            message_id: variant_message_id,
            author_signature: variant_author_signature,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_chat: Option<Box<crate::types::Chat>> = None;
        let mut variant_message_id: Option<i64> = None;
        let mut variant_author_signature: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(&mut variant_chat, &mut map, self.metadata.variant_fields[0])?;
                }
                2 => {
                    read_map_field(
                        &mut variant_message_id,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_author_signature,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_date = shared.date.ok_or_else(|| A::Error::missing_field("date"))?;
        let variant_chat =
            variant_chat.ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_message_id = variant_message_id
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_author_signature = variant_author_signature.unwrap_or_default();
        Ok(ChannelFields {
            common: CommonFields {
                date: common_date,
            },
            chat: variant_chat,
            message_id: variant_message_id,
            author_signature: variant_author_signature,
        })
    }
}

pub(crate) fn deserialize_channel<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ChannelFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ChannelVisitor {
            metadata,
        },
    )
}

const fn chat_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 3] {
    ["date", variant_fields[0], variant_fields[1]]
}

pub(crate) struct ChatFields {
    pub(crate) common: CommonFields,
    pub(crate) sender_chat: Box<crate::types::Chat>,
    pub(crate) author_signature: Option<Box<str>>,
}

struct ChatVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for ChatVisitor {
    type Value = ChatFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_date = read_seq_field(&mut seq, 0, &expected)?;
        let variant_sender_chat = read_seq_field(&mut seq, 1, &expected)?;
        let variant_author_signature = read_seq_field(&mut seq, 2, &expected)?;
        Ok(ChatFields {
            common: CommonFields {
                date: common_date,
            },
            sender_chat: variant_sender_chat,
            author_signature: variant_author_signature,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_sender_chat: Option<Box<crate::types::Chat>> = None;
        let mut variant_author_signature: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_sender_chat,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                2 => {
                    read_map_field(
                        &mut variant_author_signature,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_date = shared.date.ok_or_else(|| A::Error::missing_field("date"))?;
        let variant_sender_chat = variant_sender_chat
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_author_signature = variant_author_signature.unwrap_or_default();
        Ok(ChatFields {
            common: CommonFields {
                date: common_date,
            },
            sender_chat: variant_sender_chat,
            author_signature: variant_author_signature,
        })
    }
}

pub(crate) fn deserialize_chat<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ChatFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ChatVisitor {
            metadata,
        },
    )
}

const fn content_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 2] {
    ["date", variant_fields[0]]
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
        let common_date = read_seq_field(&mut seq, 0, &expected)?;
        let variant_content = read_seq_field(&mut seq, 1, &expected)?;
        Ok(ContentFields {
            common: CommonFields {
                date: common_date,
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
                1 => {
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
        let common_date = shared.date.ok_or_else(|| A::Error::missing_field("date"))?;
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(ContentFields {
            common: CommonFields {
                date: common_date,
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

pub(crate) static CHANNEL: Metadata = Metadata {
    name: "MessageOriginChannel",
    fields: &channel_field_names(&["chat", "message_id", "author_signature"]),
    variant_fields: &["chat", "message_id", "author_signature"],
};

pub(crate) static CHAT: Metadata = Metadata {
    name: "MessageOriginChat",
    fields: &chat_field_names(&["sender_chat", "author_signature"]),
    variant_fields: &["sender_chat", "author_signature"],
};

pub(crate) static HIDDEN_USER: Metadata = Metadata {
    name: "MessageOriginHiddenUser",
    fields: &content_field_names(&["sender_user_name"]),
    variant_fields: &["sender_user_name"],
};

pub(crate) static USER: Metadata = Metadata {
    name: "MessageOriginUser",
    fields: &content_field_names(&["sender_user"]),
    variant_fields: &["sender_user"],
};
