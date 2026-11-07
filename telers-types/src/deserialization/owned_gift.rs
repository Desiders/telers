#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) is_saved: Option<bool>,
    pub(crate) owned_gift_id: Option<Box<str>>,
    pub(crate) send_date: i64,
    pub(crate) sender_user: Option<Box<crate::types::User>>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    is_saved: Option<Option<bool>>,
    owned_gift_id: Option<Option<Box<str>>>,
    send_date: Option<i64>,
    sender_user: Option<Option<Box<crate::types::User>>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 4;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "is_saved" => Some(0),
            "owned_gift_id" => Some(1),
            "send_date" => Some(2),
            "sender_user" => Some(3),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"is_saved" => Some(0),
            b"owned_gift_id" => Some(1),
            b"send_date" => Some(2),
            b"sender_user" => Some(3),
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
            0 => read_map_field(&mut self.is_saved, map, "is_saved"),
            1 => read_map_field(&mut self.owned_gift_id, map, "owned_gift_id"),
            2 => read_map_field(&mut self.send_date, map, "send_date"),
            3 => read_map_field(&mut self.sender_user, map, "sender_user"),
            _ => unreachable!(),
        }
    }
}

const fn regular_field_names(variant_fields: &[&'static str; 10]) -> [&'static str; 14] {
    [
        variant_fields[0],
        "owned_gift_id",
        "sender_user",
        "send_date",
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        "is_saved",
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
        variant_fields[8],
        variant_fields[9],
    ]
}

pub(crate) struct RegularFields {
    pub(crate) common: CommonFields,
    pub(crate) gift: Box<crate::types::Gift>,
    pub(crate) text: Option<Box<str>>,
    pub(crate) entities: Option<Box<[crate::types::MessageEntity]>>,
    pub(crate) is_private: Option<bool>,
    pub(crate) can_be_upgraded: Option<bool>,
    pub(crate) was_refunded: Option<bool>,
    pub(crate) convert_star_count: Option<i64>,
    pub(crate) prepaid_upgrade_star_count: Option<i64>,
    pub(crate) is_upgrade_separate: Option<bool>,
    pub(crate) unique_gift_number: Option<i64>,
}

struct RegularVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for RegularVisitor {
    type Value = RegularFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_gift = read_seq_field(&mut seq, 0, &expected)?;
        let common_owned_gift_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_sender_user = read_seq_field(&mut seq, 2, &expected)?;
        let common_send_date = read_seq_field(&mut seq, 3, &expected)?;
        let variant_text = read_seq_field(&mut seq, 4, &expected)?;
        let variant_entities = read_seq_field(&mut seq, 5, &expected)?;
        let variant_is_private = read_seq_field(&mut seq, 6, &expected)?;
        let common_is_saved = read_seq_field(&mut seq, 7, &expected)?;
        let variant_can_be_upgraded = read_seq_field(&mut seq, 8, &expected)?;
        let variant_was_refunded = read_seq_field(&mut seq, 9, &expected)?;
        let variant_convert_star_count = read_seq_field(&mut seq, 10, &expected)?;
        let variant_prepaid_upgrade_star_count = read_seq_field(&mut seq, 11, &expected)?;
        let variant_is_upgrade_separate = read_seq_field(&mut seq, 12, &expected)?;
        let variant_unique_gift_number = read_seq_field(&mut seq, 13, &expected)?;
        Ok(RegularFields {
            common: CommonFields {
                is_saved: common_is_saved,
                owned_gift_id: common_owned_gift_id,
                send_date: common_send_date,
                sender_user: common_sender_user,
            },
            gift: variant_gift,
            text: variant_text,
            entities: variant_entities,
            is_private: variant_is_private,
            can_be_upgraded: variant_can_be_upgraded,
            was_refunded: variant_was_refunded,
            convert_star_count: variant_convert_star_count,
            prepaid_upgrade_star_count: variant_prepaid_upgrade_star_count,
            is_upgrade_separate: variant_is_upgrade_separate,
            unique_gift_number: variant_unique_gift_number,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_gift: Option<Box<crate::types::Gift>> = None;
        let mut variant_text: Option<Option<Box<str>>> = None;
        let mut variant_entities: Option<Option<Box<[crate::types::MessageEntity]>>> = None;
        let mut variant_is_private: Option<Option<bool>> = None;
        let mut variant_can_be_upgraded: Option<Option<bool>> = None;
        let mut variant_was_refunded: Option<Option<bool>> = None;
        let mut variant_convert_star_count: Option<Option<i64>> = None;
        let mut variant_prepaid_upgrade_star_count: Option<Option<i64>> = None;
        let mut variant_is_upgrade_separate: Option<Option<bool>> = None;
        let mut variant_unique_gift_number: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(&mut variant_gift, &mut map, self.metadata.variant_fields[0])?;
                }
                5 => {
                    read_map_field(&mut variant_text, &mut map, self.metadata.variant_fields[1])?;
                }
                6 => {
                    read_map_field(
                        &mut variant_entities,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_is_private,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_can_be_upgraded,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_was_refunded,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_convert_star_count,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_prepaid_upgrade_star_count,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_is_upgrade_separate,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                13 => {
                    read_map_field(
                        &mut variant_unique_gift_number,
                        &mut map,
                        self.metadata.variant_fields[9],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_gift =
            variant_gift.ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_owned_gift_id = shared.owned_gift_id.unwrap_or(None);
        let common_sender_user = shared.sender_user.unwrap_or(None);
        let common_send_date = shared
            .send_date
            .ok_or_else(|| A::Error::missing_field("send_date"))?;
        let variant_text = variant_text.unwrap_or_default();
        let variant_entities = variant_entities.unwrap_or_default();
        let variant_is_private = variant_is_private.unwrap_or_default();
        let common_is_saved = shared.is_saved.unwrap_or(None);
        let variant_can_be_upgraded = variant_can_be_upgraded.unwrap_or_default();
        let variant_was_refunded = variant_was_refunded.unwrap_or_default();
        let variant_convert_star_count = variant_convert_star_count.unwrap_or_default();
        let variant_prepaid_upgrade_star_count =
            variant_prepaid_upgrade_star_count.unwrap_or_default();
        let variant_is_upgrade_separate = variant_is_upgrade_separate.unwrap_or_default();
        let variant_unique_gift_number = variant_unique_gift_number.unwrap_or_default();
        Ok(RegularFields {
            common: CommonFields {
                is_saved: common_is_saved,
                owned_gift_id: common_owned_gift_id,
                send_date: common_send_date,
                sender_user: common_sender_user,
            },
            gift: variant_gift,
            text: variant_text,
            entities: variant_entities,
            is_private: variant_is_private,
            can_be_upgraded: variant_can_be_upgraded,
            was_refunded: variant_was_refunded,
            convert_star_count: variant_convert_star_count,
            prepaid_upgrade_star_count: variant_prepaid_upgrade_star_count,
            is_upgrade_separate: variant_is_upgrade_separate,
            unique_gift_number: variant_unique_gift_number,
        })
    }
}

pub(crate) fn deserialize_regular<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<RegularFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        RegularVisitor {
            metadata,
        },
    )
}

const fn unique_field_names(variant_fields: &[&'static str; 4]) -> [&'static str; 8] {
    [
        variant_fields[0],
        "owned_gift_id",
        "sender_user",
        "send_date",
        "is_saved",
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
    ]
}

pub(crate) struct UniqueFields {
    pub(crate) common: CommonFields,
    pub(crate) gift: Box<crate::types::UniqueGift>,
    pub(crate) can_be_transferred: Option<bool>,
    pub(crate) transfer_star_count: Option<i64>,
    pub(crate) next_transfer_date: Option<i64>,
}

struct UniqueVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for UniqueVisitor {
    type Value = UniqueFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_gift = read_seq_field(&mut seq, 0, &expected)?;
        let common_owned_gift_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_sender_user = read_seq_field(&mut seq, 2, &expected)?;
        let common_send_date = read_seq_field(&mut seq, 3, &expected)?;
        let common_is_saved = read_seq_field(&mut seq, 4, &expected)?;
        let variant_can_be_transferred = read_seq_field(&mut seq, 5, &expected)?;
        let variant_transfer_star_count = read_seq_field(&mut seq, 6, &expected)?;
        let variant_next_transfer_date = read_seq_field(&mut seq, 7, &expected)?;
        Ok(UniqueFields {
            common: CommonFields {
                is_saved: common_is_saved,
                owned_gift_id: common_owned_gift_id,
                send_date: common_send_date,
                sender_user: common_sender_user,
            },
            gift: variant_gift,
            can_be_transferred: variant_can_be_transferred,
            transfer_star_count: variant_transfer_star_count,
            next_transfer_date: variant_next_transfer_date,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_gift: Option<Box<crate::types::UniqueGift>> = None;
        let mut variant_can_be_transferred: Option<Option<bool>> = None;
        let mut variant_transfer_star_count: Option<Option<i64>> = None;
        let mut variant_next_transfer_date: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
                    read_map_field(&mut variant_gift, &mut map, self.metadata.variant_fields[0])?;
                }
                5 => {
                    read_map_field(
                        &mut variant_can_be_transferred,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_transfer_star_count,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_next_transfer_date,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_gift =
            variant_gift.ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_owned_gift_id = shared.owned_gift_id.unwrap_or(None);
        let common_sender_user = shared.sender_user.unwrap_or(None);
        let common_send_date = shared
            .send_date
            .ok_or_else(|| A::Error::missing_field("send_date"))?;
        let common_is_saved = shared.is_saved.unwrap_or(None);
        let variant_can_be_transferred = variant_can_be_transferred.unwrap_or_default();
        let variant_transfer_star_count = variant_transfer_star_count.unwrap_or_default();
        let variant_next_transfer_date = variant_next_transfer_date.unwrap_or_default();
        Ok(UniqueFields {
            common: CommonFields {
                is_saved: common_is_saved,
                owned_gift_id: common_owned_gift_id,
                send_date: common_send_date,
                sender_user: common_sender_user,
            },
            gift: variant_gift,
            can_be_transferred: variant_can_be_transferred,
            transfer_star_count: variant_transfer_star_count,
            next_transfer_date: variant_next_transfer_date,
        })
    }
}

pub(crate) fn deserialize_unique<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<UniqueFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        UniqueVisitor {
            metadata,
        },
    )
}

pub(crate) static REGULAR: Metadata = Metadata {
    name: "OwnedGiftRegular",
    fields: &regular_field_names(&[
        "gift",
        "text",
        "entities",
        "is_private",
        "can_be_upgraded",
        "was_refunded",
        "convert_star_count",
        "prepaid_upgrade_star_count",
        "is_upgrade_separate",
        "unique_gift_number",
    ]),
    variant_fields: &[
        "gift",
        "text",
        "entities",
        "is_private",
        "can_be_upgraded",
        "was_refunded",
        "convert_star_count",
        "prepaid_upgrade_star_count",
        "is_upgrade_separate",
        "unique_gift_number",
    ],
};

pub(crate) static UNIQUE: Metadata = Metadata {
    name: "OwnedGiftUnique",
    fields: &unique_field_names(&[
        "gift",
        "can_be_transferred",
        "transfer_star_count",
        "next_transfer_date",
    ]),
    variant_fields: &[
        "gift",
        "can_be_transferred",
        "transfer_star_count",
        "next_transfer_date",
    ],
};
