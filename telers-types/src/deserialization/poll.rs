#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

#[allow(clippy::struct_excessive_bools)]
pub(crate) struct CommonFields {
    pub(crate) allows_multiple_answers: bool,
    pub(crate) allows_revoting: bool,
    pub(crate) close_date: Option<i64>,
    pub(crate) country_codes: Option<Box<[Box<str>]>>,
    pub(crate) description: Option<Box<str>>,
    pub(crate) description_entities: Option<Box<[crate::types::MessageEntity]>>,
    pub(crate) explanation_entities: Option<Box<[crate::types::MessageEntity]>>,
    pub(crate) id: Box<str>,
    pub(crate) is_anonymous: bool,
    pub(crate) is_closed: bool,
    pub(crate) media: Option<crate::types::PollMedia>,
    pub(crate) members_only: bool,
    pub(crate) open_period: Option<i64>,
    pub(crate) options: Box<[crate::types::PollOption]>,
    pub(crate) question: Box<str>,
    pub(crate) question_entities: Option<Box<[crate::types::MessageEntity]>>,
    pub(crate) total_voter_count: i64,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    allows_multiple_answers: Option<bool>,
    allows_revoting: Option<bool>,
    close_date: Option<Option<i64>>,
    country_codes: Option<Option<Box<[Box<str>]>>>,
    description: Option<Option<Box<str>>>,
    description_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
    explanation_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
    id: Option<Box<str>>,
    is_anonymous: Option<bool>,
    is_closed: Option<bool>,
    media: Option<Option<crate::types::PollMedia>>,
    members_only: Option<bool>,
    open_period: Option<Option<i64>>,
    options: Option<Box<[crate::types::PollOption]>>,
    question: Option<Box<str>>,
    question_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
    total_voter_count: Option<i64>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 17;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "allows_multiple_answers" => Some(0),
            "allows_revoting" => Some(1),
            "close_date" => Some(2),
            "country_codes" => Some(3),
            "description" => Some(4),
            "description_entities" => Some(5),
            "explanation_entities" => Some(6),
            "id" => Some(7),
            "is_anonymous" => Some(8),
            "is_closed" => Some(9),
            "media" => Some(10),
            "members_only" => Some(11),
            "open_period" => Some(12),
            "options" => Some(13),
            "question" => Some(14),
            "question_entities" => Some(15),
            "total_voter_count" => Some(16),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"allows_multiple_answers" => Some(0),
            b"allows_revoting" => Some(1),
            b"close_date" => Some(2),
            b"country_codes" => Some(3),
            b"description" => Some(4),
            b"description_entities" => Some(5),
            b"explanation_entities" => Some(6),
            b"id" => Some(7),
            b"is_anonymous" => Some(8),
            b"is_closed" => Some(9),
            b"media" => Some(10),
            b"members_only" => Some(11),
            b"open_period" => Some(12),
            b"options" => Some(13),
            b"question" => Some(14),
            b"question_entities" => Some(15),
            b"total_voter_count" => Some(16),
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
                &mut self.allows_multiple_answers,
                map,
                "allows_multiple_answers",
            ),
            1 => read_map_field(&mut self.allows_revoting, map, "allows_revoting"),
            2 => read_map_field(&mut self.close_date, map, "close_date"),
            3 => read_map_field(&mut self.country_codes, map, "country_codes"),
            4 => read_map_field(&mut self.description, map, "description"),
            5 => read_map_field(&mut self.description_entities, map, "description_entities"),
            6 => read_map_field(&mut self.explanation_entities, map, "explanation_entities"),
            7 => read_map_field(&mut self.id, map, "id"),
            8 => read_map_field(&mut self.is_anonymous, map, "is_anonymous"),
            9 => read_map_field(&mut self.is_closed, map, "is_closed"),
            10 => read_map_field(&mut self.media, map, "media"),
            11 => read_map_field(&mut self.members_only, map, "members_only"),
            12 => read_map_field(&mut self.open_period, map, "open_period"),
            13 => read_map_field(&mut self.options, map, "options"),
            14 => read_map_field(&mut self.question, map, "question"),
            15 => read_map_field(&mut self.question_entities, map, "question_entities"),
            16 => read_map_field(&mut self.total_voter_count, map, "total_voter_count"),
            _ => unreachable!(),
        }
    }
}

const fn quiz_field_names(variant_fields: &[&'static str; 3]) -> [&'static str; 20] {
    [
        "id",
        "question",
        "question_entities",
        "options",
        "total_voter_count",
        "is_closed",
        "is_anonymous",
        "allows_multiple_answers",
        "allows_revoting",
        "members_only",
        "country_codes",
        variant_fields[0],
        variant_fields[1],
        "explanation_entities",
        variant_fields[2],
        "open_period",
        "close_date",
        "description",
        "description_entities",
        "media",
    ]
}

pub(crate) struct QuizFields {
    pub(crate) common: CommonFields,
    pub(crate) correct_option_ids: Option<Box<[i64]>>,
    pub(crate) explanation: Option<Box<str>>,
    pub(crate) explanation_media: Option<crate::types::PollMedia>,
}

struct QuizVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for QuizVisitor {
    type Value = QuizFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let common_question = read_seq_field(&mut seq, 1, &expected)?;
        let common_question_entities = read_seq_field(&mut seq, 2, &expected)?;
        let common_options = read_seq_field(&mut seq, 3, &expected)?;
        let common_total_voter_count = read_seq_field(&mut seq, 4, &expected)?;
        let common_is_closed = read_seq_field(&mut seq, 5, &expected)?;
        let common_is_anonymous = read_seq_field(&mut seq, 6, &expected)?;
        let common_allows_multiple_answers = read_seq_field(&mut seq, 7, &expected)?;
        let common_allows_revoting = read_seq_field(&mut seq, 8, &expected)?;
        let common_members_only = seq.next_element()?.unwrap_or_default();
        let common_country_codes = read_seq_field(&mut seq, 10, &expected)?;
        let variant_correct_option_ids = read_seq_field(&mut seq, 11, &expected)?;
        let variant_explanation = read_seq_field(&mut seq, 12, &expected)?;
        let common_explanation_entities = read_seq_field(&mut seq, 13, &expected)?;
        let variant_explanation_media = read_seq_field(&mut seq, 14, &expected)?;
        let common_open_period = read_seq_field(&mut seq, 15, &expected)?;
        let common_close_date = read_seq_field(&mut seq, 16, &expected)?;
        let common_description = read_seq_field(&mut seq, 17, &expected)?;
        let common_description_entities = read_seq_field(&mut seq, 18, &expected)?;
        let common_media = read_seq_field(&mut seq, 19, &expected)?;
        Ok(QuizFields {
            common: CommonFields {
                allows_multiple_answers: common_allows_multiple_answers,
                allows_revoting: common_allows_revoting,
                close_date: common_close_date,
                country_codes: common_country_codes,
                description: common_description,
                description_entities: common_description_entities,
                explanation_entities: common_explanation_entities,
                id: common_id,
                is_anonymous: common_is_anonymous,
                is_closed: common_is_closed,
                media: common_media,
                members_only: common_members_only,
                open_period: common_open_period,
                options: common_options,
                question: common_question,
                question_entities: common_question_entities,
                total_voter_count: common_total_voter_count,
            },
            correct_option_ids: variant_correct_option_ids,
            explanation: variant_explanation,
            explanation_media: variant_explanation_media,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_correct_option_ids: Option<Option<Box<[i64]>>> = None;
        let mut variant_explanation: Option<Option<Box<str>>> = None;
        let mut variant_explanation_media: Option<Option<crate::types::PollMedia>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                17 => {
                    read_map_field(
                        &mut variant_correct_option_ids,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                18 => {
                    read_map_field(
                        &mut variant_explanation,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                19 => {
                    read_map_field(
                        &mut variant_explanation_media,
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
        let common_question = shared
            .question
            .ok_or_else(|| A::Error::missing_field("question"))?;
        let common_question_entities = shared.question_entities.unwrap_or(None);
        let common_options = shared
            .options
            .ok_or_else(|| A::Error::missing_field("options"))?;
        let common_total_voter_count = shared
            .total_voter_count
            .ok_or_else(|| A::Error::missing_field("total_voter_count"))?;
        let common_is_closed = shared
            .is_closed
            .ok_or_else(|| A::Error::missing_field("is_closed"))?;
        let common_is_anonymous = shared
            .is_anonymous
            .ok_or_else(|| A::Error::missing_field("is_anonymous"))?;
        let common_allows_multiple_answers = shared
            .allows_multiple_answers
            .ok_or_else(|| A::Error::missing_field("allows_multiple_answers"))?;
        let common_allows_revoting = shared
            .allows_revoting
            .ok_or_else(|| A::Error::missing_field("allows_revoting"))?;
        let common_members_only = shared.members_only.unwrap_or_default();
        let common_country_codes = shared.country_codes.unwrap_or(None);
        let variant_correct_option_ids = variant_correct_option_ids.unwrap_or_default();
        let variant_explanation = variant_explanation.unwrap_or_default();
        let common_explanation_entities = shared.explanation_entities.unwrap_or(None);
        let variant_explanation_media = variant_explanation_media.unwrap_or_default();
        let common_open_period = shared.open_period.unwrap_or(None);
        let common_close_date = shared.close_date.unwrap_or(None);
        let common_description = shared.description.unwrap_or(None);
        let common_description_entities = shared.description_entities.unwrap_or(None);
        let common_media = shared.media.unwrap_or(None);
        Ok(QuizFields {
            common: CommonFields {
                allows_multiple_answers: common_allows_multiple_answers,
                allows_revoting: common_allows_revoting,
                close_date: common_close_date,
                country_codes: common_country_codes,
                description: common_description,
                description_entities: common_description_entities,
                explanation_entities: common_explanation_entities,
                id: common_id,
                is_anonymous: common_is_anonymous,
                is_closed: common_is_closed,
                media: common_media,
                members_only: common_members_only,
                open_period: common_open_period,
                options: common_options,
                question: common_question,
                question_entities: common_question_entities,
                total_voter_count: common_total_voter_count,
            },
            correct_option_ids: variant_correct_option_ids,
            explanation: variant_explanation,
            explanation_media: variant_explanation_media,
        })
    }
}

pub(crate) fn deserialize_quiz<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<QuizFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        QuizVisitor {
            metadata,
        },
    )
}

const COMMON_FIELD_NAMES: [&str; 17] = [
    "id",
    "question",
    "question_entities",
    "options",
    "total_voter_count",
    "is_closed",
    "is_anonymous",
    "allows_multiple_answers",
    "allows_revoting",
    "members_only",
    "country_codes",
    "explanation_entities",
    "open_period",
    "close_date",
    "description",
    "description_entities",
    "media",
];

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
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let common_question = read_seq_field(&mut seq, 1, &expected)?;
        let common_question_entities = read_seq_field(&mut seq, 2, &expected)?;
        let common_options = read_seq_field(&mut seq, 3, &expected)?;
        let common_total_voter_count = read_seq_field(&mut seq, 4, &expected)?;
        let common_is_closed = read_seq_field(&mut seq, 5, &expected)?;
        let common_is_anonymous = read_seq_field(&mut seq, 6, &expected)?;
        let common_allows_multiple_answers = read_seq_field(&mut seq, 7, &expected)?;
        let common_allows_revoting = read_seq_field(&mut seq, 8, &expected)?;
        let common_members_only = seq.next_element()?.unwrap_or_default();
        let common_country_codes = read_seq_field(&mut seq, 10, &expected)?;
        let common_explanation_entities = read_seq_field(&mut seq, 11, &expected)?;
        let common_open_period = read_seq_field(&mut seq, 12, &expected)?;
        let common_close_date = read_seq_field(&mut seq, 13, &expected)?;
        let common_description = read_seq_field(&mut seq, 14, &expected)?;
        let common_description_entities = read_seq_field(&mut seq, 15, &expected)?;
        let common_media = read_seq_field(&mut seq, 16, &expected)?;
        Ok(CommonFields {
            allows_multiple_answers: common_allows_multiple_answers,
            allows_revoting: common_allows_revoting,
            close_date: common_close_date,
            country_codes: common_country_codes,
            description: common_description,
            description_entities: common_description_entities,
            explanation_entities: common_explanation_entities,
            id: common_id,
            is_anonymous: common_is_anonymous,
            is_closed: common_is_closed,
            media: common_media,
            members_only: common_members_only,
            open_period: common_open_period,
            options: common_options,
            question: common_question,
            question_entities: common_question_entities,
            total_voter_count: common_total_voter_count,
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
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let common_question = shared
            .question
            .ok_or_else(|| A::Error::missing_field("question"))?;
        let common_question_entities = shared.question_entities.unwrap_or(None);
        let common_options = shared
            .options
            .ok_or_else(|| A::Error::missing_field("options"))?;
        let common_total_voter_count = shared
            .total_voter_count
            .ok_or_else(|| A::Error::missing_field("total_voter_count"))?;
        let common_is_closed = shared
            .is_closed
            .ok_or_else(|| A::Error::missing_field("is_closed"))?;
        let common_is_anonymous = shared
            .is_anonymous
            .ok_or_else(|| A::Error::missing_field("is_anonymous"))?;
        let common_allows_multiple_answers = shared
            .allows_multiple_answers
            .ok_or_else(|| A::Error::missing_field("allows_multiple_answers"))?;
        let common_allows_revoting = shared
            .allows_revoting
            .ok_or_else(|| A::Error::missing_field("allows_revoting"))?;
        let common_members_only = shared.members_only.unwrap_or_default();
        let common_country_codes = shared.country_codes.unwrap_or(None);
        let common_explanation_entities = shared.explanation_entities.unwrap_or(None);
        let common_open_period = shared.open_period.unwrap_or(None);
        let common_close_date = shared.close_date.unwrap_or(None);
        let common_description = shared.description.unwrap_or(None);
        let common_description_entities = shared.description_entities.unwrap_or(None);
        let common_media = shared.media.unwrap_or(None);
        Ok(CommonFields {
            allows_multiple_answers: common_allows_multiple_answers,
            allows_revoting: common_allows_revoting,
            close_date: common_close_date,
            country_codes: common_country_codes,
            description: common_description,
            description_entities: common_description_entities,
            explanation_entities: common_explanation_entities,
            id: common_id,
            is_anonymous: common_is_anonymous,
            is_closed: common_is_closed,
            media: common_media,
            members_only: common_members_only,
            open_period: common_open_period,
            options: common_options,
            question: common_question,
            question_entities: common_question_entities,
            total_voter_count: common_total_voter_count,
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

pub(crate) static QUIZ: Metadata = Metadata {
    name: "PollQuiz",
    fields: &quiz_field_names(&["correct_option_ids", "explanation", "explanation_media"]),
    variant_fields: &["correct_option_ids", "explanation", "explanation_media"],
};

pub(crate) static REGULAR: Metadata = Metadata {
    name: "PollRegular",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};
