#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) update_id: i64,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    update_id: Option<i64>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 1;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "update_id" => Some(0),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"update_id" => Some(0),
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
            0 => read_map_field(&mut self.update_id, map, "update_id"),
            _ => unreachable!(),
        }
    }
}

const fn field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 2] {
    ["update_id", variant_fields[0]]
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
        let common_update_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_content = read_seq_field(&mut seq, 1, &expected)?;
        Ok(DecodedFields {
            common: CommonFields {
                update_id: common_update_id,
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
                1 => {
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
        let common_update_id = shared
            .update_id
            .ok_or_else(|| A::Error::missing_field("update_id"))?;
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(DecodedFields {
            common: CommonFields {
                update_id: common_update_id,
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

pub(crate) static BUSINESS_CONNECTION: Metadata = Metadata {
    name: "UpdateBusinessConnection",
    fields: &field_names(&["business_connection"]),
    variant_fields: &["business_connection"],
};

pub(crate) static BUSINESS_MESSAGE: Metadata = Metadata {
    name: "UpdateBusinessMessage",
    fields: &field_names(&["business_message"]),
    variant_fields: &["business_message"],
};

pub(crate) static CALLBACK_QUERY: Metadata = Metadata {
    name: "UpdateCallbackQuery",
    fields: &field_names(&["callback_query"]),
    variant_fields: &["callback_query"],
};

pub(crate) static CHANNEL_POST: Metadata = Metadata {
    name: "UpdateChannelPost",
    fields: &field_names(&["channel_post"]),
    variant_fields: &["channel_post"],
};

pub(crate) static CHAT_BOOST: Metadata = Metadata {
    name: "UpdateChatBoost",
    fields: &field_names(&["chat_boost"]),
    variant_fields: &["chat_boost"],
};

pub(crate) static CHAT_JOIN_REQUEST: Metadata = Metadata {
    name: "UpdateChatJoinRequest",
    fields: &field_names(&["chat_join_request"]),
    variant_fields: &["chat_join_request"],
};

pub(crate) static CHAT_MEMBER: Metadata = Metadata {
    name: "UpdateChatMember",
    fields: &field_names(&["chat_member"]),
    variant_fields: &["chat_member"],
};

pub(crate) static CHOSEN_INLINE_RESULT: Metadata = Metadata {
    name: "UpdateChosenInlineResult",
    fields: &field_names(&["chosen_inline_result"]),
    variant_fields: &["chosen_inline_result"],
};

pub(crate) static DELETED_BUSINESS_MESSAGES: Metadata = Metadata {
    name: "UpdateDeletedBusinessMessages",
    fields: &field_names(&["deleted_business_messages"]),
    variant_fields: &["deleted_business_messages"],
};

pub(crate) static EDITED_BUSINESS_MESSAGE: Metadata = Metadata {
    name: "UpdateEditedBusinessMessage",
    fields: &field_names(&["edited_business_message"]),
    variant_fields: &["edited_business_message"],
};

pub(crate) static EDITED_CHANNEL_POST: Metadata = Metadata {
    name: "UpdateEditedChannelPost",
    fields: &field_names(&["edited_channel_post"]),
    variant_fields: &["edited_channel_post"],
};

pub(crate) static EDITED_MESSAGE: Metadata = Metadata {
    name: "UpdateEditedMessage",
    fields: &field_names(&["edited_message"]),
    variant_fields: &["edited_message"],
};

pub(crate) static GUEST_MESSAGE: Metadata = Metadata {
    name: "UpdateGuestMessage",
    fields: &field_names(&["guest_message"]),
    variant_fields: &["guest_message"],
};

pub(crate) static INLINE_QUERY: Metadata = Metadata {
    name: "UpdateInlineQuery",
    fields: &field_names(&["inline_query"]),
    variant_fields: &["inline_query"],
};

pub(crate) static MANAGED_BOT: Metadata = Metadata {
    name: "UpdateManagedBot",
    fields: &field_names(&["managed_bot"]),
    variant_fields: &["managed_bot"],
};

pub(crate) static MESSAGE: Metadata = Metadata {
    name: "UpdateMessage",
    fields: &field_names(&["message"]),
    variant_fields: &["message"],
};

pub(crate) static MESSAGE_REACTION: Metadata = Metadata {
    name: "UpdateMessageReaction",
    fields: &field_names(&["message_reaction"]),
    variant_fields: &["message_reaction"],
};

pub(crate) static MESSAGE_REACTION_COUNT: Metadata = Metadata {
    name: "UpdateMessageReactionCount",
    fields: &field_names(&["message_reaction_count"]),
    variant_fields: &["message_reaction_count"],
};

pub(crate) static MY_CHAT_MEMBER: Metadata = Metadata {
    name: "UpdateMyChatMember",
    fields: &field_names(&["my_chat_member"]),
    variant_fields: &["my_chat_member"],
};

pub(crate) static POLL: Metadata = Metadata {
    name: "UpdatePoll",
    fields: &field_names(&["poll"]),
    variant_fields: &["poll"],
};

pub(crate) static POLL_ANSWER: Metadata = Metadata {
    name: "UpdatePollAnswer",
    fields: &field_names(&["poll_answer"]),
    variant_fields: &["poll_answer"],
};

pub(crate) static PRE_CHECKOUT_QUERY: Metadata = Metadata {
    name: "UpdatePreCheckoutQuery",
    fields: &field_names(&["pre_checkout_query"]),
    variant_fields: &["pre_checkout_query"],
};

pub(crate) static PURCHASED_PAID_MEDIA: Metadata = Metadata {
    name: "UpdatePurchasedPaidMedia",
    fields: &field_names(&["purchased_paid_media"]),
    variant_fields: &["purchased_paid_media"],
};

pub(crate) static REMOVED_CHAT_BOOST: Metadata = Metadata {
    name: "UpdateRemovedChatBoost",
    fields: &field_names(&["removed_chat_boost"]),
    variant_fields: &["removed_chat_boost"],
};

pub(crate) static SHIPPING_QUERY: Metadata = Metadata {
    name: "UpdateShippingQuery",
    fields: &field_names(&["shipping_query"]),
    variant_fields: &["shipping_query"],
};

pub(crate) static STOPPED_MESSAGE_GENERATION: Metadata = Metadata {
    name: "UpdateStoppedMessageGeneration",
    fields: &field_names(&["stopped_message_generation"]),
    variant_fields: &["stopped_message_generation"],
};

pub(crate) static SUBSCRIPTION: Metadata = Metadata {
    name: "UpdateSubscription",
    fields: &field_names(&["subscription"]),
    variant_fields: &["subscription"],
};
