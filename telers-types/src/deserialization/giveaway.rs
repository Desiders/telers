#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) chats: Box<[crate::types::Chat]>,
    pub(crate) country_codes: Option<Box<[Box<str>]>>,
    pub(crate) has_public_winners: Option<bool>,
    pub(crate) only_new_members: Option<bool>,
    pub(crate) prize_description: Option<Box<str>>,
    pub(crate) winner_count: i64,
    pub(crate) winners_selection_date: i64,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    chats: Option<Box<[crate::types::Chat]>>,
    country_codes: Option<Option<Box<[Box<str>]>>>,
    has_public_winners: Option<Option<bool>>,
    only_new_members: Option<Option<bool>>,
    prize_description: Option<Option<Box<str>>>,
    winner_count: Option<i64>,
    winners_selection_date: Option<i64>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 7;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "chats" => Some(0),
            "country_codes" => Some(1),
            "has_public_winners" => Some(2),
            "only_new_members" => Some(3),
            "prize_description" => Some(4),
            "winner_count" => Some(5),
            "winners_selection_date" => Some(6),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"chats" => Some(0),
            b"country_codes" => Some(1),
            b"has_public_winners" => Some(2),
            b"only_new_members" => Some(3),
            b"prize_description" => Some(4),
            b"winner_count" => Some(5),
            b"winners_selection_date" => Some(6),
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
            0 => read_map_field(&mut self.chats, map, "chats"),
            1 => read_map_field(&mut self.country_codes, map, "country_codes"),
            2 => read_map_field(&mut self.has_public_winners, map, "has_public_winners"),
            3 => read_map_field(&mut self.only_new_members, map, "only_new_members"),
            4 => read_map_field(&mut self.prize_description, map, "prize_description"),
            5 => read_map_field(&mut self.winner_count, map, "winner_count"),
            6 => read_map_field(
                &mut self.winners_selection_date,
                map,
                "winners_selection_date",
            ),
            _ => unreachable!(),
        }
    }
}

const fn field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 8] {
    [
        "chats",
        "winners_selection_date",
        "winner_count",
        "only_new_members",
        "has_public_winners",
        "prize_description",
        "country_codes",
        variant_fields[0],
    ]
}

pub(crate) struct DecodedFields {
    pub(crate) common: CommonFields,
    pub(crate) content: i64,
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
        let common_chats = read_seq_field(&mut seq, 0, &expected)?;
        let common_winners_selection_date = read_seq_field(&mut seq, 1, &expected)?;
        let common_winner_count = read_seq_field(&mut seq, 2, &expected)?;
        let common_only_new_members = read_seq_field(&mut seq, 3, &expected)?;
        let common_has_public_winners = read_seq_field(&mut seq, 4, &expected)?;
        let common_prize_description = read_seq_field(&mut seq, 5, &expected)?;
        let common_country_codes = read_seq_field(&mut seq, 6, &expected)?;
        let variant_content = read_seq_field(&mut seq, 7, &expected)?;
        Ok(DecodedFields {
            common: CommonFields {
                chats: common_chats,
                country_codes: common_country_codes,
                has_public_winners: common_has_public_winners,
                only_new_members: common_only_new_members,
                prize_description: common_prize_description,
                winner_count: common_winner_count,
                winners_selection_date: common_winners_selection_date,
            },
            content: variant_content,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_content: Option<i64> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                7 => {
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
        let common_chats = shared
            .chats
            .ok_or_else(|| A::Error::missing_field("chats"))?;
        let common_winners_selection_date = shared
            .winners_selection_date
            .ok_or_else(|| A::Error::missing_field("winners_selection_date"))?;
        let common_winner_count = shared
            .winner_count
            .ok_or_else(|| A::Error::missing_field("winner_count"))?;
        let common_only_new_members = shared.only_new_members.unwrap_or(None);
        let common_has_public_winners = shared.has_public_winners.unwrap_or(None);
        let common_prize_description = shared.prize_description.unwrap_or(None);
        let common_country_codes = shared.country_codes.unwrap_or(None);
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(DecodedFields {
            common: CommonFields {
                chats: common_chats,
                country_codes: common_country_codes,
                has_public_winners: common_has_public_winners,
                only_new_members: common_only_new_members,
                prize_description: common_prize_description,
                winner_count: common_winner_count,
                winners_selection_date: common_winners_selection_date,
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

pub(crate) static PREMIUM: Metadata = Metadata {
    name: "GiveawayPremium",
    fields: &field_names(&["premium_subscription_month_count"]),
    variant_fields: &["premium_subscription_month_count"],
};

pub(crate) static STAR: Metadata = Metadata {
    name: "GiveawayStar",
    fields: &field_names(&["prize_star_count"]),
    variant_fields: &["prize_star_count"],
};
