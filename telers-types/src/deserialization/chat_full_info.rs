#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) accent_color_id: i64,
    pub(crate) accepted_gift_types: crate::types::AcceptedGiftTypes,
    pub(crate) available_reactions: Option<Box<[crate::types::ReactionType]>>,
    pub(crate) background_custom_emoji_id: Option<Box<str>>,
    pub(crate) community: Option<crate::types::Community>,
    pub(crate) guard_bot: Option<Box<crate::types::User>>,
    pub(crate) has_hidden_members: Option<bool>,
    pub(crate) has_protected_content: Option<bool>,
    pub(crate) has_visible_history: Option<bool>,
    pub(crate) id: i64,
    pub(crate) is_direct_messages: Option<bool>,
    pub(crate) max_reaction_count: i64,
    pub(crate) message_auto_delete_time: Option<i64>,
    pub(crate) paid_message_star_count: Option<i64>,
    pub(crate) photo: Option<crate::types::ChatPhoto>,
    pub(crate) pinned_message: Option<Box<crate::types::Message>>,
    pub(crate) profile_accent_color_id: Option<i64>,
    pub(crate) profile_background_custom_emoji_id: Option<Box<str>>,
    pub(crate) unique_gift_colors: Option<crate::types::UniqueGiftColors>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    accent_color_id: Option<i64>,
    accepted_gift_types: Option<crate::types::AcceptedGiftTypes>,
    available_reactions: Option<Option<Box<[crate::types::ReactionType]>>>,
    background_custom_emoji_id: Option<Option<Box<str>>>,
    community: Option<Option<crate::types::Community>>,
    guard_bot: Option<Option<Box<crate::types::User>>>,
    has_hidden_members: Option<Option<bool>>,
    has_protected_content: Option<Option<bool>>,
    has_visible_history: Option<Option<bool>>,
    id: Option<i64>,
    is_direct_messages: Option<Option<bool>>,
    max_reaction_count: Option<i64>,
    message_auto_delete_time: Option<Option<i64>>,
    paid_message_star_count: Option<Option<i64>>,
    photo: Option<Option<crate::types::ChatPhoto>>,
    pinned_message: Option<Option<Box<crate::types::Message>>>,
    profile_accent_color_id: Option<Option<i64>>,
    profile_background_custom_emoji_id: Option<Option<Box<str>>>,
    unique_gift_colors: Option<Option<crate::types::UniqueGiftColors>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 19;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "accent_color_id" => Some(0),
            "accepted_gift_types" => Some(1),
            "available_reactions" => Some(2),
            "background_custom_emoji_id" => Some(3),
            "community" => Some(4),
            "guard_bot" => Some(5),
            "has_hidden_members" => Some(6),
            "has_protected_content" => Some(7),
            "has_visible_history" => Some(8),
            "id" => Some(9),
            "is_direct_messages" => Some(10),
            "max_reaction_count" => Some(11),
            "message_auto_delete_time" => Some(12),
            "paid_message_star_count" => Some(13),
            "photo" => Some(14),
            "pinned_message" => Some(15),
            "profile_accent_color_id" => Some(16),
            "profile_background_custom_emoji_id" => Some(17),
            "unique_gift_colors" => Some(18),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"accent_color_id" => Some(0),
            b"accepted_gift_types" => Some(1),
            b"available_reactions" => Some(2),
            b"background_custom_emoji_id" => Some(3),
            b"community" => Some(4),
            b"guard_bot" => Some(5),
            b"has_hidden_members" => Some(6),
            b"has_protected_content" => Some(7),
            b"has_visible_history" => Some(8),
            b"id" => Some(9),
            b"is_direct_messages" => Some(10),
            b"max_reaction_count" => Some(11),
            b"message_auto_delete_time" => Some(12),
            b"paid_message_star_count" => Some(13),
            b"photo" => Some(14),
            b"pinned_message" => Some(15),
            b"profile_accent_color_id" => Some(16),
            b"profile_background_custom_emoji_id" => Some(17),
            b"unique_gift_colors" => Some(18),
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
            0 => read_map_field(&mut self.accent_color_id, map, "accent_color_id"),
            1 => read_map_field(&mut self.accepted_gift_types, map, "accepted_gift_types"),
            2 => read_map_field(&mut self.available_reactions, map, "available_reactions"),
            3 => read_map_field(
                &mut self.background_custom_emoji_id,
                map,
                "background_custom_emoji_id",
            ),
            4 => read_map_field(&mut self.community, map, "community"),
            5 => read_map_field(&mut self.guard_bot, map, "guard_bot"),
            6 => read_map_field(&mut self.has_hidden_members, map, "has_hidden_members"),
            7 => read_map_field(
                &mut self.has_protected_content,
                map,
                "has_protected_content",
            ),
            8 => read_map_field(&mut self.has_visible_history, map, "has_visible_history"),
            9 => read_map_field(&mut self.id, map, "id"),
            10 => read_map_field(&mut self.is_direct_messages, map, "is_direct_messages"),
            11 => read_map_field(&mut self.max_reaction_count, map, "max_reaction_count"),
            12 => read_map_field(
                &mut self.message_auto_delete_time,
                map,
                "message_auto_delete_time",
            ),
            13 => read_map_field(
                &mut self.paid_message_star_count,
                map,
                "paid_message_star_count",
            ),
            14 => read_map_field(&mut self.photo, map, "photo"),
            15 => read_map_field(&mut self.pinned_message, map, "pinned_message"),
            16 => read_map_field(
                &mut self.profile_accent_color_id,
                map,
                "profile_accent_color_id",
            ),
            17 => read_map_field(
                &mut self.profile_background_custom_emoji_id,
                map,
                "profile_background_custom_emoji_id",
            ),
            18 => read_map_field(&mut self.unique_gift_colors, map, "unique_gift_colors"),
            _ => unreachable!(),
        }
    }
}

const fn channel_field_names(variant_fields: &[&'static str; 7]) -> [&'static str; 26] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        "is_direct_messages",
        "accent_color_id",
        "max_reaction_count",
        "photo",
        variant_fields[2],
        "available_reactions",
        "background_custom_emoji_id",
        "profile_accent_color_id",
        "profile_background_custom_emoji_id",
        variant_fields[3],
        variant_fields[4],
        "pinned_message",
        "accepted_gift_types",
        variant_fields[5],
        "message_auto_delete_time",
        "has_hidden_members",
        "has_protected_content",
        "has_visible_history",
        variant_fields[6],
        "unique_gift_colors",
        "paid_message_star_count",
        "guard_bot",
        "community",
    ]
}

pub(crate) struct ChannelFields {
    pub(crate) common: CommonFields,
    pub(crate) title: Option<Box<str>>,
    pub(crate) username: Option<Box<str>>,
    pub(crate) active_usernames: Option<Box<[Box<str>]>>,
    pub(crate) description: Option<Box<str>>,
    pub(crate) invite_link: Option<Box<str>>,
    pub(crate) can_send_paid_media: Option<bool>,
    pub(crate) linked_chat_id: Option<i64>,
}

struct ChannelVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for ChannelVisitor {
    type Value = ChannelFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_title = read_seq_field(&mut seq, 1, &expected)?;
        let variant_username = read_seq_field(&mut seq, 2, &expected)?;
        let common_is_direct_messages = read_seq_field(&mut seq, 3, &expected)?;
        let common_accent_color_id = read_seq_field(&mut seq, 4, &expected)?;
        let common_max_reaction_count = read_seq_field(&mut seq, 5, &expected)?;
        let common_photo = read_seq_field(&mut seq, 6, &expected)?;
        let variant_active_usernames = read_seq_field(&mut seq, 7, &expected)?;
        let common_available_reactions = read_seq_field(&mut seq, 8, &expected)?;
        let common_background_custom_emoji_id = read_seq_field(&mut seq, 9, &expected)?;
        let common_profile_accent_color_id = read_seq_field(&mut seq, 10, &expected)?;
        let common_profile_background_custom_emoji_id = read_seq_field(&mut seq, 11, &expected)?;
        let variant_description = read_seq_field(&mut seq, 12, &expected)?;
        let variant_invite_link = read_seq_field(&mut seq, 13, &expected)?;
        let common_pinned_message = read_seq_field(&mut seq, 14, &expected)?;
        let common_accepted_gift_types = read_seq_field(&mut seq, 15, &expected)?;
        let variant_can_send_paid_media = read_seq_field(&mut seq, 16, &expected)?;
        let common_message_auto_delete_time = read_seq_field(&mut seq, 17, &expected)?;
        let common_has_hidden_members = read_seq_field(&mut seq, 18, &expected)?;
        let common_has_protected_content = read_seq_field(&mut seq, 19, &expected)?;
        let common_has_visible_history = read_seq_field(&mut seq, 20, &expected)?;
        let variant_linked_chat_id = read_seq_field(&mut seq, 21, &expected)?;
        let common_unique_gift_colors = read_seq_field(&mut seq, 22, &expected)?;
        let common_paid_message_star_count = read_seq_field(&mut seq, 23, &expected)?;
        let common_guard_bot = read_seq_field(&mut seq, 24, &expected)?;
        let common_community = read_seq_field(&mut seq, 25, &expected)?;
        Ok(ChannelFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            title: variant_title,
            username: variant_username,
            active_usernames: variant_active_usernames,
            description: variant_description,
            invite_link: variant_invite_link,
            can_send_paid_media: variant_can_send_paid_media,
            linked_chat_id: variant_linked_chat_id,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_title: Option<Option<Box<str>>> = None;
        let mut variant_username: Option<Option<Box<str>>> = None;
        let mut variant_active_usernames: Option<Option<Box<[Box<str>]>>> = None;
        let mut variant_description: Option<Option<Box<str>>> = None;
        let mut variant_invite_link: Option<Option<Box<str>>> = None;
        let mut variant_can_send_paid_media: Option<Option<bool>> = None;
        let mut variant_linked_chat_id: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                19 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                20 => {
                    read_map_field(
                        &mut variant_username,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                21 => {
                    read_map_field(
                        &mut variant_active_usernames,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                22 => {
                    read_map_field(
                        &mut variant_description,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                23 => {
                    read_map_field(
                        &mut variant_invite_link,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                24 => {
                    read_map_field(
                        &mut variant_can_send_paid_media,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                25 => {
                    read_map_field(
                        &mut variant_linked_chat_id,
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
        let variant_title = variant_title.unwrap_or_default();
        let variant_username = variant_username.unwrap_or_default();
        let common_is_direct_messages = shared.is_direct_messages.unwrap_or(None);
        let common_accent_color_id = shared
            .accent_color_id
            .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
        let common_max_reaction_count = shared
            .max_reaction_count
            .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
        let common_photo = shared.photo.unwrap_or(None);
        let variant_active_usernames = variant_active_usernames.unwrap_or_default();
        let common_available_reactions = shared.available_reactions.unwrap_or(None);
        let common_background_custom_emoji_id = shared.background_custom_emoji_id.unwrap_or(None);
        let common_profile_accent_color_id = shared.profile_accent_color_id.unwrap_or(None);
        let common_profile_background_custom_emoji_id =
            shared.profile_background_custom_emoji_id.unwrap_or(None);
        let variant_description = variant_description.unwrap_or_default();
        let variant_invite_link = variant_invite_link.unwrap_or_default();
        let common_pinned_message = shared.pinned_message.unwrap_or(None);
        let common_accepted_gift_types = shared
            .accepted_gift_types
            .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
        let variant_can_send_paid_media = variant_can_send_paid_media.unwrap_or_default();
        let common_message_auto_delete_time = shared.message_auto_delete_time.unwrap_or(None);
        let common_has_hidden_members = shared.has_hidden_members.unwrap_or(None);
        let common_has_protected_content = shared.has_protected_content.unwrap_or(None);
        let common_has_visible_history = shared.has_visible_history.unwrap_or(None);
        let variant_linked_chat_id = variant_linked_chat_id.unwrap_or_default();
        let common_unique_gift_colors = shared.unique_gift_colors.unwrap_or(None);
        let common_paid_message_star_count = shared.paid_message_star_count.unwrap_or(None);
        let common_guard_bot = shared.guard_bot.unwrap_or(None);
        let common_community = shared.community.unwrap_or(None);
        Ok(ChannelFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            title: variant_title,
            username: variant_username,
            active_usernames: variant_active_usernames,
            description: variant_description,
            invite_link: variant_invite_link,
            can_send_paid_media: variant_can_send_paid_media,
            linked_chat_id: variant_linked_chat_id,
        })
    }
}

pub(crate) fn deserialize_channel<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ChannelFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ChannelVisitor {
            metadata,
        },
    )
}

const fn group_field_names(variant_fields: &[&'static str; 4]) -> [&'static str; 23] {
    [
        "id",
        variant_fields[0],
        "is_direct_messages",
        "accent_color_id",
        "max_reaction_count",
        "photo",
        "available_reactions",
        "background_custom_emoji_id",
        "profile_accent_color_id",
        "profile_background_custom_emoji_id",
        variant_fields[1],
        variant_fields[2],
        "pinned_message",
        variant_fields[3],
        "accepted_gift_types",
        "message_auto_delete_time",
        "has_hidden_members",
        "has_protected_content",
        "has_visible_history",
        "unique_gift_colors",
        "paid_message_star_count",
        "guard_bot",
        "community",
    ]
}

pub(crate) struct GroupFields {
    pub(crate) common: CommonFields,
    pub(crate) title: Option<Box<str>>,
    pub(crate) description: Option<Box<str>>,
    pub(crate) invite_link: Option<Box<str>>,
    pub(crate) permissions: Option<crate::types::ChatPermissions>,
}

struct GroupVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for GroupVisitor {
    type Value = GroupFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_title = read_seq_field(&mut seq, 1, &expected)?;
        let common_is_direct_messages = read_seq_field(&mut seq, 2, &expected)?;
        let common_accent_color_id = read_seq_field(&mut seq, 3, &expected)?;
        let common_max_reaction_count = read_seq_field(&mut seq, 4, &expected)?;
        let common_photo = read_seq_field(&mut seq, 5, &expected)?;
        let common_available_reactions = read_seq_field(&mut seq, 6, &expected)?;
        let common_background_custom_emoji_id = read_seq_field(&mut seq, 7, &expected)?;
        let common_profile_accent_color_id = read_seq_field(&mut seq, 8, &expected)?;
        let common_profile_background_custom_emoji_id = read_seq_field(&mut seq, 9, &expected)?;
        let variant_description = read_seq_field(&mut seq, 10, &expected)?;
        let variant_invite_link = read_seq_field(&mut seq, 11, &expected)?;
        let common_pinned_message = read_seq_field(&mut seq, 12, &expected)?;
        let variant_permissions = read_seq_field(&mut seq, 13, &expected)?;
        let common_accepted_gift_types = read_seq_field(&mut seq, 14, &expected)?;
        let common_message_auto_delete_time = read_seq_field(&mut seq, 15, &expected)?;
        let common_has_hidden_members = read_seq_field(&mut seq, 16, &expected)?;
        let common_has_protected_content = read_seq_field(&mut seq, 17, &expected)?;
        let common_has_visible_history = read_seq_field(&mut seq, 18, &expected)?;
        let common_unique_gift_colors = read_seq_field(&mut seq, 19, &expected)?;
        let common_paid_message_star_count = read_seq_field(&mut seq, 20, &expected)?;
        let common_guard_bot = read_seq_field(&mut seq, 21, &expected)?;
        let common_community = read_seq_field(&mut seq, 22, &expected)?;
        Ok(GroupFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            title: variant_title,
            description: variant_description,
            invite_link: variant_invite_link,
            permissions: variant_permissions,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_title: Option<Option<Box<str>>> = None;
        let mut variant_description: Option<Option<Box<str>>> = None;
        let mut variant_invite_link: Option<Option<Box<str>>> = None;
        let mut variant_permissions: Option<Option<crate::types::ChatPermissions>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                19 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                20 => {
                    read_map_field(
                        &mut variant_description,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                21 => {
                    read_map_field(
                        &mut variant_invite_link,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                22 => {
                    read_map_field(
                        &mut variant_permissions,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_title = variant_title.unwrap_or_default();
        let common_is_direct_messages = shared.is_direct_messages.unwrap_or(None);
        let common_accent_color_id = shared
            .accent_color_id
            .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
        let common_max_reaction_count = shared
            .max_reaction_count
            .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
        let common_photo = shared.photo.unwrap_or(None);
        let common_available_reactions = shared.available_reactions.unwrap_or(None);
        let common_background_custom_emoji_id = shared.background_custom_emoji_id.unwrap_or(None);
        let common_profile_accent_color_id = shared.profile_accent_color_id.unwrap_or(None);
        let common_profile_background_custom_emoji_id =
            shared.profile_background_custom_emoji_id.unwrap_or(None);
        let variant_description = variant_description.unwrap_or_default();
        let variant_invite_link = variant_invite_link.unwrap_or_default();
        let common_pinned_message = shared.pinned_message.unwrap_or(None);
        let variant_permissions = variant_permissions.unwrap_or_default();
        let common_accepted_gift_types = shared
            .accepted_gift_types
            .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
        let common_message_auto_delete_time = shared.message_auto_delete_time.unwrap_or(None);
        let common_has_hidden_members = shared.has_hidden_members.unwrap_or(None);
        let common_has_protected_content = shared.has_protected_content.unwrap_or(None);
        let common_has_visible_history = shared.has_visible_history.unwrap_or(None);
        let common_unique_gift_colors = shared.unique_gift_colors.unwrap_or(None);
        let common_paid_message_star_count = shared.paid_message_star_count.unwrap_or(None);
        let common_guard_bot = shared.guard_bot.unwrap_or(None);
        let common_community = shared.community.unwrap_or(None);
        Ok(GroupFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            title: variant_title,
            description: variant_description,
            invite_link: variant_invite_link,
            permissions: variant_permissions,
        })
    }
}

pub(crate) fn deserialize_group<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<GroupFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        GroupVisitor {
            metadata,
        },
    )
}

const fn private_field_names(variant_fields: &[&'static str; 16]) -> [&'static str; 35] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        "is_direct_messages",
        "accent_color_id",
        "max_reaction_count",
        "photo",
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
        variant_fields[8],
        "available_reactions",
        "background_custom_emoji_id",
        "profile_accent_color_id",
        "profile_background_custom_emoji_id",
        variant_fields[9],
        variant_fields[10],
        variant_fields[11],
        variant_fields[12],
        variant_fields[13],
        "pinned_message",
        "accepted_gift_types",
        "message_auto_delete_time",
        "has_hidden_members",
        "has_protected_content",
        "has_visible_history",
        variant_fields[14],
        variant_fields[15],
        "unique_gift_colors",
        "paid_message_star_count",
        "guard_bot",
        "community",
    ]
}

pub(crate) struct PrivateFields {
    pub(crate) common: CommonFields,
    pub(crate) username: Option<Box<str>>,
    pub(crate) first_name: Option<Box<str>>,
    pub(crate) last_name: Option<Box<str>>,
    pub(crate) active_usernames: Option<Box<[Box<str>]>>,
    pub(crate) birthdate: Option<crate::types::Birthdate>,
    pub(crate) business_intro: Option<crate::types::BusinessIntro>,
    pub(crate) business_location: Option<crate::types::BusinessLocation>,
    pub(crate) business_opening_hours: Option<crate::types::BusinessOpeningHours>,
    pub(crate) personal_chat: Option<Box<crate::types::Chat>>,
    pub(crate) emoji_status_custom_emoji_id: Option<Box<str>>,
    pub(crate) emoji_status_expiration_date: Option<i64>,
    pub(crate) bio: Option<Box<str>>,
    pub(crate) has_private_forwards: Option<bool>,
    pub(crate) has_restricted_voice_and_video_messages: Option<bool>,
    pub(crate) rating: Option<crate::types::UserRating>,
    pub(crate) first_profile_audio: Option<Box<crate::types::Audio>>,
}

struct PrivateVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for PrivateVisitor {
    type Value = PrivateFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_username = read_seq_field(&mut seq, 1, &expected)?;
        let variant_first_name = read_seq_field(&mut seq, 2, &expected)?;
        let variant_last_name = read_seq_field(&mut seq, 3, &expected)?;
        let common_is_direct_messages = read_seq_field(&mut seq, 4, &expected)?;
        let common_accent_color_id = read_seq_field(&mut seq, 5, &expected)?;
        let common_max_reaction_count = read_seq_field(&mut seq, 6, &expected)?;
        let common_photo = read_seq_field(&mut seq, 7, &expected)?;
        let variant_active_usernames = read_seq_field(&mut seq, 8, &expected)?;
        let variant_birthdate = read_seq_field(&mut seq, 9, &expected)?;
        let variant_business_intro = read_seq_field(&mut seq, 10, &expected)?;
        let variant_business_location = read_seq_field(&mut seq, 11, &expected)?;
        let variant_business_opening_hours = read_seq_field(&mut seq, 12, &expected)?;
        let variant_personal_chat = read_seq_field(&mut seq, 13, &expected)?;
        let common_available_reactions = read_seq_field(&mut seq, 14, &expected)?;
        let common_background_custom_emoji_id = read_seq_field(&mut seq, 15, &expected)?;
        let common_profile_accent_color_id = read_seq_field(&mut seq, 16, &expected)?;
        let common_profile_background_custom_emoji_id = read_seq_field(&mut seq, 17, &expected)?;
        let variant_emoji_status_custom_emoji_id = read_seq_field(&mut seq, 18, &expected)?;
        let variant_emoji_status_expiration_date = read_seq_field(&mut seq, 19, &expected)?;
        let variant_bio = read_seq_field(&mut seq, 20, &expected)?;
        let variant_has_private_forwards = read_seq_field(&mut seq, 21, &expected)?;
        let variant_has_restricted_voice_and_video_messages =
            read_seq_field(&mut seq, 22, &expected)?;
        let common_pinned_message = read_seq_field(&mut seq, 23, &expected)?;
        let common_accepted_gift_types = read_seq_field(&mut seq, 24, &expected)?;
        let common_message_auto_delete_time = read_seq_field(&mut seq, 25, &expected)?;
        let common_has_hidden_members = read_seq_field(&mut seq, 26, &expected)?;
        let common_has_protected_content = read_seq_field(&mut seq, 27, &expected)?;
        let common_has_visible_history = read_seq_field(&mut seq, 28, &expected)?;
        let variant_rating = read_seq_field(&mut seq, 29, &expected)?;
        let variant_first_profile_audio = read_seq_field(&mut seq, 30, &expected)?;
        let common_unique_gift_colors = read_seq_field(&mut seq, 31, &expected)?;
        let common_paid_message_star_count = read_seq_field(&mut seq, 32, &expected)?;
        let common_guard_bot = read_seq_field(&mut seq, 33, &expected)?;
        let common_community = read_seq_field(&mut seq, 34, &expected)?;
        Ok(PrivateFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            username: variant_username,
            first_name: variant_first_name,
            last_name: variant_last_name,
            active_usernames: variant_active_usernames,
            birthdate: variant_birthdate,
            business_intro: variant_business_intro,
            business_location: variant_business_location,
            business_opening_hours: variant_business_opening_hours,
            personal_chat: variant_personal_chat,
            emoji_status_custom_emoji_id: variant_emoji_status_custom_emoji_id,
            emoji_status_expiration_date: variant_emoji_status_expiration_date,
            bio: variant_bio,
            has_private_forwards: variant_has_private_forwards,
            has_restricted_voice_and_video_messages:
                variant_has_restricted_voice_and_video_messages,
            rating: variant_rating,
            first_profile_audio: variant_first_profile_audio,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_username: Option<Option<Box<str>>> = None;
        let mut variant_first_name: Option<Option<Box<str>>> = None;
        let mut variant_last_name: Option<Option<Box<str>>> = None;
        let mut variant_active_usernames: Option<Option<Box<[Box<str>]>>> = None;
        let mut variant_birthdate: Option<Option<crate::types::Birthdate>> = None;
        let mut variant_business_intro: Option<Option<crate::types::BusinessIntro>> = None;
        let mut variant_business_location: Option<Option<crate::types::BusinessLocation>> = None;
        let mut variant_business_opening_hours: Option<Option<crate::types::BusinessOpeningHours>> =
            None;
        let mut variant_personal_chat: Option<Option<Box<crate::types::Chat>>> = None;
        let mut variant_emoji_status_custom_emoji_id: Option<Option<Box<str>>> = None;
        let mut variant_emoji_status_expiration_date: Option<Option<i64>> = None;
        let mut variant_bio: Option<Option<Box<str>>> = None;
        let mut variant_has_private_forwards: Option<Option<bool>> = None;
        let mut variant_has_restricted_voice_and_video_messages: Option<Option<bool>> = None;
        let mut variant_rating: Option<Option<crate::types::UserRating>> = None;
        let mut variant_first_profile_audio: Option<Option<Box<crate::types::Audio>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                19 => {
                    read_map_field(
                        &mut variant_username,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                20 => {
                    read_map_field(
                        &mut variant_first_name,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                21 => {
                    read_map_field(
                        &mut variant_last_name,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                22 => {
                    read_map_field(
                        &mut variant_active_usernames,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                23 => {
                    read_map_field(
                        &mut variant_birthdate,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                24 => {
                    read_map_field(
                        &mut variant_business_intro,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                25 => {
                    read_map_field(
                        &mut variant_business_location,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                26 => {
                    read_map_field(
                        &mut variant_business_opening_hours,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                27 => {
                    read_map_field(
                        &mut variant_personal_chat,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                28 => {
                    read_map_field(
                        &mut variant_emoji_status_custom_emoji_id,
                        &mut map,
                        self.metadata.variant_fields[9],
                    )?;
                }
                29 => {
                    read_map_field(
                        &mut variant_emoji_status_expiration_date,
                        &mut map,
                        self.metadata.variant_fields[10],
                    )?;
                }
                30 => {
                    read_map_field(&mut variant_bio, &mut map, self.metadata.variant_fields[11])?;
                }
                31 => {
                    read_map_field(
                        &mut variant_has_private_forwards,
                        &mut map,
                        self.metadata.variant_fields[12],
                    )?;
                }
                32 => {
                    read_map_field(
                        &mut variant_has_restricted_voice_and_video_messages,
                        &mut map,
                        self.metadata.variant_fields[13],
                    )?;
                }
                33 => {
                    read_map_field(
                        &mut variant_rating,
                        &mut map,
                        self.metadata.variant_fields[14],
                    )?;
                }
                34 => {
                    read_map_field(
                        &mut variant_first_profile_audio,
                        &mut map,
                        self.metadata.variant_fields[15],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_username = variant_username.unwrap_or_default();
        let variant_first_name = variant_first_name.unwrap_or_default();
        let variant_last_name = variant_last_name.unwrap_or_default();
        let common_is_direct_messages = shared.is_direct_messages.unwrap_or(None);
        let common_accent_color_id = shared
            .accent_color_id
            .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
        let common_max_reaction_count = shared
            .max_reaction_count
            .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
        let common_photo = shared.photo.unwrap_or(None);
        let variant_active_usernames = variant_active_usernames.unwrap_or_default();
        let variant_birthdate = variant_birthdate.unwrap_or_default();
        let variant_business_intro = variant_business_intro.unwrap_or_default();
        let variant_business_location = variant_business_location.unwrap_or_default();
        let variant_business_opening_hours = variant_business_opening_hours.unwrap_or_default();
        let variant_personal_chat = variant_personal_chat.unwrap_or_default();
        let common_available_reactions = shared.available_reactions.unwrap_or(None);
        let common_background_custom_emoji_id = shared.background_custom_emoji_id.unwrap_or(None);
        let common_profile_accent_color_id = shared.profile_accent_color_id.unwrap_or(None);
        let common_profile_background_custom_emoji_id =
            shared.profile_background_custom_emoji_id.unwrap_or(None);
        let variant_emoji_status_custom_emoji_id =
            variant_emoji_status_custom_emoji_id.unwrap_or_default();
        let variant_emoji_status_expiration_date =
            variant_emoji_status_expiration_date.unwrap_or_default();
        let variant_bio = variant_bio.unwrap_or_default();
        let variant_has_private_forwards = variant_has_private_forwards.unwrap_or_default();
        let variant_has_restricted_voice_and_video_messages =
            variant_has_restricted_voice_and_video_messages.unwrap_or_default();
        let common_pinned_message = shared.pinned_message.unwrap_or(None);
        let common_accepted_gift_types = shared
            .accepted_gift_types
            .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
        let common_message_auto_delete_time = shared.message_auto_delete_time.unwrap_or(None);
        let common_has_hidden_members = shared.has_hidden_members.unwrap_or(None);
        let common_has_protected_content = shared.has_protected_content.unwrap_or(None);
        let common_has_visible_history = shared.has_visible_history.unwrap_or(None);
        let variant_rating = variant_rating.unwrap_or_default();
        let variant_first_profile_audio = variant_first_profile_audio.unwrap_or_default();
        let common_unique_gift_colors = shared.unique_gift_colors.unwrap_or(None);
        let common_paid_message_star_count = shared.paid_message_star_count.unwrap_or(None);
        let common_guard_bot = shared.guard_bot.unwrap_or(None);
        let common_community = shared.community.unwrap_or(None);
        Ok(PrivateFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            username: variant_username,
            first_name: variant_first_name,
            last_name: variant_last_name,
            active_usernames: variant_active_usernames,
            birthdate: variant_birthdate,
            business_intro: variant_business_intro,
            business_location: variant_business_location,
            business_opening_hours: variant_business_opening_hours,
            personal_chat: variant_personal_chat,
            emoji_status_custom_emoji_id: variant_emoji_status_custom_emoji_id,
            emoji_status_expiration_date: variant_emoji_status_expiration_date,
            bio: variant_bio,
            has_private_forwards: variant_has_private_forwards,
            has_restricted_voice_and_video_messages:
                variant_has_restricted_voice_and_video_messages,
            rating: variant_rating,
            first_profile_audio: variant_first_profile_audio,
        })
    }
}

pub(crate) fn deserialize_private<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<PrivateFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        PrivateVisitor {
            metadata,
        },
    )
}

const fn supergroup_field_names(variant_fields: &[&'static str; 18]) -> [&'static str; 37] {
    [
        "id",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        "is_direct_messages",
        "accent_color_id",
        "max_reaction_count",
        "photo",
        variant_fields[3],
        variant_fields[4],
        "available_reactions",
        "background_custom_emoji_id",
        "profile_accent_color_id",
        "profile_background_custom_emoji_id",
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
        variant_fields[8],
        "pinned_message",
        variant_fields[9],
        "accepted_gift_types",
        variant_fields[10],
        variant_fields[11],
        "message_auto_delete_time",
        variant_fields[12],
        "has_hidden_members",
        "has_protected_content",
        "has_visible_history",
        variant_fields[13],
        variant_fields[14],
        variant_fields[15],
        variant_fields[16],
        variant_fields[17],
        "unique_gift_colors",
        "paid_message_star_count",
        "guard_bot",
        "community",
    ]
}

pub(crate) struct SupergroupFields {
    pub(crate) common: CommonFields,
    pub(crate) title: Option<Box<str>>,
    pub(crate) username: Option<Box<str>>,
    pub(crate) is_forum: Option<bool>,
    pub(crate) active_usernames: Option<Box<[Box<str>]>>,
    pub(crate) parent_chat: Option<Box<crate::types::Chat>>,
    pub(crate) join_to_send_messages: Option<bool>,
    pub(crate) join_by_request: Option<bool>,
    pub(crate) description: Option<Box<str>>,
    pub(crate) invite_link: Option<Box<str>>,
    pub(crate) permissions: Option<crate::types::ChatPermissions>,
    pub(crate) slow_mode_delay: Option<i64>,
    pub(crate) unrestrict_boost_count: Option<i64>,
    pub(crate) has_aggressive_anti_spam_enabled: Option<bool>,
    pub(crate) sticker_set_name: Option<Box<str>>,
    pub(crate) can_set_sticker_set: Option<bool>,
    pub(crate) custom_emoji_sticker_set_name: Option<Box<str>>,
    pub(crate) linked_chat_id: Option<i64>,
    pub(crate) location: Option<crate::types::ChatLocation>,
}

struct SupergroupVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for SupergroupVisitor {
    type Value = SupergroupFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_id = read_seq_field(&mut seq, 0, &expected)?;
        let variant_title = read_seq_field(&mut seq, 1, &expected)?;
        let variant_username = read_seq_field(&mut seq, 2, &expected)?;
        let variant_is_forum = read_seq_field(&mut seq, 3, &expected)?;
        let common_is_direct_messages = read_seq_field(&mut seq, 4, &expected)?;
        let common_accent_color_id = read_seq_field(&mut seq, 5, &expected)?;
        let common_max_reaction_count = read_seq_field(&mut seq, 6, &expected)?;
        let common_photo = read_seq_field(&mut seq, 7, &expected)?;
        let variant_active_usernames = read_seq_field(&mut seq, 8, &expected)?;
        let variant_parent_chat = read_seq_field(&mut seq, 9, &expected)?;
        let common_available_reactions = read_seq_field(&mut seq, 10, &expected)?;
        let common_background_custom_emoji_id = read_seq_field(&mut seq, 11, &expected)?;
        let common_profile_accent_color_id = read_seq_field(&mut seq, 12, &expected)?;
        let common_profile_background_custom_emoji_id = read_seq_field(&mut seq, 13, &expected)?;
        let variant_join_to_send_messages = read_seq_field(&mut seq, 14, &expected)?;
        let variant_join_by_request = read_seq_field(&mut seq, 15, &expected)?;
        let variant_description = read_seq_field(&mut seq, 16, &expected)?;
        let variant_invite_link = read_seq_field(&mut seq, 17, &expected)?;
        let common_pinned_message = read_seq_field(&mut seq, 18, &expected)?;
        let variant_permissions = read_seq_field(&mut seq, 19, &expected)?;
        let common_accepted_gift_types = read_seq_field(&mut seq, 20, &expected)?;
        let variant_slow_mode_delay = read_seq_field(&mut seq, 21, &expected)?;
        let variant_unrestrict_boost_count = read_seq_field(&mut seq, 22, &expected)?;
        let common_message_auto_delete_time = read_seq_field(&mut seq, 23, &expected)?;
        let variant_has_aggressive_anti_spam_enabled = read_seq_field(&mut seq, 24, &expected)?;
        let common_has_hidden_members = read_seq_field(&mut seq, 25, &expected)?;
        let common_has_protected_content = read_seq_field(&mut seq, 26, &expected)?;
        let common_has_visible_history = read_seq_field(&mut seq, 27, &expected)?;
        let variant_sticker_set_name = read_seq_field(&mut seq, 28, &expected)?;
        let variant_can_set_sticker_set = read_seq_field(&mut seq, 29, &expected)?;
        let variant_custom_emoji_sticker_set_name = read_seq_field(&mut seq, 30, &expected)?;
        let variant_linked_chat_id = read_seq_field(&mut seq, 31, &expected)?;
        let variant_location = read_seq_field(&mut seq, 32, &expected)?;
        let common_unique_gift_colors = read_seq_field(&mut seq, 33, &expected)?;
        let common_paid_message_star_count = read_seq_field(&mut seq, 34, &expected)?;
        let common_guard_bot = read_seq_field(&mut seq, 35, &expected)?;
        let common_community = read_seq_field(&mut seq, 36, &expected)?;
        Ok(SupergroupFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            title: variant_title,
            username: variant_username,
            is_forum: variant_is_forum,
            active_usernames: variant_active_usernames,
            parent_chat: variant_parent_chat,
            join_to_send_messages: variant_join_to_send_messages,
            join_by_request: variant_join_by_request,
            description: variant_description,
            invite_link: variant_invite_link,
            permissions: variant_permissions,
            slow_mode_delay: variant_slow_mode_delay,
            unrestrict_boost_count: variant_unrestrict_boost_count,
            has_aggressive_anti_spam_enabled: variant_has_aggressive_anti_spam_enabled,
            sticker_set_name: variant_sticker_set_name,
            can_set_sticker_set: variant_can_set_sticker_set,
            custom_emoji_sticker_set_name: variant_custom_emoji_sticker_set_name,
            linked_chat_id: variant_linked_chat_id,
            location: variant_location,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_title: Option<Option<Box<str>>> = None;
        let mut variant_username: Option<Option<Box<str>>> = None;
        let mut variant_is_forum: Option<Option<bool>> = None;
        let mut variant_active_usernames: Option<Option<Box<[Box<str>]>>> = None;
        let mut variant_parent_chat: Option<Option<Box<crate::types::Chat>>> = None;
        let mut variant_join_to_send_messages: Option<Option<bool>> = None;
        let mut variant_join_by_request: Option<Option<bool>> = None;
        let mut variant_description: Option<Option<Box<str>>> = None;
        let mut variant_invite_link: Option<Option<Box<str>>> = None;
        let mut variant_permissions: Option<Option<crate::types::ChatPermissions>> = None;
        let mut variant_slow_mode_delay: Option<Option<i64>> = None;
        let mut variant_unrestrict_boost_count: Option<Option<i64>> = None;
        let mut variant_has_aggressive_anti_spam_enabled: Option<Option<bool>> = None;
        let mut variant_sticker_set_name: Option<Option<Box<str>>> = None;
        let mut variant_can_set_sticker_set: Option<Option<bool>> = None;
        let mut variant_custom_emoji_sticker_set_name: Option<Option<Box<str>>> = None;
        let mut variant_linked_chat_id: Option<Option<i64>> = None;
        let mut variant_location: Option<Option<crate::types::ChatLocation>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                19 => {
                    read_map_field(
                        &mut variant_title,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                20 => {
                    read_map_field(
                        &mut variant_username,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                21 => {
                    read_map_field(
                        &mut variant_is_forum,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                22 => {
                    read_map_field(
                        &mut variant_active_usernames,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                23 => {
                    read_map_field(
                        &mut variant_parent_chat,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                24 => {
                    read_map_field(
                        &mut variant_join_to_send_messages,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                25 => {
                    read_map_field(
                        &mut variant_join_by_request,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                26 => {
                    read_map_field(
                        &mut variant_description,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                27 => {
                    read_map_field(
                        &mut variant_invite_link,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                28 => {
                    read_map_field(
                        &mut variant_permissions,
                        &mut map,
                        self.metadata.variant_fields[9],
                    )?;
                }
                29 => {
                    read_map_field(
                        &mut variant_slow_mode_delay,
                        &mut map,
                        self.metadata.variant_fields[10],
                    )?;
                }
                30 => {
                    read_map_field(
                        &mut variant_unrestrict_boost_count,
                        &mut map,
                        self.metadata.variant_fields[11],
                    )?;
                }
                31 => {
                    read_map_field(
                        &mut variant_has_aggressive_anti_spam_enabled,
                        &mut map,
                        self.metadata.variant_fields[12],
                    )?;
                }
                32 => {
                    read_map_field(
                        &mut variant_sticker_set_name,
                        &mut map,
                        self.metadata.variant_fields[13],
                    )?;
                }
                33 => {
                    read_map_field(
                        &mut variant_can_set_sticker_set,
                        &mut map,
                        self.metadata.variant_fields[14],
                    )?;
                }
                34 => {
                    read_map_field(
                        &mut variant_custom_emoji_sticker_set_name,
                        &mut map,
                        self.metadata.variant_fields[15],
                    )?;
                }
                35 => {
                    read_map_field(
                        &mut variant_linked_chat_id,
                        &mut map,
                        self.metadata.variant_fields[16],
                    )?;
                }
                36 => {
                    read_map_field(
                        &mut variant_location,
                        &mut map,
                        self.metadata.variant_fields[17],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_id = shared.id.ok_or_else(|| A::Error::missing_field("id"))?;
        let variant_title = variant_title.unwrap_or_default();
        let variant_username = variant_username.unwrap_or_default();
        let variant_is_forum = variant_is_forum.unwrap_or_default();
        let common_is_direct_messages = shared.is_direct_messages.unwrap_or(None);
        let common_accent_color_id = shared
            .accent_color_id
            .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
        let common_max_reaction_count = shared
            .max_reaction_count
            .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
        let common_photo = shared.photo.unwrap_or(None);
        let variant_active_usernames = variant_active_usernames.unwrap_or_default();
        let variant_parent_chat = variant_parent_chat.unwrap_or_default();
        let common_available_reactions = shared.available_reactions.unwrap_or(None);
        let common_background_custom_emoji_id = shared.background_custom_emoji_id.unwrap_or(None);
        let common_profile_accent_color_id = shared.profile_accent_color_id.unwrap_or(None);
        let common_profile_background_custom_emoji_id =
            shared.profile_background_custom_emoji_id.unwrap_or(None);
        let variant_join_to_send_messages = variant_join_to_send_messages.unwrap_or_default();
        let variant_join_by_request = variant_join_by_request.unwrap_or_default();
        let variant_description = variant_description.unwrap_or_default();
        let variant_invite_link = variant_invite_link.unwrap_or_default();
        let common_pinned_message = shared.pinned_message.unwrap_or(None);
        let variant_permissions = variant_permissions.unwrap_or_default();
        let common_accepted_gift_types = shared
            .accepted_gift_types
            .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
        let variant_slow_mode_delay = variant_slow_mode_delay.unwrap_or_default();
        let variant_unrestrict_boost_count = variant_unrestrict_boost_count.unwrap_or_default();
        let common_message_auto_delete_time = shared.message_auto_delete_time.unwrap_or(None);
        let variant_has_aggressive_anti_spam_enabled =
            variant_has_aggressive_anti_spam_enabled.unwrap_or_default();
        let common_has_hidden_members = shared.has_hidden_members.unwrap_or(None);
        let common_has_protected_content = shared.has_protected_content.unwrap_or(None);
        let common_has_visible_history = shared.has_visible_history.unwrap_or(None);
        let variant_sticker_set_name = variant_sticker_set_name.unwrap_or_default();
        let variant_can_set_sticker_set = variant_can_set_sticker_set.unwrap_or_default();
        let variant_custom_emoji_sticker_set_name =
            variant_custom_emoji_sticker_set_name.unwrap_or_default();
        let variant_linked_chat_id = variant_linked_chat_id.unwrap_or_default();
        let variant_location = variant_location.unwrap_or_default();
        let common_unique_gift_colors = shared.unique_gift_colors.unwrap_or(None);
        let common_paid_message_star_count = shared.paid_message_star_count.unwrap_or(None);
        let common_guard_bot = shared.guard_bot.unwrap_or(None);
        let common_community = shared.community.unwrap_or(None);
        Ok(SupergroupFields {
            common: CommonFields {
                accent_color_id: common_accent_color_id,
                accepted_gift_types: common_accepted_gift_types,
                available_reactions: common_available_reactions,
                background_custom_emoji_id: common_background_custom_emoji_id,
                community: common_community,
                guard_bot: common_guard_bot,
                has_hidden_members: common_has_hidden_members,
                has_protected_content: common_has_protected_content,
                has_visible_history: common_has_visible_history,
                id: common_id,
                is_direct_messages: common_is_direct_messages,
                max_reaction_count: common_max_reaction_count,
                message_auto_delete_time: common_message_auto_delete_time,
                paid_message_star_count: common_paid_message_star_count,
                photo: common_photo,
                pinned_message: common_pinned_message,
                profile_accent_color_id: common_profile_accent_color_id,
                profile_background_custom_emoji_id: common_profile_background_custom_emoji_id,
                unique_gift_colors: common_unique_gift_colors,
            },
            title: variant_title,
            username: variant_username,
            is_forum: variant_is_forum,
            active_usernames: variant_active_usernames,
            parent_chat: variant_parent_chat,
            join_to_send_messages: variant_join_to_send_messages,
            join_by_request: variant_join_by_request,
            description: variant_description,
            invite_link: variant_invite_link,
            permissions: variant_permissions,
            slow_mode_delay: variant_slow_mode_delay,
            unrestrict_boost_count: variant_unrestrict_boost_count,
            has_aggressive_anti_spam_enabled: variant_has_aggressive_anti_spam_enabled,
            sticker_set_name: variant_sticker_set_name,
            can_set_sticker_set: variant_can_set_sticker_set,
            custom_emoji_sticker_set_name: variant_custom_emoji_sticker_set_name,
            linked_chat_id: variant_linked_chat_id,
            location: variant_location,
        })
    }
}

pub(crate) fn deserialize_supergroup<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<SupergroupFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        SupergroupVisitor {
            metadata,
        },
    )
}

pub(crate) static CHANNEL: Metadata = Metadata {
    name: "ChatFullInfoChannel",
    fields: &channel_field_names(&[
        "title",
        "username",
        "active_usernames",
        "description",
        "invite_link",
        "can_send_paid_media",
        "linked_chat_id",
    ]),
    variant_fields: &[
        "title",
        "username",
        "active_usernames",
        "description",
        "invite_link",
        "can_send_paid_media",
        "linked_chat_id",
    ],
};

pub(crate) static GROUP: Metadata = Metadata {
    name: "ChatFullInfoGroup",
    fields: &group_field_names(&["title", "description", "invite_link", "permissions"]),
    variant_fields: &["title", "description", "invite_link", "permissions"],
};

pub(crate) static PRIVATE: Metadata = Metadata {
    name: "ChatFullInfoPrivate",
    fields: &private_field_names(&[
        "username",
        "first_name",
        "last_name",
        "active_usernames",
        "birthdate",
        "business_intro",
        "business_location",
        "business_opening_hours",
        "personal_chat",
        "emoji_status_custom_emoji_id",
        "emoji_status_expiration_date",
        "bio",
        "has_private_forwards",
        "has_restricted_voice_and_video_messages",
        "rating",
        "first_profile_audio",
    ]),
    variant_fields: &[
        "username",
        "first_name",
        "last_name",
        "active_usernames",
        "birthdate",
        "business_intro",
        "business_location",
        "business_opening_hours",
        "personal_chat",
        "emoji_status_custom_emoji_id",
        "emoji_status_expiration_date",
        "bio",
        "has_private_forwards",
        "has_restricted_voice_and_video_messages",
        "rating",
        "first_profile_audio",
    ],
};

pub(crate) static SUPERGROUP: Metadata = Metadata {
    name: "ChatFullInfoSupergroup",
    fields: &supergroup_field_names(&[
        "title",
        "username",
        "is_forum",
        "active_usernames",
        "parent_chat",
        "join_to_send_messages",
        "join_by_request",
        "description",
        "invite_link",
        "permissions",
        "slow_mode_delay",
        "unrestrict_boost_count",
        "has_aggressive_anti_spam_enabled",
        "sticker_set_name",
        "can_set_sticker_set",
        "custom_emoji_sticker_set_name",
        "linked_chat_id",
        "location",
    ]),
    variant_fields: &[
        "title",
        "username",
        "is_forum",
        "active_usernames",
        "parent_chat",
        "join_to_send_messages",
        "join_by_request",
        "description",
        "invite_link",
        "permissions",
        "slow_mode_delay",
        "unrestrict_boost_count",
        "has_aggressive_anti_spam_enabled",
        "sticker_set_name",
        "can_set_sticker_set",
        "custom_emoji_sticker_set_name",
        "linked_chat_id",
        "location",
    ],
};
