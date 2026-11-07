#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) chat: Option<Box<crate::types::Chat>>,
    pub(crate) has_media_spoiler: Option<bool>,
    pub(crate) link_preview_options: Option<crate::types::LinkPreviewOptions>,
    pub(crate) message_id: Option<i64>,
    pub(crate) origin: crate::types::MessageOrigin,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    chat: Option<Option<Box<crate::types::Chat>>>,
    has_media_spoiler: Option<Option<bool>>,
    link_preview_options: Option<Option<crate::types::LinkPreviewOptions>>,
    message_id: Option<Option<i64>>,
    origin: Option<crate::types::MessageOrigin>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 5;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "chat" => Some(0),
            "has_media_spoiler" => Some(1),
            "link_preview_options" => Some(2),
            "message_id" => Some(3),
            "origin" => Some(4),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"chat" => Some(0),
            b"has_media_spoiler" => Some(1),
            b"link_preview_options" => Some(2),
            b"message_id" => Some(3),
            b"origin" => Some(4),
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
            0 => read_map_field(&mut self.chat, map, "chat"),
            1 => read_map_field(&mut self.has_media_spoiler, map, "has_media_spoiler"),
            2 => read_map_field(&mut self.link_preview_options, map, "link_preview_options"),
            3 => read_map_field(&mut self.message_id, map, "message_id"),
            4 => read_map_field(&mut self.origin, map, "origin"),
            _ => unreachable!(),
        }
    }
}

const fn content_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 6] {
    [
        "origin",
        "chat",
        "message_id",
        "link_preview_options",
        "has_media_spoiler",
        variant_fields[0],
    ]
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
        let common_origin = read_seq_field(&mut seq, 0, &expected)?;
        let common_chat = read_seq_field(&mut seq, 1, &expected)?;
        let common_message_id = read_seq_field(&mut seq, 2, &expected)?;
        let common_link_preview_options = read_seq_field(&mut seq, 3, &expected)?;
        let common_has_media_spoiler = read_seq_field(&mut seq, 4, &expected)?;
        let variant_content = read_seq_field(&mut seq, 5, &expected)?;
        Ok(ContentFields {
            common: CommonFields {
                chat: common_chat,
                has_media_spoiler: common_has_media_spoiler,
                link_preview_options: common_link_preview_options,
                message_id: common_message_id,
                origin: common_origin,
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
                5 => {
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
        let common_origin = shared
            .origin
            .ok_or_else(|| A::Error::missing_field("origin"))?;
        let common_chat = shared.chat.unwrap_or(None);
        let common_message_id = shared.message_id.unwrap_or(None);
        let common_link_preview_options = shared.link_preview_options.unwrap_or(None);
        let common_has_media_spoiler = shared.has_media_spoiler.unwrap_or(None);
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(ContentFields {
            common: CommonFields {
                chat: common_chat,
                has_media_spoiler: common_has_media_spoiler,
                link_preview_options: common_link_preview_options,
                message_id: common_message_id,
                origin: common_origin,
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

const COMMON_FIELD_NAMES: [&str; 5] = [
    "origin",
    "chat",
    "message_id",
    "link_preview_options",
    "has_media_spoiler",
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
        let common_origin = read_seq_field(&mut seq, 0, &expected)?;
        let common_chat = read_seq_field(&mut seq, 1, &expected)?;
        let common_message_id = read_seq_field(&mut seq, 2, &expected)?;
        let common_link_preview_options = read_seq_field(&mut seq, 3, &expected)?;
        let common_has_media_spoiler = read_seq_field(&mut seq, 4, &expected)?;
        Ok(CommonFields {
            chat: common_chat,
            has_media_spoiler: common_has_media_spoiler,
            link_preview_options: common_link_preview_options,
            message_id: common_message_id,
            origin: common_origin,
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
        let common_origin = shared
            .origin
            .ok_or_else(|| A::Error::missing_field("origin"))?;
        let common_chat = shared.chat.unwrap_or(None);
        let common_message_id = shared.message_id.unwrap_or(None);
        let common_link_preview_options = shared.link_preview_options.unwrap_or(None);
        let common_has_media_spoiler = shared.has_media_spoiler.unwrap_or(None);
        Ok(CommonFields {
            chat: common_chat,
            has_media_spoiler: common_has_media_spoiler,
            link_preview_options: common_link_preview_options,
            message_id: common_message_id,
            origin: common_origin,
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

pub(crate) static ANIMATION: Metadata = Metadata {
    name: "ExternalReplyInfoAnimation",
    fields: &content_field_names(&["animation"]),
    variant_fields: &["animation"],
};

pub(crate) static AUDIO: Metadata = Metadata {
    name: "ExternalReplyInfoAudio",
    fields: &content_field_names(&["audio"]),
    variant_fields: &["audio"],
};

pub(crate) static CHECKLIST: Metadata = Metadata {
    name: "ExternalReplyInfoChecklist",
    fields: &content_field_names(&["checklist"]),
    variant_fields: &["checklist"],
};

pub(crate) static CONTACT: Metadata = Metadata {
    name: "ExternalReplyInfoContact",
    fields: &content_field_names(&["contact"]),
    variant_fields: &["contact"],
};

pub(crate) static DICE: Metadata = Metadata {
    name: "ExternalReplyInfoDice",
    fields: &content_field_names(&["dice"]),
    variant_fields: &["dice"],
};

pub(crate) static DOCUMENT: Metadata = Metadata {
    name: "ExternalReplyInfoDocument",
    fields: &content_field_names(&["document"]),
    variant_fields: &["document"],
};

pub(crate) static GAME: Metadata = Metadata {
    name: "ExternalReplyInfoGame",
    fields: &content_field_names(&["game"]),
    variant_fields: &["game"],
};

pub(crate) static GIVEAWAY: Metadata = Metadata {
    name: "ExternalReplyInfoGiveaway",
    fields: &content_field_names(&["giveaway"]),
    variant_fields: &["giveaway"],
};

pub(crate) static GIVEAWAY_WINNERS: Metadata = Metadata {
    name: "ExternalReplyInfoGiveawayWinners",
    fields: &content_field_names(&["giveaway_winners"]),
    variant_fields: &["giveaway_winners"],
};

pub(crate) static INVOICE: Metadata = Metadata {
    name: "ExternalReplyInfoInvoice",
    fields: &content_field_names(&["invoice"]),
    variant_fields: &["invoice"],
};

pub(crate) static LIVE_PHOTO: Metadata = Metadata {
    name: "ExternalReplyInfoLivePhoto",
    fields: &content_field_names(&["live_photo"]),
    variant_fields: &["live_photo"],
};

pub(crate) static LOCATION: Metadata = Metadata {
    name: "ExternalReplyInfoLocation",
    fields: &content_field_names(&["location"]),
    variant_fields: &["location"],
};

pub(crate) static PAID_MEDIA: Metadata = Metadata {
    name: "ExternalReplyInfoPaidMedia",
    fields: &content_field_names(&["paid_media"]),
    variant_fields: &["paid_media"],
};

pub(crate) static PHOTO: Metadata = Metadata {
    name: "ExternalReplyInfoPhoto",
    fields: &content_field_names(&["photo"]),
    variant_fields: &["photo"],
};

pub(crate) static POLL: Metadata = Metadata {
    name: "ExternalReplyInfoPoll",
    fields: &content_field_names(&["poll"]),
    variant_fields: &["poll"],
};

pub(crate) static STICKER: Metadata = Metadata {
    name: "ExternalReplyInfoSticker",
    fields: &content_field_names(&["sticker"]),
    variant_fields: &["sticker"],
};

pub(crate) static STORY: Metadata = Metadata {
    name: "ExternalReplyInfoStory",
    fields: &content_field_names(&["story"]),
    variant_fields: &["story"],
};

pub(crate) static VENUE: Metadata = Metadata {
    name: "ExternalReplyInfoVenue",
    fields: &content_field_names(&["venue"]),
    variant_fields: &["venue"],
};

pub(crate) static VIDEO: Metadata = Metadata {
    name: "ExternalReplyInfoVideo",
    fields: &content_field_names(&["video"]),
    variant_fields: &["video"],
};

pub(crate) static VIDEO_NOTE: Metadata = Metadata {
    name: "ExternalReplyInfoVideoNote",
    fields: &content_field_names(&["video_note"]),
    variant_fields: &["video_note"],
};

pub(crate) static VOICE: Metadata = Metadata {
    name: "ExternalReplyInfoVoice",
    fields: &content_field_names(&["voice"]),
    variant_fields: &["voice"],
};

pub(crate) static UNKNOWN: Metadata = Metadata {
    name: "ExternalReplyInfoUnknown",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};
