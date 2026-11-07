use serde::{
    de::{DeserializeSeed, Error, Expected, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) mod chat;
pub(crate) mod chat_full_info;
pub(crate) mod chat_member;
pub(crate) mod encrypted_passport_element;
pub(crate) mod external_reply_info;
pub(crate) mod giveaway;
pub(crate) mod giveaway_winners;
pub(crate) mod inline_query_result;
pub(crate) mod inline_query_result_audio_kind;
pub(crate) mod inline_query_result_document_kind;
pub(crate) mod inline_query_result_gif_kind;
pub(crate) mod inline_query_result_mpeg4_gif_kind;
pub(crate) mod inline_query_result_photo_kind;
pub(crate) mod inline_query_result_video_kind;
pub(crate) mod inline_query_result_voice_kind;
pub(crate) mod input_media;
pub(crate) mod input_paid_media;
pub(crate) mod message;
pub(crate) mod message_entity;
pub(crate) mod message_origin;
pub(crate) mod owned_gift;
pub(crate) mod passport_element_error;
pub(crate) mod poll;
pub(crate) mod star_transaction;
pub(crate) mod sticker;
pub(crate) mod transaction_partner_user;
pub(crate) mod update;

#[inline]
fn read_map_field<'de, A: MapAccess<'de>, T: Deserialize<'de>>(
    slot: &mut Option<T>,
    map: &mut A,
    name: &'static str,
) -> Result<(), A::Error> {
    if slot.is_some() {
        return Err(A::Error::duplicate_field(name));
    }
    *slot = Some(map.next_value()?);
    Ok(())
}

#[inline]
fn read_seq_field<'de, A: SeqAccess<'de>, T: Deserialize<'de>>(
    seq: &mut A,
    index: usize,
    expected: &dyn Expected,
) -> Result<T, A::Error> {
    seq.next_element()?
        .ok_or_else(|| A::Error::invalid_length(index, expected))
}

pub(crate) struct Metadata {
    name: &'static str,
    fields: &'static [&'static str],
    variant_fields: &'static [&'static str],
}

struct SequenceExpected(&'static Metadata);

impl Expected for SequenceExpected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count = self.0.fields.len();
        let suffix = if count == 1 { "element" } else { "elements" };
        write!(formatter, "struct {} with {count} {suffix}", self.0.name)
    }
}

trait FieldLookup {
    const COUNT: usize;
    fn field(value: &str) -> Option<usize>;
    fn field_bytes(value: &[u8]) -> Option<usize>;
}

const UNKNOWN_FIELD: usize = usize::MAX;

struct FieldKey<Fields> {
    metadata: &'static Metadata,
    marker: PhantomData<Fields>,
}

impl<Fields> FieldKey<Fields> {
    fn new(metadata: &'static Metadata) -> Self {
        Self {
            metadata,
            marker: PhantomData,
        }
    }
}

impl<'de, Fields: FieldLookup> DeserializeSeed<'de> for FieldKey<Fields> {
    type Value = usize;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<usize, D::Error> {
        deserializer.deserialize_identifier(self)
    }
}

impl<Fields: FieldLookup> Visitor<'_> for FieldKey<Fields> {
    type Value = usize;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("field identifier")
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<usize, E> {
        if let Some(name) = usize::try_from(value)
            .ok()
            .and_then(|index| self.metadata.fields.get(index))
        {
            self.visit_str(name)
        } else {
            Ok(UNKNOWN_FIELD)
        }
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<usize, E> {
        Ok(if let Some(index) = Fields::field(value) {
            index
        } else if let Some(index) = self
            .metadata
            .variant_fields
            .iter()
            .position(|name| *name == value)
        {
            Fields::COUNT + index
        } else {
            UNKNOWN_FIELD
        })
    }

    fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<usize, E> {
        Ok(if let Some(index) = Fields::field_bytes(value) {
            index
        } else if let Some(index) = self
            .metadata
            .variant_fields
            .iter()
            .position(|name| name.as_bytes() == value)
        {
            Fields::COUNT + index
        } else {
            UNKNOWN_FIELD
        })
    }
}
