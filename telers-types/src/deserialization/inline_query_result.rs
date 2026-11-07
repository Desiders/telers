#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) id: Box<str>,
    pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    id: Option<Box<str>>,
    reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 2;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "id" => Some(0),
            "reply_markup" => Some(1),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"id" => Some(0),
            b"reply_markup" => Some(1),
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
            1 => read_map_field(&mut self.reply_markup, map, "reply_markup"),
            _ => unreachable!(),
        }
    }
}

const fn article_field_names(variant_fields: &[&'static str; 7]) -> [&'static str; 9] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        "reply_markup",
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
    ]
}

pub(crate) struct ArticleFields {
    pub(crate) common: CommonFields,
    pub(crate) title: Box<str>,
    pub(crate) input_message_content: crate::types::InputMessageContent,
    pub(crate) url: Option<Box<str>>,
    pub(crate) description: Option<Box<str>>,
    pub(crate) thumbnail_url: Option<Box<str>>,
    pub(crate) thumbnail_width: Option<i64>,
    pub(crate) thumbnail_height: Option<i64>,
}

struct ArticleVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for ArticleVisitor {
    type Value = ArticleFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_title = read_seq_field(&mut seq, 1, &expected)?;
        let variant_input_message_content = read_seq_field(&mut seq, 2, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 3, &expected)?;
        let variant_url = read_seq_field(&mut seq, 4, &expected)?;
        let variant_description = read_seq_field(&mut seq, 5, &expected)?;
        let variant_thumbnail_url = read_seq_field(&mut seq, 6, &expected)?;
        let variant_thumbnail_width = read_seq_field(&mut seq, 7, &expected)?;
        let variant_thumbnail_height = read_seq_field(&mut seq, 8, &expected)?;
        Ok(ArticleFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            title: variant_title,
            input_message_content: variant_input_message_content,
            url: variant_url,
            description: variant_description,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_title: Option<Box<str>> = None;
        let mut variant_input_message_content: Option<crate::types::InputMessageContent> = None;
        let mut variant_url: Option<Option<Box<str>>> = None;
        let mut variant_description: Option<Option<Box<str>>> = None;
        let mut variant_thumbnail_url: Option<Option<Box<str>>> = None;
        let mut variant_thumbnail_width: Option<Option<i64>> = None;
        let mut variant_thumbnail_height: Option<Option<i64>> = None;
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
                        &mut variant_input_message_content,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                4 => {
                    read_map_field(&mut variant_url, &mut map, self.metadata.variant_fields[2])?;
                }
                5 => {
                    read_map_field(
                        &mut variant_description,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_thumbnail_url,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_thumbnail_width,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_thumbnail_height,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_title = variant_title
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_input_message_content = variant_input_message_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let variant_url = variant_url.unwrap_or_default();
        let variant_description = variant_description.unwrap_or_default();
        let variant_thumbnail_url = variant_thumbnail_url.unwrap_or_default();
        let variant_thumbnail_width = variant_thumbnail_width.unwrap_or_default();
        let variant_thumbnail_height = variant_thumbnail_height.unwrap_or_default();
        Ok(ArticleFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            title: variant_title,
            input_message_content: variant_input_message_content,
            url: variant_url,
            description: variant_description,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }
}

pub(crate) fn deserialize_article<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ArticleFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ArticleVisitor {
            metadata,
        },
    )
}

const fn cached_sticker_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 4] {
    ["id", variant_fields[0], "reply_markup", variant_fields[1]]
}

pub(crate) struct CachedStickerFields {
    pub(crate) common: CommonFields,
    pub(crate) sticker_file_id: Box<str>,
    pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
}

struct CachedStickerVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for CachedStickerVisitor {
    type Value = CachedStickerFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_sticker_file_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 2, &expected)?;
        let variant_input_message_content = read_seq_field(&mut seq, 3, &expected)?;
        Ok(CachedStickerFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            sticker_file_id: variant_sticker_file_id,
            input_message_content: variant_input_message_content,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_sticker_file_id: Option<Box<str>> = None;
        let mut variant_input_message_content: Option<Option<crate::types::InputMessageContent>> =
            None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_sticker_file_id,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_input_message_content,
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
        let variant_sticker_file_id = variant_sticker_file_id
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let variant_input_message_content = variant_input_message_content.unwrap_or_default();
        Ok(CachedStickerFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            sticker_file_id: variant_sticker_file_id,
            input_message_content: variant_input_message_content,
        })
    }
}

pub(crate) fn deserialize_cached_sticker<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<CachedStickerFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        CachedStickerVisitor {
            metadata,
        },
    )
}

const fn contact_field_names(variant_fields: &[&'static str; 8]) -> [&'static str; 10] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        "reply_markup",
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
    ]
}

pub(crate) struct ContactFields {
    pub(crate) common: CommonFields,
    pub(crate) phone_number: Box<str>,
    pub(crate) first_name: Box<str>,
    pub(crate) last_name: Option<Box<str>>,
    pub(crate) vcard: Option<Box<str>>,
    pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
    pub(crate) thumbnail_url: Option<Box<str>>,
    pub(crate) thumbnail_width: Option<i64>,
    pub(crate) thumbnail_height: Option<i64>,
}

struct ContactVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for ContactVisitor {
    type Value = ContactFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_phone_number = read_seq_field(&mut seq, 1, &expected)?;
        let variant_first_name = read_seq_field(&mut seq, 2, &expected)?;
        let variant_last_name = read_seq_field(&mut seq, 3, &expected)?;
        let variant_vcard = read_seq_field(&mut seq, 4, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 5, &expected)?;
        let variant_input_message_content = read_seq_field(&mut seq, 6, &expected)?;
        let variant_thumbnail_url = read_seq_field(&mut seq, 7, &expected)?;
        let variant_thumbnail_width = read_seq_field(&mut seq, 8, &expected)?;
        let variant_thumbnail_height = read_seq_field(&mut seq, 9, &expected)?;
        Ok(ContactFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            phone_number: variant_phone_number,
            first_name: variant_first_name,
            last_name: variant_last_name,
            vcard: variant_vcard,
            input_message_content: variant_input_message_content,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_phone_number: Option<Box<str>> = None;
        let mut variant_first_name: Option<Box<str>> = None;
        let mut variant_last_name: Option<Option<Box<str>>> = None;
        let mut variant_vcard: Option<Option<Box<str>>> = None;
        let mut variant_input_message_content: Option<Option<crate::types::InputMessageContent>> =
            None;
        let mut variant_thumbnail_url: Option<Option<Box<str>>> = None;
        let mut variant_thumbnail_width: Option<Option<i64>> = None;
        let mut variant_thumbnail_height: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_phone_number,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_first_name,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_last_name,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_vcard,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_input_message_content,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_thumbnail_url,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_thumbnail_width,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_thumbnail_height,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_phone_number = variant_phone_number
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_first_name = variant_first_name
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_last_name = variant_last_name.unwrap_or_default();
        let variant_vcard = variant_vcard.unwrap_or_default();
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let variant_input_message_content = variant_input_message_content.unwrap_or_default();
        let variant_thumbnail_url = variant_thumbnail_url.unwrap_or_default();
        let variant_thumbnail_width = variant_thumbnail_width.unwrap_or_default();
        let variant_thumbnail_height = variant_thumbnail_height.unwrap_or_default();
        Ok(ContactFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            phone_number: variant_phone_number,
            first_name: variant_first_name,
            last_name: variant_last_name,
            vcard: variant_vcard,
            input_message_content: variant_input_message_content,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }
}

pub(crate) fn deserialize_contact<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ContactFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ContactVisitor {
            metadata,
        },
    )
}

const fn game_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 3] {
    ["id", variant_fields[0], "reply_markup"]
}

pub(crate) struct GameFields {
    pub(crate) common: CommonFields,
    pub(crate) game_short_name: Box<str>,
}

struct GameVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for GameVisitor {
    type Value = GameFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_game_short_name = read_seq_field(&mut seq, 1, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 2, &expected)?;
        Ok(GameFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            game_short_name: variant_game_short_name,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_game_short_name: Option<Box<str>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_game_short_name,
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
        let variant_game_short_name = variant_game_short_name
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        Ok(GameFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            game_short_name: variant_game_short_name,
        })
    }
}

pub(crate) fn deserialize_game<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<GameFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        GameVisitor {
            metadata,
        },
    )
}

const fn location_field_names(variant_fields: &[&'static str; 11]) -> [&'static str; 13] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        "reply_markup",
        variant_fields[7],
        variant_fields[8],
        variant_fields[9],
        variant_fields[10],
    ]
}

pub(crate) struct LocationFields {
    pub(crate) common: CommonFields,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) title: Box<str>,
    pub(crate) horizontal_accuracy: Option<f64>,
    pub(crate) live_period: Option<u32>,
    pub(crate) heading: Option<u16>,
    pub(crate) proximity_alert_radius: Option<u32>,
    pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
    pub(crate) thumbnail_url: Option<Box<str>>,
    pub(crate) thumbnail_width: Option<i64>,
    pub(crate) thumbnail_height: Option<i64>,
}

struct LocationVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for LocationVisitor {
    type Value = LocationFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_latitude = read_seq_field(&mut seq, 1, &expected)?;
        let variant_longitude = read_seq_field(&mut seq, 2, &expected)?;
        let variant_title = read_seq_field(&mut seq, 3, &expected)?;
        let variant_horizontal_accuracy = read_seq_field(&mut seq, 4, &expected)?;
        let variant_live_period = read_seq_field(&mut seq, 5, &expected)?;
        let variant_heading = read_seq_field(&mut seq, 6, &expected)?;
        let variant_proximity_alert_radius = read_seq_field(&mut seq, 7, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 8, &expected)?;
        let variant_input_message_content = read_seq_field(&mut seq, 9, &expected)?;
        let variant_thumbnail_url = read_seq_field(&mut seq, 10, &expected)?;
        let variant_thumbnail_width = read_seq_field(&mut seq, 11, &expected)?;
        let variant_thumbnail_height = read_seq_field(&mut seq, 12, &expected)?;
        Ok(LocationFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            latitude: variant_latitude,
            longitude: variant_longitude,
            title: variant_title,
            horizontal_accuracy: variant_horizontal_accuracy,
            live_period: variant_live_period,
            heading: variant_heading,
            proximity_alert_radius: variant_proximity_alert_radius,
            input_message_content: variant_input_message_content,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_latitude: Option<f64> = None;
        let mut variant_longitude: Option<f64> = None;
        let mut variant_title: Option<Box<str>> = None;
        let mut variant_horizontal_accuracy: Option<Option<f64>> = None;
        let mut variant_live_period: Option<Option<u32>> = None;
        let mut variant_heading: Option<Option<u16>> = None;
        let mut variant_proximity_alert_radius: Option<Option<u32>> = None;
        let mut variant_input_message_content: Option<Option<crate::types::InputMessageContent>> =
            None;
        let mut variant_thumbnail_url: Option<Option<Box<str>>> = None;
        let mut variant_thumbnail_width: Option<Option<i64>> = None;
        let mut variant_thumbnail_height: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_latitude,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_longitude,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_horizontal_accuracy,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_live_period,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_heading,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_proximity_alert_radius,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_input_message_content,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_thumbnail_url,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_thumbnail_width,
                        &mut map,
                        self.metadata.variant_fields[9],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_thumbnail_height,
                        &mut map,
                        self.metadata.variant_fields[10],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_latitude = variant_latitude
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_longitude = variant_longitude
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_title = variant_title
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[2]))?;
        let variant_horizontal_accuracy = variant_horizontal_accuracy.unwrap_or_default();
        let variant_live_period = variant_live_period.unwrap_or_default();
        let variant_heading = variant_heading.unwrap_or_default();
        let variant_proximity_alert_radius = variant_proximity_alert_radius.unwrap_or_default();
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let variant_input_message_content = variant_input_message_content.unwrap_or_default();
        let variant_thumbnail_url = variant_thumbnail_url.unwrap_or_default();
        let variant_thumbnail_width = variant_thumbnail_width.unwrap_or_default();
        let variant_thumbnail_height = variant_thumbnail_height.unwrap_or_default();
        Ok(LocationFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            latitude: variant_latitude,
            longitude: variant_longitude,
            title: variant_title,
            horizontal_accuracy: variant_horizontal_accuracy,
            live_period: variant_live_period,
            heading: variant_heading,
            proximity_alert_radius: variant_proximity_alert_radius,
            input_message_content: variant_input_message_content,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }
}

pub(crate) fn deserialize_location<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<LocationFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        LocationVisitor {
            metadata,
        },
    )
}

const fn venue_field_names(variant_fields: &[&'static str; 12]) -> [&'static str; 14] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
        "reply_markup",
        variant_fields[8],
        variant_fields[9],
        variant_fields[10],
        variant_fields[11],
    ]
}

pub(crate) struct VenueFields {
    pub(crate) common: CommonFields,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) title: Box<str>,
    pub(crate) address: Box<str>,
    pub(crate) foursquare_id: Option<Box<str>>,
    pub(crate) foursquare_type: Option<Box<str>>,
    pub(crate) google_place_id: Option<Box<str>>,
    pub(crate) google_place_type: Option<Box<str>>,
    pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
    pub(crate) thumbnail_url: Option<Box<str>>,
    pub(crate) thumbnail_width: Option<i64>,
    pub(crate) thumbnail_height: Option<i64>,
}

struct VenueVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for VenueVisitor {
    type Value = VenueFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_latitude = read_seq_field(&mut seq, 1, &expected)?;
        let variant_longitude = read_seq_field(&mut seq, 2, &expected)?;
        let variant_title = read_seq_field(&mut seq, 3, &expected)?;
        let variant_address = read_seq_field(&mut seq, 4, &expected)?;
        let variant_foursquare_id = read_seq_field(&mut seq, 5, &expected)?;
        let variant_foursquare_type = read_seq_field(&mut seq, 6, &expected)?;
        let variant_google_place_id = read_seq_field(&mut seq, 7, &expected)?;
        let variant_google_place_type = read_seq_field(&mut seq, 8, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 9, &expected)?;
        let variant_input_message_content = read_seq_field(&mut seq, 10, &expected)?;
        let variant_thumbnail_url = read_seq_field(&mut seq, 11, &expected)?;
        let variant_thumbnail_width = read_seq_field(&mut seq, 12, &expected)?;
        let variant_thumbnail_height = read_seq_field(&mut seq, 13, &expected)?;
        Ok(VenueFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            latitude: variant_latitude,
            longitude: variant_longitude,
            title: variant_title,
            address: variant_address,
            foursquare_id: variant_foursquare_id,
            foursquare_type: variant_foursquare_type,
            google_place_id: variant_google_place_id,
            google_place_type: variant_google_place_type,
            input_message_content: variant_input_message_content,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_latitude: Option<f64> = None;
        let mut variant_longitude: Option<f64> = None;
        let mut variant_title: Option<Box<str>> = None;
        let mut variant_address: Option<Box<str>> = None;
        let mut variant_foursquare_id: Option<Option<Box<str>>> = None;
        let mut variant_foursquare_type: Option<Option<Box<str>>> = None;
        let mut variant_google_place_id: Option<Option<Box<str>>> = None;
        let mut variant_google_place_type: Option<Option<Box<str>>> = None;
        let mut variant_input_message_content: Option<Option<crate::types::InputMessageContent>> =
            None;
        let mut variant_thumbnail_url: Option<Option<Box<str>>> = None;
        let mut variant_thumbnail_width: Option<Option<i64>> = None;
        let mut variant_thumbnail_height: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                2 => {
                    read_map_field(
                        &mut variant_latitude,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_longitude,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_address,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_foursquare_id,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_foursquare_type,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_google_place_id,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_google_place_type,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_input_message_content,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_thumbnail_url,
                        &mut map,
                        self.metadata.variant_fields[9],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_thumbnail_width,
                        &mut map,
                        self.metadata.variant_fields[10],
                    )?;
                }
                13 => {
                    read_map_field(
                        &mut variant_thumbnail_height,
                        &mut map,
                        self.metadata.variant_fields[11],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_latitude = variant_latitude
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_longitude = variant_longitude
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_title = variant_title
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[2]))?;
        let variant_address = variant_address
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[3]))?;
        let variant_foursquare_id = variant_foursquare_id.unwrap_or_default();
        let variant_foursquare_type = variant_foursquare_type.unwrap_or_default();
        let variant_google_place_id = variant_google_place_id.unwrap_or_default();
        let variant_google_place_type = variant_google_place_type.unwrap_or_default();
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let variant_input_message_content = variant_input_message_content.unwrap_or_default();
        let variant_thumbnail_url = variant_thumbnail_url.unwrap_or_default();
        let variant_thumbnail_width = variant_thumbnail_width.unwrap_or_default();
        let variant_thumbnail_height = variant_thumbnail_height.unwrap_or_default();
        Ok(VenueFields {
            common: CommonFields {
                id: common_id,
                reply_markup: common_reply_markup,
            },
            latitude: variant_latitude,
            longitude: variant_longitude,
            title: variant_title,
            address: variant_address,
            foursquare_id: variant_foursquare_id,
            foursquare_type: variant_foursquare_type,
            google_place_id: variant_google_place_id,
            google_place_type: variant_google_place_type,
            input_message_content: variant_input_message_content,
            thumbnail_url: variant_thumbnail_url,
            thumbnail_width: variant_thumbnail_width,
            thumbnail_height: variant_thumbnail_height,
        })
    }
}

pub(crate) fn deserialize_venue<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<VenueFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        VenueVisitor {
            metadata,
        },
    )
}

pub(crate) static ARTICLE: Metadata = Metadata {
    name: "InlineQueryResultArticle",
    fields: &article_field_names(&[
        "title",
        "input_message_content",
        "url",
        "description",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ]),
    variant_fields: &[
        "title",
        "input_message_content",
        "url",
        "description",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ],
};

pub(crate) static CACHED_STICKER: Metadata = Metadata {
    name: "InlineQueryResultCachedSticker",
    fields: &cached_sticker_field_names(&["sticker_file_id", "input_message_content"]),
    variant_fields: &["sticker_file_id", "input_message_content"],
};

pub(crate) static CONTACT: Metadata = Metadata {
    name: "InlineQueryResultContact",
    fields: &contact_field_names(&[
        "phone_number",
        "first_name",
        "last_name",
        "vcard",
        "input_message_content",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ]),
    variant_fields: &[
        "phone_number",
        "first_name",
        "last_name",
        "vcard",
        "input_message_content",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ],
};

pub(crate) static GAME: Metadata = Metadata {
    name: "InlineQueryResultGame",
    fields: &game_field_names(&["game_short_name"]),
    variant_fields: &["game_short_name"],
};

pub(crate) static LOCATION: Metadata = Metadata {
    name: "InlineQueryResultLocation",
    fields: &location_field_names(&[
        "latitude",
        "longitude",
        "title",
        "horizontal_accuracy",
        "live_period",
        "heading",
        "proximity_alert_radius",
        "input_message_content",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ]),
    variant_fields: &[
        "latitude",
        "longitude",
        "title",
        "horizontal_accuracy",
        "live_period",
        "heading",
        "proximity_alert_radius",
        "input_message_content",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ],
};

pub(crate) static VENUE: Metadata = Metadata {
    name: "InlineQueryResultVenue",
    fields: &venue_field_names(&[
        "latitude",
        "longitude",
        "title",
        "address",
        "foursquare_id",
        "foursquare_type",
        "google_place_id",
        "google_place_type",
        "input_message_content",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ]),
    variant_fields: &[
        "latitude",
        "longitude",
        "title",
        "address",
        "foursquare_id",
        "foursquare_type",
        "google_place_id",
        "google_place_type",
        "input_message_content",
        "thumbnail_url",
        "thumbnail_width",
        "thumbnail_height",
    ],
};
