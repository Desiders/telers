#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) additional_chat_count: Option<i64>,
    pub(crate) chat: Box<crate::types::Chat>,
    pub(crate) giveaway_message_id: i64,
    pub(crate) only_new_members: Option<bool>,
    pub(crate) prize_description: Option<Box<str>>,
    pub(crate) unclaimed_prize_count: Option<i64>,
    pub(crate) was_refunded: Option<bool>,
    pub(crate) winner_count: i64,
    pub(crate) winners: Box<[crate::types::User]>,
    pub(crate) winners_selection_date: i64,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    additional_chat_count: Option<Option<i64>>,
    chat: Option<Box<crate::types::Chat>>,
    giveaway_message_id: Option<i64>,
    only_new_members: Option<Option<bool>>,
    prize_description: Option<Option<Box<str>>>,
    unclaimed_prize_count: Option<Option<i64>>,
    was_refunded: Option<Option<bool>>,
    winner_count: Option<i64>,
    winners: Option<Box<[crate::types::User]>>,
    winners_selection_date: Option<i64>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 10;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "additional_chat_count" => Some(0),
            "chat" => Some(1),
            "giveaway_message_id" => Some(2),
            "only_new_members" => Some(3),
            "prize_description" => Some(4),
            "unclaimed_prize_count" => Some(5),
            "was_refunded" => Some(6),
            "winner_count" => Some(7),
            "winners" => Some(8),
            "winners_selection_date" => Some(9),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"additional_chat_count" => Some(0),
            b"chat" => Some(1),
            b"giveaway_message_id" => Some(2),
            b"only_new_members" => Some(3),
            b"prize_description" => Some(4),
            b"unclaimed_prize_count" => Some(5),
            b"was_refunded" => Some(6),
            b"winner_count" => Some(7),
            b"winners" => Some(8),
            b"winners_selection_date" => Some(9),
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
            0 => read_map_field(
                &mut self.additional_chat_count,
                map,
                "additional_chat_count",
            ),
            1 => read_map_field(&mut self.chat, map, "chat"),
            2 => read_map_field(&mut self.giveaway_message_id, map, "giveaway_message_id"),
            3 => read_map_field(&mut self.only_new_members, map, "only_new_members"),
            4 => read_map_field(&mut self.prize_description, map, "prize_description"),
            5 => read_map_field(
                &mut self.unclaimed_prize_count,
                map,
                "unclaimed_prize_count",
            ),
            6 => read_map_field(&mut self.was_refunded, map, "was_refunded"),
            7 => read_map_field(&mut self.winner_count, map, "winner_count"),
            8 => read_map_field(&mut self.winners, map, "winners"),
            9 => read_map_field(
                &mut self.winners_selection_date,
                map,
                "winners_selection_date",
            ),
            _ => unreachable!(),
        }
    }
}

const fn field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 11] {
    [
        "chat",
        "giveaway_message_id",
        "winners_selection_date",
        "winner_count",
        "winners",
        "additional_chat_count",
        variant_fields[0],
        "unclaimed_prize_count",
        "only_new_members",
        "was_refunded",
        "prize_description",
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
        let common_chat = read_seq_field(&mut seq, 0, &expected)?;
        let common_giveaway_message_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_winners_selection_date = read_seq_field(&mut seq, 2, &expected)?;
        let common_winner_count = read_seq_field(&mut seq, 3, &expected)?;
        let common_winners = read_seq_field(&mut seq, 4, &expected)?;
        let common_additional_chat_count = read_seq_field(&mut seq, 5, &expected)?;
        let variant_content = read_seq_field(&mut seq, 6, &expected)?;
        let common_unclaimed_prize_count = read_seq_field(&mut seq, 7, &expected)?;
        let common_only_new_members = read_seq_field(&mut seq, 8, &expected)?;
        let common_was_refunded = read_seq_field(&mut seq, 9, &expected)?;
        let common_prize_description = read_seq_field(&mut seq, 10, &expected)?;
        Ok(DecodedFields {
            common: CommonFields {
                additional_chat_count: common_additional_chat_count,
                chat: common_chat,
                giveaway_message_id: common_giveaway_message_id,
                only_new_members: common_only_new_members,
                prize_description: common_prize_description,
                unclaimed_prize_count: common_unclaimed_prize_count,
                was_refunded: common_was_refunded,
                winner_count: common_winner_count,
                winners: common_winners,
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
                10 => {
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
        let common_chat = shared.chat.ok_or_else(|| A::Error::missing_field("chat"))?;
        let common_giveaway_message_id = shared
            .giveaway_message_id
            .ok_or_else(|| A::Error::missing_field("giveaway_message_id"))?;
        let common_winners_selection_date = shared
            .winners_selection_date
            .ok_or_else(|| A::Error::missing_field("winners_selection_date"))?;
        let common_winner_count = shared
            .winner_count
            .ok_or_else(|| A::Error::missing_field("winner_count"))?;
        let common_winners = shared
            .winners
            .ok_or_else(|| A::Error::missing_field("winners"))?;
        let common_additional_chat_count = shared.additional_chat_count.unwrap_or(None);
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_unclaimed_prize_count = shared.unclaimed_prize_count.unwrap_or(None);
        let common_only_new_members = shared.only_new_members.unwrap_or(None);
        let common_was_refunded = shared.was_refunded.unwrap_or(None);
        let common_prize_description = shared.prize_description.unwrap_or(None);
        Ok(DecodedFields {
            common: CommonFields {
                additional_chat_count: common_additional_chat_count,
                chat: common_chat,
                giveaway_message_id: common_giveaway_message_id,
                only_new_members: common_only_new_members,
                prize_description: common_prize_description,
                unclaimed_prize_count: common_unclaimed_prize_count,
                was_refunded: common_was_refunded,
                winner_count: common_winner_count,
                winners: common_winners,
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
    name: "GiveawayWinnersPremium",
    fields: &field_names(&["premium_subscription_month_count"]),
    variant_fields: &["premium_subscription_month_count"],
};

pub(crate) static STAR: Metadata = Metadata {
    name: "GiveawayWinnersStar",
    fields: &field_names(&["prize_star_count"]),
    variant_fields: &["prize_star_count"],
};
