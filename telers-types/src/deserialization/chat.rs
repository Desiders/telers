#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) id: i64,
    pub(crate) is_direct_messages: Option<bool>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    id: Option<i64>,
    is_direct_messages: Option<Option<bool>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 2;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "id" => Some(0),
            "is_direct_messages" => Some(1),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"id" => Some(0),
            b"is_direct_messages" => Some(1),
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
            0 => read_map_field(&mut self.id, map, "id"),
            1 => read_map_field(&mut self.is_direct_messages, map, "is_direct_messages"),
            _ => unreachable!(),
        }
    }
}

const fn channel_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 4] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        "is_direct_messages",
    ]
}

pub(crate) struct ChannelFields {
    pub(crate) common: CommonFields,
    pub(crate) title: Option<Box<str>>,
    pub(crate) username: Option<Box<str>>,
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
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_title = read_seq_field(&mut seq, 1, &expected)?;
        let variant_username = read_seq_field(&mut seq, 2, &expected)?;
        let common_is_direct_messages = read_seq_field(&mut seq, 3, &expected)?;
        Ok(ChannelFields {
            common: CommonFields {
                id: common_id,
                is_direct_messages: common_is_direct_messages,
            },
            title: variant_title,
            username: variant_username,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_title: Option<Option<Box<str>>> = None;
        let mut variant_username: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_username,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_title = variant_title.unwrap_or_default();
        let variant_username = variant_username.unwrap_or_default();
        let common_is_direct_messages = shared.is_direct_messages.unwrap_or(None);
        Ok(ChannelFields {
            common: CommonFields {
                id: common_id,
                is_direct_messages: common_is_direct_messages,
            },
            title: variant_title,
            username: variant_username,
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

const fn group_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 3] {
    ["id", variant_fields[0], "is_direct_messages"]
}

pub(crate) struct GroupFields {
    pub(crate) common: CommonFields,
    pub(crate) title: Option<Box<str>>,
}

struct GroupVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for GroupVisitor {
    type Value = GroupFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_title = read_seq_field(&mut seq, 1, &expected)?;
        let common_is_direct_messages = read_seq_field(&mut seq, 2, &expected)?;
        Ok(GroupFields {
            common: CommonFields {
                id: common_id,
                is_direct_messages: common_is_direct_messages,
            },
            title: variant_title,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_title: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_title,
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
        let variant_title = variant_title.unwrap_or_default();
        let common_is_direct_messages = shared.is_direct_messages.unwrap_or(None);
        Ok(GroupFields {
            common: CommonFields {
                id: common_id,
                is_direct_messages: common_is_direct_messages,
            },
            title: variant_title,
        })
    }
}

pub(crate) fn deserialize_group<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<GroupFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        GroupVisitor {
            metadata,
        },
    )
}

const fn private_or_supergroup_field_names(
    variant_fields: &[&'static str; 3],
) -> [&'static str; 5] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        "is_direct_messages",
    ]
}

pub(crate) struct PrivateOrSupergroupFields<TField1, TField2, TField3> {
    pub(crate) common: CommonFields,
    pub(crate) field_1: TField1,
    pub(crate) field_2: TField2,
    pub(crate) field_3: TField3,
}

struct PrivateOrSupergroupVisitor<TField1, TField2, TField3> {
    metadata: &'static Metadata,
    marker: PhantomData<PrivateOrSupergroupFields<TField1, TField2, TField3>>,
}

impl<'de, TField1, TField2, TField3> Visitor<'de>
    for PrivateOrSupergroupVisitor<TField1, TField2, TField3>
where
    TField1: Deserialize<'de> + Default,
    TField2: Deserialize<'de> + Default,
    TField3: Deserialize<'de> + Default,
{
    type Value = PrivateOrSupergroupFields<TField1, TField2, TField3>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_field_1 = read_seq_field(&mut seq, 1, &expected)?;
        let variant_field_2 = read_seq_field(&mut seq, 2, &expected)?;
        let variant_field_3 = read_seq_field(&mut seq, 3, &expected)?;
        let common_is_direct_messages = read_seq_field(&mut seq, 4, &expected)?;
        Ok(PrivateOrSupergroupFields {
            common: CommonFields {
                id: common_id,
                is_direct_messages: common_is_direct_messages,
            },
            field_1: variant_field_1,
            field_2: variant_field_2,
            field_3: variant_field_3,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_field_1: Option<TField1> = None;
        let mut variant_field_2: Option<TField2> = None;
        let mut variant_field_3: Option<TField3> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_field_1,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_field_2,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_field_3,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_field_1 = variant_field_1.unwrap_or_default();
        let variant_field_2 = variant_field_2.unwrap_or_default();
        let variant_field_3 = variant_field_3.unwrap_or_default();
        let common_is_direct_messages = shared.is_direct_messages.unwrap_or(None);
        Ok(PrivateOrSupergroupFields {
            common: CommonFields {
                id: common_id,
                is_direct_messages: common_is_direct_messages,
            },
            field_1: variant_field_1,
            field_2: variant_field_2,
            field_3: variant_field_3,
        })
    }
}

pub(crate) fn deserialize_private_or_supergroup<'de, D, TField1, TField2, TField3>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<PrivateOrSupergroupFields<TField1, TField2, TField3>, D::Error>
where
    D: Deserializer<'de>,
    TField1: Deserialize<'de> + Default,
    TField2: Deserialize<'de> + Default,
    TField3: Deserialize<'de> + Default,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        PrivateOrSupergroupVisitor {
            metadata,
            marker: PhantomData,
        },
    )
}

pub(crate) static CHANNEL: Metadata = Metadata {
    name: "ChatChannel",
    fields: &channel_field_names(&["title", "username"]),
    variant_fields: &["title", "username"],
};

pub(crate) static GROUP: Metadata = Metadata {
    name: "ChatGroup",
    fields: &group_field_names(&["title"]),
    variant_fields: &["title"],
};

pub(crate) static PRIVATE: Metadata = Metadata {
    name: "ChatPrivate",
    fields: &private_or_supergroup_field_names(&["username", "first_name", "last_name"]),
    variant_fields: &["username", "first_name", "last_name"],
};

pub(crate) static SUPERGROUP: Metadata = Metadata {
    name: "ChatSupergroup",
    fields: &private_or_supergroup_field_names(&["title", "username", "is_forum"]),
    variant_fields: &["title", "username", "is_forum"],
};
