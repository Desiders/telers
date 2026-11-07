#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) user: Box<crate::types::User>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    user: Option<Box<crate::types::User>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 1;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "user" => Some(0),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"user" => Some(0),
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
            0 => read_map_field(&mut self.user, map, "user"),
            _ => unreachable!(),
        }
    }
}

const fn administrator_field_names(variant_fields: &[&'static str; 20]) -> [&'static str; 21] {
    [
        "user",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
        variant_fields[8],
        variant_fields[9],
        variant_fields[10],
        variant_fields[11],
        variant_fields[12],
        variant_fields[13],
        variant_fields[14],
        variant_fields[15],
        variant_fields[16],
        variant_fields[17],
        variant_fields[18],
        variant_fields[19],
    ]
}

#[allow(clippy::struct_excessive_bools)]
pub(crate) struct AdministratorFields {
    pub(crate) common: CommonFields,
    pub(crate) can_be_edited: bool,
    pub(crate) is_anonymous: bool,
    pub(crate) can_manage_chat: bool,
    pub(crate) can_delete_messages: bool,
    pub(crate) can_manage_video_chats: bool,
    pub(crate) can_restrict_members: bool,
    pub(crate) can_promote_members: bool,
    pub(crate) can_change_info: bool,
    pub(crate) can_invite_users: bool,
    pub(crate) can_post_stories: bool,
    pub(crate) can_edit_stories: bool,
    pub(crate) can_delete_stories: bool,
    pub(crate) can_post_messages: Option<bool>,
    pub(crate) can_edit_messages: Option<bool>,
    pub(crate) can_pin_messages: Option<bool>,
    pub(crate) can_manage_topics: Option<bool>,
    pub(crate) can_manage_direct_messages: Option<bool>,
    pub(crate) can_manage_tags: Option<bool>,
    pub(crate) can_send_welcome_messages: bool,
    pub(crate) custom_title: Option<Box<str>>,
}

struct AdministratorVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for AdministratorVisitor {
    type Value = AdministratorFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        let variant_can_be_edited = read_seq_field(&mut seq, 1, &expected)?;
        let variant_is_anonymous = read_seq_field(&mut seq, 2, &expected)?;
        let variant_can_manage_chat = read_seq_field(&mut seq, 3, &expected)?;
        let variant_can_delete_messages = read_seq_field(&mut seq, 4, &expected)?;
        let variant_can_manage_video_chats = read_seq_field(&mut seq, 5, &expected)?;
        let variant_can_restrict_members = read_seq_field(&mut seq, 6, &expected)?;
        let variant_can_promote_members = read_seq_field(&mut seq, 7, &expected)?;
        let variant_can_change_info = read_seq_field(&mut seq, 8, &expected)?;
        let variant_can_invite_users = read_seq_field(&mut seq, 9, &expected)?;
        let variant_can_post_stories = read_seq_field(&mut seq, 10, &expected)?;
        let variant_can_edit_stories = read_seq_field(&mut seq, 11, &expected)?;
        let variant_can_delete_stories = read_seq_field(&mut seq, 12, &expected)?;
        let variant_can_post_messages = read_seq_field(&mut seq, 13, &expected)?;
        let variant_can_edit_messages = read_seq_field(&mut seq, 14, &expected)?;
        let variant_can_pin_messages = read_seq_field(&mut seq, 15, &expected)?;
        let variant_can_manage_topics = read_seq_field(&mut seq, 16, &expected)?;
        let variant_can_manage_direct_messages = read_seq_field(&mut seq, 17, &expected)?;
        let variant_can_manage_tags = read_seq_field(&mut seq, 18, &expected)?;
        let variant_can_send_welcome_messages = read_seq_field(&mut seq, 19, &expected)?;
        let variant_custom_title = read_seq_field(&mut seq, 20, &expected)?;
        Ok(AdministratorFields {
            common: CommonFields {
                user: common_user,
            },
            can_be_edited: variant_can_be_edited,
            is_anonymous: variant_is_anonymous,
            can_manage_chat: variant_can_manage_chat,
            can_delete_messages: variant_can_delete_messages,
            can_manage_video_chats: variant_can_manage_video_chats,
            can_restrict_members: variant_can_restrict_members,
            can_promote_members: variant_can_promote_members,
            can_change_info: variant_can_change_info,
            can_invite_users: variant_can_invite_users,
            can_post_stories: variant_can_post_stories,
            can_edit_stories: variant_can_edit_stories,
            can_delete_stories: variant_can_delete_stories,
            can_post_messages: variant_can_post_messages,
            can_edit_messages: variant_can_edit_messages,
            can_pin_messages: variant_can_pin_messages,
            can_manage_topics: variant_can_manage_topics,
            can_manage_direct_messages: variant_can_manage_direct_messages,
            can_manage_tags: variant_can_manage_tags,
            can_send_welcome_messages: variant_can_send_welcome_messages,
            custom_title: variant_custom_title,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_can_be_edited: Option<bool> = None;
        let mut variant_is_anonymous: Option<bool> = None;
        let mut variant_can_manage_chat: Option<bool> = None;
        let mut variant_can_delete_messages: Option<bool> = None;
        let mut variant_can_manage_video_chats: Option<bool> = None;
        let mut variant_can_restrict_members: Option<bool> = None;
        let mut variant_can_promote_members: Option<bool> = None;
        let mut variant_can_change_info: Option<bool> = None;
        let mut variant_can_invite_users: Option<bool> = None;
        let mut variant_can_post_stories: Option<bool> = None;
        let mut variant_can_edit_stories: Option<bool> = None;
        let mut variant_can_delete_stories: Option<bool> = None;
        let mut variant_can_post_messages: Option<Option<bool>> = None;
        let mut variant_can_edit_messages: Option<Option<bool>> = None;
        let mut variant_can_pin_messages: Option<Option<bool>> = None;
        let mut variant_can_manage_topics: Option<Option<bool>> = None;
        let mut variant_can_manage_direct_messages: Option<Option<bool>> = None;
        let mut variant_can_manage_tags: Option<Option<bool>> = None;
        let mut variant_can_send_welcome_messages: Option<bool> = None;
        let mut variant_custom_title: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_can_be_edited,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                2 => {
                    read_map_field(
                        &mut variant_is_anonymous,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_can_manage_chat,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_can_delete_messages,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_can_manage_video_chats,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_can_restrict_members,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_can_promote_members,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_can_change_info,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_can_invite_users,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_can_post_stories,
                        &mut map,
                        self.metadata.variant_fields[9],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_can_edit_stories,
                        &mut map,
                        self.metadata.variant_fields[10],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_can_delete_stories,
                        &mut map,
                        self.metadata.variant_fields[11],
                    )?;
                }
                13 => {
                    read_map_field(
                        &mut variant_can_post_messages,
                        &mut map,
                        self.metadata.variant_fields[12],
                    )?;
                }
                14 => {
                    read_map_field(
                        &mut variant_can_edit_messages,
                        &mut map,
                        self.metadata.variant_fields[13],
                    )?;
                }
                15 => {
                    read_map_field(
                        &mut variant_can_pin_messages,
                        &mut map,
                        self.metadata.variant_fields[14],
                    )?;
                }
                16 => {
                    read_map_field(
                        &mut variant_can_manage_topics,
                        &mut map,
                        self.metadata.variant_fields[15],
                    )?;
                }
                17 => {
                    read_map_field(
                        &mut variant_can_manage_direct_messages,
                        &mut map,
                        self.metadata.variant_fields[16],
                    )?;
                }
                18 => {
                    read_map_field(
                        &mut variant_can_manage_tags,
                        &mut map,
                        self.metadata.variant_fields[17],
                    )?;
                }
                19 => {
                    read_map_field(
                        &mut variant_can_send_welcome_messages,
                        &mut map,
                        self.metadata.variant_fields[18],
                    )?;
                }
                20 => {
                    read_map_field(
                        &mut variant_custom_title,
                        &mut map,
                        self.metadata.variant_fields[19],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_can_be_edited = variant_can_be_edited
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_is_anonymous = variant_is_anonymous
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_can_manage_chat = variant_can_manage_chat
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[2]))?;
        let variant_can_delete_messages = variant_can_delete_messages
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[3]))?;
        let variant_can_manage_video_chats = variant_can_manage_video_chats
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[4]))?;
        let variant_can_restrict_members = variant_can_restrict_members
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[5]))?;
        let variant_can_promote_members = variant_can_promote_members
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[6]))?;
        let variant_can_change_info = variant_can_change_info
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[7]))?;
        let variant_can_invite_users = variant_can_invite_users
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[8]))?;
        let variant_can_post_stories = variant_can_post_stories
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[9]))?;
        let variant_can_edit_stories = variant_can_edit_stories
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[10]))?;
        let variant_can_delete_stories = variant_can_delete_stories
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[11]))?;
        let variant_can_post_messages = variant_can_post_messages.unwrap_or_default();
        let variant_can_edit_messages = variant_can_edit_messages.unwrap_or_default();
        let variant_can_pin_messages = variant_can_pin_messages.unwrap_or_default();
        let variant_can_manage_topics = variant_can_manage_topics.unwrap_or_default();
        let variant_can_manage_direct_messages =
            variant_can_manage_direct_messages.unwrap_or_default();
        let variant_can_manage_tags = variant_can_manage_tags.unwrap_or_default();
        let variant_can_send_welcome_messages = variant_can_send_welcome_messages
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[18]))?;
        let variant_custom_title = variant_custom_title.unwrap_or_default();
        Ok(AdministratorFields {
            common: CommonFields {
                user: common_user,
            },
            can_be_edited: variant_can_be_edited,
            is_anonymous: variant_is_anonymous,
            can_manage_chat: variant_can_manage_chat,
            can_delete_messages: variant_can_delete_messages,
            can_manage_video_chats: variant_can_manage_video_chats,
            can_restrict_members: variant_can_restrict_members,
            can_promote_members: variant_can_promote_members,
            can_change_info: variant_can_change_info,
            can_invite_users: variant_can_invite_users,
            can_post_stories: variant_can_post_stories,
            can_edit_stories: variant_can_edit_stories,
            can_delete_stories: variant_can_delete_stories,
            can_post_messages: variant_can_post_messages,
            can_edit_messages: variant_can_edit_messages,
            can_pin_messages: variant_can_pin_messages,
            can_manage_topics: variant_can_manage_topics,
            can_manage_direct_messages: variant_can_manage_direct_messages,
            can_manage_tags: variant_can_manage_tags,
            can_send_welcome_messages: variant_can_send_welcome_messages,
            custom_title: variant_custom_title,
        })
    }
}

pub(crate) fn deserialize_administrator<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<AdministratorFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        AdministratorVisitor {
            metadata,
        },
    )
}

const fn banned_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 2] {
    ["user", variant_fields[0]]
}

pub(crate) struct BannedFields {
    pub(crate) common: CommonFields,
    pub(crate) until_date: i64,
}

struct BannedVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for BannedVisitor {
    type Value = BannedFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        let variant_until_date = read_seq_field(&mut seq, 1, &expected)?;
        Ok(BannedFields {
            common: CommonFields {
                user: common_user,
            },
            until_date: variant_until_date,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_until_date: Option<i64> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_until_date,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_until_date = variant_until_date
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(BannedFields {
            common: CommonFields {
                user: common_user,
            },
            until_date: variant_until_date,
        })
    }
}

pub(crate) fn deserialize_banned<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<BannedFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        BannedVisitor {
            metadata,
        },
    )
}

const COMMON_FIELD_NAMES: [&str; 1] = ["user"];

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
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        Ok(CommonFields {
            user: common_user,
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
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        Ok(CommonFields {
            user: common_user,
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

const fn member_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 3] {
    [variant_fields[0], "user", variant_fields[1]]
}

pub(crate) struct MemberFields {
    pub(crate) common: CommonFields,
    pub(crate) tag: Option<Box<str>>,
    pub(crate) until_date: Option<i64>,
}

struct MemberVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for MemberVisitor {
    type Value = MemberFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_tag = read_seq_field(&mut seq, 0, &expected)?;
        let common_user = read_seq_field(&mut seq, 1, &expected)?;
        let variant_until_date = read_seq_field(&mut seq, 2, &expected)?;
        Ok(MemberFields {
            common: CommonFields {
                user: common_user,
            },
            tag: variant_tag,
            until_date: variant_until_date,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_tag: Option<Option<Box<str>>> = None;
        let mut variant_until_date: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(&mut variant_tag, &mut map, self.metadata.variant_fields[0])?;
                }
                2 => {
                    read_map_field(
                        &mut variant_until_date,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_tag = variant_tag.unwrap_or_default();
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_until_date = variant_until_date.unwrap_or_default();
        Ok(MemberFields {
            common: CommonFields {
                user: common_user,
            },
            tag: variant_tag,
            until_date: variant_until_date,
        })
    }
}

pub(crate) fn deserialize_member<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<MemberFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        MemberVisitor {
            metadata,
        },
    )
}

const fn owner_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 3] {
    ["user", variant_fields[0], variant_fields[1]]
}

pub(crate) struct OwnerFields {
    pub(crate) common: CommonFields,
    pub(crate) is_anonymous: bool,
    pub(crate) custom_title: Option<Box<str>>,
}

struct OwnerVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for OwnerVisitor {
    type Value = OwnerFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        let variant_is_anonymous = read_seq_field(&mut seq, 1, &expected)?;
        let variant_custom_title = read_seq_field(&mut seq, 2, &expected)?;
        Ok(OwnerFields {
            common: CommonFields {
                user: common_user,
            },
            is_anonymous: variant_is_anonymous,
            custom_title: variant_custom_title,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_is_anonymous: Option<bool> = None;
        let mut variant_custom_title: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_is_anonymous,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                2 => {
                    read_map_field(
                        &mut variant_custom_title,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_is_anonymous = variant_is_anonymous
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_custom_title = variant_custom_title.unwrap_or_default();
        Ok(OwnerFields {
            common: CommonFields {
                user: common_user,
            },
            is_anonymous: variant_is_anonymous,
            custom_title: variant_custom_title,
        })
    }
}

pub(crate) fn deserialize_owner<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<OwnerFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        OwnerVisitor {
            metadata,
        },
    )
}

const fn restricted_field_names(variant_fields: &[&'static str; 19]) -> [&'static str; 20] {
    [
        variant_fields[0],
        "user",
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        variant_fields[5],
        variant_fields[6],
        variant_fields[7],
        variant_fields[8],
        variant_fields[9],
        variant_fields[10],
        variant_fields[11],
        variant_fields[12],
        variant_fields[13],
        variant_fields[14],
        variant_fields[15],
        variant_fields[16],
        variant_fields[17],
        variant_fields[18],
    ]
}

#[allow(clippy::struct_excessive_bools)]
pub(crate) struct RestrictedFields {
    pub(crate) common: CommonFields,
    pub(crate) tag: Option<Box<str>>,
    pub(crate) is_member: bool,
    pub(crate) can_send_messages: bool,
    pub(crate) can_send_audios: bool,
    pub(crate) can_send_documents: bool,
    pub(crate) can_send_photos: bool,
    pub(crate) can_send_videos: bool,
    pub(crate) can_send_video_notes: bool,
    pub(crate) can_send_voice_notes: bool,
    pub(crate) can_send_polls: bool,
    pub(crate) can_send_other_messages: bool,
    pub(crate) can_add_web_page_previews: bool,
    pub(crate) can_react_to_messages: bool,
    pub(crate) can_edit_tag: bool,
    pub(crate) can_change_info: bool,
    pub(crate) can_invite_users: bool,
    pub(crate) can_pin_messages: bool,
    pub(crate) can_manage_topics: bool,
    pub(crate) until_date: i64,
}

struct RestrictedVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for RestrictedVisitor {
    type Value = RestrictedFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_tag = read_seq_field(&mut seq, 0, &expected)?;
        let common_user = read_seq_field(&mut seq, 1, &expected)?;
        let variant_is_member = read_seq_field(&mut seq, 2, &expected)?;
        let variant_can_send_messages = read_seq_field(&mut seq, 3, &expected)?;
        let variant_can_send_audios = read_seq_field(&mut seq, 4, &expected)?;
        let variant_can_send_documents = read_seq_field(&mut seq, 5, &expected)?;
        let variant_can_send_photos = read_seq_field(&mut seq, 6, &expected)?;
        let variant_can_send_videos = read_seq_field(&mut seq, 7, &expected)?;
        let variant_can_send_video_notes = read_seq_field(&mut seq, 8, &expected)?;
        let variant_can_send_voice_notes = read_seq_field(&mut seq, 9, &expected)?;
        let variant_can_send_polls = read_seq_field(&mut seq, 10, &expected)?;
        let variant_can_send_other_messages = read_seq_field(&mut seq, 11, &expected)?;
        let variant_can_add_web_page_previews = read_seq_field(&mut seq, 12, &expected)?;
        let variant_can_react_to_messages = read_seq_field(&mut seq, 13, &expected)?;
        let variant_can_edit_tag = read_seq_field(&mut seq, 14, &expected)?;
        let variant_can_change_info = read_seq_field(&mut seq, 15, &expected)?;
        let variant_can_invite_users = read_seq_field(&mut seq, 16, &expected)?;
        let variant_can_pin_messages = read_seq_field(&mut seq, 17, &expected)?;
        let variant_can_manage_topics = read_seq_field(&mut seq, 18, &expected)?;
        let variant_until_date = read_seq_field(&mut seq, 19, &expected)?;
        Ok(RestrictedFields {
            common: CommonFields {
                user: common_user,
            },
            tag: variant_tag,
            is_member: variant_is_member,
            can_send_messages: variant_can_send_messages,
            can_send_audios: variant_can_send_audios,
            can_send_documents: variant_can_send_documents,
            can_send_photos: variant_can_send_photos,
            can_send_videos: variant_can_send_videos,
            can_send_video_notes: variant_can_send_video_notes,
            can_send_voice_notes: variant_can_send_voice_notes,
            can_send_polls: variant_can_send_polls,
            can_send_other_messages: variant_can_send_other_messages,
            can_add_web_page_previews: variant_can_add_web_page_previews,
            can_react_to_messages: variant_can_react_to_messages,
            can_edit_tag: variant_can_edit_tag,
            can_change_info: variant_can_change_info,
            can_invite_users: variant_can_invite_users,
            can_pin_messages: variant_can_pin_messages,
            can_manage_topics: variant_can_manage_topics,
            until_date: variant_until_date,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_tag: Option<Option<Box<str>>> = None;
        let mut variant_is_member: Option<bool> = None;
        let mut variant_can_send_messages: Option<bool> = None;
        let mut variant_can_send_audios: Option<bool> = None;
        let mut variant_can_send_documents: Option<bool> = None;
        let mut variant_can_send_photos: Option<bool> = None;
        let mut variant_can_send_videos: Option<bool> = None;
        let mut variant_can_send_video_notes: Option<bool> = None;
        let mut variant_can_send_voice_notes: Option<bool> = None;
        let mut variant_can_send_polls: Option<bool> = None;
        let mut variant_can_send_other_messages: Option<bool> = None;
        let mut variant_can_add_web_page_previews: Option<bool> = None;
        let mut variant_can_react_to_messages: Option<bool> = None;
        let mut variant_can_edit_tag: Option<bool> = None;
        let mut variant_can_change_info: Option<bool> = None;
        let mut variant_can_invite_users: Option<bool> = None;
        let mut variant_can_pin_messages: Option<bool> = None;
        let mut variant_can_manage_topics: Option<bool> = None;
        let mut variant_until_date: Option<i64> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(&mut variant_tag, &mut map, self.metadata.variant_fields[0])?;
                }
                2 => {
                    read_map_field(
                        &mut variant_is_member,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_can_send_messages,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_can_send_audios,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_can_send_documents,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                6 => {
                    read_map_field(
                        &mut variant_can_send_photos,
                        &mut map,
                        self.metadata.variant_fields[5],
                    )?;
                }
                7 => {
                    read_map_field(
                        &mut variant_can_send_videos,
                        &mut map,
                        self.metadata.variant_fields[6],
                    )?;
                }
                8 => {
                    read_map_field(
                        &mut variant_can_send_video_notes,
                        &mut map,
                        self.metadata.variant_fields[7],
                    )?;
                }
                9 => {
                    read_map_field(
                        &mut variant_can_send_voice_notes,
                        &mut map,
                        self.metadata.variant_fields[8],
                    )?;
                }
                10 => {
                    read_map_field(
                        &mut variant_can_send_polls,
                        &mut map,
                        self.metadata.variant_fields[9],
                    )?;
                }
                11 => {
                    read_map_field(
                        &mut variant_can_send_other_messages,
                        &mut map,
                        self.metadata.variant_fields[10],
                    )?;
                }
                12 => {
                    read_map_field(
                        &mut variant_can_add_web_page_previews,
                        &mut map,
                        self.metadata.variant_fields[11],
                    )?;
                }
                13 => {
                    read_map_field(
                        &mut variant_can_react_to_messages,
                        &mut map,
                        self.metadata.variant_fields[12],
                    )?;
                }
                14 => {
                    read_map_field(
                        &mut variant_can_edit_tag,
                        &mut map,
                        self.metadata.variant_fields[13],
                    )?;
                }
                15 => {
                    read_map_field(
                        &mut variant_can_change_info,
                        &mut map,
                        self.metadata.variant_fields[14],
                    )?;
                }
                16 => {
                    read_map_field(
                        &mut variant_can_invite_users,
                        &mut map,
                        self.metadata.variant_fields[15],
                    )?;
                }
                17 => {
                    read_map_field(
                        &mut variant_can_pin_messages,
                        &mut map,
                        self.metadata.variant_fields[16],
                    )?;
                }
                18 => {
                    read_map_field(
                        &mut variant_can_manage_topics,
                        &mut map,
                        self.metadata.variant_fields[17],
                    )?;
                }
                19 => {
                    read_map_field(
                        &mut variant_until_date,
                        &mut map,
                        self.metadata.variant_fields[18],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_tag = variant_tag.unwrap_or_default();
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_is_member = variant_is_member
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_can_send_messages = variant_can_send_messages
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[2]))?;
        let variant_can_send_audios = variant_can_send_audios
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[3]))?;
        let variant_can_send_documents = variant_can_send_documents
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[4]))?;
        let variant_can_send_photos = variant_can_send_photos
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[5]))?;
        let variant_can_send_videos = variant_can_send_videos
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[6]))?;
        let variant_can_send_video_notes = variant_can_send_video_notes
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[7]))?;
        let variant_can_send_voice_notes = variant_can_send_voice_notes
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[8]))?;
        let variant_can_send_polls = variant_can_send_polls
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[9]))?;
        let variant_can_send_other_messages = variant_can_send_other_messages
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[10]))?;
        let variant_can_add_web_page_previews = variant_can_add_web_page_previews
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[11]))?;
        let variant_can_react_to_messages = variant_can_react_to_messages
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[12]))?;
        let variant_can_edit_tag = variant_can_edit_tag
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[13]))?;
        let variant_can_change_info = variant_can_change_info
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[14]))?;
        let variant_can_invite_users = variant_can_invite_users
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[15]))?;
        let variant_can_pin_messages = variant_can_pin_messages
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[16]))?;
        let variant_can_manage_topics = variant_can_manage_topics
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[17]))?;
        let variant_until_date = variant_until_date
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[18]))?;
        Ok(RestrictedFields {
            common: CommonFields {
                user: common_user,
            },
            tag: variant_tag,
            is_member: variant_is_member,
            can_send_messages: variant_can_send_messages,
            can_send_audios: variant_can_send_audios,
            can_send_documents: variant_can_send_documents,
            can_send_photos: variant_can_send_photos,
            can_send_videos: variant_can_send_videos,
            can_send_video_notes: variant_can_send_video_notes,
            can_send_voice_notes: variant_can_send_voice_notes,
            can_send_polls: variant_can_send_polls,
            can_send_other_messages: variant_can_send_other_messages,
            can_add_web_page_previews: variant_can_add_web_page_previews,
            can_react_to_messages: variant_can_react_to_messages,
            can_edit_tag: variant_can_edit_tag,
            can_change_info: variant_can_change_info,
            can_invite_users: variant_can_invite_users,
            can_pin_messages: variant_can_pin_messages,
            can_manage_topics: variant_can_manage_topics,
            until_date: variant_until_date,
        })
    }
}

pub(crate) fn deserialize_restricted<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<RestrictedFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        RestrictedVisitor {
            metadata,
        },
    )
}

pub(crate) static ADMINISTRATOR: Metadata = Metadata {
    name: "ChatMemberAdministrator",
    fields: &administrator_field_names(&[
        "can_be_edited",
        "is_anonymous",
        "can_manage_chat",
        "can_delete_messages",
        "can_manage_video_chats",
        "can_restrict_members",
        "can_promote_members",
        "can_change_info",
        "can_invite_users",
        "can_post_stories",
        "can_edit_stories",
        "can_delete_stories",
        "can_post_messages",
        "can_edit_messages",
        "can_pin_messages",
        "can_manage_topics",
        "can_manage_direct_messages",
        "can_manage_tags",
        "can_send_welcome_messages",
        "custom_title",
    ]),
    variant_fields: &[
        "can_be_edited",
        "is_anonymous",
        "can_manage_chat",
        "can_delete_messages",
        "can_manage_video_chats",
        "can_restrict_members",
        "can_promote_members",
        "can_change_info",
        "can_invite_users",
        "can_post_stories",
        "can_edit_stories",
        "can_delete_stories",
        "can_post_messages",
        "can_edit_messages",
        "can_pin_messages",
        "can_manage_topics",
        "can_manage_direct_messages",
        "can_manage_tags",
        "can_send_welcome_messages",
        "custom_title",
    ],
};

pub(crate) static BANNED: Metadata = Metadata {
    name: "ChatMemberBanned",
    fields: &banned_field_names(&["until_date"]),
    variant_fields: &["until_date"],
};

pub(crate) static LEFT: Metadata = Metadata {
    name: "ChatMemberLeft",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static MEMBER: Metadata = Metadata {
    name: "ChatMemberMember",
    fields: &member_field_names(&["tag", "until_date"]),
    variant_fields: &["tag", "until_date"],
};

pub(crate) static OWNER: Metadata = Metadata {
    name: "ChatMemberOwner",
    fields: &owner_field_names(&["is_anonymous", "custom_title"]),
    variant_fields: &["is_anonymous", "custom_title"],
};

pub(crate) static RESTRICTED: Metadata = Metadata {
    name: "ChatMemberRestricted",
    fields: &restricted_field_names(&[
        "tag",
        "is_member",
        "can_send_messages",
        "can_send_audios",
        "can_send_documents",
        "can_send_photos",
        "can_send_videos",
        "can_send_video_notes",
        "can_send_voice_notes",
        "can_send_polls",
        "can_send_other_messages",
        "can_add_web_page_previews",
        "can_react_to_messages",
        "can_edit_tag",
        "can_change_info",
        "can_invite_users",
        "can_pin_messages",
        "can_manage_topics",
        "until_date",
    ]),
    variant_fields: &[
        "tag",
        "is_member",
        "can_send_messages",
        "can_send_audios",
        "can_send_documents",
        "can_send_photos",
        "can_send_videos",
        "can_send_video_notes",
        "can_send_voice_notes",
        "can_send_polls",
        "can_send_other_messages",
        "can_add_web_page_previews",
        "can_react_to_messages",
        "can_edit_tag",
        "can_change_info",
        "can_invite_users",
        "can_pin_messages",
        "can_manage_topics",
        "until_date",
    ],
};
