#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) amount: i64,
    pub(crate) date: i64,
    pub(crate) id: Box<str>,
    pub(crate) nanostar_amount: Option<u32>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    amount: Option<i64>,
    date: Option<i64>,
    id: Option<Box<str>>,
    nanostar_amount: Option<Option<u32>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 4;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "amount" => Some(0),
            "date" => Some(1),
            "id" => Some(2),
            "nanostar_amount" => Some(3),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"amount" => Some(0),
            b"date" => Some(1),
            b"id" => Some(2),
            b"nanostar_amount" => Some(3),
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
            0 => read_map_field(&mut self.amount, map, "amount"),
            1 => read_map_field(&mut self.date, map, "date"),
            2 => read_map_field(&mut self.id, map, "id"),
            3 => read_map_field(&mut self.nanostar_amount, map, "nanostar_amount"),
            _ => unreachable!(),
        }
    }
}

const fn field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 5] {
    ["id", "amount", "nanostar_amount", "date", variant_fields[0]]
}

pub(crate) struct DecodedFields {
    pub(crate) common: CommonFields,
    pub(crate) content: crate::types::TransactionPartner,
}

struct FieldsVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for FieldsVisitor {
    type Value = DecodedFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let common_amount = read_seq_field(&mut seq, 1, &expected)?;
        let common_nanostar_amount = read_seq_field(&mut seq, 2, &expected)?;
        let common_date = read_seq_field(&mut seq, 3, &expected)?;
        let variant_content = read_seq_field(&mut seq, 4, &expected)?;
        Ok(DecodedFields {
            common: CommonFields {
                amount: common_amount,
                date: common_date,
                id: common_id,
                nanostar_amount: common_nanostar_amount,
            },
            content: variant_content,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_content: Option<crate::types::TransactionPartner> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                4 => {
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
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let common_amount = shared
            .amount
            .ok_or_else(|| A::Error::missing_field("amount"))?;
        let common_nanostar_amount = shared.nanostar_amount.unwrap_or(None);
        let common_date = shared.date.ok_or_else(|| A::Error::missing_field("date"))?;
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(DecodedFields {
            common: CommonFields {
                amount: common_amount,
                date: common_date,
                id: common_id,
                nanostar_amount: common_nanostar_amount,
            },
            content: variant_content,
        })
    }
}

pub(crate) fn deserialize<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<DecodedFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        FieldsVisitor {
            metadata,
        },
    )
}

pub(crate) static INCOMING: Metadata = Metadata {
    name: "StarTransactionIncoming",
    fields: &field_names(&["source"]),
    variant_fields: &["source"],
};

pub(crate) static OUTGOING: Metadata = Metadata {
    name: "StarTransactionOutgoing",
    fields: &field_names(&["receiver"]),
    variant_fields: &["receiver"],
};
