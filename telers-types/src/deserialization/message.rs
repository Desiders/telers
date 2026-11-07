#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) author_signature: Option<Box<str>>,
    pub(crate) business_connection_id: Option<Box<str>>,
    pub(crate) caption: Option<Box<str>>,
    pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    pub(crate) chat: Box<crate::types::Chat>,
    pub(crate) date: i64,
    pub(crate) direct_messages_topic: Option<crate::types::DirectMessagesTopic>,
    pub(crate) edit_date: Option<i64>,
    pub(crate) effect_id: Option<Box<str>>,
    pub(crate) entities: Option<Box<[crate::types::MessageEntity]>>,
    pub(crate) ephemeral_message_id: Option<i64>,
    pub(crate) external_reply: Option<Box<crate::types::ExternalReplyInfo>>,
    pub(crate) forward_origin: Option<crate::types::MessageOrigin>,
    pub(crate) from: Option<Box<crate::types::User>>,
    pub(crate) guest_bot_caller_chat: Option<Box<crate::types::Chat>>,
    pub(crate) guest_bot_caller_user: Option<Box<crate::types::User>>,
    pub(crate) guest_query_id: Option<Box<str>>,
    pub(crate) has_media_spoiler: Option<bool>,
    pub(crate) has_protected_content: Option<bool>,
    pub(crate) is_automatic_forward: Option<bool>,
    pub(crate) is_from_offline: Option<bool>,
    pub(crate) is_paid_post: Option<bool>,
    pub(crate) is_topic_message: Option<bool>,
    pub(crate) link_preview_options: Option<crate::types::LinkPreviewOptions>,
    pub(crate) media_group_id: Option<Box<str>>,
    pub(crate) message_id: i64,
    pub(crate) message_thread_id: Option<i64>,
    pub(crate) paid_star_count: Option<i64>,
    pub(crate) quote: Option<crate::types::TextQuote>,
    pub(crate) receiver_user: Option<Box<crate::types::User>>,
    pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    pub(crate) reply_to_checklist_task_id: Option<i64>,
    pub(crate) reply_to_message: Option<Box<crate::types::Message>>,
    pub(crate) reply_to_poll_option_id: Option<Box<str>>,
    pub(crate) reply_to_story: Option<crate::types::Story>,
    pub(crate) sender_boost_count: Option<i64>,
    pub(crate) sender_business_bot: Option<Box<crate::types::User>>,
    pub(crate) sender_chat: Option<Box<crate::types::Chat>>,
    pub(crate) sender_tag: Option<Box<str>>,
    pub(crate) show_caption_above_media: Option<bool>,
    pub(crate) suggested_post_info: Option<crate::types::SuggestedPostInfo>,
    pub(crate) via_bot: Option<Box<crate::types::User>>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    author_signature: Option<Option<Box<str>>>,
    business_connection_id: Option<Option<Box<str>>>,
    caption: Option<Option<Box<str>>>,
    caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
    chat: Option<Box<crate::types::Chat>>,
    date: Option<i64>,
    direct_messages_topic: Option<Option<crate::types::DirectMessagesTopic>>,
    edit_date: Option<Option<i64>>,
    effect_id: Option<Option<Box<str>>>,
    entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
    ephemeral_message_id: Option<Option<i64>>,
    external_reply: Option<Option<Box<crate::types::ExternalReplyInfo>>>,
    forward_origin: Option<Option<crate::types::MessageOrigin>>,
    from: Option<Option<Box<crate::types::User>>>,
    guest_bot_caller_chat: Option<Option<Box<crate::types::Chat>>>,
    guest_bot_caller_user: Option<Option<Box<crate::types::User>>>,
    guest_query_id: Option<Option<Box<str>>>,
    has_media_spoiler: Option<Option<bool>>,
    has_protected_content: Option<Option<bool>>,
    is_automatic_forward: Option<Option<bool>>,
    is_from_offline: Option<Option<bool>>,
    is_paid_post: Option<Option<bool>>,
    is_topic_message: Option<Option<bool>>,
    link_preview_options: Option<Option<crate::types::LinkPreviewOptions>>,
    media_group_id: Option<Option<Box<str>>>,
    message_id: Option<i64>,
    message_thread_id: Option<Option<i64>>,
    paid_star_count: Option<Option<i64>>,
    quote: Option<Option<crate::types::TextQuote>>,
    receiver_user: Option<Option<Box<crate::types::User>>>,
    reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
    reply_to_checklist_task_id: Option<Option<i64>>,
    reply_to_message: Option<Option<Box<crate::types::Message>>>,
    reply_to_poll_option_id: Option<Option<Box<str>>>,
    reply_to_story: Option<Option<crate::types::Story>>,
    sender_boost_count: Option<Option<i64>>,
    sender_business_bot: Option<Option<Box<crate::types::User>>>,
    sender_chat: Option<Option<Box<crate::types::Chat>>>,
    sender_tag: Option<Option<Box<str>>>,
    show_caption_above_media: Option<Option<bool>>,
    suggested_post_info: Option<Option<crate::types::SuggestedPostInfo>>,
    via_bot: Option<Option<Box<crate::types::User>>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 42;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "author_signature" => Some(0),
            "business_connection_id" => Some(1),
            "caption" => Some(2),
            "caption_entities" => Some(3),
            "chat" => Some(4),
            "date" => Some(5),
            "direct_messages_topic" => Some(6),
            "edit_date" => Some(7),
            "effect_id" => Some(8),
            "entities" => Some(9),
            "ephemeral_message_id" => Some(10),
            "external_reply" => Some(11),
            "forward_origin" => Some(12),
            "from" => Some(13),
            "guest_bot_caller_chat" => Some(14),
            "guest_bot_caller_user" => Some(15),
            "guest_query_id" => Some(16),
            "has_media_spoiler" => Some(17),
            "has_protected_content" => Some(18),
            "is_automatic_forward" => Some(19),
            "is_from_offline" => Some(20),
            "is_paid_post" => Some(21),
            "is_topic_message" => Some(22),
            "link_preview_options" => Some(23),
            "media_group_id" => Some(24),
            "message_id" => Some(25),
            "message_thread_id" => Some(26),
            "paid_star_count" => Some(27),
            "quote" => Some(28),
            "receiver_user" => Some(29),
            "reply_markup" => Some(30),
            "reply_to_checklist_task_id" => Some(31),
            "reply_to_message" => Some(32),
            "reply_to_poll_option_id" => Some(33),
            "reply_to_story" => Some(34),
            "sender_boost_count" => Some(35),
            "sender_business_bot" => Some(36),
            "sender_chat" => Some(37),
            "sender_tag" => Some(38),
            "show_caption_above_media" => Some(39),
            "suggested_post_info" => Some(40),
            "via_bot" => Some(41),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"author_signature" => Some(0),
            b"business_connection_id" => Some(1),
            b"caption" => Some(2),
            b"caption_entities" => Some(3),
            b"chat" => Some(4),
            b"date" => Some(5),
            b"direct_messages_topic" => Some(6),
            b"edit_date" => Some(7),
            b"effect_id" => Some(8),
            b"entities" => Some(9),
            b"ephemeral_message_id" => Some(10),
            b"external_reply" => Some(11),
            b"forward_origin" => Some(12),
            b"from" => Some(13),
            b"guest_bot_caller_chat" => Some(14),
            b"guest_bot_caller_user" => Some(15),
            b"guest_query_id" => Some(16),
            b"has_media_spoiler" => Some(17),
            b"has_protected_content" => Some(18),
            b"is_automatic_forward" => Some(19),
            b"is_from_offline" => Some(20),
            b"is_paid_post" => Some(21),
            b"is_topic_message" => Some(22),
            b"link_preview_options" => Some(23),
            b"media_group_id" => Some(24),
            b"message_id" => Some(25),
            b"message_thread_id" => Some(26),
            b"paid_star_count" => Some(27),
            b"quote" => Some(28),
            b"receiver_user" => Some(29),
            b"reply_markup" => Some(30),
            b"reply_to_checklist_task_id" => Some(31),
            b"reply_to_message" => Some(32),
            b"reply_to_poll_option_id" => Some(33),
            b"reply_to_story" => Some(34),
            b"sender_boost_count" => Some(35),
            b"sender_business_bot" => Some(36),
            b"sender_chat" => Some(37),
            b"sender_tag" => Some(38),
            b"show_caption_above_media" => Some(39),
            b"suggested_post_info" => Some(40),
            b"via_bot" => Some(41),
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
            0 => read_map_field(&mut self.author_signature, map, "author_signature"),
            1 => read_map_field(
                &mut self.business_connection_id,
                map,
                "business_connection_id",
            ),
            2 => read_map_field(&mut self.caption, map, "caption"),
            3 => read_map_field(&mut self.caption_entities, map, "caption_entities"),
            4 => read_map_field(&mut self.chat, map, "chat"),
            5 => read_map_field(&mut self.date, map, "date"),
            6 => read_map_field(
                &mut self.direct_messages_topic,
                map,
                "direct_messages_topic",
            ),
            7 => read_map_field(&mut self.edit_date, map, "edit_date"),
            8 => read_map_field(&mut self.effect_id, map, "effect_id"),
            9 => read_map_field(&mut self.entities, map, "entities"),
            10 => read_map_field(&mut self.ephemeral_message_id, map, "ephemeral_message_id"),
            11 => read_map_field(&mut self.external_reply, map, "external_reply"),
            12 => read_map_field(&mut self.forward_origin, map, "forward_origin"),
            13 => read_map_field(&mut self.from, map, "from"),
            14 => read_map_field(
                &mut self.guest_bot_caller_chat,
                map,
                "guest_bot_caller_chat",
            ),
            15 => read_map_field(
                &mut self.guest_bot_caller_user,
                map,
                "guest_bot_caller_user",
            ),
            16 => read_map_field(&mut self.guest_query_id, map, "guest_query_id"),
            17 => read_map_field(&mut self.has_media_spoiler, map, "has_media_spoiler"),
            18 => read_map_field(
                &mut self.has_protected_content,
                map,
                "has_protected_content",
            ),
            19 => read_map_field(&mut self.is_automatic_forward, map, "is_automatic_forward"),
            20 => read_map_field(&mut self.is_from_offline, map, "is_from_offline"),
            21 => read_map_field(&mut self.is_paid_post, map, "is_paid_post"),
            22 => read_map_field(&mut self.is_topic_message, map, "is_topic_message"),
            23 => read_map_field(&mut self.link_preview_options, map, "link_preview_options"),
            24 => read_map_field(&mut self.media_group_id, map, "media_group_id"),
            25 => read_map_field(&mut self.message_id, map, "message_id"),
            26 => read_map_field(&mut self.message_thread_id, map, "message_thread_id"),
            27 => read_map_field(&mut self.paid_star_count, map, "paid_star_count"),
            28 => read_map_field(&mut self.quote, map, "quote"),
            29 => read_map_field(&mut self.receiver_user, map, "receiver_user"),
            30 => read_map_field(&mut self.reply_markup, map, "reply_markup"),
            31 => read_map_field(
                &mut self.reply_to_checklist_task_id,
                map,
                "reply_to_checklist_task_id",
            ),
            32 => read_map_field(&mut self.reply_to_message, map, "reply_to_message"),
            33 => read_map_field(
                &mut self.reply_to_poll_option_id,
                map,
                "reply_to_poll_option_id",
            ),
            34 => read_map_field(&mut self.reply_to_story, map, "reply_to_story"),
            35 => read_map_field(&mut self.sender_boost_count, map, "sender_boost_count"),
            36 => read_map_field(&mut self.sender_business_bot, map, "sender_business_bot"),
            37 => read_map_field(&mut self.sender_chat, map, "sender_chat"),
            38 => read_map_field(&mut self.sender_tag, map, "sender_tag"),
            39 => read_map_field(
                &mut self.show_caption_above_media,
                map,
                "show_caption_above_media",
            ),
            40 => read_map_field(&mut self.suggested_post_info, map, "suggested_post_info"),
            41 => read_map_field(&mut self.via_bot, map, "via_bot"),
            _ => unreachable!(),
        }
    }
}

const fn field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 43] {
    [
        "message_id",
        "message_thread_id",
        "direct_messages_topic",
        "from",
        "sender_chat",
        "sender_boost_count",
        "sender_business_bot",
        "sender_tag",
        "receiver_user",
        "ephemeral_message_id",
        "date",
        "guest_query_id",
        "business_connection_id",
        "chat",
        "forward_origin",
        "is_topic_message",
        "is_automatic_forward",
        "reply_to_message",
        "external_reply",
        "quote",
        "reply_to_story",
        "reply_to_checklist_task_id",
        "reply_to_poll_option_id",
        "via_bot",
        "guest_bot_caller_user",
        "guest_bot_caller_chat",
        "edit_date",
        "has_protected_content",
        "is_from_offline",
        "is_paid_post",
        "media_group_id",
        "author_signature",
        "paid_star_count",
        "entities",
        "link_preview_options",
        "suggested_post_info",
        "effect_id",
        "caption",
        "caption_entities",
        "show_caption_above_media",
        "has_media_spoiler",
        "reply_markup",
        variant_fields[0],
    ]
}

pub(crate) struct DecodedFields<TContent> {
    pub(crate) common: CommonFields,
    pub(crate) content: TContent,
}

struct FieldsVisitor<TContent> {
    metadata: &'static Metadata,
    marker: PhantomData<DecodedFields<TContent>>,
}

impl<'de, TContent> Visitor<'de> for FieldsVisitor<TContent>
where
    TContent: Deserialize<'de>,
{
    type Value = DecodedFields<TContent>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_message_id = read_seq_field(&mut seq, 0, &expected)?;
        let common_message_thread_id = read_seq_field(&mut seq, 1, &expected)?;
        let common_direct_messages_topic = read_seq_field(&mut seq, 2, &expected)?;
        let common_from = read_seq_field(&mut seq, 3, &expected)?;
        let common_sender_chat = read_seq_field(&mut seq, 4, &expected)?;
        let common_sender_boost_count = read_seq_field(&mut seq, 5, &expected)?;
        let common_sender_business_bot = read_seq_field(&mut seq, 6, &expected)?;
        let common_sender_tag = read_seq_field(&mut seq, 7, &expected)?;
        let common_receiver_user = read_seq_field(&mut seq, 8, &expected)?;
        let common_ephemeral_message_id = read_seq_field(&mut seq, 9, &expected)?;
        let common_date = read_seq_field(&mut seq, 10, &expected)?;
        let common_guest_query_id = read_seq_field(&mut seq, 11, &expected)?;
        let common_business_connection_id = read_seq_field(&mut seq, 12, &expected)?;
        let common_chat = read_seq_field(&mut seq, 13, &expected)?;
        let common_forward_origin = read_seq_field(&mut seq, 14, &expected)?;
        let common_is_topic_message = read_seq_field(&mut seq, 15, &expected)?;
        let common_is_automatic_forward = read_seq_field(&mut seq, 16, &expected)?;
        let common_reply_to_message = read_seq_field(&mut seq, 17, &expected)?;
        let common_external_reply = read_seq_field(&mut seq, 18, &expected)?;
        let common_quote = read_seq_field(&mut seq, 19, &expected)?;
        let common_reply_to_story = read_seq_field(&mut seq, 20, &expected)?;
        let common_reply_to_checklist_task_id = read_seq_field(&mut seq, 21, &expected)?;
        let common_reply_to_poll_option_id = read_seq_field(&mut seq, 22, &expected)?;
        let common_via_bot = read_seq_field(&mut seq, 23, &expected)?;
        let common_guest_bot_caller_user = read_seq_field(&mut seq, 24, &expected)?;
        let common_guest_bot_caller_chat = read_seq_field(&mut seq, 25, &expected)?;
        let common_edit_date = read_seq_field(&mut seq, 26, &expected)?;
        let common_has_protected_content = read_seq_field(&mut seq, 27, &expected)?;
        let common_is_from_offline = read_seq_field(&mut seq, 28, &expected)?;
        let common_is_paid_post = read_seq_field(&mut seq, 29, &expected)?;
        let common_media_group_id = read_seq_field(&mut seq, 30, &expected)?;
        let common_author_signature = read_seq_field(&mut seq, 31, &expected)?;
        let common_paid_star_count = read_seq_field(&mut seq, 32, &expected)?;
        let common_entities = read_seq_field(&mut seq, 33, &expected)?;
        let common_link_preview_options = read_seq_field(&mut seq, 34, &expected)?;
        let common_suggested_post_info = read_seq_field(&mut seq, 35, &expected)?;
        let common_effect_id = read_seq_field(&mut seq, 36, &expected)?;
        let common_caption = read_seq_field(&mut seq, 37, &expected)?;
        let common_caption_entities = read_seq_field(&mut seq, 38, &expected)?;
        let common_show_caption_above_media = read_seq_field(&mut seq, 39, &expected)?;
        let common_has_media_spoiler = read_seq_field(&mut seq, 40, &expected)?;
        let common_reply_markup = read_seq_field(&mut seq, 41, &expected)?;
        let variant_content = read_seq_field(&mut seq, 42, &expected)?;
        Ok(DecodedFields {
            common: CommonFields {
                author_signature: common_author_signature,
                business_connection_id: common_business_connection_id,
                caption: common_caption,
                caption_entities: common_caption_entities,
                chat: common_chat,
                date: common_date,
                direct_messages_topic: common_direct_messages_topic,
                edit_date: common_edit_date,
                effect_id: common_effect_id,
                entities: common_entities,
                ephemeral_message_id: common_ephemeral_message_id,
                external_reply: common_external_reply,
                forward_origin: common_forward_origin,
                from: common_from,
                guest_bot_caller_chat: common_guest_bot_caller_chat,
                guest_bot_caller_user: common_guest_bot_caller_user,
                guest_query_id: common_guest_query_id,
                has_media_spoiler: common_has_media_spoiler,
                has_protected_content: common_has_protected_content,
                is_automatic_forward: common_is_automatic_forward,
                is_from_offline: common_is_from_offline,
                is_paid_post: common_is_paid_post,
                is_topic_message: common_is_topic_message,
                link_preview_options: common_link_preview_options,
                media_group_id: common_media_group_id,
                message_id: common_message_id,
                message_thread_id: common_message_thread_id,
                paid_star_count: common_paid_star_count,
                quote: common_quote,
                receiver_user: common_receiver_user,
                reply_markup: common_reply_markup,
                reply_to_checklist_task_id: common_reply_to_checklist_task_id,
                reply_to_message: common_reply_to_message,
                reply_to_poll_option_id: common_reply_to_poll_option_id,
                reply_to_story: common_reply_to_story,
                sender_boost_count: common_sender_boost_count,
                sender_business_bot: common_sender_business_bot,
                sender_chat: common_sender_chat,
                sender_tag: common_sender_tag,
                show_caption_above_media: common_show_caption_above_media,
                suggested_post_info: common_suggested_post_info,
                via_bot: common_via_bot,
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
                42 => {
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
        let common_message_id = shared
            .message_id
            .ok_or_else(|| A::Error::missing_field("message_id"))?;
        let common_message_thread_id = shared.message_thread_id.unwrap_or(None);
        let common_direct_messages_topic = shared.direct_messages_topic.unwrap_or(None);
        let common_from = shared.from.unwrap_or(None);
        let common_sender_chat = shared.sender_chat.unwrap_or(None);
        let common_sender_boost_count = shared.sender_boost_count.unwrap_or(None);
        let common_sender_business_bot = shared.sender_business_bot.unwrap_or(None);
        let common_sender_tag = shared.sender_tag.unwrap_or(None);
        let common_receiver_user = shared.receiver_user.unwrap_or(None);
        let common_ephemeral_message_id = shared.ephemeral_message_id.unwrap_or(None);
        let common_date = shared.date.ok_or_else(|| A::Error::missing_field("date"))?;
        let common_guest_query_id = shared.guest_query_id.unwrap_or(None);
        let common_business_connection_id = shared.business_connection_id.unwrap_or(None);
        let common_chat = shared.chat.ok_or_else(|| A::Error::missing_field("chat"))?;
        let common_forward_origin = shared.forward_origin.unwrap_or(None);
        let common_is_topic_message = shared.is_topic_message.unwrap_or(None);
        let common_is_automatic_forward = shared.is_automatic_forward.unwrap_or(None);
        let common_reply_to_message = shared.reply_to_message.unwrap_or(None);
        let common_external_reply = shared.external_reply.unwrap_or(None);
        let common_quote = shared.quote.unwrap_or(None);
        let common_reply_to_story = shared.reply_to_story.unwrap_or(None);
        let common_reply_to_checklist_task_id = shared.reply_to_checklist_task_id.unwrap_or(None);
        let common_reply_to_poll_option_id = shared.reply_to_poll_option_id.unwrap_or(None);
        let common_via_bot = shared.via_bot.unwrap_or(None);
        let common_guest_bot_caller_user = shared.guest_bot_caller_user.unwrap_or(None);
        let common_guest_bot_caller_chat = shared.guest_bot_caller_chat.unwrap_or(None);
        let common_edit_date = shared.edit_date.unwrap_or(None);
        let common_has_protected_content = shared.has_protected_content.unwrap_or(None);
        let common_is_from_offline = shared.is_from_offline.unwrap_or(None);
        let common_is_paid_post = shared.is_paid_post.unwrap_or(None);
        let common_media_group_id = shared.media_group_id.unwrap_or(None);
        let common_author_signature = shared.author_signature.unwrap_or(None);
        let common_paid_star_count = shared.paid_star_count.unwrap_or(None);
        let common_entities = shared.entities.unwrap_or(None);
        let common_link_preview_options = shared.link_preview_options.unwrap_or(None);
        let common_suggested_post_info = shared.suggested_post_info.unwrap_or(None);
        let common_effect_id = shared.effect_id.unwrap_or(None);
        let common_caption = shared.caption.unwrap_or(None);
        let common_caption_entities = shared.caption_entities.unwrap_or(None);
        let common_show_caption_above_media = shared.show_caption_above_media.unwrap_or(None);
        let common_has_media_spoiler = shared.has_media_spoiler.unwrap_or(None);
        let common_reply_markup = shared.reply_markup.unwrap_or(None);
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(DecodedFields {
            common: CommonFields {
                author_signature: common_author_signature,
                business_connection_id: common_business_connection_id,
                caption: common_caption,
                caption_entities: common_caption_entities,
                chat: common_chat,
                date: common_date,
                direct_messages_topic: common_direct_messages_topic,
                edit_date: common_edit_date,
                effect_id: common_effect_id,
                entities: common_entities,
                ephemeral_message_id: common_ephemeral_message_id,
                external_reply: common_external_reply,
                forward_origin: common_forward_origin,
                from: common_from,
                guest_bot_caller_chat: common_guest_bot_caller_chat,
                guest_bot_caller_user: common_guest_bot_caller_user,
                guest_query_id: common_guest_query_id,
                has_media_spoiler: common_has_media_spoiler,
                has_protected_content: common_has_protected_content,
                is_automatic_forward: common_is_automatic_forward,
                is_from_offline: common_is_from_offline,
                is_paid_post: common_is_paid_post,
                is_topic_message: common_is_topic_message,
                link_preview_options: common_link_preview_options,
                media_group_id: common_media_group_id,
                message_id: common_message_id,
                message_thread_id: common_message_thread_id,
                paid_star_count: common_paid_star_count,
                quote: common_quote,
                receiver_user: common_receiver_user,
                reply_markup: common_reply_markup,
                reply_to_checklist_task_id: common_reply_to_checklist_task_id,
                reply_to_message: common_reply_to_message,
                reply_to_poll_option_id: common_reply_to_poll_option_id,
                reply_to_story: common_reply_to_story,
                sender_boost_count: common_sender_boost_count,
                sender_business_bot: common_sender_business_bot,
                sender_chat: common_sender_chat,
                sender_tag: common_sender_tag,
                show_caption_above_media: common_show_caption_above_media,
                suggested_post_info: common_suggested_post_info,
                via_bot: common_via_bot,
            },
            content: variant_content,
        })
    }
}

pub(crate) fn deserialize<'de, D, TContent>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<DecodedFields<TContent>, D::Error>
where
    D: Deserializer<'de>,
    TContent: Deserialize<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        FieldsVisitor {
            metadata,
            marker: PhantomData,
        },
    )
}

pub(crate) static ANIMATION: Metadata = Metadata {
    name: "MessageAnimation",
    fields: &field_names(&["animation"]),
    variant_fields: &["animation"],
};

pub(crate) static AUDIO: Metadata = Metadata {
    name: "MessageAudio",
    fields: &field_names(&["audio"]),
    variant_fields: &["audio"],
};

pub(crate) static BOOST_ADDED: Metadata = Metadata {
    name: "MessageBoostAdded",
    fields: &field_names(&["boost_added"]),
    variant_fields: &["boost_added"],
};

pub(crate) static CHANNEL_CHAT_CREATED: Metadata = Metadata {
    name: "MessageChannelChatCreated",
    fields: &field_names(&["channel_chat_created"]),
    variant_fields: &["channel_chat_created"],
};

pub(crate) static CHAT_BACKGROUND_SET: Metadata = Metadata {
    name: "MessageChatBackgroundSet",
    fields: &field_names(&["chat_background_set"]),
    variant_fields: &["chat_background_set"],
};

pub(crate) static CHAT_OWNER_CHANGED: Metadata = Metadata {
    name: "MessageChatOwnerChanged",
    fields: &field_names(&["chat_owner_changed"]),
    variant_fields: &["chat_owner_changed"],
};

pub(crate) static CHAT_OWNER_LEFT: Metadata = Metadata {
    name: "MessageChatOwnerLeft",
    fields: &field_names(&["chat_owner_left"]),
    variant_fields: &["chat_owner_left"],
};

pub(crate) static CHAT_SHARED: Metadata = Metadata {
    name: "MessageChatShared",
    fields: &field_names(&["chat_shared"]),
    variant_fields: &["chat_shared"],
};

pub(crate) static CHECKLIST: Metadata = Metadata {
    name: "MessageChecklist",
    fields: &field_names(&["checklist"]),
    variant_fields: &["checklist"],
};

pub(crate) static CHECKLIST_TASKS_ADDED: Metadata = Metadata {
    name: "MessageChecklistTasksAdded",
    fields: &field_names(&["checklist_tasks_added"]),
    variant_fields: &["checklist_tasks_added"],
};

pub(crate) static CHECKLIST_TASKS_DONE: Metadata = Metadata {
    name: "MessageChecklistTasksDone",
    fields: &field_names(&["checklist_tasks_done"]),
    variant_fields: &["checklist_tasks_done"],
};

pub(crate) static COMMUNITY_CHAT_ADDED: Metadata = Metadata {
    name: "MessageCommunityChatAdded",
    fields: &field_names(&["community_chat_added"]),
    variant_fields: &["community_chat_added"],
};

pub(crate) static COMMUNITY_CHAT_JOINED: Metadata = Metadata {
    name: "MessageCommunityChatJoined",
    fields: &field_names(&["community_chat_joined"]),
    variant_fields: &["community_chat_joined"],
};

pub(crate) static COMMUNITY_CHAT_REMOVED: Metadata = Metadata {
    name: "MessageCommunityChatRemoved",
    fields: &field_names(&["community_chat_removed"]),
    variant_fields: &["community_chat_removed"],
};

pub(crate) static CONNECTED_WEBSITE: Metadata = Metadata {
    name: "MessageConnectedWebsite",
    fields: &field_names(&["connected_website"]),
    variant_fields: &["connected_website"],
};

pub(crate) static CONTACT: Metadata = Metadata {
    name: "MessageContact",
    fields: &field_names(&["contact"]),
    variant_fields: &["contact"],
};

pub(crate) static DELETE_CHAT_PHOTO: Metadata = Metadata {
    name: "MessageDeleteChatPhoto",
    fields: &field_names(&["delete_chat_photo"]),
    variant_fields: &["delete_chat_photo"],
};

pub(crate) static DICE: Metadata = Metadata {
    name: "MessageDice",
    fields: &field_names(&["dice"]),
    variant_fields: &["dice"],
};

pub(crate) static DIRECT_MESSAGE_PRICE_CHANGED: Metadata = Metadata {
    name: "MessageDirectMessagePriceChanged",
    fields: &field_names(&["direct_message_price_changed"]),
    variant_fields: &["direct_message_price_changed"],
};

pub(crate) static DOCUMENT: Metadata = Metadata {
    name: "MessageDocument",
    fields: &field_names(&["document"]),
    variant_fields: &["document"],
};

pub(crate) static FORUM_TOPIC_CLOSED: Metadata = Metadata {
    name: "MessageForumTopicClosed",
    fields: &field_names(&["forum_topic_closed"]),
    variant_fields: &["forum_topic_closed"],
};

pub(crate) static FORUM_TOPIC_CREATED: Metadata = Metadata {
    name: "MessageForumTopicCreated",
    fields: &field_names(&["forum_topic_created"]),
    variant_fields: &["forum_topic_created"],
};

pub(crate) static FORUM_TOPIC_EDITED: Metadata = Metadata {
    name: "MessageForumTopicEdited",
    fields: &field_names(&["forum_topic_edited"]),
    variant_fields: &["forum_topic_edited"],
};

pub(crate) static FORUM_TOPIC_REOPENED: Metadata = Metadata {
    name: "MessageForumTopicReopened",
    fields: &field_names(&["forum_topic_reopened"]),
    variant_fields: &["forum_topic_reopened"],
};

pub(crate) static GAME: Metadata = Metadata {
    name: "MessageGame",
    fields: &field_names(&["game"]),
    variant_fields: &["game"],
};

pub(crate) static GENERAL_FORUM_TOPIC_HIDDEN: Metadata = Metadata {
    name: "MessageGeneralForumTopicHidden",
    fields: &field_names(&["general_forum_topic_hidden"]),
    variant_fields: &["general_forum_topic_hidden"],
};

pub(crate) static GENERAL_FORUM_TOPIC_UNHIDDEN: Metadata = Metadata {
    name: "MessageGeneralForumTopicUnhidden",
    fields: &field_names(&["general_forum_topic_unhidden"]),
    variant_fields: &["general_forum_topic_unhidden"],
};

pub(crate) static GIFT: Metadata = Metadata {
    name: "MessageGift",
    fields: &field_names(&["gift"]),
    variant_fields: &["gift"],
};

pub(crate) static GIFT_UPGRADE_SENT: Metadata = Metadata {
    name: "MessageGiftUpgradeSent",
    fields: &field_names(&["gift_upgrade_sent"]),
    variant_fields: &["gift_upgrade_sent"],
};

pub(crate) static GIVEAWAY: Metadata = Metadata {
    name: "MessageGiveaway",
    fields: &field_names(&["giveaway"]),
    variant_fields: &["giveaway"],
};

pub(crate) static GIVEAWAY_COMPLETED: Metadata = Metadata {
    name: "MessageGiveawayCompleted",
    fields: &field_names(&["giveaway_completed"]),
    variant_fields: &["giveaway_completed"],
};

pub(crate) static GIVEAWAY_CREATED: Metadata = Metadata {
    name: "MessageGiveawayCreated",
    fields: &field_names(&["giveaway_created"]),
    variant_fields: &["giveaway_created"],
};

pub(crate) static GIVEAWAY_WINNERS: Metadata = Metadata {
    name: "MessageGiveawayWinners",
    fields: &field_names(&["giveaway_winners"]),
    variant_fields: &["giveaway_winners"],
};

pub(crate) static GROUP_CHAT_CREATED: Metadata = Metadata {
    name: "MessageGroupChatCreated",
    fields: &field_names(&["group_chat_created"]),
    variant_fields: &["group_chat_created"],
};

pub(crate) static INVOICE: Metadata = Metadata {
    name: "MessageInvoice",
    fields: &field_names(&["invoice"]),
    variant_fields: &["invoice"],
};

pub(crate) static LEFT_CHAT_MEMBER: Metadata = Metadata {
    name: "MessageLeftChatMember",
    fields: &field_names(&["left_chat_member"]),
    variant_fields: &["left_chat_member"],
};

pub(crate) static LIVE_PHOTO: Metadata = Metadata {
    name: "MessageLivePhoto",
    fields: &field_names(&["live_photo"]),
    variant_fields: &["live_photo"],
};

pub(crate) static LOCATION: Metadata = Metadata {
    name: "MessageLocation",
    fields: &field_names(&["location"]),
    variant_fields: &["location"],
};

pub(crate) static MANAGED_BOT_CREATED: Metadata = Metadata {
    name: "MessageManagedBotCreated",
    fields: &field_names(&["managed_bot_created"]),
    variant_fields: &["managed_bot_created"],
};

pub(crate) static MESSAGE_AUTO_DELETE_TIMER_CHANGED: Metadata = Metadata {
    name: "MessageMessageAutoDeleteTimerChanged",
    fields: &field_names(&["message_auto_delete_timer_changed"]),
    variant_fields: &["message_auto_delete_timer_changed"],
};

pub(crate) static MIGRATE_FROM_CHAT_ID: Metadata = Metadata {
    name: "MessageMigrateFromChatId",
    fields: &field_names(&["migrate_from_chat_id"]),
    variant_fields: &["migrate_from_chat_id"],
};

pub(crate) static MIGRATE_TO_CHAT_ID: Metadata = Metadata {
    name: "MessageMigrateToChatId",
    fields: &field_names(&["migrate_to_chat_id"]),
    variant_fields: &["migrate_to_chat_id"],
};

pub(crate) static NEW_CHAT_MEMBERS: Metadata = Metadata {
    name: "MessageNewChatMembers",
    fields: &field_names(&["new_chat_members"]),
    variant_fields: &["new_chat_members"],
};

pub(crate) static NEW_CHAT_PHOTO: Metadata = Metadata {
    name: "MessageNewChatPhoto",
    fields: &field_names(&["new_chat_photo"]),
    variant_fields: &["new_chat_photo"],
};

pub(crate) static NEW_CHAT_TITLE: Metadata = Metadata {
    name: "MessageNewChatTitle",
    fields: &field_names(&["new_chat_title"]),
    variant_fields: &["new_chat_title"],
};

pub(crate) static PAID_MEDIA: Metadata = Metadata {
    name: "MessagePaidMedia",
    fields: &field_names(&["paid_media"]),
    variant_fields: &["paid_media"],
};

pub(crate) static PAID_MESSAGE_PRICE_CHANGED: Metadata = Metadata {
    name: "MessagePaidMessagePriceChanged",
    fields: &field_names(&["paid_message_price_changed"]),
    variant_fields: &["paid_message_price_changed"],
};

pub(crate) static PASSPORT_DATA: Metadata = Metadata {
    name: "MessagePassportData",
    fields: &field_names(&["passport_data"]),
    variant_fields: &["passport_data"],
};

pub(crate) static PHOTO: Metadata = Metadata {
    name: "MessagePhoto",
    fields: &field_names(&["photo"]),
    variant_fields: &["photo"],
};

pub(crate) static PINNED_MESSAGE: Metadata = Metadata {
    name: "MessagePinnedMessage",
    fields: &field_names(&["pinned_message"]),
    variant_fields: &["pinned_message"],
};

pub(crate) static POLL: Metadata = Metadata {
    name: "MessagePoll",
    fields: &field_names(&["poll"]),
    variant_fields: &["poll"],
};

pub(crate) static POLL_OPTION_ADDED: Metadata = Metadata {
    name: "MessagePollOptionAdded",
    fields: &field_names(&["poll_option_added"]),
    variant_fields: &["poll_option_added"],
};

pub(crate) static POLL_OPTION_DELETED: Metadata = Metadata {
    name: "MessagePollOptionDeleted",
    fields: &field_names(&["poll_option_deleted"]),
    variant_fields: &["poll_option_deleted"],
};

pub(crate) static PROXIMITY_ALERT_TRIGGERED: Metadata = Metadata {
    name: "MessageProximityAlertTriggered",
    fields: &field_names(&["proximity_alert_triggered"]),
    variant_fields: &["proximity_alert_triggered"],
};

pub(crate) static REFUNDED_PAYMENT: Metadata = Metadata {
    name: "MessageRefundedPayment",
    fields: &field_names(&["refunded_payment"]),
    variant_fields: &["refunded_payment"],
};

pub(crate) static RICH_MESSAGE: Metadata = Metadata {
    name: "MessageRichMessage",
    fields: &field_names(&["rich_message"]),
    variant_fields: &["rich_message"],
};

pub(crate) static STICKER: Metadata = Metadata {
    name: "MessageSticker",
    fields: &field_names(&["sticker"]),
    variant_fields: &["sticker"],
};

pub(crate) static STORY: Metadata = Metadata {
    name: "MessageStory",
    fields: &field_names(&["story"]),
    variant_fields: &["story"],
};

pub(crate) static SUCCESSFUL_PAYMENT: Metadata = Metadata {
    name: "MessageSuccessfulPayment",
    fields: &field_names(&["successful_payment"]),
    variant_fields: &["successful_payment"],
};

pub(crate) static SUGGESTED_POST_APPROVAL_FAILED: Metadata = Metadata {
    name: "MessageSuggestedPostApprovalFailed",
    fields: &field_names(&["suggested_post_approval_failed"]),
    variant_fields: &["suggested_post_approval_failed"],
};

pub(crate) static SUGGESTED_POST_APPROVED: Metadata = Metadata {
    name: "MessageSuggestedPostApproved",
    fields: &field_names(&["suggested_post_approved"]),
    variant_fields: &["suggested_post_approved"],
};

pub(crate) static SUGGESTED_POST_DECLINED: Metadata = Metadata {
    name: "MessageSuggestedPostDeclined",
    fields: &field_names(&["suggested_post_declined"]),
    variant_fields: &["suggested_post_declined"],
};

pub(crate) static SUGGESTED_POST_PAID: Metadata = Metadata {
    name: "MessageSuggestedPostPaid",
    fields: &field_names(&["suggested_post_paid"]),
    variant_fields: &["suggested_post_paid"],
};

pub(crate) static SUGGESTED_POST_REFUNDED: Metadata = Metadata {
    name: "MessageSuggestedPostRefunded",
    fields: &field_names(&["suggested_post_refunded"]),
    variant_fields: &["suggested_post_refunded"],
};

pub(crate) static SUPERGROUP_CHAT_CREATED: Metadata = Metadata {
    name: "MessageSupergroupChatCreated",
    fields: &field_names(&["supergroup_chat_created"]),
    variant_fields: &["supergroup_chat_created"],
};

pub(crate) static TEXT: Metadata = Metadata {
    name: "MessageText",
    fields: &field_names(&["text"]),
    variant_fields: &["text"],
};

pub(crate) static UNIQUE_GIFT: Metadata = Metadata {
    name: "MessageUniqueGift",
    fields: &field_names(&["unique_gift"]),
    variant_fields: &["unique_gift"],
};

pub(crate) static USERS_SHARED: Metadata = Metadata {
    name: "MessageUsersShared",
    fields: &field_names(&["users_shared"]),
    variant_fields: &["users_shared"],
};

pub(crate) static VENUE: Metadata = Metadata {
    name: "MessageVenue",
    fields: &field_names(&["venue"]),
    variant_fields: &["venue"],
};

pub(crate) static VIDEO: Metadata = Metadata {
    name: "MessageVideo",
    fields: &field_names(&["video"]),
    variant_fields: &["video"],
};

pub(crate) static VIDEO_CHAT_ENDED: Metadata = Metadata {
    name: "MessageVideoChatEnded",
    fields: &field_names(&["video_chat_ended"]),
    variant_fields: &["video_chat_ended"],
};

pub(crate) static VIDEO_CHAT_PARTICIPANTS_INVITED: Metadata = Metadata {
    name: "MessageVideoChatParticipantsInvited",
    fields: &field_names(&["video_chat_participants_invited"]),
    variant_fields: &["video_chat_participants_invited"],
};

pub(crate) static VIDEO_CHAT_SCHEDULED: Metadata = Metadata {
    name: "MessageVideoChatScheduled",
    fields: &field_names(&["video_chat_scheduled"]),
    variant_fields: &["video_chat_scheduled"],
};

pub(crate) static VIDEO_CHAT_STARTED: Metadata = Metadata {
    name: "MessageVideoChatStarted",
    fields: &field_names(&["video_chat_started"]),
    variant_fields: &["video_chat_started"],
};

pub(crate) static VIDEO_NOTE: Metadata = Metadata {
    name: "MessageVideoNote",
    fields: &field_names(&["video_note"]),
    variant_fields: &["video_note"],
};

pub(crate) static VOICE: Metadata = Metadata {
    name: "MessageVoice",
    fields: &field_names(&["voice"]),
    variant_fields: &["voice"],
};

pub(crate) static WEB_APP_DATA: Metadata = Metadata {
    name: "MessageWebAppData",
    fields: &field_names(&["web_app_data"]),
    variant_fields: &["web_app_data"],
};

pub(crate) static WRITE_ACCESS_ALLOWED: Metadata = Metadata {
    name: "MessageWriteAccessAllowed",
    fields: &field_names(&["write_access_allowed"]),
    variant_fields: &["write_access_allowed"],
};
