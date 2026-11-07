#![allow(
    clippy::option_option,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::type_complexity
)]
use serde::{
    de::{DeserializeSeed, Error, Expected, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};
#[inline]
fn map_field<'de, A: MapAccess<'de>, T: Deserialize<'de>>(
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
fn seq_field<'de, A: SeqAccess<'de>, T: Deserialize<'de>>(
    seq: &mut A,
    index: usize,
    expected: &dyn Expected,
) -> Result<T, A::Error> {
    seq.next_element()?
        .ok_or_else(|| A::Error::invalid_length(index, expected))
}
pub(crate) const fn field_name(name: &'static str) -> &'static str {
    match name.as_bytes() {
        [b'r', b'#', ..] => name.split_at(2).1,
        _ => name,
    }
}
#[derive(Clone, Copy)]
pub(crate) struct Meta {
    pub(crate) name: &'static str,
    pub(crate) fields: &'static [&'static str],
    pub(crate) specific: &'static [&'static str],
    pub(crate) sequence: &'static str,
}
trait CommonFields {
    const COUNT: usize;
    fn field(value: &str) -> Option<usize>;
    fn field_bytes(value: &[u8]) -> Option<usize>;
}
struct Key<C> {
    meta: Meta,
    marker: PhantomData<C>,
}
impl<C> Key<C> {
    fn new(meta: Meta) -> Self {
        Self {
            meta,
            marker: PhantomData,
        }
    }
}
impl<'de, C: CommonFields> DeserializeSeed<'de> for Key<C> {
    type Value = usize;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<usize, D::Error> {
        deserializer.deserialize_identifier(self)
    }
}
impl<C: CommonFields> Visitor<'_> for Key<C> {
    type Value = usize;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("field identifier")
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<usize, E> {
        if let Some(name) = usize::try_from(value)
            .ok()
            .and_then(|index| self.meta.fields.get(index))
        {
            self.visit_str(name)
        } else {
            Ok(usize::MAX)
        }
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<usize, E> {
        Ok(if let Some(index) = C::field(value) {
            index
        } else if let Some(index) = self.meta.specific.iter().position(|name| *name == value) {
            C::COUNT + index
        } else {
            usize::MAX
        })
    }

    fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<usize, E> {
        Ok(if let Some(index) = C::field_bytes(value) {
            index
        } else if let Some(index) = self
            .meta
            .specific
            .iter()
            .position(|name| name.as_bytes() == value)
        {
            C::COUNT + index
        } else {
            usize::MAX
        })
    }
}
pub(crate) mod chat {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) id: i64,
        pub(crate) is_direct_messages: Option<bool>,
    }
    #[derive(Default)]
    struct Slots {
        id: Option<i64>,
        is_direct_messages: Option<Option<bool>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 2usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "id" => Some(0usize),
                "is_direct_messages" => Some(1usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"id" => Some(0usize),
                b"is_direct_messages" => Some(1usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.id, map, "id"),
                1usize => map_field(&mut self.is_direct_messages, map, "is_direct_messages"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de> + Default, P1: Deserialize<'de> + Default> Visitor<'de>
        for Visitor0<P0, P1>
    {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let is_direct_messages =
                seq_field::<A, Option<bool>>(&mut seq, 3usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    is_direct_messages,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let is_direct_messages = common.is_direct_messages.unwrap_or(None);
            Ok((
                Common {
                    id,
                    is_direct_messages,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::chat::read_0:: < D, $P0, $P1
            > (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            "is_direct_messages"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 4 elements"), },) ?; Ok(Self {
            id : common.id, is_direct_messages : common.is_direct_messages, $field_0 :
            value_0, $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de> + Default> Visitor<'de> for Visitor1<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let is_direct_messages =
                seq_field::<A, Option<bool>>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    is_direct_messages,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 = value_0.unwrap_or_default();
            let is_direct_messages = common.is_direct_messages.unwrap_or(None);
            Ok((
                Common {
                    id,
                    is_direct_messages,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>, P0: Deserialize<'de> + Default>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::chat::read_1:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            "is_direct_messages"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 3 elements"), },) ?; Ok(Self {
            id : common.id, is_direct_messages : common.is_direct_messages, $field_0 :
            value_0, }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0, P1, P2> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor2<P0, P1, P2>
    {
        type Value = (Common, (P0, P1, P2));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let is_direct_messages =
                seq_field::<A, Option<bool>>(&mut seq, 4usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    is_direct_messages,
                },
                (value_0, value_1, value_2),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    4usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let is_direct_messages = common.is_direct_messages.unwrap_or(None);
            Ok((
                Common {
                    id,
                    is_direct_messages,
                },
                (value_0, value_1, value_2),
            ))
        }
    }
    pub(crate) fn read_2<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2,)) = $crate::deserialization::chat::read_2:: < D,
            $P0, $P1, $P2 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            "is_direct_messages"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, sequence :
            concat!("struct ", stringify!($name), " with 5 elements"), },) ?; Ok(Self {
            id : common.id, is_direct_messages : common.is_direct_messages, $field_0 :
            value_0, $field_1 : value_1, $field_2 : value_2, }) } }
        };
    }
    pub(crate) use deserialize_2;
}
pub(crate) mod chat_full_info {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
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
    struct Slots {
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
    impl CommonFields for Slots {
        const COUNT: usize = 19usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "accent_color_id" => Some(0usize),
                "accepted_gift_types" => Some(1usize),
                "available_reactions" => Some(2usize),
                "background_custom_emoji_id" => Some(3usize),
                "community" => Some(4usize),
                "guard_bot" => Some(5usize),
                "has_hidden_members" => Some(6usize),
                "has_protected_content" => Some(7usize),
                "has_visible_history" => Some(8usize),
                "id" => Some(9usize),
                "is_direct_messages" => Some(10usize),
                "max_reaction_count" => Some(11usize),
                "message_auto_delete_time" => Some(12usize),
                "paid_message_star_count" => Some(13usize),
                "photo" => Some(14usize),
                "pinned_message" => Some(15usize),
                "profile_accent_color_id" => Some(16usize),
                "profile_background_custom_emoji_id" => Some(17usize),
                "unique_gift_colors" => Some(18usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"accent_color_id" => Some(0usize),
                b"accepted_gift_types" => Some(1usize),
                b"available_reactions" => Some(2usize),
                b"background_custom_emoji_id" => Some(3usize),
                b"community" => Some(4usize),
                b"guard_bot" => Some(5usize),
                b"has_hidden_members" => Some(6usize),
                b"has_protected_content" => Some(7usize),
                b"has_visible_history" => Some(8usize),
                b"id" => Some(9usize),
                b"is_direct_messages" => Some(10usize),
                b"max_reaction_count" => Some(11usize),
                b"message_auto_delete_time" => Some(12usize),
                b"paid_message_star_count" => Some(13usize),
                b"photo" => Some(14usize),
                b"pinned_message" => Some(15usize),
                b"profile_accent_color_id" => Some(16usize),
                b"profile_background_custom_emoji_id" => Some(17usize),
                b"unique_gift_colors" => Some(18usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.accent_color_id, map, "accent_color_id"),
                1usize => map_field(&mut self.accepted_gift_types, map, "accepted_gift_types"),
                2usize => map_field(&mut self.available_reactions, map, "available_reactions"),
                3usize => map_field(
                    &mut self.background_custom_emoji_id,
                    map,
                    "background_custom_emoji_id",
                ),
                4usize => map_field(&mut self.community, map, "community"),
                5usize => map_field(&mut self.guard_bot, map, "guard_bot"),
                6usize => map_field(&mut self.has_hidden_members, map, "has_hidden_members"),
                7usize => map_field(
                    &mut self.has_protected_content,
                    map,
                    "has_protected_content",
                ),
                8usize => map_field(&mut self.has_visible_history, map, "has_visible_history"),
                9usize => map_field(&mut self.id, map, "id"),
                10usize => map_field(&mut self.is_direct_messages, map, "is_direct_messages"),
                11usize => map_field(&mut self.max_reaction_count, map, "max_reaction_count"),
                12usize => map_field(
                    &mut self.message_auto_delete_time,
                    map,
                    "message_auto_delete_time",
                ),
                13usize => map_field(
                    &mut self.paid_message_star_count,
                    map,
                    "paid_message_star_count",
                ),
                14usize => map_field(&mut self.photo, map, "photo"),
                15usize => map_field(&mut self.pinned_message, map, "pinned_message"),
                16usize => map_field(
                    &mut self.profile_accent_color_id,
                    map,
                    "profile_accent_color_id",
                ),
                17usize => map_field(
                    &mut self.profile_background_custom_emoji_id,
                    map,
                    "profile_background_custom_emoji_id",
                ),
                18usize => map_field(&mut self.unique_gift_colors, map, "unique_gift_colors"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1, P2, P3, P4, P5, P6> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor0<P0, P1, P2, P3, P4, P5, P6>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let is_direct_messages =
                seq_field::<A, Option<bool>>(&mut seq, 3usize, &self.meta.sequence)?;
            let accent_color_id = seq_field::<A, i64>(&mut seq, 4usize, &self.meta.sequence)?;
            let max_reaction_count = seq_field::<A, i64>(&mut seq, 5usize, &self.meta.sequence)?;
            let photo = seq_field::<A, Option<crate::types::ChatPhoto>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let value_2 = seq_field::<A, P2>(&mut seq, 7usize, &self.meta.sequence)?;
            let available_reactions = seq_field::<A, Option<Box<[crate::types::ReactionType]>>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            let background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 9usize, &self.meta.sequence)?;
            let profile_accent_color_id =
                seq_field::<A, Option<i64>>(&mut seq, 10usize, &self.meta.sequence)?;
            let profile_background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 12usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 13usize, &self.meta.sequence)?;
            let pinned_message = seq_field::<A, Option<Box<crate::types::Message>>>(
                &mut seq,
                14usize,
                &self.meta.sequence,
            )?;
            let accepted_gift_types = seq_field::<A, crate::types::AcceptedGiftTypes>(
                &mut seq,
                15usize,
                &self.meta.sequence,
            )?;
            let value_5 = seq_field::<A, P5>(&mut seq, 16usize, &self.meta.sequence)?;
            let message_auto_delete_time =
                seq_field::<A, Option<i64>>(&mut seq, 17usize, &self.meta.sequence)?;
            let has_hidden_members =
                seq_field::<A, Option<bool>>(&mut seq, 18usize, &self.meta.sequence)?;
            let has_protected_content =
                seq_field::<A, Option<bool>>(&mut seq, 19usize, &self.meta.sequence)?;
            let has_visible_history =
                seq_field::<A, Option<bool>>(&mut seq, 20usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 21usize, &self.meta.sequence)?;
            let unique_gift_colors = seq_field::<A, Option<crate::types::UniqueGiftColors>>(
                &mut seq,
                22usize,
                &self.meta.sequence,
            )?;
            let paid_message_star_count =
                seq_field::<A, Option<i64>>(&mut seq, 23usize, &self.meta.sequence)?;
            let guard_bot = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                24usize,
                &self.meta.sequence,
            )?;
            let community = seq_field::<A, Option<crate::types::Community>>(
                &mut seq,
                25usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 19usize => common.read(index, &mut map)?,
                    19usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    20usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    21usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    22usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    23usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    24usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    25usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let is_direct_messages = common.is_direct_messages.unwrap_or(None);
            let accent_color_id = common
                .accent_color_id
                .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
            let max_reaction_count = common
                .max_reaction_count
                .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
            let photo = common.photo.unwrap_or(None);
            let value_2 = value_2.unwrap_or_default();
            let available_reactions = common.available_reactions.unwrap_or(None);
            let background_custom_emoji_id = common.background_custom_emoji_id.unwrap_or(None);
            let profile_accent_color_id = common.profile_accent_color_id.unwrap_or(None);
            let profile_background_custom_emoji_id =
                common.profile_background_custom_emoji_id.unwrap_or(None);
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let pinned_message = common.pinned_message.unwrap_or(None);
            let accepted_gift_types = common
                .accepted_gift_types
                .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
            let value_5 = value_5.unwrap_or_default();
            let message_auto_delete_time = common.message_auto_delete_time.unwrap_or(None);
            let has_hidden_members = common.has_hidden_members.unwrap_or(None);
            let has_protected_content = common.has_protected_content.unwrap_or(None);
            let has_visible_history = common.has_visible_history.unwrap_or(None);
            let value_6 = value_6.unwrap_or_default();
            let unique_gift_colors = common.unique_gift_colors.unwrap_or(None);
            let paid_message_star_count = common.paid_message_star_count.unwrap_or(None);
            let guard_bot = common.guard_bot.unwrap_or(None);
            let community = common.community.unwrap_or(None);
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6,
                ),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6,)) =
            $crate::deserialization::chat_full_info::read_0:: < D, $P0, $P1, $P2, $P3,
            $P4, $P5, $P6 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            "is_direct_messages", "accent_color_id", "max_reaction_count", "photo",
            $crate::deserialization::field_name(stringify!($field_2)),
            "available_reactions", "background_custom_emoji_id",
            "profile_accent_color_id", "profile_background_custom_emoji_id",
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)), "pinned_message",
            "accepted_gift_types",
            $crate::deserialization::field_name(stringify!($field_5)),
            "message_auto_delete_time", "has_hidden_members", "has_protected_content",
            "has_visible_history",
            $crate::deserialization::field_name(stringify!($field_6)),
            "unique_gift_colors", "paid_message_star_count", "guard_bot", "community"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6))] }, sequence :
            concat!("struct ", stringify!($name), " with 26 elements"), },) ?; Ok(Self {
            accent_color_id : common.accent_color_id, accepted_gift_types : common
            .accepted_gift_types, available_reactions : common.available_reactions,
            background_custom_emoji_id : common.background_custom_emoji_id, community :
            common.community, guard_bot : common.guard_bot, has_hidden_members : common
            .has_hidden_members, has_protected_content : common.has_protected_content,
            has_visible_history : common.has_visible_history, id : common.id,
            is_direct_messages : common.is_direct_messages, max_reaction_count : common
            .max_reaction_count, message_auto_delete_time : common
            .message_auto_delete_time, paid_message_star_count : common
            .paid_message_star_count, photo : common.photo, pinned_message : common
            .pinned_message, profile_accent_color_id : common.profile_accent_color_id,
            profile_background_custom_emoji_id : common
            .profile_background_custom_emoji_id, unique_gift_colors : common
            .unique_gift_colors, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, $field_4 : value_4, $field_5 : value_5, $field_6
            : value_6, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1, P2, P3> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor1<P0, P1, P2, P3>
    {
        type Value = (Common, (P0, P1, P2, P3));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let is_direct_messages =
                seq_field::<A, Option<bool>>(&mut seq, 2usize, &self.meta.sequence)?;
            let accent_color_id = seq_field::<A, i64>(&mut seq, 3usize, &self.meta.sequence)?;
            let max_reaction_count = seq_field::<A, i64>(&mut seq, 4usize, &self.meta.sequence)?;
            let photo = seq_field::<A, Option<crate::types::ChatPhoto>>(
                &mut seq,
                5usize,
                &self.meta.sequence,
            )?;
            let available_reactions = seq_field::<A, Option<Box<[crate::types::ReactionType]>>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 7usize, &self.meta.sequence)?;
            let profile_accent_color_id =
                seq_field::<A, Option<i64>>(&mut seq, 8usize, &self.meta.sequence)?;
            let profile_background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 11usize, &self.meta.sequence)?;
            let pinned_message = seq_field::<A, Option<Box<crate::types::Message>>>(
                &mut seq,
                12usize,
                &self.meta.sequence,
            )?;
            let value_3 = seq_field::<A, P3>(&mut seq, 13usize, &self.meta.sequence)?;
            let accepted_gift_types = seq_field::<A, crate::types::AcceptedGiftTypes>(
                &mut seq,
                14usize,
                &self.meta.sequence,
            )?;
            let message_auto_delete_time =
                seq_field::<A, Option<i64>>(&mut seq, 15usize, &self.meta.sequence)?;
            let has_hidden_members =
                seq_field::<A, Option<bool>>(&mut seq, 16usize, &self.meta.sequence)?;
            let has_protected_content =
                seq_field::<A, Option<bool>>(&mut seq, 17usize, &self.meta.sequence)?;
            let has_visible_history =
                seq_field::<A, Option<bool>>(&mut seq, 18usize, &self.meta.sequence)?;
            let unique_gift_colors = seq_field::<A, Option<crate::types::UniqueGiftColors>>(
                &mut seq,
                19usize,
                &self.meta.sequence,
            )?;
            let paid_message_star_count =
                seq_field::<A, Option<i64>>(&mut seq, 20usize, &self.meta.sequence)?;
            let guard_bot = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                21usize,
                &self.meta.sequence,
            )?;
            let community = seq_field::<A, Option<crate::types::Community>>(
                &mut seq,
                22usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 19usize => common.read(index, &mut map)?,
                    19usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    20usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    21usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    22usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 = value_0.unwrap_or_default();
            let is_direct_messages = common.is_direct_messages.unwrap_or(None);
            let accent_color_id = common
                .accent_color_id
                .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
            let max_reaction_count = common
                .max_reaction_count
                .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
            let photo = common.photo.unwrap_or(None);
            let available_reactions = common.available_reactions.unwrap_or(None);
            let background_custom_emoji_id = common.background_custom_emoji_id.unwrap_or(None);
            let profile_accent_color_id = common.profile_accent_color_id.unwrap_or(None);
            let profile_background_custom_emoji_id =
                common.profile_background_custom_emoji_id.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let pinned_message = common.pinned_message.unwrap_or(None);
            let value_3 = value_3.unwrap_or_default();
            let accepted_gift_types = common
                .accepted_gift_types
                .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
            let message_auto_delete_time = common.message_auto_delete_time.unwrap_or(None);
            let has_hidden_members = common.has_hidden_members.unwrap_or(None);
            let has_protected_content = common.has_protected_content.unwrap_or(None);
            let has_visible_history = common.has_visible_history.unwrap_or(None);
            let unique_gift_colors = common.unique_gift_colors.unwrap_or(None);
            let paid_message_star_count = common.paid_message_star_count.unwrap_or(None);
            let guard_bot = common.guard_bot.unwrap_or(None);
            let community = common.community.unwrap_or(None);
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3,)) =
            $crate::deserialization::chat_full_info::read_1:: < D, $P0, $P1, $P2, $P3 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            "is_direct_messages", "accent_color_id", "max_reaction_count", "photo",
            "available_reactions", "background_custom_emoji_id",
            "profile_accent_color_id", "profile_background_custom_emoji_id",
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)), "pinned_message",
            $crate::deserialization::field_name(stringify!($field_3)),
            "accepted_gift_types", "message_auto_delete_time", "has_hidden_members",
            "has_protected_content", "has_visible_history", "unique_gift_colors",
            "paid_message_star_count", "guard_bot", "community"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, sequence :
            concat!("struct ", stringify!($name), " with 23 elements"), },) ?; Ok(Self {
            accent_color_id : common.accent_color_id, accepted_gift_types : common
            .accepted_gift_types, available_reactions : common.available_reactions,
            background_custom_emoji_id : common.background_custom_emoji_id, community :
            common.community, guard_bot : common.guard_bot, has_hidden_members : common
            .has_hidden_members, has_protected_content : common.has_protected_content,
            has_visible_history : common.has_visible_history, id : common.id,
            is_direct_messages : common.is_direct_messages, max_reaction_count : common
            .max_reaction_count, message_auto_delete_time : common
            .message_auto_delete_time, paid_message_star_count : common
            .paid_message_star_count, photo : common.photo, pinned_message : common
            .pinned_message, profile_accent_color_id : common.profile_accent_color_id,
            profile_background_custom_emoji_id : common
            .profile_background_custom_emoji_id, unique_gift_colors : common
            .unique_gift_colors, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15> {
        meta: Meta,
        marker: PhantomData<(
            P0,
            P1,
            P2,
            P3,
            P4,
            P5,
            P6,
            P7,
            P8,
            P9,
            P10,
            P11,
            P12,
            P13,
            P14,
            P15,
        )>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
            P7: Deserialize<'de> + Default,
            P8: Deserialize<'de> + Default,
            P9: Deserialize<'de> + Default,
            P10: Deserialize<'de> + Default,
            P11: Deserialize<'de> + Default,
            P12: Deserialize<'de> + Default,
            P13: Deserialize<'de> + Default,
            P14: Deserialize<'de> + Default,
            P15: Deserialize<'de> + Default,
        > Visitor<'de>
        for Visitor2<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15>
    {
        type Value = (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
            ),
        );

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let is_direct_messages =
                seq_field::<A, Option<bool>>(&mut seq, 4usize, &self.meta.sequence)?;
            let accent_color_id = seq_field::<A, i64>(&mut seq, 5usize, &self.meta.sequence)?;
            let max_reaction_count = seq_field::<A, i64>(&mut seq, 6usize, &self.meta.sequence)?;
            let photo = seq_field::<A, Option<crate::types::ChatPhoto>>(
                &mut seq,
                7usize,
                &self.meta.sequence,
            )?;
            let value_3 = seq_field::<A, P3>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 12usize, &self.meta.sequence)?;
            let value_8 = seq_field::<A, P8>(&mut seq, 13usize, &self.meta.sequence)?;
            let available_reactions = seq_field::<A, Option<Box<[crate::types::ReactionType]>>>(
                &mut seq,
                14usize,
                &self.meta.sequence,
            )?;
            let background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 15usize, &self.meta.sequence)?;
            let profile_accent_color_id =
                seq_field::<A, Option<i64>>(&mut seq, 16usize, &self.meta.sequence)?;
            let profile_background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 17usize, &self.meta.sequence)?;
            let value_9 = seq_field::<A, P9>(&mut seq, 18usize, &self.meta.sequence)?;
            let value_10 = seq_field::<A, P10>(&mut seq, 19usize, &self.meta.sequence)?;
            let value_11 = seq_field::<A, P11>(&mut seq, 20usize, &self.meta.sequence)?;
            let value_12 = seq_field::<A, P12>(&mut seq, 21usize, &self.meta.sequence)?;
            let value_13 = seq_field::<A, P13>(&mut seq, 22usize, &self.meta.sequence)?;
            let pinned_message = seq_field::<A, Option<Box<crate::types::Message>>>(
                &mut seq,
                23usize,
                &self.meta.sequence,
            )?;
            let accepted_gift_types = seq_field::<A, crate::types::AcceptedGiftTypes>(
                &mut seq,
                24usize,
                &self.meta.sequence,
            )?;
            let message_auto_delete_time =
                seq_field::<A, Option<i64>>(&mut seq, 25usize, &self.meta.sequence)?;
            let has_hidden_members =
                seq_field::<A, Option<bool>>(&mut seq, 26usize, &self.meta.sequence)?;
            let has_protected_content =
                seq_field::<A, Option<bool>>(&mut seq, 27usize, &self.meta.sequence)?;
            let has_visible_history =
                seq_field::<A, Option<bool>>(&mut seq, 28usize, &self.meta.sequence)?;
            let value_14 = seq_field::<A, P14>(&mut seq, 29usize, &self.meta.sequence)?;
            let value_15 = seq_field::<A, P15>(&mut seq, 30usize, &self.meta.sequence)?;
            let unique_gift_colors = seq_field::<A, Option<crate::types::UniqueGiftColors>>(
                &mut seq,
                31usize,
                &self.meta.sequence,
            )?;
            let paid_message_star_count =
                seq_field::<A, Option<i64>>(&mut seq, 32usize, &self.meta.sequence)?;
            let guard_bot = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                33usize,
                &self.meta.sequence,
            )?;
            let community = seq_field::<A, Option<crate::types::Community>>(
                &mut seq,
                34usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            let mut value_9 = None;
            let mut value_10 = None;
            let mut value_11 = None;
            let mut value_12 = None;
            let mut value_13 = None;
            let mut value_14 = None;
            let mut value_15 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 19usize => common.read(index, &mut map)?,
                    19usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    20usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    21usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    22usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    23usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    24usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    25usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    26usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    27usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    28usize => {
                        map_field(&mut value_9, &mut map, self.meta.specific[9usize])?;
                    }
                    29usize => {
                        map_field(&mut value_10, &mut map, self.meta.specific[10usize])?;
                    }
                    30usize => {
                        map_field(&mut value_11, &mut map, self.meta.specific[11usize])?;
                    }
                    31usize => {
                        map_field(&mut value_12, &mut map, self.meta.specific[12usize])?;
                    }
                    32usize => {
                        map_field(&mut value_13, &mut map, self.meta.specific[13usize])?;
                    }
                    33usize => {
                        map_field(&mut value_14, &mut map, self.meta.specific[14usize])?;
                    }
                    34usize => {
                        map_field(&mut value_15, &mut map, self.meta.specific[15usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let is_direct_messages = common.is_direct_messages.unwrap_or(None);
            let accent_color_id = common
                .accent_color_id
                .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
            let max_reaction_count = common
                .max_reaction_count
                .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
            let photo = common.photo.unwrap_or(None);
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            let value_7 = value_7.unwrap_or_default();
            let value_8 = value_8.unwrap_or_default();
            let available_reactions = common.available_reactions.unwrap_or(None);
            let background_custom_emoji_id = common.background_custom_emoji_id.unwrap_or(None);
            let profile_accent_color_id = common.profile_accent_color_id.unwrap_or(None);
            let profile_background_custom_emoji_id =
                common.profile_background_custom_emoji_id.unwrap_or(None);
            let value_9 = value_9.unwrap_or_default();
            let value_10 = value_10.unwrap_or_default();
            let value_11 = value_11.unwrap_or_default();
            let value_12 = value_12.unwrap_or_default();
            let value_13 = value_13.unwrap_or_default();
            let pinned_message = common.pinned_message.unwrap_or(None);
            let accepted_gift_types = common
                .accepted_gift_types
                .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
            let message_auto_delete_time = common.message_auto_delete_time.unwrap_or(None);
            let has_hidden_members = common.has_hidden_members.unwrap_or(None);
            let has_protected_content = common.has_protected_content.unwrap_or(None);
            let has_visible_history = common.has_visible_history.unwrap_or(None);
            let value_14 = value_14.unwrap_or_default();
            let value_15 = value_15.unwrap_or_default();
            let unique_gift_colors = common.unique_gift_colors.unwrap_or(None);
            let paid_message_star_count = common.paid_message_star_count.unwrap_or(None);
            let guard_bot = common.guard_bot.unwrap_or(None);
            let community = common.community.unwrap_or(None);
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                ),
            ))
        }
    }
    pub(crate) fn read_2<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
        P7: Deserialize<'de> + Default,
        P8: Deserialize<'de> + Default,
        P9: Deserialize<'de> + Default,
        P10: Deserialize<'de> + Default,
        P11: Deserialize<'de> + Default,
        P12: Deserialize<'de> + Default,
        P13: Deserialize<'de> + Default,
        P14: Deserialize<'de> + Default,
        P15: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<
        (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
            ),
        ),
        D::Error,
    > {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty, $field_9:ident : $P9:ty, $field_10:ident : $P10:ty, $field_11:ident :
            $P11:ty, $field_12:ident : $P12:ty, $field_13:ident : $P13:ty,
            $field_14:ident : $P14:ty, $field_15:ident : $P15:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8, value_9, value_10, value_11, value_12, value_13, value_14,
            value_15,)) = $crate::deserialization::chat_full_info::read_2:: < D, $P0,
            $P1, $P2, $P3, $P4, $P5, $P6, $P7, $P8, $P9, $P10, $P11, $P12, $P13, $P14,
            $P15 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            "is_direct_messages", "accent_color_id", "max_reaction_count", "photo",
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            "available_reactions", "background_custom_emoji_id",
            "profile_accent_color_id", "profile_background_custom_emoji_id",
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            $crate::deserialization::field_name(stringify!($field_12)),
            $crate::deserialization::field_name(stringify!($field_13)), "pinned_message",
            "accepted_gift_types", "message_auto_delete_time", "has_hidden_members",
            "has_protected_content", "has_visible_history",
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15)),
            "unique_gift_colors", "paid_message_star_count", "guard_bot", "community"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            $crate::deserialization::field_name(stringify!($field_12)),
            $crate::deserialization::field_name(stringify!($field_13)),
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15))] }, sequence :
            concat!("struct ", stringify!($name), " with 35 elements"), },) ?; Ok(Self {
            accent_color_id : common.accent_color_id, accepted_gift_types : common
            .accepted_gift_types, available_reactions : common.available_reactions,
            background_custom_emoji_id : common.background_custom_emoji_id, community :
            common.community, guard_bot : common.guard_bot, has_hidden_members : common
            .has_hidden_members, has_protected_content : common.has_protected_content,
            has_visible_history : common.has_visible_history, id : common.id,
            is_direct_messages : common.is_direct_messages, max_reaction_count : common
            .max_reaction_count, message_auto_delete_time : common
            .message_auto_delete_time, paid_message_star_count : common
            .paid_message_star_count, photo : common.photo, pinned_message : common
            .pinned_message, profile_accent_color_id : common.profile_accent_color_id,
            profile_background_custom_emoji_id : common
            .profile_background_custom_emoji_id, unique_gift_colors : common
            .unique_gift_colors, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, $field_4 : value_4, $field_5 : value_5, $field_6
            : value_6, $field_7 : value_7, $field_8 : value_8, $field_9 : value_9,
            $field_10 : value_10, $field_11 : value_11, $field_12 : value_12, $field_13 :
            value_13, $field_14 : value_14, $field_15 : value_15, }) } }
        };
    }
    pub(crate) use deserialize_2;
    struct Visitor3<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17> {
        meta: Meta,
        marker: PhantomData<(
            P0,
            P1,
            P2,
            P3,
            P4,
            P5,
            P6,
            P7,
            P8,
            P9,
            P10,
            P11,
            P12,
            P13,
            P14,
            P15,
            P16,
            P17,
        )>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
            P7: Deserialize<'de> + Default,
            P8: Deserialize<'de> + Default,
            P9: Deserialize<'de> + Default,
            P10: Deserialize<'de> + Default,
            P11: Deserialize<'de> + Default,
            P12: Deserialize<'de> + Default,
            P13: Deserialize<'de> + Default,
            P14: Deserialize<'de> + Default,
            P15: Deserialize<'de> + Default,
            P16: Deserialize<'de> + Default,
            P17: Deserialize<'de> + Default,
        > Visitor<'de>
        for Visitor3<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17>
    {
        type Value = (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
                P16,
                P17,
            ),
        );

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let is_direct_messages =
                seq_field::<A, Option<bool>>(&mut seq, 4usize, &self.meta.sequence)?;
            let accent_color_id = seq_field::<A, i64>(&mut seq, 5usize, &self.meta.sequence)?;
            let max_reaction_count = seq_field::<A, i64>(&mut seq, 6usize, &self.meta.sequence)?;
            let photo = seq_field::<A, Option<crate::types::ChatPhoto>>(
                &mut seq,
                7usize,
                &self.meta.sequence,
            )?;
            let value_3 = seq_field::<A, P3>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 9usize, &self.meta.sequence)?;
            let available_reactions = seq_field::<A, Option<Box<[crate::types::ReactionType]>>>(
                &mut seq,
                10usize,
                &self.meta.sequence,
            )?;
            let background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 11usize, &self.meta.sequence)?;
            let profile_accent_color_id =
                seq_field::<A, Option<i64>>(&mut seq, 12usize, &self.meta.sequence)?;
            let profile_background_custom_emoji_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 13usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 14usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 15usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 16usize, &self.meta.sequence)?;
            let value_8 = seq_field::<A, P8>(&mut seq, 17usize, &self.meta.sequence)?;
            let pinned_message = seq_field::<A, Option<Box<crate::types::Message>>>(
                &mut seq,
                18usize,
                &self.meta.sequence,
            )?;
            let value_9 = seq_field::<A, P9>(&mut seq, 19usize, &self.meta.sequence)?;
            let accepted_gift_types = seq_field::<A, crate::types::AcceptedGiftTypes>(
                &mut seq,
                20usize,
                &self.meta.sequence,
            )?;
            let value_10 = seq_field::<A, P10>(&mut seq, 21usize, &self.meta.sequence)?;
            let value_11 = seq_field::<A, P11>(&mut seq, 22usize, &self.meta.sequence)?;
            let message_auto_delete_time =
                seq_field::<A, Option<i64>>(&mut seq, 23usize, &self.meta.sequence)?;
            let value_12 = seq_field::<A, P12>(&mut seq, 24usize, &self.meta.sequence)?;
            let has_hidden_members =
                seq_field::<A, Option<bool>>(&mut seq, 25usize, &self.meta.sequence)?;
            let has_protected_content =
                seq_field::<A, Option<bool>>(&mut seq, 26usize, &self.meta.sequence)?;
            let has_visible_history =
                seq_field::<A, Option<bool>>(&mut seq, 27usize, &self.meta.sequence)?;
            let value_13 = seq_field::<A, P13>(&mut seq, 28usize, &self.meta.sequence)?;
            let value_14 = seq_field::<A, P14>(&mut seq, 29usize, &self.meta.sequence)?;
            let value_15 = seq_field::<A, P15>(&mut seq, 30usize, &self.meta.sequence)?;
            let value_16 = seq_field::<A, P16>(&mut seq, 31usize, &self.meta.sequence)?;
            let value_17 = seq_field::<A, P17>(&mut seq, 32usize, &self.meta.sequence)?;
            let unique_gift_colors = seq_field::<A, Option<crate::types::UniqueGiftColors>>(
                &mut seq,
                33usize,
                &self.meta.sequence,
            )?;
            let paid_message_star_count =
                seq_field::<A, Option<i64>>(&mut seq, 34usize, &self.meta.sequence)?;
            let guard_bot = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                35usize,
                &self.meta.sequence,
            )?;
            let community = seq_field::<A, Option<crate::types::Community>>(
                &mut seq,
                36usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                    value_16, value_17,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            let mut value_9 = None;
            let mut value_10 = None;
            let mut value_11 = None;
            let mut value_12 = None;
            let mut value_13 = None;
            let mut value_14 = None;
            let mut value_15 = None;
            let mut value_16 = None;
            let mut value_17 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 19usize => common.read(index, &mut map)?,
                    19usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    20usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    21usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    22usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    23usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    24usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    25usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    26usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    27usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    28usize => {
                        map_field(&mut value_9, &mut map, self.meta.specific[9usize])?;
                    }
                    29usize => {
                        map_field(&mut value_10, &mut map, self.meta.specific[10usize])?;
                    }
                    30usize => {
                        map_field(&mut value_11, &mut map, self.meta.specific[11usize])?;
                    }
                    31usize => {
                        map_field(&mut value_12, &mut map, self.meta.specific[12usize])?;
                    }
                    32usize => {
                        map_field(&mut value_13, &mut map, self.meta.specific[13usize])?;
                    }
                    33usize => {
                        map_field(&mut value_14, &mut map, self.meta.specific[14usize])?;
                    }
                    34usize => {
                        map_field(&mut value_15, &mut map, self.meta.specific[15usize])?;
                    }
                    35usize => {
                        map_field(&mut value_16, &mut map, self.meta.specific[16usize])?;
                    }
                    36usize => {
                        map_field(&mut value_17, &mut map, self.meta.specific[17usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let is_direct_messages = common.is_direct_messages.unwrap_or(None);
            let accent_color_id = common
                .accent_color_id
                .ok_or_else(|| A::Error::missing_field("accent_color_id"))?;
            let max_reaction_count = common
                .max_reaction_count
                .ok_or_else(|| A::Error::missing_field("max_reaction_count"))?;
            let photo = common.photo.unwrap_or(None);
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let available_reactions = common.available_reactions.unwrap_or(None);
            let background_custom_emoji_id = common.background_custom_emoji_id.unwrap_or(None);
            let profile_accent_color_id = common.profile_accent_color_id.unwrap_or(None);
            let profile_background_custom_emoji_id =
                common.profile_background_custom_emoji_id.unwrap_or(None);
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            let value_7 = value_7.unwrap_or_default();
            let value_8 = value_8.unwrap_or_default();
            let pinned_message = common.pinned_message.unwrap_or(None);
            let value_9 = value_9.unwrap_or_default();
            let accepted_gift_types = common
                .accepted_gift_types
                .ok_or_else(|| A::Error::missing_field("accepted_gift_types"))?;
            let value_10 = value_10.unwrap_or_default();
            let value_11 = value_11.unwrap_or_default();
            let message_auto_delete_time = common.message_auto_delete_time.unwrap_or(None);
            let value_12 = value_12.unwrap_or_default();
            let has_hidden_members = common.has_hidden_members.unwrap_or(None);
            let has_protected_content = common.has_protected_content.unwrap_or(None);
            let has_visible_history = common.has_visible_history.unwrap_or(None);
            let value_13 = value_13.unwrap_or_default();
            let value_14 = value_14.unwrap_or_default();
            let value_15 = value_15.unwrap_or_default();
            let value_16 = value_16.unwrap_or_default();
            let value_17 = value_17.unwrap_or_default();
            let unique_gift_colors = common.unique_gift_colors.unwrap_or(None);
            let paid_message_star_count = common.paid_message_star_count.unwrap_or(None);
            let guard_bot = common.guard_bot.unwrap_or(None);
            let community = common.community.unwrap_or(None);
            Ok((
                Common {
                    accent_color_id,
                    accepted_gift_types,
                    available_reactions,
                    background_custom_emoji_id,
                    community,
                    guard_bot,
                    has_hidden_members,
                    has_protected_content,
                    has_visible_history,
                    id,
                    is_direct_messages,
                    max_reaction_count,
                    message_auto_delete_time,
                    paid_message_star_count,
                    photo,
                    pinned_message,
                    profile_accent_color_id,
                    profile_background_custom_emoji_id,
                    unique_gift_colors,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                    value_16, value_17,
                ),
            ))
        }
    }
    pub(crate) fn read_3<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
        P7: Deserialize<'de> + Default,
        P8: Deserialize<'de> + Default,
        P9: Deserialize<'de> + Default,
        P10: Deserialize<'de> + Default,
        P11: Deserialize<'de> + Default,
        P12: Deserialize<'de> + Default,
        P13: Deserialize<'de> + Default,
        P14: Deserialize<'de> + Default,
        P15: Deserialize<'de> + Default,
        P16: Deserialize<'de> + Default,
        P17: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<
        (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
                P16,
                P17,
            ),
        ),
        D::Error,
    > {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor3 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_3 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty, $field_9:ident : $P9:ty, $field_10:ident : $P10:ty, $field_11:ident :
            $P11:ty, $field_12:ident : $P12:ty, $field_13:ident : $P13:ty,
            $field_14:ident : $P14:ty, $field_15:ident : $P15:ty, $field_16:ident :
            $P16:ty, $field_17:ident : $P17:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
            value_16, value_17,)) = $crate::deserialization::chat_full_info::read_3:: <
            D, $P0, $P1, $P2, $P3, $P4, $P5, $P6, $P7, $P8, $P9, $P10, $P11, $P12, $P13,
            $P14, $P15, $P16, $P17 > (deserializer, $crate::deserialization::Meta { name
            : stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            "is_direct_messages", "accent_color_id", "max_reaction_count", "photo",
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            "available_reactions", "background_custom_emoji_id",
            "profile_accent_color_id", "profile_background_custom_emoji_id",
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)), "pinned_message",
            $crate::deserialization::field_name(stringify!($field_9)),
            "accepted_gift_types",
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            "message_auto_delete_time",
            $crate::deserialization::field_name(stringify!($field_12)),
            "has_hidden_members", "has_protected_content", "has_visible_history",
            $crate::deserialization::field_name(stringify!($field_13)),
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15)),
            $crate::deserialization::field_name(stringify!($field_16)),
            $crate::deserialization::field_name(stringify!($field_17)),
            "unique_gift_colors", "paid_message_star_count", "guard_bot", "community"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            $crate::deserialization::field_name(stringify!($field_12)),
            $crate::deserialization::field_name(stringify!($field_13)),
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15)),
            $crate::deserialization::field_name(stringify!($field_16)),
            $crate::deserialization::field_name(stringify!($field_17))] }, sequence :
            concat!("struct ", stringify!($name), " with 37 elements"), },) ?; Ok(Self {
            accent_color_id : common.accent_color_id, accepted_gift_types : common
            .accepted_gift_types, available_reactions : common.available_reactions,
            background_custom_emoji_id : common.background_custom_emoji_id, community :
            common.community, guard_bot : common.guard_bot, has_hidden_members : common
            .has_hidden_members, has_protected_content : common.has_protected_content,
            has_visible_history : common.has_visible_history, id : common.id,
            is_direct_messages : common.is_direct_messages, max_reaction_count : common
            .max_reaction_count, message_auto_delete_time : common
            .message_auto_delete_time, paid_message_star_count : common
            .paid_message_star_count, photo : common.photo, pinned_message : common
            .pinned_message, profile_accent_color_id : common.profile_accent_color_id,
            profile_background_custom_emoji_id : common
            .profile_background_custom_emoji_id, unique_gift_colors : common
            .unique_gift_colors, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, $field_4 : value_4, $field_5 : value_5, $field_6
            : value_6, $field_7 : value_7, $field_8 : value_8, $field_9 : value_9,
            $field_10 : value_10, $field_11 : value_11, $field_12 : value_12, $field_13 :
            value_13, $field_14 : value_14, $field_15 : value_15, $field_16 : value_16,
            $field_17 : value_17, }) } }
        };
    }
    pub(crate) use deserialize_3;
}
pub(crate) mod chat_member {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) user: Box<crate::types::User>,
    }
    #[derive(Default)]
    struct Slots {
        user: Option<Box<crate::types::User>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 1usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "user" => Some(0usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"user" => Some(0usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.user, map, "user"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
        P19,
    > {
        meta: Meta,
        marker: PhantomData<(
            P0,
            P1,
            P2,
            P3,
            P4,
            P5,
            P6,
            P7,
            P8,
            P9,
            P10,
            P11,
            P12,
            P13,
            P14,
            P15,
            P16,
            P17,
            P18,
            P19,
        )>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de>,
            P3: Deserialize<'de>,
            P4: Deserialize<'de>,
            P5: Deserialize<'de>,
            P6: Deserialize<'de>,
            P7: Deserialize<'de>,
            P8: Deserialize<'de>,
            P9: Deserialize<'de>,
            P10: Deserialize<'de>,
            P11: Deserialize<'de>,
            P12: Deserialize<'de> + Default,
            P13: Deserialize<'de> + Default,
            P14: Deserialize<'de> + Default,
            P15: Deserialize<'de> + Default,
            P16: Deserialize<'de> + Default,
            P17: Deserialize<'de> + Default,
            P18: Deserialize<'de>,
            P19: Deserialize<'de> + Default,
        > Visitor<'de>
        for Visitor0<
            P0,
            P1,
            P2,
            P3,
            P4,
            P5,
            P6,
            P7,
            P8,
            P9,
            P10,
            P11,
            P12,
            P13,
            P14,
            P15,
            P16,
            P17,
            P18,
            P19,
        >
    {
        type Value = (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
                P16,
                P17,
                P18,
                P19,
            ),
        );

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_8 = seq_field::<A, P8>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_9 = seq_field::<A, P9>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_10 = seq_field::<A, P10>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_11 = seq_field::<A, P11>(&mut seq, 12usize, &self.meta.sequence)?;
            let value_12 = seq_field::<A, P12>(&mut seq, 13usize, &self.meta.sequence)?;
            let value_13 = seq_field::<A, P13>(&mut seq, 14usize, &self.meta.sequence)?;
            let value_14 = seq_field::<A, P14>(&mut seq, 15usize, &self.meta.sequence)?;
            let value_15 = seq_field::<A, P15>(&mut seq, 16usize, &self.meta.sequence)?;
            let value_16 = seq_field::<A, P16>(&mut seq, 17usize, &self.meta.sequence)?;
            let value_17 = seq_field::<A, P17>(&mut seq, 18usize, &self.meta.sequence)?;
            let value_18 = seq_field::<A, P18>(&mut seq, 19usize, &self.meta.sequence)?;
            let value_19 = seq_field::<A, P19>(&mut seq, 20usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                    value_16, value_17, value_18, value_19,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            let mut value_9 = None;
            let mut value_10 = None;
            let mut value_11 = None;
            let mut value_12 = None;
            let mut value_13 = None;
            let mut value_14 = None;
            let mut value_15 = None;
            let mut value_16 = None;
            let mut value_17 = None;
            let mut value_18 = None;
            let mut value_19 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    4usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    5usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    6usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    7usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    8usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    9usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    10usize => {
                        map_field(&mut value_9, &mut map, self.meta.specific[9usize])?;
                    }
                    11usize => {
                        map_field(&mut value_10, &mut map, self.meta.specific[10usize])?;
                    }
                    12usize => {
                        map_field(&mut value_11, &mut map, self.meta.specific[11usize])?;
                    }
                    13usize => {
                        map_field(&mut value_12, &mut map, self.meta.specific[12usize])?;
                    }
                    14usize => {
                        map_field(&mut value_13, &mut map, self.meta.specific[13usize])?;
                    }
                    15usize => {
                        map_field(&mut value_14, &mut map, self.meta.specific[14usize])?;
                    }
                    16usize => {
                        map_field(&mut value_15, &mut map, self.meta.specific[15usize])?;
                    }
                    17usize => {
                        map_field(&mut value_16, &mut map, self.meta.specific[16usize])?;
                    }
                    18usize => {
                        map_field(&mut value_17, &mut map, self.meta.specific[17usize])?;
                    }
                    19usize => {
                        map_field(&mut value_18, &mut map, self.meta.specific[18usize])?;
                    }
                    20usize => {
                        map_field(&mut value_19, &mut map, self.meta.specific[19usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 =
                value_2.ok_or_else(|| A::Error::missing_field(self.meta.specific[2usize]))?;
            let value_3 =
                value_3.ok_or_else(|| A::Error::missing_field(self.meta.specific[3usize]))?;
            let value_4 =
                value_4.ok_or_else(|| A::Error::missing_field(self.meta.specific[4usize]))?;
            let value_5 =
                value_5.ok_or_else(|| A::Error::missing_field(self.meta.specific[5usize]))?;
            let value_6 =
                value_6.ok_or_else(|| A::Error::missing_field(self.meta.specific[6usize]))?;
            let value_7 =
                value_7.ok_or_else(|| A::Error::missing_field(self.meta.specific[7usize]))?;
            let value_8 =
                value_8.ok_or_else(|| A::Error::missing_field(self.meta.specific[8usize]))?;
            let value_9 =
                value_9.ok_or_else(|| A::Error::missing_field(self.meta.specific[9usize]))?;
            let value_10 =
                value_10.ok_or_else(|| A::Error::missing_field(self.meta.specific[10usize]))?;
            let value_11 =
                value_11.ok_or_else(|| A::Error::missing_field(self.meta.specific[11usize]))?;
            let value_12 = value_12.unwrap_or_default();
            let value_13 = value_13.unwrap_or_default();
            let value_14 = value_14.unwrap_or_default();
            let value_15 = value_15.unwrap_or_default();
            let value_16 = value_16.unwrap_or_default();
            let value_17 = value_17.unwrap_or_default();
            let value_18 =
                value_18.ok_or_else(|| A::Error::missing_field(self.meta.specific[18usize]))?;
            let value_19 = value_19.unwrap_or_default();
            Ok((
                Common {
                    user,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                    value_16, value_17, value_18, value_19,
                ),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de>,
        P3: Deserialize<'de>,
        P4: Deserialize<'de>,
        P5: Deserialize<'de>,
        P6: Deserialize<'de>,
        P7: Deserialize<'de>,
        P8: Deserialize<'de>,
        P9: Deserialize<'de>,
        P10: Deserialize<'de>,
        P11: Deserialize<'de>,
        P12: Deserialize<'de> + Default,
        P13: Deserialize<'de> + Default,
        P14: Deserialize<'de> + Default,
        P15: Deserialize<'de> + Default,
        P16: Deserialize<'de> + Default,
        P17: Deserialize<'de> + Default,
        P18: Deserialize<'de>,
        P19: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<
        (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
                P16,
                P17,
                P18,
                P19,
            ),
        ),
        D::Error,
    > {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty, $field_9:ident : $P9:ty, $field_10:ident : $P10:ty, $field_11:ident :
            $P11:ty, $field_12:ident : $P12:ty, $field_13:ident : $P13:ty,
            $field_14:ident : $P14:ty, $field_15:ident : $P15:ty, $field_16:ident :
            $P16:ty, $field_17:ident : $P17:ty, $field_18:ident : $P18:ty,
            $field_19:ident : $P19:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
            value_16, value_17, value_18, value_19,)) =
            $crate::deserialization::chat_member::read_0:: < D, $P0, $P1, $P2, $P3, $P4,
            $P5, $P6, $P7, $P8, $P9, $P10, $P11, $P12, $P13, $P14, $P15, $P16, $P17,
            $P18, $P19 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["user",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            $crate::deserialization::field_name(stringify!($field_12)),
            $crate::deserialization::field_name(stringify!($field_13)),
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15)),
            $crate::deserialization::field_name(stringify!($field_16)),
            $crate::deserialization::field_name(stringify!($field_17)),
            $crate::deserialization::field_name(stringify!($field_18)),
            $crate::deserialization::field_name(stringify!($field_19))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            $crate::deserialization::field_name(stringify!($field_12)),
            $crate::deserialization::field_name(stringify!($field_13)),
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15)),
            $crate::deserialization::field_name(stringify!($field_16)),
            $crate::deserialization::field_name(stringify!($field_17)),
            $crate::deserialization::field_name(stringify!($field_18)),
            $crate::deserialization::field_name(stringify!($field_19))] }, sequence :
            concat!("struct ", stringify!($name), " with 21 elements"), },) ?; Ok(Self {
            user : common.user, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, $field_4 : value_4, $field_5 : value_5, $field_6
            : value_6, $field_7 : value_7, $field_8 : value_8, $field_9 : value_9,
            $field_10 : value_10, $field_11 : value_11, $field_12 : value_12, $field_13 :
            value_13, $field_14 : value_14, $field_15 : value_15, $field_16 : value_16,
            $field_17 : value_17, $field_18 : value_18, $field_19 : value_19, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor1<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    user,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::chat_member::read_1:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["user",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 2 elements"), },) ?;
            Ok(Self { user : common.user, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2 {
        meta: Meta,
        marker: PhantomData<()>,
    }
    impl<'de> Visitor<'de> for Visitor2 {
        type Value = (Common, ());

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 0usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            Ok((
                Common {
                    user,
                },
                (),
            ))
        }
    }
    pub(crate) fn read_2<'de, D: Deserializer<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, ()), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        ($name:ident,) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common, ()) =
            $crate::deserialization::chat_member::read_2:: < D, > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["user"] }, specific : const { & [] }, sequence : concat!("struct ",
            stringify!($name), " with 1 element"), },) ?; Ok(Self { user : common.user,
            }) } }
        };
    }
    pub(crate) use deserialize_2;
    struct Visitor3<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de> + Default, P1: Deserialize<'de> + Default> Visitor<'de>
        for Visitor3<P0, P1>
    {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 = value_0.unwrap_or_default();
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            let value_1 = value_1.unwrap_or_default();
            Ok((
                Common {
                    user,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_3<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor3 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_3 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::chat_member::read_3:: < D,
            $P0, $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)), "user",
            $crate::deserialization::field_name(stringify!($field_1))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 3 elements"), },) ?; Ok(Self {
            user : common.user, $field_0 : value_0, $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_3;
    struct Visitor4<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de> + Default> Visitor<'de> for Visitor4<P0, P1> {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 = value_1.unwrap_or_default();
            Ok((
                Common {
                    user,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_4<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor4 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_4 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::chat_member::read_4:: < D,
            $P0, $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["user",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 3 elements"), },) ?; Ok(Self {
            user : common.user, $field_0 : value_0, $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_4;
    struct Visitor5<
        P0,
        P1,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
        P17,
        P18,
    > {
        meta: Meta,
        marker: PhantomData<(
            P0,
            P1,
            P2,
            P3,
            P4,
            P5,
            P6,
            P7,
            P8,
            P9,
            P10,
            P11,
            P12,
            P13,
            P14,
            P15,
            P16,
            P17,
            P18,
        )>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de>,
            P2: Deserialize<'de>,
            P3: Deserialize<'de>,
            P4: Deserialize<'de>,
            P5: Deserialize<'de>,
            P6: Deserialize<'de>,
            P7: Deserialize<'de>,
            P8: Deserialize<'de>,
            P9: Deserialize<'de>,
            P10: Deserialize<'de>,
            P11: Deserialize<'de>,
            P12: Deserialize<'de>,
            P13: Deserialize<'de>,
            P14: Deserialize<'de>,
            P15: Deserialize<'de>,
            P16: Deserialize<'de>,
            P17: Deserialize<'de>,
            P18: Deserialize<'de>,
        > Visitor<'de>
        for Visitor5<
            P0,
            P1,
            P2,
            P3,
            P4,
            P5,
            P6,
            P7,
            P8,
            P9,
            P10,
            P11,
            P12,
            P13,
            P14,
            P15,
            P16,
            P17,
            P18,
        >
    {
        type Value = (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
                P16,
                P17,
                P18,
            ),
        );

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_8 = seq_field::<A, P8>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_9 = seq_field::<A, P9>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_10 = seq_field::<A, P10>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_11 = seq_field::<A, P11>(&mut seq, 12usize, &self.meta.sequence)?;
            let value_12 = seq_field::<A, P12>(&mut seq, 13usize, &self.meta.sequence)?;
            let value_13 = seq_field::<A, P13>(&mut seq, 14usize, &self.meta.sequence)?;
            let value_14 = seq_field::<A, P14>(&mut seq, 15usize, &self.meta.sequence)?;
            let value_15 = seq_field::<A, P15>(&mut seq, 16usize, &self.meta.sequence)?;
            let value_16 = seq_field::<A, P16>(&mut seq, 17usize, &self.meta.sequence)?;
            let value_17 = seq_field::<A, P17>(&mut seq, 18usize, &self.meta.sequence)?;
            let value_18 = seq_field::<A, P18>(&mut seq, 19usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                    value_16, value_17, value_18,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            let mut value_9 = None;
            let mut value_10 = None;
            let mut value_11 = None;
            let mut value_12 = None;
            let mut value_13 = None;
            let mut value_14 = None;
            let mut value_15 = None;
            let mut value_16 = None;
            let mut value_17 = None;
            let mut value_18 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    4usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    5usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    6usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    7usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    8usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    9usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    10usize => {
                        map_field(&mut value_9, &mut map, self.meta.specific[9usize])?;
                    }
                    11usize => {
                        map_field(&mut value_10, &mut map, self.meta.specific[10usize])?;
                    }
                    12usize => {
                        map_field(&mut value_11, &mut map, self.meta.specific[11usize])?;
                    }
                    13usize => {
                        map_field(&mut value_12, &mut map, self.meta.specific[12usize])?;
                    }
                    14usize => {
                        map_field(&mut value_13, &mut map, self.meta.specific[13usize])?;
                    }
                    15usize => {
                        map_field(&mut value_14, &mut map, self.meta.specific[14usize])?;
                    }
                    16usize => {
                        map_field(&mut value_15, &mut map, self.meta.specific[15usize])?;
                    }
                    17usize => {
                        map_field(&mut value_16, &mut map, self.meta.specific[16usize])?;
                    }
                    18usize => {
                        map_field(&mut value_17, &mut map, self.meta.specific[17usize])?;
                    }
                    19usize => {
                        map_field(&mut value_18, &mut map, self.meta.specific[18usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 = value_0.unwrap_or_default();
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 =
                value_2.ok_or_else(|| A::Error::missing_field(self.meta.specific[2usize]))?;
            let value_3 =
                value_3.ok_or_else(|| A::Error::missing_field(self.meta.specific[3usize]))?;
            let value_4 =
                value_4.ok_or_else(|| A::Error::missing_field(self.meta.specific[4usize]))?;
            let value_5 =
                value_5.ok_or_else(|| A::Error::missing_field(self.meta.specific[5usize]))?;
            let value_6 =
                value_6.ok_or_else(|| A::Error::missing_field(self.meta.specific[6usize]))?;
            let value_7 =
                value_7.ok_or_else(|| A::Error::missing_field(self.meta.specific[7usize]))?;
            let value_8 =
                value_8.ok_or_else(|| A::Error::missing_field(self.meta.specific[8usize]))?;
            let value_9 =
                value_9.ok_or_else(|| A::Error::missing_field(self.meta.specific[9usize]))?;
            let value_10 =
                value_10.ok_or_else(|| A::Error::missing_field(self.meta.specific[10usize]))?;
            let value_11 =
                value_11.ok_or_else(|| A::Error::missing_field(self.meta.specific[11usize]))?;
            let value_12 =
                value_12.ok_or_else(|| A::Error::missing_field(self.meta.specific[12usize]))?;
            let value_13 =
                value_13.ok_or_else(|| A::Error::missing_field(self.meta.specific[13usize]))?;
            let value_14 =
                value_14.ok_or_else(|| A::Error::missing_field(self.meta.specific[14usize]))?;
            let value_15 =
                value_15.ok_or_else(|| A::Error::missing_field(self.meta.specific[15usize]))?;
            let value_16 =
                value_16.ok_or_else(|| A::Error::missing_field(self.meta.specific[16usize]))?;
            let value_17 =
                value_17.ok_or_else(|| A::Error::missing_field(self.meta.specific[17usize]))?;
            let value_18 =
                value_18.ok_or_else(|| A::Error::missing_field(self.meta.specific[18usize]))?;
            Ok((
                Common {
                    user,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
                    value_16, value_17, value_18,
                ),
            ))
        }
    }
    pub(crate) fn read_5<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de>,
        P2: Deserialize<'de>,
        P3: Deserialize<'de>,
        P4: Deserialize<'de>,
        P5: Deserialize<'de>,
        P6: Deserialize<'de>,
        P7: Deserialize<'de>,
        P8: Deserialize<'de>,
        P9: Deserialize<'de>,
        P10: Deserialize<'de>,
        P11: Deserialize<'de>,
        P12: Deserialize<'de>,
        P13: Deserialize<'de>,
        P14: Deserialize<'de>,
        P15: Deserialize<'de>,
        P16: Deserialize<'de>,
        P17: Deserialize<'de>,
        P18: Deserialize<'de>,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<
        (
            Common,
            (
                P0,
                P1,
                P2,
                P3,
                P4,
                P5,
                P6,
                P7,
                P8,
                P9,
                P10,
                P11,
                P12,
                P13,
                P14,
                P15,
                P16,
                P17,
                P18,
            ),
        ),
        D::Error,
    > {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor5 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_5 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty, $field_9:ident : $P9:ty, $field_10:ident : $P10:ty, $field_11:ident :
            $P11:ty, $field_12:ident : $P12:ty, $field_13:ident : $P13:ty,
            $field_14:ident : $P14:ty, $field_15:ident : $P15:ty, $field_16:ident :
            $P16:ty, $field_17:ident : $P17:ty, $field_18:ident : $P18:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8, value_9, value_10, value_11, value_12, value_13, value_14, value_15,
            value_16, value_17, value_18,)) =
            $crate::deserialization::chat_member::read_5:: < D, $P0, $P1, $P2, $P3, $P4,
            $P5, $P6, $P7, $P8, $P9, $P10, $P11, $P12, $P13, $P14, $P15, $P16, $P17, $P18
            > (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)), "user",
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            $crate::deserialization::field_name(stringify!($field_12)),
            $crate::deserialization::field_name(stringify!($field_13)),
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15)),
            $crate::deserialization::field_name(stringify!($field_16)),
            $crate::deserialization::field_name(stringify!($field_17)),
            $crate::deserialization::field_name(stringify!($field_18))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11)),
            $crate::deserialization::field_name(stringify!($field_12)),
            $crate::deserialization::field_name(stringify!($field_13)),
            $crate::deserialization::field_name(stringify!($field_14)),
            $crate::deserialization::field_name(stringify!($field_15)),
            $crate::deserialization::field_name(stringify!($field_16)),
            $crate::deserialization::field_name(stringify!($field_17)),
            $crate::deserialization::field_name(stringify!($field_18))] }, sequence :
            concat!("struct ", stringify!($name), " with 20 elements"), },) ?; Ok(Self {
            user : common.user, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, $field_4 : value_4, $field_5 : value_5, $field_6
            : value_6, $field_7 : value_7, $field_8 : value_8, $field_9 : value_9,
            $field_10 : value_10, $field_11 : value_11, $field_12 : value_12, $field_13 :
            value_13, $field_14 : value_14, $field_15 : value_15, $field_16 : value_16,
            $field_17 : value_17, $field_18 : value_18, }) } }
        };
    }
    pub(crate) use deserialize_5;
    struct Visitor6<P0, P1, P2> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor6<P0, P1, P2>
    {
        type Value = (Common, (P0, P1, P2));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (value_0, value_1, value_2),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            Ok((
                Common {
                    user,
                },
                (value_0, value_1, value_2),
            ))
        }
    }
    pub(crate) fn read_6<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor6 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_6 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2,)) =
            $crate::deserialization::chat_member::read_6:: < D, $P0, $P1, $P2 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["user",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, sequence :
            concat!("struct ", stringify!($name), " with 4 elements"), },) ?; Ok(Self {
            user : common.user, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, }) } }
        };
    }
    pub(crate) use deserialize_6;
    struct Visitor7<P0, P1, P2> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de>,
            P2: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor7<P0, P1, P2>
    {
        type Value = (Common, (P0, P1, P2));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let user =
                seq_field::<A, Box<crate::types::User>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            Ok((
                Common {
                    user,
                },
                (value_0, value_1, value_2),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let user = common.user.ok_or_else(|| A::Error::missing_field("user"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 = value_2.unwrap_or_default();
            Ok((
                Common {
                    user,
                },
                (value_0, value_1, value_2),
            ))
        }
    }
    pub(crate) fn read_7<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor7 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_7 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2,)) =
            $crate::deserialization::chat_member::read_7:: < D, $P0, $P1, $P2 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["user",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, sequence :
            concat!("struct ", stringify!($name), " with 4 elements"), },) ?; Ok(Self {
            user : common.user, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, }) } }
        };
    }
    pub(crate) use deserialize_7;
}
pub(crate) mod encrypted_passport_element {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) hash: Box<str>,
    }
    #[derive(Default)]
    struct Slots {
        hash: Option<Box<str>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 1usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "hash" => Some(0usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"hash" => Some(0usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.hash, map, "hash"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let hash = seq_field::<A, Box<str>>(&mut seq, 1usize, &self.meta.sequence)?;
            Ok((
                Common {
                    hash,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let hash = common.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
            Ok((
                Common {
                    hash,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::encrypted_passport_element::read_0:: <
            D, $P0 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)), "hash"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 2 elements"), },) ?; Ok(Self {
            hash : common.hash, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de> + Default> Visitor<'de> for Visitor1<P0, P1> {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 1usize, &self.meta.sequence)?;
            let hash = seq_field::<A, Box<str>>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    hash,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 = value_1.unwrap_or_default();
            let hash = common.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
            Ok((
                Common {
                    hash,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) =
            $crate::deserialization::encrypted_passport_element::read_1:: < D, $P0, $P1 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)), "hash"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 3 elements"), },) ?; Ok(Self {
            hash : common.hash, $field_0 : value_0, $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0, P1, P2, P3, P4> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de>,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor2<P0, P1, P2, P3, P4>
    {
        type Value = (Common, (P0, P1, P2, P3, P4));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 4usize, &self.meta.sequence)?;
            let hash = seq_field::<A, Box<str>>(&mut seq, 5usize, &self.meta.sequence)?;
            Ok((
                Common {
                    hash,
                },
                (value_0, value_1, value_2, value_3, value_4),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    4usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    5usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 =
                value_2.ok_or_else(|| A::Error::missing_field(self.meta.specific[2usize]))?;
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let hash = common.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
            Ok((
                Common {
                    hash,
                },
                (value_0, value_1, value_2, value_3, value_4),
            ))
        }
    }
    pub(crate) fn read_2<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de>,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4,)) =
            $crate::deserialization::encrypted_passport_element::read_2:: < D, $P0, $P1,
            $P2, $P3, $P4 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)), "hash"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4))] }, sequence :
            concat!("struct ", stringify!($name), " with 6 elements"), },) ?; Ok(Self {
            hash : common.hash, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, $field_4 : value_4, }) } }
        };
    }
    pub(crate) use deserialize_2;
    struct Visitor3<P0, P1, P2, P3> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor3<P0, P1, P2, P3>
    {
        type Value = (Common, (P0, P1, P2, P3));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 3usize, &self.meta.sequence)?;
            let hash = seq_field::<A, Box<str>>(&mut seq, 4usize, &self.meta.sequence)?;
            Ok((
                Common {
                    hash,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    4usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let hash = common.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
            Ok((
                Common {
                    hash,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }
    }
    pub(crate) fn read_3<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor3 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_3 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3,)) =
            $crate::deserialization::encrypted_passport_element::read_3:: < D, $P0, $P1,
            $P2, $P3 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)), "hash"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, sequence :
            concat!("struct ", stringify!($name), " with 5 elements"), },) ?; Ok(Self {
            hash : common.hash, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, }) } }
        };
    }
    pub(crate) use deserialize_3;
}
pub(crate) mod external_reply_info {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) chat: Option<Box<crate::types::Chat>>,
        pub(crate) has_media_spoiler: Option<bool>,
        pub(crate) link_preview_options: Option<crate::types::LinkPreviewOptions>,
        pub(crate) message_id: Option<i64>,
        pub(crate) origin: crate::types::MessageOrigin,
    }
    #[derive(Default)]
    struct Slots {
        chat: Option<Option<Box<crate::types::Chat>>>,
        has_media_spoiler: Option<Option<bool>>,
        link_preview_options: Option<Option<crate::types::LinkPreviewOptions>>,
        message_id: Option<Option<i64>>,
        origin: Option<crate::types::MessageOrigin>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 5usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "chat" => Some(0usize),
                "has_media_spoiler" => Some(1usize),
                "link_preview_options" => Some(2usize),
                "message_id" => Some(3usize),
                "origin" => Some(4usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"chat" => Some(0usize),
                b"has_media_spoiler" => Some(1usize),
                b"link_preview_options" => Some(2usize),
                b"message_id" => Some(3usize),
                b"origin" => Some(4usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.chat, map, "chat"),
                1usize => map_field(&mut self.has_media_spoiler, map, "has_media_spoiler"),
                2usize => map_field(&mut self.link_preview_options, map, "link_preview_options"),
                3usize => map_field(&mut self.message_id, map, "message_id"),
                4usize => map_field(&mut self.origin, map, "origin"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let origin =
                seq_field::<A, crate::types::MessageOrigin>(&mut seq, 0usize, &self.meta.sequence)?;
            let chat = seq_field::<A, Option<Box<crate::types::Chat>>>(
                &mut seq,
                1usize,
                &self.meta.sequence,
            )?;
            let message_id = seq_field::<A, Option<i64>>(&mut seq, 2usize, &self.meta.sequence)?;
            let link_preview_options = seq_field::<A, Option<crate::types::LinkPreviewOptions>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let has_media_spoiler =
                seq_field::<A, Option<bool>>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 5usize, &self.meta.sequence)?;
            Ok((
                Common {
                    chat,
                    has_media_spoiler,
                    link_preview_options,
                    message_id,
                    origin,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 5usize => common.read(index, &mut map)?,
                    5usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let origin = common
                .origin
                .ok_or_else(|| A::Error::missing_field("origin"))?;
            let chat = common.chat.unwrap_or(None);
            let message_id = common.message_id.unwrap_or(None);
            let link_preview_options = common.link_preview_options.unwrap_or(None);
            let has_media_spoiler = common.has_media_spoiler.unwrap_or(None);
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    chat,
                    has_media_spoiler,
                    link_preview_options,
                    message_id,
                    origin,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::external_reply_info::read_0:: < D, $P0
            > (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["origin", "chat", "message_id", "link_preview_options",
            "has_media_spoiler",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 6 elements"), },) ?;
            Ok(Self { chat : common.chat, has_media_spoiler : common.has_media_spoiler,
            link_preview_options : common.link_preview_options, message_id : common
            .message_id, origin : common.origin, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1 {
        meta: Meta,
        marker: PhantomData<()>,
    }
    impl<'de> Visitor<'de> for Visitor1 {
        type Value = (Common, ());

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let origin =
                seq_field::<A, crate::types::MessageOrigin>(&mut seq, 0usize, &self.meta.sequence)?;
            let chat = seq_field::<A, Option<Box<crate::types::Chat>>>(
                &mut seq,
                1usize,
                &self.meta.sequence,
            )?;
            let message_id = seq_field::<A, Option<i64>>(&mut seq, 2usize, &self.meta.sequence)?;
            let link_preview_options = seq_field::<A, Option<crate::types::LinkPreviewOptions>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let has_media_spoiler =
                seq_field::<A, Option<bool>>(&mut seq, 4usize, &self.meta.sequence)?;
            Ok((
                Common {
                    chat,
                    has_media_spoiler,
                    link_preview_options,
                    message_id,
                    origin,
                },
                (),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 5usize => common.read(index, &mut map)?,
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let origin = common
                .origin
                .ok_or_else(|| A::Error::missing_field("origin"))?;
            let chat = common.chat.unwrap_or(None);
            let message_id = common.message_id.unwrap_or(None);
            let link_preview_options = common.link_preview_options.unwrap_or(None);
            let has_media_spoiler = common.has_media_spoiler.unwrap_or(None);
            Ok((
                Common {
                    chat,
                    has_media_spoiler,
                    link_preview_options,
                    message_id,
                    origin,
                },
                (),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, ()), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident,) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common, ()) =
            $crate::deserialization::external_reply_info::read_1:: < D, > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["origin", "chat", "message_id", "link_preview_options", "has_media_spoiler"]
            }, specific : const { & [] }, sequence : concat!("struct ",
            stringify!($name), " with 5 elements"), },) ?; Ok(Self { chat : common.chat,
            has_media_spoiler : common.has_media_spoiler, link_preview_options : common
            .link_preview_options, message_id : common.message_id, origin : common
            .origin, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod giveaway {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) chats: Box<[crate::types::Chat]>,
        pub(crate) country_codes: Option<Box<[Box<str>]>>,
        pub(crate) has_public_winners: Option<bool>,
        pub(crate) only_new_members: Option<bool>,
        pub(crate) prize_description: Option<Box<str>>,
        pub(crate) winner_count: i64,
        pub(crate) winners_selection_date: i64,
    }
    #[derive(Default)]
    struct Slots {
        chats: Option<Box<[crate::types::Chat]>>,
        country_codes: Option<Option<Box<[Box<str>]>>>,
        has_public_winners: Option<Option<bool>>,
        only_new_members: Option<Option<bool>>,
        prize_description: Option<Option<Box<str>>>,
        winner_count: Option<i64>,
        winners_selection_date: Option<i64>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 7usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "chats" => Some(0usize),
                "country_codes" => Some(1usize),
                "has_public_winners" => Some(2usize),
                "only_new_members" => Some(3usize),
                "prize_description" => Some(4usize),
                "winner_count" => Some(5usize),
                "winners_selection_date" => Some(6usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"chats" => Some(0usize),
                b"country_codes" => Some(1usize),
                b"has_public_winners" => Some(2usize),
                b"only_new_members" => Some(3usize),
                b"prize_description" => Some(4usize),
                b"winner_count" => Some(5usize),
                b"winners_selection_date" => Some(6usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.chats, map, "chats"),
                1usize => map_field(&mut self.country_codes, map, "country_codes"),
                2usize => map_field(&mut self.has_public_winners, map, "has_public_winners"),
                3usize => map_field(&mut self.only_new_members, map, "only_new_members"),
                4usize => map_field(&mut self.prize_description, map, "prize_description"),
                5usize => map_field(&mut self.winner_count, map, "winner_count"),
                6usize => map_field(
                    &mut self.winners_selection_date,
                    map,
                    "winners_selection_date",
                ),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let chats =
                seq_field::<A, Box<[crate::types::Chat]>>(&mut seq, 0usize, &self.meta.sequence)?;
            let winners_selection_date =
                seq_field::<A, i64>(&mut seq, 1usize, &self.meta.sequence)?;
            let winner_count = seq_field::<A, i64>(&mut seq, 2usize, &self.meta.sequence)?;
            let only_new_members =
                seq_field::<A, Option<bool>>(&mut seq, 3usize, &self.meta.sequence)?;
            let has_public_winners =
                seq_field::<A, Option<bool>>(&mut seq, 4usize, &self.meta.sequence)?;
            let prize_description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 5usize, &self.meta.sequence)?;
            let country_codes =
                seq_field::<A, Option<Box<[Box<str>]>>>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 7usize, &self.meta.sequence)?;
            Ok((
                Common {
                    chats,
                    country_codes,
                    has_public_winners,
                    only_new_members,
                    prize_description,
                    winner_count,
                    winners_selection_date,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 7usize => common.read(index, &mut map)?,
                    7usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let chats = common
                .chats
                .ok_or_else(|| A::Error::missing_field("chats"))?;
            let winners_selection_date = common
                .winners_selection_date
                .ok_or_else(|| A::Error::missing_field("winners_selection_date"))?;
            let winner_count = common
                .winner_count
                .ok_or_else(|| A::Error::missing_field("winner_count"))?;
            let only_new_members = common.only_new_members.unwrap_or(None);
            let has_public_winners = common.has_public_winners.unwrap_or(None);
            let prize_description = common.prize_description.unwrap_or(None);
            let country_codes = common.country_codes.unwrap_or(None);
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    chats,
                    country_codes,
                    has_public_winners,
                    only_new_members,
                    prize_description,
                    winner_count,
                    winners_selection_date,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::giveaway::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["chats", "winners_selection_date", "winner_count",
            "only_new_members", "has_public_winners", "prize_description",
            "country_codes", $crate::deserialization::field_name(stringify!($field_0))]
            }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 8 elements"), },) ?; Ok(Self {
            chats : common.chats, country_codes : common.country_codes,
            has_public_winners : common.has_public_winners, only_new_members : common
            .only_new_members, prize_description : common.prize_description, winner_count
            : common.winner_count, winners_selection_date : common
            .winners_selection_date, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
}
pub(crate) mod giveaway_winners {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
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
    struct Slots {
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
    impl CommonFields for Slots {
        const COUNT: usize = 10usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "additional_chat_count" => Some(0usize),
                "chat" => Some(1usize),
                "giveaway_message_id" => Some(2usize),
                "only_new_members" => Some(3usize),
                "prize_description" => Some(4usize),
                "unclaimed_prize_count" => Some(5usize),
                "was_refunded" => Some(6usize),
                "winner_count" => Some(7usize),
                "winners" => Some(8usize),
                "winners_selection_date" => Some(9usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"additional_chat_count" => Some(0usize),
                b"chat" => Some(1usize),
                b"giveaway_message_id" => Some(2usize),
                b"only_new_members" => Some(3usize),
                b"prize_description" => Some(4usize),
                b"unclaimed_prize_count" => Some(5usize),
                b"was_refunded" => Some(6usize),
                b"winner_count" => Some(7usize),
                b"winners" => Some(8usize),
                b"winners_selection_date" => Some(9usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(
                    &mut self.additional_chat_count,
                    map,
                    "additional_chat_count",
                ),
                1usize => map_field(&mut self.chat, map, "chat"),
                2usize => map_field(&mut self.giveaway_message_id, map, "giveaway_message_id"),
                3usize => map_field(&mut self.only_new_members, map, "only_new_members"),
                4usize => map_field(&mut self.prize_description, map, "prize_description"),
                5usize => map_field(
                    &mut self.unclaimed_prize_count,
                    map,
                    "unclaimed_prize_count",
                ),
                6usize => map_field(&mut self.was_refunded, map, "was_refunded"),
                7usize => map_field(&mut self.winner_count, map, "winner_count"),
                8usize => map_field(&mut self.winners, map, "winners"),
                9usize => map_field(
                    &mut self.winners_selection_date,
                    map,
                    "winners_selection_date",
                ),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let chat =
                seq_field::<A, Box<crate::types::Chat>>(&mut seq, 0usize, &self.meta.sequence)?;
            let giveaway_message_id = seq_field::<A, i64>(&mut seq, 1usize, &self.meta.sequence)?;
            let winners_selection_date =
                seq_field::<A, i64>(&mut seq, 2usize, &self.meta.sequence)?;
            let winner_count = seq_field::<A, i64>(&mut seq, 3usize, &self.meta.sequence)?;
            let winners =
                seq_field::<A, Box<[crate::types::User]>>(&mut seq, 4usize, &self.meta.sequence)?;
            let additional_chat_count =
                seq_field::<A, Option<i64>>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 6usize, &self.meta.sequence)?;
            let unclaimed_prize_count =
                seq_field::<A, Option<i64>>(&mut seq, 7usize, &self.meta.sequence)?;
            let only_new_members =
                seq_field::<A, Option<bool>>(&mut seq, 8usize, &self.meta.sequence)?;
            let was_refunded = seq_field::<A, Option<bool>>(&mut seq, 9usize, &self.meta.sequence)?;
            let prize_description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 10usize, &self.meta.sequence)?;
            Ok((
                Common {
                    additional_chat_count,
                    chat,
                    giveaway_message_id,
                    only_new_members,
                    prize_description,
                    unclaimed_prize_count,
                    was_refunded,
                    winner_count,
                    winners,
                    winners_selection_date,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 10usize => common.read(index, &mut map)?,
                    10usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let chat = common.chat.ok_or_else(|| A::Error::missing_field("chat"))?;
            let giveaway_message_id = common
                .giveaway_message_id
                .ok_or_else(|| A::Error::missing_field("giveaway_message_id"))?;
            let winners_selection_date = common
                .winners_selection_date
                .ok_or_else(|| A::Error::missing_field("winners_selection_date"))?;
            let winner_count = common
                .winner_count
                .ok_or_else(|| A::Error::missing_field("winner_count"))?;
            let winners = common
                .winners
                .ok_or_else(|| A::Error::missing_field("winners"))?;
            let additional_chat_count = common.additional_chat_count.unwrap_or(None);
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let unclaimed_prize_count = common.unclaimed_prize_count.unwrap_or(None);
            let only_new_members = common.only_new_members.unwrap_or(None);
            let was_refunded = common.was_refunded.unwrap_or(None);
            let prize_description = common.prize_description.unwrap_or(None);
            Ok((
                Common {
                    additional_chat_count,
                    chat,
                    giveaway_message_id,
                    only_new_members,
                    prize_description,
                    unclaimed_prize_count,
                    was_refunded,
                    winner_count,
                    winners,
                    winners_selection_date,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::giveaway_winners::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["chat", "giveaway_message_id", "winners_selection_date",
            "winner_count", "winners", "additional_chat_count",
            $crate::deserialization::field_name(stringify!($field_0)),
            "unclaimed_prize_count", "only_new_members", "was_refunded",
            "prize_description"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 11 elements"), },) ?; Ok(Self {
            additional_chat_count : common.additional_chat_count, chat : common.chat,
            giveaway_message_id : common.giveaway_message_id, only_new_members : common
            .only_new_members, prize_description : common.prize_description,
            unclaimed_prize_count : common.unclaimed_prize_count, was_refunded : common
            .was_refunded, winner_count : common.winner_count, winners : common.winners,
            winners_selection_date : common.winners_selection_date, $field_0 : value_0,
            }) } }
        };
    }
    pub(crate) use deserialize_0;
}
pub(crate) mod inline_query_result {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) id: Box<str>,
        pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    }
    #[derive(Default)]
    struct Slots {
        id: Option<Box<str>>,
        reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 2usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "id" => Some(0usize),
                "reply_markup" => Some(1usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"id" => Some(0usize),
                b"reply_markup" => Some(1usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.id, map, "id"),
                1usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1, P2, P3, P4, P5, P6> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor0<P0, P1, P2, P3, P4, P5, P6>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let value_2 = seq_field::<A, P2>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 8usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    4usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    5usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    6usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    7usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    8usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let reply_markup = common.reply_markup.unwrap_or(None);
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6,
                ),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6,)) =
            $crate::deserialization::inline_query_result::read_0:: < D, $P0, $P1, $P2,
            $P3, $P4, $P5, $P6 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)), "reply_markup",
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6))] }, sequence :
            concat!("struct ", stringify!($name), " with 9 elements"), },) ?; Ok(Self {
            id : common.id, reply_markup : common.reply_markup, $field_0 : value_0,
            $field_1 : value_1, $field_2 : value_2, $field_3 : value_3, $field_4 :
            value_4, $field_5 : value_5, $field_6 : value_6, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de> + Default> Visitor<'de> for Visitor1<P0, P1> {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                2usize,
                &self.meta.sequence,
            )?;
            let value_1 = seq_field::<A, P1>(&mut seq, 3usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let reply_markup = common.reply_markup.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::inline_query_result::read_1::
            < D, $P0, $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "reply_markup",
            $crate::deserialization::field_name(stringify!($field_1))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 4 elements"), },) ?; Ok(Self {
            id : common.id, reply_markup : common.reply_markup, $field_0 : value_0,
            $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0, P1, P2, P3, P4, P5, P6, P7> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6, P7)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
            P7: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor2<P0, P1, P2, P3, P4, P5, P6, P7>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6, P7));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                5usize,
                &self.meta.sequence,
            )?;
            let value_4 = seq_field::<A, P4>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 9usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    4usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    5usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    6usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    7usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    8usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    9usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let reply_markup = common.reply_markup.unwrap_or(None);
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            let value_7 = value_7.unwrap_or_default();
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                ),
            ))
        }
    }
    pub(crate) fn read_2<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
        P7: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6, P7)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,)) =
            $crate::deserialization::inline_query_result::read_2:: < D, $P0, $P1, $P2,
            $P3, $P4, $P5, $P6, $P7 > (deserializer, $crate::deserialization::Meta { name
            : stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)), "reply_markup",
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7))] }, sequence :
            concat!("struct ", stringify!($name), " with 10 elements"), },) ?; Ok(Self {
            id : common.id, reply_markup : common.reply_markup, $field_0 : value_0,
            $field_1 : value_1, $field_2 : value_2, $field_3 : value_3, $field_4 :
            value_4, $field_5 : value_5, $field_6 : value_6, $field_7 : value_7, }) } }
        };
    }
    pub(crate) use deserialize_2;
    struct Visitor3<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor3<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                2usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let reply_markup = common.reply_markup.unwrap_or(None);
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_3<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor3 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_3 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::inline_query_result::read_3:: < D, $P0
            > (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "reply_markup"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 3 elements"), },) ?; Ok(Self {
            id : common.id, reply_markup : common.reply_markup, $field_0 : value_0, }) }
            }
        };
    }
    pub(crate) use deserialize_3;
    struct Visitor4<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de>,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
            P7: Deserialize<'de> + Default,
            P8: Deserialize<'de> + Default,
            P9: Deserialize<'de> + Default,
            P10: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor4<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 7usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            let value_7 = seq_field::<A, P7>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_8 = seq_field::<A, P8>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_9 = seq_field::<A, P9>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_10 = seq_field::<A, P10>(&mut seq, 12usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            let mut value_9 = None;
            let mut value_10 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    4usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    5usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    6usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    7usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    8usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    9usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    10usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    11usize => {
                        map_field(&mut value_9, &mut map, self.meta.specific[9usize])?;
                    }
                    12usize => {
                        map_field(&mut value_10, &mut map, self.meta.specific[10usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 =
                value_2.ok_or_else(|| A::Error::missing_field(self.meta.specific[2usize]))?;
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            let reply_markup = common.reply_markup.unwrap_or(None);
            let value_7 = value_7.unwrap_or_default();
            let value_8 = value_8.unwrap_or_default();
            let value_9 = value_9.unwrap_or_default();
            let value_10 = value_10.unwrap_or_default();
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10,
                ),
            ))
        }
    }
    pub(crate) fn read_4<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de>,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
        P7: Deserialize<'de> + Default,
        P8: Deserialize<'de> + Default,
        P9: Deserialize<'de> + Default,
        P10: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor4 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_4 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty, $field_9:ident : $P9:ty, $field_10:ident : $P10:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8, value_9, value_10,)) =
            $crate::deserialization::inline_query_result::read_4:: < D, $P0, $P1, $P2,
            $P3, $P4, $P5, $P6, $P7, $P8, $P9, $P10 > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["id", $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)), "reply_markup",
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10))] }, sequence :
            concat!("struct ", stringify!($name), " with 13 elements"), },) ?; Ok(Self {
            id : common.id, reply_markup : common.reply_markup, $field_0 : value_0,
            $field_1 : value_1, $field_2 : value_2, $field_3 : value_3, $field_4 :
            value_4, $field_5 : value_5, $field_6 : value_6, $field_7 : value_7, $field_8
            : value_8, $field_9 : value_9, $field_10 : value_10, }) } }
        };
    }
    pub(crate) use deserialize_4;
    struct Visitor5<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de>,
            P3: Deserialize<'de>,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
            P7: Deserialize<'de> + Default,
            P8: Deserialize<'de> + Default,
            P9: Deserialize<'de> + Default,
            P10: Deserialize<'de> + Default,
            P11: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor5<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 8usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                9usize,
                &self.meta.sequence,
            )?;
            let value_8 = seq_field::<A, P8>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_9 = seq_field::<A, P9>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_10 = seq_field::<A, P10>(&mut seq, 12usize, &self.meta.sequence)?;
            let value_11 = seq_field::<A, P11>(&mut seq, 13usize, &self.meta.sequence)?;
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            let mut value_9 = None;
            let mut value_10 = None;
            let mut value_11 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    4usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    5usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    6usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    7usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    8usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    9usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    10usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    11usize => {
                        map_field(&mut value_9, &mut map, self.meta.specific[9usize])?;
                    }
                    12usize => {
                        map_field(&mut value_10, &mut map, self.meta.specific[10usize])?;
                    }
                    13usize => {
                        map_field(&mut value_11, &mut map, self.meta.specific[11usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 =
                value_2.ok_or_else(|| A::Error::missing_field(self.meta.specific[2usize]))?;
            let value_3 =
                value_3.ok_or_else(|| A::Error::missing_field(self.meta.specific[3usize]))?;
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            let value_7 = value_7.unwrap_or_default();
            let reply_markup = common.reply_markup.unwrap_or(None);
            let value_8 = value_8.unwrap_or_default();
            let value_9 = value_9.unwrap_or_default();
            let value_10 = value_10.unwrap_or_default();
            let value_11 = value_11.unwrap_or_default();
            Ok((
                Common {
                    id,
                    reply_markup,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9, value_10, value_11,
                ),
            ))
        }
    }
    pub(crate) fn read_5<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de>,
        P3: Deserialize<'de>,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
        P7: Deserialize<'de> + Default,
        P8: Deserialize<'de> + Default,
        P9: Deserialize<'de> + Default,
        P10: Deserialize<'de> + Default,
        P11: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor5 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_5 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty, $field_9:ident : $P9:ty, $field_10:ident : $P10:ty, $field_11:ident :
            $P11:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8, value_9, value_10, value_11,)) =
            $crate::deserialization::inline_query_result::read_5:: < D, $P0, $P1, $P2,
            $P3, $P4, $P5, $P6, $P7, $P8, $P9, $P10, $P11 > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["id", $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)), "reply_markup",
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9)),
            $crate::deserialization::field_name(stringify!($field_10)),
            $crate::deserialization::field_name(stringify!($field_11))] }, sequence :
            concat!("struct ", stringify!($name), " with 14 elements"), },) ?; Ok(Self {
            id : common.id, reply_markup : common.reply_markup, $field_0 : value_0,
            $field_1 : value_1, $field_2 : value_2, $field_3 : value_3, $field_4 :
            value_4, $field_5 : value_5, $field_6 : value_6, $field_7 : value_7, $field_8
            : value_8, $field_9 : value_9, $field_10 : value_10, $field_11 : value_11, })
            } }
        };
    }
    pub(crate) use deserialize_5;
}
pub(crate) mod inline_query_result_audio_kind {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) caption: Option<Box<str>>,
        pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
        pub(crate) id: Box<str>,
        pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
        pub(crate) parse_mode: Option<Box<str>>,
        pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    }
    #[derive(Default)]
    struct Slots {
        caption: Option<Option<Box<str>>>,
        caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
        id: Option<Box<str>>,
        input_message_content: Option<Option<crate::types::InputMessageContent>>,
        parse_mode: Option<Option<Box<str>>>,
        reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 6usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "caption" => Some(0usize),
                "caption_entities" => Some(1usize),
                "id" => Some(2usize),
                "input_message_content" => Some(3usize),
                "parse_mode" => Some(4usize),
                "reply_markup" => Some(5usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"caption" => Some(0usize),
                b"caption_entities" => Some(1usize),
                b"id" => Some(2usize),
                b"input_message_content" => Some(3usize),
                b"parse_mode" => Some(4usize),
                b"reply_markup" => Some(5usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.caption, map, "caption"),
                1usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                2usize => map_field(&mut self.id, map, "id"),
                3usize => map_field(
                    &mut self.input_message_content,
                    map,
                    "input_message_content",
                ),
                4usize => map_field(&mut self.parse_mode, map, "parse_mode"),
                5usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1, P2, P3> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor0<P0, P1, P2, P3>
    {
        type Value = (Common, (P0, P1, P2, P3));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                5usize,
                &self.meta.sequence,
            )?;
            let value_2 = seq_field::<A, P2>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 7usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                9usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 6usize => common.read(index, &mut map)?,
                    6usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    7usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    8usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    9usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3,)) =
            $crate::deserialization::inline_query_result_audio_kind::read_0:: < D, $P0,
            $P1, $P2, $P3 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)), "caption",
            "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)), "reply_markup",
            "input_message_content"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, sequence :
            concat!("struct ", stringify!($name), " with 10 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, id :
            common.id, input_message_content : common.input_message_content, parse_mode :
            common.parse_mode, reply_markup : common.reply_markup, $field_0 : value_0,
            $field_1 : value_1, $field_2 : value_2, $field_3 : value_3, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor1<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                4usize,
                &self.meta.sequence,
            )?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                5usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 6usize => common.read(index, &mut map)?,
                    6usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) =
            $crate::deserialization::inline_query_result_audio_kind::read_1:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "caption",
            "parse_mode", "caption_entities", "reply_markup", "input_message_content"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 7 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, id :
            common.id, input_message_content : common.input_message_content, parse_mode :
            common.parse_mode, reply_markup : common.reply_markup, $field_0 : value_0, })
            } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod inline_query_result_document_kind {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) caption: Option<Box<str>>,
        pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
        pub(crate) description: Option<Box<str>>,
        pub(crate) id: Box<str>,
        pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
        pub(crate) parse_mode: Option<Box<str>>,
        pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
        pub(crate) title: Box<str>,
    }
    #[derive(Default)]
    struct Slots {
        caption: Option<Option<Box<str>>>,
        caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
        description: Option<Option<Box<str>>>,
        id: Option<Box<str>>,
        input_message_content: Option<Option<crate::types::InputMessageContent>>,
        parse_mode: Option<Option<Box<str>>>,
        reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
        title: Option<Box<str>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 8usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "caption" => Some(0usize),
                "caption_entities" => Some(1usize),
                "description" => Some(2usize),
                "id" => Some(3usize),
                "input_message_content" => Some(4usize),
                "parse_mode" => Some(5usize),
                "reply_markup" => Some(6usize),
                "title" => Some(7usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"caption" => Some(0usize),
                b"caption_entities" => Some(1usize),
                b"description" => Some(2usize),
                b"id" => Some(3usize),
                b"input_message_content" => Some(4usize),
                b"parse_mode" => Some(5usize),
                b"reply_markup" => Some(6usize),
                b"title" => Some(7usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.caption, map, "caption"),
                1usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                2usize => map_field(&mut self.description, map, "description"),
                3usize => map_field(&mut self.id, map, "id"),
                4usize => map_field(
                    &mut self.input_message_content,
                    map,
                    "input_message_content",
                ),
                5usize => map_field(&mut self.parse_mode, map, "parse_mode"),
                6usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                7usize => map_field(&mut self.title, map, "title"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let title = seq_field::<A, Box<str>>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 2usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 5usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                7usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 8usize => common.read(index, &mut map)?,
                    8usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let title = common
                .title
                .ok_or_else(|| A::Error::missing_field("title"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let description = common.description.unwrap_or(None);
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) =
            $crate::deserialization::inline_query_result_document_kind::read_0:: < D, $P0
            > (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id", "title",
            $crate::deserialization::field_name(stringify!($field_0)), "description",
            "caption", "parse_mode", "caption_entities", "reply_markup",
            "input_message_content"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 9 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities,
            description : common.description, id : common.id, input_message_content :
            common.input_message_content, parse_mode : common.parse_mode, reply_markup :
            common.reply_markup, title : common.title, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1, P2, P3, P4> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor1<P0, P1, P2, P3, P4>
    {
        type Value = (Common, (P0, P1, P2, P3, P4));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let title = seq_field::<A, Box<str>>(&mut seq, 1usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                4usize,
                &self.meta.sequence,
            )?;
            let value_0 = seq_field::<A, P0>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 6usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 7usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                9usize,
                &self.meta.sequence,
            )?;
            let value_2 = seq_field::<A, P2>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 12usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0, value_1, value_2, value_3, value_4),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 8usize => common.read(index, &mut map)?,
                    8usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    9usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    10usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    11usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    12usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let title = common
                .title
                .ok_or_else(|| A::Error::missing_field("title"))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let description = common.description.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0, value_1, value_2, value_3, value_4),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4,)) =
            $crate::deserialization::inline_query_result_document_kind::read_1:: < D,
            $P0, $P1, $P2, $P3, $P4 > (deserializer, $crate::deserialization::Meta { name
            : stringify!($name), fields : const { & ["id", "title", "caption",
            "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)), "description",
            "reply_markup", "input_message_content",
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4))] }, sequence :
            concat!("struct ", stringify!($name), " with 13 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities,
            description : common.description, id : common.id, input_message_content :
            common.input_message_content, parse_mode : common.parse_mode, reply_markup :
            common.reply_markup, title : common.title, $field_0 : value_0, $field_1 :
            value_1, $field_2 : value_2, $field_3 : value_3, $field_4 : value_4, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod inline_query_result_gif_kind {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) caption: Option<Box<str>>,
        pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
        pub(crate) id: Box<str>,
        pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
        pub(crate) parse_mode: Option<Box<str>>,
        pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
        pub(crate) show_caption_above_media: Option<bool>,
        pub(crate) title: Option<Box<str>>,
    }
    #[derive(Default)]
    struct Slots {
        caption: Option<Option<Box<str>>>,
        caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
        id: Option<Box<str>>,
        input_message_content: Option<Option<crate::types::InputMessageContent>>,
        parse_mode: Option<Option<Box<str>>>,
        reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
        show_caption_above_media: Option<Option<bool>>,
        title: Option<Option<Box<str>>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 8usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "caption" => Some(0usize),
                "caption_entities" => Some(1usize),
                "id" => Some(2usize),
                "input_message_content" => Some(3usize),
                "parse_mode" => Some(4usize),
                "reply_markup" => Some(5usize),
                "show_caption_above_media" => Some(6usize),
                "title" => Some(7usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"caption" => Some(0usize),
                b"caption_entities" => Some(1usize),
                b"id" => Some(2usize),
                b"input_message_content" => Some(3usize),
                b"parse_mode" => Some(4usize),
                b"reply_markup" => Some(5usize),
                b"show_caption_above_media" => Some(6usize),
                b"title" => Some(7usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.caption, map, "caption"),
                1usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                2usize => map_field(&mut self.id, map, "id"),
                3usize => map_field(
                    &mut self.input_message_content,
                    map,
                    "input_message_content",
                ),
                4usize => map_field(&mut self.parse_mode, map, "parse_mode"),
                5usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                6usize => map_field(
                    &mut self.show_caption_above_media,
                    map,
                    "show_caption_above_media",
                ),
                7usize => map_field(&mut self.title, map, "title"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let title = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                5usize,
                &self.meta.sequence,
            )?;
            let show_caption_above_media =
                seq_field::<A, Option<bool>>(&mut seq, 6usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                7usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 8usize => common.read(index, &mut map)?,
                    8usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let title = common.title.unwrap_or(None);
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let show_caption_above_media = common.show_caption_above_media.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::inline_query_result_gif_kind::read_0::
            < D, $P0 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "title",
            "caption", "parse_mode", "caption_entities", "show_caption_above_media",
            "reply_markup", "input_message_content"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 9 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, id :
            common.id, input_message_content : common.input_message_content, parse_mode :
            common.parse_mode, reply_markup : common.reply_markup,
            show_caption_above_media : common.show_caption_above_media, title : common
            .title, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1, P2, P3, P4, P5> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de>,
            P5: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor1<P0, P1, P2, P3, P4, P5>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 6usize, &self.meta.sequence)?;
            let title = seq_field::<A, Option<Box<str>>>(&mut seq, 7usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 8usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 9usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                10usize,
                &self.meta.sequence,
            )?;
            let show_caption_above_media =
                seq_field::<A, Option<bool>>(&mut seq, 11usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                12usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                13usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0, value_1, value_2, value_3, value_4, value_5),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 8usize => common.read(index, &mut map)?,
                    8usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    9usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    10usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    11usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    12usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    13usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let value_4 =
                value_4.ok_or_else(|| A::Error::missing_field(self.meta.specific[4usize]))?;
            let value_5 = value_5.unwrap_or_default();
            let title = common.title.unwrap_or(None);
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let show_caption_above_media = common.show_caption_above_media.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0, value_1, value_2, value_3, value_4, value_5),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de>,
        P5: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5,)) =
            $crate::deserialization::inline_query_result_gif_kind::read_1:: < D, $P0,
            $P1, $P2, $P3, $P4, $P5 > (deserializer, $crate::deserialization::Meta { name
            : stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)), "title",
            "caption", "parse_mode", "caption_entities", "show_caption_above_media",
            "reply_markup", "input_message_content"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5))] }, sequence :
            concat!("struct ", stringify!($name), " with 14 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, id :
            common.id, input_message_content : common.input_message_content, parse_mode :
            common.parse_mode, reply_markup : common.reply_markup,
            show_caption_above_media : common.show_caption_above_media, title : common
            .title, $field_0 : value_0, $field_1 : value_1, $field_2 : value_2, $field_3
            : value_3, $field_4 : value_4, $field_5 : value_5, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod inline_query_result_photo_kind {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) caption: Option<Box<str>>,
        pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
        pub(crate) description: Option<Box<str>>,
        pub(crate) id: Box<str>,
        pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
        pub(crate) parse_mode: Option<Box<str>>,
        pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
        pub(crate) show_caption_above_media: Option<bool>,
        pub(crate) title: Option<Box<str>>,
    }
    #[derive(Default)]
    struct Slots {
        caption: Option<Option<Box<str>>>,
        caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
        description: Option<Option<Box<str>>>,
        id: Option<Box<str>>,
        input_message_content: Option<Option<crate::types::InputMessageContent>>,
        parse_mode: Option<Option<Box<str>>>,
        reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
        show_caption_above_media: Option<Option<bool>>,
        title: Option<Option<Box<str>>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 9usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "caption" => Some(0usize),
                "caption_entities" => Some(1usize),
                "description" => Some(2usize),
                "id" => Some(3usize),
                "input_message_content" => Some(4usize),
                "parse_mode" => Some(5usize),
                "reply_markup" => Some(6usize),
                "show_caption_above_media" => Some(7usize),
                "title" => Some(8usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"caption" => Some(0usize),
                b"caption_entities" => Some(1usize),
                b"description" => Some(2usize),
                b"id" => Some(3usize),
                b"input_message_content" => Some(4usize),
                b"parse_mode" => Some(5usize),
                b"reply_markup" => Some(6usize),
                b"show_caption_above_media" => Some(7usize),
                b"title" => Some(8usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.caption, map, "caption"),
                1usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                2usize => map_field(&mut self.description, map, "description"),
                3usize => map_field(&mut self.id, map, "id"),
                4usize => map_field(
                    &mut self.input_message_content,
                    map,
                    "input_message_content",
                ),
                5usize => map_field(&mut self.parse_mode, map, "parse_mode"),
                6usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                7usize => map_field(
                    &mut self.show_caption_above_media,
                    map,
                    "show_caption_above_media",
                ),
                8usize => map_field(&mut self.title, map, "title"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let title = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 5usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let show_caption_above_media =
                seq_field::<A, Option<bool>>(&mut seq, 7usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                9usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 9usize => common.read(index, &mut map)?,
                    9usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let title = common.title.unwrap_or(None);
            let description = common.description.unwrap_or(None);
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let show_caption_above_media = common.show_caption_above_media.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) =
            $crate::deserialization::inline_query_result_photo_kind::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "title",
            "description", "caption", "parse_mode", "caption_entities",
            "show_caption_above_media", "reply_markup", "input_message_content"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 10 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities,
            description : common.description, id : common.id, input_message_content :
            common.input_message_content, parse_mode : common.parse_mode, reply_markup :
            common.reply_markup, show_caption_above_media : common
            .show_caption_above_media, title : common.title, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1, P2, P3> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor1<P0, P1, P2, P3>
    {
        type Value = (Common, (P0, P1, P2, P3));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let title = seq_field::<A, Option<Box<str>>>(&mut seq, 5usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 6usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 7usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 8usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                9usize,
                &self.meta.sequence,
            )?;
            let show_caption_above_media =
                seq_field::<A, Option<bool>>(&mut seq, 10usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                11usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                12usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 9usize => common.read(index, &mut map)?,
                    9usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    10usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    11usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    12usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let title = common.title.unwrap_or(None);
            let description = common.description.unwrap_or(None);
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let show_caption_above_media = common.show_caption_above_media.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3,)) =
            $crate::deserialization::inline_query_result_photo_kind::read_1:: < D, $P0,
            $P1, $P2, $P3 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)), "title",
            "description", "caption", "parse_mode", "caption_entities",
            "show_caption_above_media", "reply_markup", "input_message_content"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, sequence :
            concat!("struct ", stringify!($name), " with 13 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities,
            description : common.description, id : common.id, input_message_content :
            common.input_message_content, parse_mode : common.parse_mode, reply_markup :
            common.reply_markup, show_caption_above_media : common
            .show_caption_above_media, title : common.title, $field_0 : value_0, $field_1
            : value_1, $field_2 : value_2, $field_3 : value_3, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod inline_query_result_video_kind {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) caption: Option<Box<str>>,
        pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
        pub(crate) description: Option<Box<str>>,
        pub(crate) id: Box<str>,
        pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
        pub(crate) parse_mode: Option<Box<str>>,
        pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
        pub(crate) show_caption_above_media: Option<bool>,
        pub(crate) title: Box<str>,
    }
    #[derive(Default)]
    struct Slots {
        caption: Option<Option<Box<str>>>,
        caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
        description: Option<Option<Box<str>>>,
        id: Option<Box<str>>,
        input_message_content: Option<Option<crate::types::InputMessageContent>>,
        parse_mode: Option<Option<Box<str>>>,
        reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
        show_caption_above_media: Option<Option<bool>>,
        title: Option<Box<str>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 9usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "caption" => Some(0usize),
                "caption_entities" => Some(1usize),
                "description" => Some(2usize),
                "id" => Some(3usize),
                "input_message_content" => Some(4usize),
                "parse_mode" => Some(5usize),
                "reply_markup" => Some(6usize),
                "show_caption_above_media" => Some(7usize),
                "title" => Some(8usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"caption" => Some(0usize),
                b"caption_entities" => Some(1usize),
                b"description" => Some(2usize),
                b"id" => Some(3usize),
                b"input_message_content" => Some(4usize),
                b"parse_mode" => Some(5usize),
                b"reply_markup" => Some(6usize),
                b"show_caption_above_media" => Some(7usize),
                b"title" => Some(8usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.caption, map, "caption"),
                1usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                2usize => map_field(&mut self.description, map, "description"),
                3usize => map_field(&mut self.id, map, "id"),
                4usize => map_field(
                    &mut self.input_message_content,
                    map,
                    "input_message_content",
                ),
                5usize => map_field(&mut self.parse_mode, map, "parse_mode"),
                6usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                7usize => map_field(
                    &mut self.show_caption_above_media,
                    map,
                    "show_caption_above_media",
                ),
                8usize => map_field(&mut self.title, map, "title"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let title = seq_field::<A, Box<str>>(&mut seq, 2usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 5usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let show_caption_above_media =
                seq_field::<A, Option<bool>>(&mut seq, 7usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                9usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 9usize => common.read(index, &mut map)?,
                    9usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let title = common
                .title
                .ok_or_else(|| A::Error::missing_field("title"))?;
            let description = common.description.unwrap_or(None);
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let show_caption_above_media = common.show_caption_above_media.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) =
            $crate::deserialization::inline_query_result_video_kind::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "title",
            "description", "caption", "parse_mode", "caption_entities",
            "show_caption_above_media", "reply_markup", "input_message_content"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 10 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities,
            description : common.description, id : common.id, input_message_content :
            common.input_message_content, parse_mode : common.parse_mode, reply_markup :
            common.reply_markup, show_caption_above_media : common
            .show_caption_above_media, title : common.title, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1, P2, P3, P4, P5> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de>,
            P2: Deserialize<'de>,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor1<P0, P1, P2, P3, P4, P5>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let title = seq_field::<A, Box<str>>(&mut seq, 4usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 5usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 6usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                7usize,
                &self.meta.sequence,
            )?;
            let show_caption_above_media =
                seq_field::<A, Option<bool>>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 11usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 12usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                13usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                14usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0, value_1, value_2, value_3, value_4, value_5),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 9usize => common.read(index, &mut map)?,
                    9usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    10usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    11usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    12usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    13usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    14usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 =
                value_2.ok_or_else(|| A::Error::missing_field(self.meta.specific[2usize]))?;
            let title = common
                .title
                .ok_or_else(|| A::Error::missing_field("title"))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let show_caption_above_media = common.show_caption_above_media.unwrap_or(None);
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let description = common.description.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    description,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    show_caption_above_media,
                    title,
                },
                (value_0, value_1, value_2, value_3, value_4, value_5),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de>,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5,)) =
            $crate::deserialization::inline_query_result_video_kind::read_1:: < D, $P0,
            $P1, $P2, $P3, $P4, $P5 > (deserializer, $crate::deserialization::Meta { name
            : stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)), "title",
            "caption", "parse_mode", "caption_entities", "show_caption_above_media",
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)), "description",
            "reply_markup", "input_message_content"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5))] }, sequence :
            concat!("struct ", stringify!($name), " with 15 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities,
            description : common.description, id : common.id, input_message_content :
            common.input_message_content, parse_mode : common.parse_mode, reply_markup :
            common.reply_markup, show_caption_above_media : common
            .show_caption_above_media, title : common.title, $field_0 : value_0, $field_1
            : value_1, $field_2 : value_2, $field_3 : value_3, $field_4 : value_4,
            $field_5 : value_5, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod inline_query_result_voice_kind {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) caption: Option<Box<str>>,
        pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
        pub(crate) id: Box<str>,
        pub(crate) input_message_content: Option<crate::types::InputMessageContent>,
        pub(crate) parse_mode: Option<Box<str>>,
        pub(crate) reply_markup: Option<crate::types::InlineKeyboardMarkup>,
        pub(crate) title: Box<str>,
    }
    #[derive(Default)]
    struct Slots {
        caption: Option<Option<Box<str>>>,
        caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
        id: Option<Box<str>>,
        input_message_content: Option<Option<crate::types::InputMessageContent>>,
        parse_mode: Option<Option<Box<str>>>,
        reply_markup: Option<Option<crate::types::InlineKeyboardMarkup>>,
        title: Option<Box<str>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 7usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "caption" => Some(0usize),
                "caption_entities" => Some(1usize),
                "id" => Some(2usize),
                "input_message_content" => Some(3usize),
                "parse_mode" => Some(4usize),
                "reply_markup" => Some(5usize),
                "title" => Some(6usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"caption" => Some(0usize),
                b"caption_entities" => Some(1usize),
                b"id" => Some(2usize),
                b"input_message_content" => Some(3usize),
                b"parse_mode" => Some(4usize),
                b"reply_markup" => Some(5usize),
                b"title" => Some(6usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.caption, map, "caption"),
                1usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                2usize => map_field(&mut self.id, map, "id"),
                3usize => map_field(
                    &mut self.input_message_content,
                    map,
                    "input_message_content",
                ),
                4usize => map_field(&mut self.parse_mode, map, "parse_mode"),
                5usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                6usize => map_field(&mut self.title, map, "title"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let title = seq_field::<A, Box<str>>(&mut seq, 2usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                5usize,
                &self.meta.sequence,
            )?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                7usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 7usize => common.read(index, &mut map)?,
                    7usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let title = common
                .title
                .ok_or_else(|| A::Error::missing_field("title"))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) =
            $crate::deserialization::inline_query_result_voice_kind::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "title",
            "caption", "parse_mode", "caption_entities", "reply_markup",
            "input_message_content"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 8 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, id :
            common.id, input_message_content : common.input_message_content, parse_mode :
            common.parse_mode, reply_markup : common.reply_markup, title : common.title,
            $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de> + Default> Visitor<'de> for Visitor1<P0, P1> {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let title = seq_field::<A, Box<str>>(&mut seq, 2usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                5usize,
                &self.meta.sequence,
            )?;
            let value_1 = seq_field::<A, P1>(&mut seq, 6usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                7usize,
                &self.meta.sequence,
            )?;
            let input_message_content = seq_field::<A, Option<crate::types::InputMessageContent>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 7usize => common.read(index, &mut map)?,
                    7usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    8usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let title = common
                .title
                .ok_or_else(|| A::Error::missing_field("title"))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            let reply_markup = common.reply_markup.unwrap_or(None);
            let input_message_content = common.input_message_content.unwrap_or(None);
            Ok((
                Common {
                    caption,
                    caption_entities,
                    id,
                    input_message_content,
                    parse_mode,
                    reply_markup,
                    title,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) =
            $crate::deserialization::inline_query_result_voice_kind::read_1:: < D, $P0,
            $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id",
            $crate::deserialization::field_name(stringify!($field_0)), "title",
            "caption", "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_1)), "reply_markup",
            "input_message_content"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 9 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, id :
            common.id, input_message_content : common.input_message_content, parse_mode :
            common.parse_mode, reply_markup : common.reply_markup, title : common.title,
            $field_0 : value_0, $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod input_media {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) caption: Option<Box<str>>,
        pub(crate) caption_entities: Option<Box<[crate::types::MessageEntity]>>,
        pub(crate) media: crate::types::InputFile,
        pub(crate) parse_mode: Option<Box<str>>,
    }
    #[derive(Default)]
    struct Slots {
        caption: Option<Option<Box<str>>>,
        caption_entities: Option<Option<Box<[crate::types::MessageEntity]>>>,
        media: Option<crate::types::InputFile>,
        parse_mode: Option<Option<Box<str>>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 4usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "caption" => Some(0usize),
                "caption_entities" => Some(1usize),
                "media" => Some(2usize),
                "parse_mode" => Some(3usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"caption" => Some(0usize),
                b"caption_entities" => Some(1usize),
                b"media" => Some(2usize),
                b"parse_mode" => Some(3usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.caption, map, "caption"),
                1usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                2usize => map_field(&mut self.media, map, "media"),
                3usize => map_field(&mut self.parse_mode, map, "parse_mode"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1, P2, P3, P4, P5> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor0<P0, P1, P2, P3, P4, P5>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                4usize,
                &self.meta.sequence,
            )?;
            let value_1 = seq_field::<A, P1>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 9usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1, value_2, value_3, value_4, value_5),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    6usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    7usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    8usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    9usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let value_0 = value_0.unwrap_or_default();
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1, value_2, value_3, value_4, value_5),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5,)) =
            $crate::deserialization::input_media::read_0:: < D, $P0, $P1, $P2, $P3, $P4,
            $P5 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["media",
            $crate::deserialization::field_name(stringify!($field_0)), "caption",
            "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5))] }, sequence :
            concat!("struct ", stringify!($name), " with 10 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, media :
            common.media, parse_mode : common.parse_mode, $field_0 : value_0, $field_1 :
            value_1, $field_2 : value_2, $field_3 : value_3, $field_4 : value_4, $field_5
            : value_5, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1, P2, P3> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor1<P0, P1, P2, P3>
    {
        type Value = (Common, (P0, P1, P2, P3));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                4usize,
                &self.meta.sequence,
            )?;
            let value_1 = seq_field::<A, P1>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 7usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    6usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    7usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let value_0 = value_0.unwrap_or_default();
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3,)) =
            $crate::deserialization::input_media::read_1:: < D, $P0, $P1, $P2, $P3 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["media",
            $crate::deserialization::field_name(stringify!($field_0)), "caption",
            "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, sequence :
            concat!("struct ", stringify!($name), " with 8 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, media :
            common.media, parse_mode : common.parse_mode, $field_0 : value_0, $field_1 :
            value_1, $field_2 : value_2, $field_3 : value_3, }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de> + Default, P1: Deserialize<'de> + Default> Visitor<'de>
        for Visitor2<P0, P1>
    {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                4usize,
                &self.meta.sequence,
            )?;
            let value_1 = seq_field::<A, P1>(&mut seq, 5usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let value_0 = value_0.unwrap_or_default();
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_2<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::input_media::read_2:: < D,
            $P0, $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["media",
            $crate::deserialization::field_name(stringify!($field_0)), "caption",
            "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_1))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 6 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, media :
            common.media, parse_mode : common.parse_mode, $field_0 : value_0, $field_1 :
            value_1, }) } }
        };
    }
    pub(crate) use deserialize_2;
    struct Visitor3<P0, P1, P2> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor3<P0, P1, P2>
    {
        type Value = (Common, (P0, P1, P2));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                4usize,
                &self.meta.sequence,
            )?;
            let value_1 = seq_field::<A, P1>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 6usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1, value_2),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    6usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1, value_2),
            ))
        }
    }
    pub(crate) fn read_3<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor3 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_3 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2,)) =
            $crate::deserialization::input_media::read_3:: < D, $P0, $P1, $P2 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["media",
            $crate::deserialization::field_name(stringify!($field_0)), "caption",
            "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, sequence :
            concat!("struct ", stringify!($name), " with 7 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, media :
            common.media, parse_mode : common.parse_mode, $field_0 : value_0, $field_1 :
            value_1, $field_2 : value_2, }) } }
        };
    }
    pub(crate) use deserialize_3;
    struct Visitor4<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de> + Default, P1: Deserialize<'de> + Default> Visitor<'de>
        for Visitor4<P0, P1>
    {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 1usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let value_0 = seq_field::<A, P0>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 5usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_4<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor4 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_4 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::input_media::read_4:: < D,
            $P0, $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["media", "caption", "parse_mode",
            "caption_entities",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 6 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, media :
            common.media, parse_mode : common.parse_mode, $field_0 : value_0, $field_1 :
            value_1, }) } }
        };
    }
    pub(crate) use deserialize_4;
    struct Visitor5<P0, P1, P2, P3, P4, P5, P6, P7, P8> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6, P7, P8)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
            P7: Deserialize<'de> + Default,
            P8: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor5<P0, P1, P2, P3, P4, P5, P6, P7, P8>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 4usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 5usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let value_3 = seq_field::<A, P3>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_8 = seq_field::<A, P8>(&mut seq, 12usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7, value_8,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    6usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    7usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    8usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    9usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    10usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    11usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    12usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            let value_7 = value_7.unwrap_or_default();
            let value_8 = value_8.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7, value_8,
                ),
            ))
        }
    }
    pub(crate) fn read_5<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
        P7: Deserialize<'de> + Default,
        P8: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor5 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_5 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8,)) = $crate::deserialization::input_media::read_5:: < D, $P0, $P1,
            $P2, $P3, $P4, $P5, $P6, $P7, $P8 > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["media", $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)), "caption",
            "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8))] }, sequence :
            concat!("struct ", stringify!($name), " with 13 elements"), },) ?; Ok(Self {
            caption : common.caption, caption_entities : common.caption_entities, media :
            common.media, parse_mode : common.parse_mode, $field_0 : value_0, $field_1 :
            value_1, $field_2 : value_2, $field_3 : value_3, $field_4 : value_4, $field_5
            : value_5, $field_6 : value_6, $field_7 : value_7, $field_8 : value_8, }) } }
        };
    }
    pub(crate) use deserialize_5;
    struct Visitor6<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de> + Default> Visitor<'de> for Visitor6<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 1usize, &self.meta.sequence)?;
            let parse_mode =
                seq_field::<A, Option<Box<str>>>(&mut seq, 2usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let value_0 = seq_field::<A, P0>(&mut seq, 4usize, &self.meta.sequence)?;
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let caption = common.caption.unwrap_or(None);
            let parse_mode = common.parse_mode.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let value_0 = value_0.unwrap_or_default();
            Ok((
                Common {
                    caption,
                    caption_entities,
                    media,
                    parse_mode,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_6<'de, D: Deserializer<'de>, P0: Deserialize<'de> + Default>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor6 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_6 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::input_media::read_6:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["media", "caption", "parse_mode", "caption_entities",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 5 elements"), },) ?;
            Ok(Self { caption : common.caption, caption_entities : common
            .caption_entities, media : common.media, parse_mode : common.parse_mode,
            $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_6;
}
pub(crate) mod input_paid_media {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) media: crate::types::InputFile,
    }
    #[derive(Default)]
    struct Slots {
        media: Option<crate::types::InputFile>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 1usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "media" => Some(0usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"media" => Some(0usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.media, map, "media"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            Ok((
                Common {
                    media,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    media,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::input_paid_media::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["media",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 2 elements"), },) ?;
            Ok(Self { media : common.media, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1 {
        meta: Meta,
        marker: PhantomData<()>,
    }
    impl<'de> Visitor<'de> for Visitor1 {
        type Value = (Common, ());

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            Ok((
                Common {
                    media,
                },
                (),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            Ok((
                Common {
                    media,
                },
                (),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, ()), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident,) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common, ()) =
            $crate::deserialization::input_paid_media::read_1:: < D, > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["media"] }, specific : const { & [] }, sequence : concat!("struct ",
            stringify!($name), " with 1 element"), },) ?; Ok(Self { media : common.media,
            }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0, P1, P2, P3, P4, P5, P6> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor2<P0, P1, P2, P3, P4, P5, P6>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let media =
                seq_field::<A, crate::types::InputFile>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 7usize, &self.meta.sequence)?;
            Ok((
                Common {
                    media,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    4usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    5usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    6usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    7usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let media = common
                .media
                .ok_or_else(|| A::Error::missing_field("media"))?;
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            Ok((
                Common {
                    media,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6,
                ),
            ))
        }
    }
    pub(crate) fn read_2<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6,)) =
            $crate::deserialization::input_paid_media::read_2:: < D, $P0, $P1, $P2, $P3,
            $P4, $P5, $P6 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["media",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6))] }, sequence :
            concat!("struct ", stringify!($name), " with 8 elements"), },) ?; Ok(Self {
            media : common.media, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, $field_3 : value_3, $field_4 : value_4, $field_5 : value_5, $field_6
            : value_6, }) } }
        };
    }
    pub(crate) use deserialize_2;
}
pub(crate) mod message {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
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
    struct Slots {
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
    impl CommonFields for Slots {
        const COUNT: usize = 42usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "author_signature" => Some(0usize),
                "business_connection_id" => Some(1usize),
                "caption" => Some(2usize),
                "caption_entities" => Some(3usize),
                "chat" => Some(4usize),
                "date" => Some(5usize),
                "direct_messages_topic" => Some(6usize),
                "edit_date" => Some(7usize),
                "effect_id" => Some(8usize),
                "entities" => Some(9usize),
                "ephemeral_message_id" => Some(10usize),
                "external_reply" => Some(11usize),
                "forward_origin" => Some(12usize),
                "from" => Some(13usize),
                "guest_bot_caller_chat" => Some(14usize),
                "guest_bot_caller_user" => Some(15usize),
                "guest_query_id" => Some(16usize),
                "has_media_spoiler" => Some(17usize),
                "has_protected_content" => Some(18usize),
                "is_automatic_forward" => Some(19usize),
                "is_from_offline" => Some(20usize),
                "is_paid_post" => Some(21usize),
                "is_topic_message" => Some(22usize),
                "link_preview_options" => Some(23usize),
                "media_group_id" => Some(24usize),
                "message_id" => Some(25usize),
                "message_thread_id" => Some(26usize),
                "paid_star_count" => Some(27usize),
                "quote" => Some(28usize),
                "receiver_user" => Some(29usize),
                "reply_markup" => Some(30usize),
                "reply_to_checklist_task_id" => Some(31usize),
                "reply_to_message" => Some(32usize),
                "reply_to_poll_option_id" => Some(33usize),
                "reply_to_story" => Some(34usize),
                "sender_boost_count" => Some(35usize),
                "sender_business_bot" => Some(36usize),
                "sender_chat" => Some(37usize),
                "sender_tag" => Some(38usize),
                "show_caption_above_media" => Some(39usize),
                "suggested_post_info" => Some(40usize),
                "via_bot" => Some(41usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"author_signature" => Some(0usize),
                b"business_connection_id" => Some(1usize),
                b"caption" => Some(2usize),
                b"caption_entities" => Some(3usize),
                b"chat" => Some(4usize),
                b"date" => Some(5usize),
                b"direct_messages_topic" => Some(6usize),
                b"edit_date" => Some(7usize),
                b"effect_id" => Some(8usize),
                b"entities" => Some(9usize),
                b"ephemeral_message_id" => Some(10usize),
                b"external_reply" => Some(11usize),
                b"forward_origin" => Some(12usize),
                b"from" => Some(13usize),
                b"guest_bot_caller_chat" => Some(14usize),
                b"guest_bot_caller_user" => Some(15usize),
                b"guest_query_id" => Some(16usize),
                b"has_media_spoiler" => Some(17usize),
                b"has_protected_content" => Some(18usize),
                b"is_automatic_forward" => Some(19usize),
                b"is_from_offline" => Some(20usize),
                b"is_paid_post" => Some(21usize),
                b"is_topic_message" => Some(22usize),
                b"link_preview_options" => Some(23usize),
                b"media_group_id" => Some(24usize),
                b"message_id" => Some(25usize),
                b"message_thread_id" => Some(26usize),
                b"paid_star_count" => Some(27usize),
                b"quote" => Some(28usize),
                b"receiver_user" => Some(29usize),
                b"reply_markup" => Some(30usize),
                b"reply_to_checklist_task_id" => Some(31usize),
                b"reply_to_message" => Some(32usize),
                b"reply_to_poll_option_id" => Some(33usize),
                b"reply_to_story" => Some(34usize),
                b"sender_boost_count" => Some(35usize),
                b"sender_business_bot" => Some(36usize),
                b"sender_chat" => Some(37usize),
                b"sender_tag" => Some(38usize),
                b"show_caption_above_media" => Some(39usize),
                b"suggested_post_info" => Some(40usize),
                b"via_bot" => Some(41usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.author_signature, map, "author_signature"),
                1usize => map_field(
                    &mut self.business_connection_id,
                    map,
                    "business_connection_id",
                ),
                2usize => map_field(&mut self.caption, map, "caption"),
                3usize => map_field(&mut self.caption_entities, map, "caption_entities"),
                4usize => map_field(&mut self.chat, map, "chat"),
                5usize => map_field(&mut self.date, map, "date"),
                6usize => map_field(
                    &mut self.direct_messages_topic,
                    map,
                    "direct_messages_topic",
                ),
                7usize => map_field(&mut self.edit_date, map, "edit_date"),
                8usize => map_field(&mut self.effect_id, map, "effect_id"),
                9usize => map_field(&mut self.entities, map, "entities"),
                10usize => map_field(&mut self.ephemeral_message_id, map, "ephemeral_message_id"),
                11usize => map_field(&mut self.external_reply, map, "external_reply"),
                12usize => map_field(&mut self.forward_origin, map, "forward_origin"),
                13usize => map_field(&mut self.from, map, "from"),
                14usize => map_field(
                    &mut self.guest_bot_caller_chat,
                    map,
                    "guest_bot_caller_chat",
                ),
                15usize => map_field(
                    &mut self.guest_bot_caller_user,
                    map,
                    "guest_bot_caller_user",
                ),
                16usize => map_field(&mut self.guest_query_id, map, "guest_query_id"),
                17usize => map_field(&mut self.has_media_spoiler, map, "has_media_spoiler"),
                18usize => map_field(
                    &mut self.has_protected_content,
                    map,
                    "has_protected_content",
                ),
                19usize => map_field(&mut self.is_automatic_forward, map, "is_automatic_forward"),
                20usize => map_field(&mut self.is_from_offline, map, "is_from_offline"),
                21usize => map_field(&mut self.is_paid_post, map, "is_paid_post"),
                22usize => map_field(&mut self.is_topic_message, map, "is_topic_message"),
                23usize => map_field(&mut self.link_preview_options, map, "link_preview_options"),
                24usize => map_field(&mut self.media_group_id, map, "media_group_id"),
                25usize => map_field(&mut self.message_id, map, "message_id"),
                26usize => map_field(&mut self.message_thread_id, map, "message_thread_id"),
                27usize => map_field(&mut self.paid_star_count, map, "paid_star_count"),
                28usize => map_field(&mut self.quote, map, "quote"),
                29usize => map_field(&mut self.receiver_user, map, "receiver_user"),
                30usize => map_field(&mut self.reply_markup, map, "reply_markup"),
                31usize => map_field(
                    &mut self.reply_to_checklist_task_id,
                    map,
                    "reply_to_checklist_task_id",
                ),
                32usize => map_field(&mut self.reply_to_message, map, "reply_to_message"),
                33usize => map_field(
                    &mut self.reply_to_poll_option_id,
                    map,
                    "reply_to_poll_option_id",
                ),
                34usize => map_field(&mut self.reply_to_story, map, "reply_to_story"),
                35usize => map_field(&mut self.sender_boost_count, map, "sender_boost_count"),
                36usize => map_field(&mut self.sender_business_bot, map, "sender_business_bot"),
                37usize => map_field(&mut self.sender_chat, map, "sender_chat"),
                38usize => map_field(&mut self.sender_tag, map, "sender_tag"),
                39usize => map_field(
                    &mut self.show_caption_above_media,
                    map,
                    "show_caption_above_media",
                ),
                40usize => map_field(&mut self.suggested_post_info, map, "suggested_post_info"),
                41usize => map_field(&mut self.via_bot, map, "via_bot"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let message_id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let message_thread_id =
                seq_field::<A, Option<i64>>(&mut seq, 1usize, &self.meta.sequence)?;
            let direct_messages_topic = seq_field::<A, Option<crate::types::DirectMessagesTopic>>(
                &mut seq,
                2usize,
                &self.meta.sequence,
            )?;
            let from = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let sender_chat = seq_field::<A, Option<Box<crate::types::Chat>>>(
                &mut seq,
                4usize,
                &self.meta.sequence,
            )?;
            let sender_boost_count =
                seq_field::<A, Option<i64>>(&mut seq, 5usize, &self.meta.sequence)?;
            let sender_business_bot = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let sender_tag =
                seq_field::<A, Option<Box<str>>>(&mut seq, 7usize, &self.meta.sequence)?;
            let receiver_user = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                8usize,
                &self.meta.sequence,
            )?;
            let ephemeral_message_id =
                seq_field::<A, Option<i64>>(&mut seq, 9usize, &self.meta.sequence)?;
            let date = seq_field::<A, i64>(&mut seq, 10usize, &self.meta.sequence)?;
            let guest_query_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 11usize, &self.meta.sequence)?;
            let business_connection_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 12usize, &self.meta.sequence)?;
            let chat =
                seq_field::<A, Box<crate::types::Chat>>(&mut seq, 13usize, &self.meta.sequence)?;
            let forward_origin = seq_field::<A, Option<crate::types::MessageOrigin>>(
                &mut seq,
                14usize,
                &self.meta.sequence,
            )?;
            let is_topic_message =
                seq_field::<A, Option<bool>>(&mut seq, 15usize, &self.meta.sequence)?;
            let is_automatic_forward =
                seq_field::<A, Option<bool>>(&mut seq, 16usize, &self.meta.sequence)?;
            let reply_to_message = seq_field::<A, Option<Box<crate::types::Message>>>(
                &mut seq,
                17usize,
                &self.meta.sequence,
            )?;
            let external_reply = seq_field::<A, Option<Box<crate::types::ExternalReplyInfo>>>(
                &mut seq,
                18usize,
                &self.meta.sequence,
            )?;
            let quote = seq_field::<A, Option<crate::types::TextQuote>>(
                &mut seq,
                19usize,
                &self.meta.sequence,
            )?;
            let reply_to_story = seq_field::<A, Option<crate::types::Story>>(
                &mut seq,
                20usize,
                &self.meta.sequence,
            )?;
            let reply_to_checklist_task_id =
                seq_field::<A, Option<i64>>(&mut seq, 21usize, &self.meta.sequence)?;
            let reply_to_poll_option_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 22usize, &self.meta.sequence)?;
            let via_bot = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                23usize,
                &self.meta.sequence,
            )?;
            let guest_bot_caller_user = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                24usize,
                &self.meta.sequence,
            )?;
            let guest_bot_caller_chat = seq_field::<A, Option<Box<crate::types::Chat>>>(
                &mut seq,
                25usize,
                &self.meta.sequence,
            )?;
            let edit_date = seq_field::<A, Option<i64>>(&mut seq, 26usize, &self.meta.sequence)?;
            let has_protected_content =
                seq_field::<A, Option<bool>>(&mut seq, 27usize, &self.meta.sequence)?;
            let is_from_offline =
                seq_field::<A, Option<bool>>(&mut seq, 28usize, &self.meta.sequence)?;
            let is_paid_post =
                seq_field::<A, Option<bool>>(&mut seq, 29usize, &self.meta.sequence)?;
            let media_group_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 30usize, &self.meta.sequence)?;
            let author_signature =
                seq_field::<A, Option<Box<str>>>(&mut seq, 31usize, &self.meta.sequence)?;
            let paid_star_count =
                seq_field::<A, Option<i64>>(&mut seq, 32usize, &self.meta.sequence)?;
            let entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                33usize,
                &self.meta.sequence,
            )?;
            let link_preview_options = seq_field::<A, Option<crate::types::LinkPreviewOptions>>(
                &mut seq,
                34usize,
                &self.meta.sequence,
            )?;
            let suggested_post_info = seq_field::<A, Option<crate::types::SuggestedPostInfo>>(
                &mut seq,
                35usize,
                &self.meta.sequence,
            )?;
            let effect_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 36usize, &self.meta.sequence)?;
            let caption = seq_field::<A, Option<Box<str>>>(&mut seq, 37usize, &self.meta.sequence)?;
            let caption_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                38usize,
                &self.meta.sequence,
            )?;
            let show_caption_above_media =
                seq_field::<A, Option<bool>>(&mut seq, 39usize, &self.meta.sequence)?;
            let has_media_spoiler =
                seq_field::<A, Option<bool>>(&mut seq, 40usize, &self.meta.sequence)?;
            let reply_markup = seq_field::<A, Option<crate::types::InlineKeyboardMarkup>>(
                &mut seq,
                41usize,
                &self.meta.sequence,
            )?;
            let value_0 = seq_field::<A, P0>(&mut seq, 42usize, &self.meta.sequence)?;
            Ok((
                Common {
                    author_signature,
                    business_connection_id,
                    caption,
                    caption_entities,
                    chat,
                    date,
                    direct_messages_topic,
                    edit_date,
                    effect_id,
                    entities,
                    ephemeral_message_id,
                    external_reply,
                    forward_origin,
                    from,
                    guest_bot_caller_chat,
                    guest_bot_caller_user,
                    guest_query_id,
                    has_media_spoiler,
                    has_protected_content,
                    is_automatic_forward,
                    is_from_offline,
                    is_paid_post,
                    is_topic_message,
                    link_preview_options,
                    media_group_id,
                    message_id,
                    message_thread_id,
                    paid_star_count,
                    quote,
                    receiver_user,
                    reply_markup,
                    reply_to_checklist_task_id,
                    reply_to_message,
                    reply_to_poll_option_id,
                    reply_to_story,
                    sender_boost_count,
                    sender_business_bot,
                    sender_chat,
                    sender_tag,
                    show_caption_above_media,
                    suggested_post_info,
                    via_bot,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 42usize => common.read(index, &mut map)?,
                    42usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let message_id = common
                .message_id
                .ok_or_else(|| A::Error::missing_field("message_id"))?;
            let message_thread_id = common.message_thread_id.unwrap_or(None);
            let direct_messages_topic = common.direct_messages_topic.unwrap_or(None);
            let from = common.from.unwrap_or(None);
            let sender_chat = common.sender_chat.unwrap_or(None);
            let sender_boost_count = common.sender_boost_count.unwrap_or(None);
            let sender_business_bot = common.sender_business_bot.unwrap_or(None);
            let sender_tag = common.sender_tag.unwrap_or(None);
            let receiver_user = common.receiver_user.unwrap_or(None);
            let ephemeral_message_id = common.ephemeral_message_id.unwrap_or(None);
            let date = common.date.ok_or_else(|| A::Error::missing_field("date"))?;
            let guest_query_id = common.guest_query_id.unwrap_or(None);
            let business_connection_id = common.business_connection_id.unwrap_or(None);
            let chat = common.chat.ok_or_else(|| A::Error::missing_field("chat"))?;
            let forward_origin = common.forward_origin.unwrap_or(None);
            let is_topic_message = common.is_topic_message.unwrap_or(None);
            let is_automatic_forward = common.is_automatic_forward.unwrap_or(None);
            let reply_to_message = common.reply_to_message.unwrap_or(None);
            let external_reply = common.external_reply.unwrap_or(None);
            let quote = common.quote.unwrap_or(None);
            let reply_to_story = common.reply_to_story.unwrap_or(None);
            let reply_to_checklist_task_id = common.reply_to_checklist_task_id.unwrap_or(None);
            let reply_to_poll_option_id = common.reply_to_poll_option_id.unwrap_or(None);
            let via_bot = common.via_bot.unwrap_or(None);
            let guest_bot_caller_user = common.guest_bot_caller_user.unwrap_or(None);
            let guest_bot_caller_chat = common.guest_bot_caller_chat.unwrap_or(None);
            let edit_date = common.edit_date.unwrap_or(None);
            let has_protected_content = common.has_protected_content.unwrap_or(None);
            let is_from_offline = common.is_from_offline.unwrap_or(None);
            let is_paid_post = common.is_paid_post.unwrap_or(None);
            let media_group_id = common.media_group_id.unwrap_or(None);
            let author_signature = common.author_signature.unwrap_or(None);
            let paid_star_count = common.paid_star_count.unwrap_or(None);
            let entities = common.entities.unwrap_or(None);
            let link_preview_options = common.link_preview_options.unwrap_or(None);
            let suggested_post_info = common.suggested_post_info.unwrap_or(None);
            let effect_id = common.effect_id.unwrap_or(None);
            let caption = common.caption.unwrap_or(None);
            let caption_entities = common.caption_entities.unwrap_or(None);
            let show_caption_above_media = common.show_caption_above_media.unwrap_or(None);
            let has_media_spoiler = common.has_media_spoiler.unwrap_or(None);
            let reply_markup = common.reply_markup.unwrap_or(None);
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    author_signature,
                    business_connection_id,
                    caption,
                    caption_entities,
                    chat,
                    date,
                    direct_messages_topic,
                    edit_date,
                    effect_id,
                    entities,
                    ephemeral_message_id,
                    external_reply,
                    forward_origin,
                    from,
                    guest_bot_caller_chat,
                    guest_bot_caller_user,
                    guest_query_id,
                    has_media_spoiler,
                    has_protected_content,
                    is_automatic_forward,
                    is_from_offline,
                    is_paid_post,
                    is_topic_message,
                    link_preview_options,
                    media_group_id,
                    message_id,
                    message_thread_id,
                    paid_star_count,
                    quote,
                    receiver_user,
                    reply_markup,
                    reply_to_checklist_task_id,
                    reply_to_message,
                    reply_to_poll_option_id,
                    reply_to_story,
                    sender_boost_count,
                    sender_business_bot,
                    sender_chat,
                    sender_tag,
                    show_caption_above_media,
                    suggested_post_info,
                    via_bot,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::message::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["message_id", "message_thread_id",
            "direct_messages_topic", "from", "sender_chat", "sender_boost_count",
            "sender_business_bot", "sender_tag", "receiver_user", "ephemeral_message_id",
            "date", "guest_query_id", "business_connection_id", "chat", "forward_origin",
            "is_topic_message", "is_automatic_forward", "reply_to_message",
            "external_reply", "quote", "reply_to_story", "reply_to_checklist_task_id",
            "reply_to_poll_option_id", "via_bot", "guest_bot_caller_user",
            "guest_bot_caller_chat", "edit_date", "has_protected_content",
            "is_from_offline", "is_paid_post", "media_group_id", "author_signature",
            "paid_star_count", "entities", "link_preview_options", "suggested_post_info",
            "effect_id", "caption", "caption_entities", "show_caption_above_media",
            "has_media_spoiler", "reply_markup",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 43 elements"), },) ?;
            Ok(Self { author_signature : common.author_signature, business_connection_id
            : common.business_connection_id, caption : common.caption, caption_entities :
            common.caption_entities, chat : common.chat, date : common.date,
            direct_messages_topic : common.direct_messages_topic, edit_date : common
            .edit_date, effect_id : common.effect_id, entities : common.entities,
            ephemeral_message_id : common.ephemeral_message_id, external_reply : common
            .external_reply, forward_origin : common.forward_origin, from : common.from,
            guest_bot_caller_chat : common.guest_bot_caller_chat, guest_bot_caller_user :
            common.guest_bot_caller_user, guest_query_id : common.guest_query_id,
            has_media_spoiler : common.has_media_spoiler, has_protected_content : common
            .has_protected_content, is_automatic_forward : common.is_automatic_forward,
            is_from_offline : common.is_from_offline, is_paid_post : common.is_paid_post,
            is_topic_message : common.is_topic_message, link_preview_options : common
            .link_preview_options, media_group_id : common.media_group_id, message_id :
            common.message_id, message_thread_id : common.message_thread_id,
            paid_star_count : common.paid_star_count, quote : common.quote, receiver_user
            : common.receiver_user, reply_markup : common.reply_markup,
            reply_to_checklist_task_id : common.reply_to_checklist_task_id,
            reply_to_message : common.reply_to_message, reply_to_poll_option_id : common
            .reply_to_poll_option_id, reply_to_story : common.reply_to_story,
            sender_boost_count : common.sender_boost_count, sender_business_bot : common
            .sender_business_bot, sender_chat : common.sender_chat, sender_tag : common
            .sender_tag, show_caption_above_media : common.show_caption_above_media,
            suggested_post_info : common.suggested_post_info, via_bot : common.via_bot,
            $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
}
pub(crate) mod message_entity {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) length: i64,
        pub(crate) offset: i64,
    }
    #[derive(Default)]
    struct Slots {
        length: Option<i64>,
        offset: Option<i64>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 2usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "length" => Some(0usize),
                "offset" => Some(1usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"length" => Some(0usize),
                b"offset" => Some(1usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.length, map, "length"),
                1usize => map_field(&mut self.offset, map, "offset"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0 {
        meta: Meta,
        marker: PhantomData<()>,
    }
    impl<'de> Visitor<'de> for Visitor0 {
        type Value = (Common, ());

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let offset = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let length = seq_field::<A, i64>(&mut seq, 1usize, &self.meta.sequence)?;
            Ok((
                Common {
                    length,
                    offset,
                },
                (),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let offset = common
                .offset
                .ok_or_else(|| A::Error::missing_field("offset"))?;
            let length = common
                .length
                .ok_or_else(|| A::Error::missing_field("length"))?;
            Ok((
                Common {
                    length,
                    offset,
                },
                (),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, ()), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident,) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common, ()) =
            $crate::deserialization::message_entity::read_0:: < D, > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["offset", "length"] }, specific : const { & [] }, sequence :
            concat!("struct ", stringify!($name), " with 2 elements"), },) ?; Ok(Self {
            length : common.length, offset : common.offset, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor1<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let offset = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let length = seq_field::<A, i64>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    length,
                    offset,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let offset = common
                .offset
                .ok_or_else(|| A::Error::missing_field("offset"))?;
            let length = common
                .length
                .ok_or_else(|| A::Error::missing_field("length"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    length,
                    offset,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::message_entity::read_1:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["offset", "length",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 3 elements"), },) ?;
            Ok(Self { length : common.length, offset : common.offset, $field_0 : value_0,
            }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de> + Default> Visitor<'de> for Visitor2<P0, P1> {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let offset = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let length = seq_field::<A, i64>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 3usize, &self.meta.sequence)?;
            Ok((
                Common {
                    length,
                    offset,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let offset = common
                .offset
                .ok_or_else(|| A::Error::missing_field("offset"))?;
            let length = common
                .length
                .ok_or_else(|| A::Error::missing_field("length"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 = value_1.unwrap_or_default();
            Ok((
                Common {
                    length,
                    offset,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_2<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::message_entity::read_2:: < D,
            $P0, $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["offset", "length",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 4 elements"), },) ?; Ok(Self {
            length : common.length, offset : common.offset, $field_0 : value_0, $field_1
            : value_1, }) } }
        };
    }
    pub(crate) use deserialize_2;
    struct Visitor3<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de> + Default> Visitor<'de> for Visitor3<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let offset = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let length = seq_field::<A, i64>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    length,
                    offset,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let offset = common
                .offset
                .ok_or_else(|| A::Error::missing_field("offset"))?;
            let length = common
                .length
                .ok_or_else(|| A::Error::missing_field("length"))?;
            let value_0 = value_0.unwrap_or_default();
            Ok((
                Common {
                    length,
                    offset,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_3<'de, D: Deserializer<'de>, P0: Deserialize<'de> + Default>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor3 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_3 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::message_entity::read_3:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["offset", "length",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 3 elements"), },) ?;
            Ok(Self { length : common.length, offset : common.offset, $field_0 : value_0,
            }) } }
        };
    }
    pub(crate) use deserialize_3;
}
pub(crate) mod message_origin {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) date: i64,
    }
    #[derive(Default)]
    struct Slots {
        date: Option<i64>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 1usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "date" => Some(0usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"date" => Some(0usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.date, map, "date"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1, P2> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de>, P2: Deserialize<'de> + Default>
        Visitor<'de> for Visitor0<P0, P1, P2>
    {
        type Value = (Common, (P0, P1, P2));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let date = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 3usize, &self.meta.sequence)?;
            Ok((
                Common {
                    date,
                },
                (value_0, value_1, value_2),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    3usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let date = common.date.ok_or_else(|| A::Error::missing_field("date"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let value_2 = value_2.unwrap_or_default();
            Ok((
                Common {
                    date,
                },
                (value_0, value_1, value_2),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de>,
        P2: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2,)) =
            $crate::deserialization::message_origin::read_0:: < D, $P0, $P1, $P2 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["date",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, sequence :
            concat!("struct ", stringify!($name), " with 4 elements"), },) ?; Ok(Self {
            date : common.date, $field_0 : value_0, $field_1 : value_1, $field_2 :
            value_2, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de> + Default> Visitor<'de> for Visitor1<P0, P1> {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let date = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    date,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    2usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let date = common.date.ok_or_else(|| A::Error::missing_field("date"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 = value_1.unwrap_or_default();
            Ok((
                Common {
                    date,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) = $crate::deserialization::message_origin::read_1:: < D,
            $P0, $P1 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["date",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 3 elements"), },) ?; Ok(Self {
            date : common.date, $field_0 : value_0, $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_1;
    struct Visitor2<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor2<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let date = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            Ok((
                Common {
                    date,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let date = common.date.ok_or_else(|| A::Error::missing_field("date"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    date,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_2<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor2 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_2 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::message_origin::read_2:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["date",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 2 elements"), },) ?;
            Ok(Self { date : common.date, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_2;
}
pub(crate) mod owned_gift {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) is_saved: Option<bool>,
        pub(crate) owned_gift_id: Option<Box<str>>,
        pub(crate) send_date: i64,
        pub(crate) sender_user: Option<Box<crate::types::User>>,
    }
    #[derive(Default)]
    struct Slots {
        is_saved: Option<Option<bool>>,
        owned_gift_id: Option<Option<Box<str>>>,
        send_date: Option<i64>,
        sender_user: Option<Option<Box<crate::types::User>>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 4usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "is_saved" => Some(0usize),
                "owned_gift_id" => Some(1usize),
                "send_date" => Some(2usize),
                "sender_user" => Some(3usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"is_saved" => Some(0usize),
                b"owned_gift_id" => Some(1usize),
                b"send_date" => Some(2usize),
                b"sender_user" => Some(3usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.is_saved, map, "is_saved"),
                1usize => map_field(&mut self.owned_gift_id, map, "owned_gift_id"),
                2usize => map_field(&mut self.send_date, map, "send_date"),
                3usize => map_field(&mut self.sender_user, map, "sender_user"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
            P4: Deserialize<'de> + Default,
            P5: Deserialize<'de> + Default,
            P6: Deserialize<'de> + Default,
            P7: Deserialize<'de> + Default,
            P8: Deserialize<'de> + Default,
            P9: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor0<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9>
    {
        type Value = (Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let owned_gift_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 1usize, &self.meta.sequence)?;
            let sender_user = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                2usize,
                &self.meta.sequence,
            )?;
            let send_date = seq_field::<A, i64>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 6usize, &self.meta.sequence)?;
            let is_saved = seq_field::<A, Option<bool>>(&mut seq, 7usize, &self.meta.sequence)?;
            let value_4 = seq_field::<A, P4>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_5 = seq_field::<A, P5>(&mut seq, 9usize, &self.meta.sequence)?;
            let value_6 = seq_field::<A, P6>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_7 = seq_field::<A, P7>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_8 = seq_field::<A, P8>(&mut seq, 12usize, &self.meta.sequence)?;
            let value_9 = seq_field::<A, P9>(&mut seq, 13usize, &self.meta.sequence)?;
            Ok((
                Common {
                    is_saved,
                    owned_gift_id,
                    send_date,
                    sender_user,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9,
                ),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            let mut value_4 = None;
            let mut value_5 = None;
            let mut value_6 = None;
            let mut value_7 = None;
            let mut value_8 = None;
            let mut value_9 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    6usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    7usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    8usize => {
                        map_field(&mut value_4, &mut map, self.meta.specific[4usize])?;
                    }
                    9usize => {
                        map_field(&mut value_5, &mut map, self.meta.specific[5usize])?;
                    }
                    10usize => {
                        map_field(&mut value_6, &mut map, self.meta.specific[6usize])?;
                    }
                    11usize => {
                        map_field(&mut value_7, &mut map, self.meta.specific[7usize])?;
                    }
                    12usize => {
                        map_field(&mut value_8, &mut map, self.meta.specific[8usize])?;
                    }
                    13usize => {
                        map_field(&mut value_9, &mut map, self.meta.specific[9usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let owned_gift_id = common.owned_gift_id.unwrap_or(None);
            let sender_user = common.sender_user.unwrap_or(None);
            let send_date = common
                .send_date
                .ok_or_else(|| A::Error::missing_field("send_date"))?;
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            let is_saved = common.is_saved.unwrap_or(None);
            let value_4 = value_4.unwrap_or_default();
            let value_5 = value_5.unwrap_or_default();
            let value_6 = value_6.unwrap_or_default();
            let value_7 = value_7.unwrap_or_default();
            let value_8 = value_8.unwrap_or_default();
            let value_9 = value_9.unwrap_or_default();
            Ok((
                Common {
                    is_saved,
                    owned_gift_id,
                    send_date,
                    sender_user,
                },
                (
                    value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                    value_8, value_9,
                ),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
        P4: Deserialize<'de> + Default,
        P5: Deserialize<'de> + Default,
        P6: Deserialize<'de> + Default,
        P7: Deserialize<'de> + Default,
        P8: Deserialize<'de> + Default,
        P9: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty, $field_4:ident : $P4:ty, $field_5:ident :
            $P5:ty, $field_6:ident : $P6:ty, $field_7:ident : $P7:ty, $field_8:ident :
            $P8:ty, $field_9:ident : $P9:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7,
            value_8, value_9,)) = $crate::deserialization::owned_gift::read_0:: < D, $P0,
            $P1, $P2, $P3, $P4, $P5, $P6, $P7, $P8, $P9 > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)), "owned_gift_id",
            "sender_user", "send_date",
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)), "is_saved",
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3)),
            $crate::deserialization::field_name(stringify!($field_4)),
            $crate::deserialization::field_name(stringify!($field_5)),
            $crate::deserialization::field_name(stringify!($field_6)),
            $crate::deserialization::field_name(stringify!($field_7)),
            $crate::deserialization::field_name(stringify!($field_8)),
            $crate::deserialization::field_name(stringify!($field_9))] }, sequence :
            concat!("struct ", stringify!($name), " with 14 elements"), },) ?; Ok(Self {
            is_saved : common.is_saved, owned_gift_id : common.owned_gift_id, send_date :
            common.send_date, sender_user : common.sender_user, $field_0 : value_0,
            $field_1 : value_1, $field_2 : value_2, $field_3 : value_3, $field_4 :
            value_4, $field_5 : value_5, $field_6 : value_6, $field_7 : value_7, $field_8
            : value_8, $field_9 : value_9, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0, P1, P2, P3> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2, P3)>,
    }
    impl<
            'de,
            P0: Deserialize<'de>,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
            P3: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor1<P0, P1, P2, P3>
    {
        type Value = (Common, (P0, P1, P2, P3));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let value_0 = seq_field::<A, P0>(&mut seq, 0usize, &self.meta.sequence)?;
            let owned_gift_id =
                seq_field::<A, Option<Box<str>>>(&mut seq, 1usize, &self.meta.sequence)?;
            let sender_user = seq_field::<A, Option<Box<crate::types::User>>>(
                &mut seq,
                2usize,
                &self.meta.sequence,
            )?;
            let send_date = seq_field::<A, i64>(&mut seq, 3usize, &self.meta.sequence)?;
            let is_saved = seq_field::<A, Option<bool>>(&mut seq, 4usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 5usize, &self.meta.sequence)?;
            let value_2 = seq_field::<A, P2>(&mut seq, 6usize, &self.meta.sequence)?;
            let value_3 = seq_field::<A, P3>(&mut seq, 7usize, &self.meta.sequence)?;
            Ok((
                Common {
                    is_saved,
                    owned_gift_id,
                    send_date,
                    sender_user,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            let mut value_3 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    5usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    6usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    7usize => {
                        map_field(&mut value_3, &mut map, self.meta.specific[3usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let owned_gift_id = common.owned_gift_id.unwrap_or(None);
            let sender_user = common.sender_user.unwrap_or(None);
            let send_date = common
                .send_date
                .ok_or_else(|| A::Error::missing_field("send_date"))?;
            let is_saved = common.is_saved.unwrap_or(None);
            let value_1 = value_1.unwrap_or_default();
            let value_2 = value_2.unwrap_or_default();
            let value_3 = value_3.unwrap_or_default();
            Ok((
                Common {
                    is_saved,
                    owned_gift_id,
                    send_date,
                    sender_user,
                },
                (value_0, value_1, value_2, value_3),
            ))
        }
    }
    pub(crate) fn read_1<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de>,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
        P3: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2, P3)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty, $field_3:ident : $P3:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2, value_3,)) =
            $crate::deserialization::owned_gift::read_1:: < D, $P0, $P1, $P2, $P3 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { &
            [$crate::deserialization::field_name(stringify!($field_0)), "owned_gift_id",
            "sender_user", "send_date", "is_saved",
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2)),
            $crate::deserialization::field_name(stringify!($field_3))] }, sequence :
            concat!("struct ", stringify!($name), " with 8 elements"), },) ?; Ok(Self {
            is_saved : common.is_saved, owned_gift_id : common.owned_gift_id, send_date :
            common.send_date, sender_user : common.sender_user, $field_0 : value_0,
            $field_1 : value_1, $field_2 : value_2, $field_3 : value_3, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod passport_element_error {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) message: Box<str>,
        pub(crate) r#type: Box<str>,
    }
    #[derive(Default)]
    struct Slots {
        message: Option<Box<str>>,
        r#type: Option<Box<str>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 2usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "message" => Some(0usize),
                "type" => Some(1usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"message" => Some(0usize),
                b"type" => Some(1usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.message, map, "message"),
                1usize => map_field(&mut self.r#type, map, "type"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1> {
        meta: Meta,
        marker: PhantomData<(P0, P1)>,
    }
    impl<'de, P0: Deserialize<'de>, P1: Deserialize<'de>> Visitor<'de> for Visitor0<P0, P1> {
        type Value = (Common, (P0, P1));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let r#type = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 2usize, &self.meta.sequence)?;
            let message = seq_field::<A, Box<str>>(&mut seq, 3usize, &self.meta.sequence)?;
            Ok((
                Common {
                    message,
                    r#type,
                },
                (value_0, value_1),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    3usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let r#type = common
                .r#type
                .ok_or_else(|| A::Error::missing_field("type"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let value_1 =
                value_1.ok_or_else(|| A::Error::missing_field(self.meta.specific[1usize]))?;
            let message = common
                .message
                .ok_or_else(|| A::Error::missing_field("message"))?;
            Ok((
                Common {
                    message,
                    r#type,
                },
                (value_0, value_1),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>, P1: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1,)) =
            $crate::deserialization::passport_element_error::read_0:: < D, $P0, $P1 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["type",
            $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)), "message"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1))] }, sequence :
            concat!("struct ", stringify!($name), " with 4 elements"), },) ?; Ok(Self {
            message : common.message, r#type : common.r#type, $field_0 : value_0,
            $field_1 : value_1, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor1<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let r#type = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            let message = seq_field::<A, Box<str>>(&mut seq, 2usize, &self.meta.sequence)?;
            Ok((
                Common {
                    message,
                    r#type,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 2usize => common.read(index, &mut map)?,
                    2usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let r#type = common
                .r#type
                .ok_or_else(|| A::Error::missing_field("type"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            let message = common
                .message
                .ok_or_else(|| A::Error::missing_field("message"))?;
            Ok((
                Common {
                    message,
                    r#type,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::passport_element_error::read_1:: < D,
            $P0 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["type",
            $crate::deserialization::field_name(stringify!($field_0)), "message"] },
            specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 3 elements"), },) ?; Ok(Self {
            message : common.message, r#type : common.r#type, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod poll {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
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
    struct Slots {
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
    impl CommonFields for Slots {
        const COUNT: usize = 17usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "allows_multiple_answers" => Some(0usize),
                "allows_revoting" => Some(1usize),
                "close_date" => Some(2usize),
                "country_codes" => Some(3usize),
                "description" => Some(4usize),
                "description_entities" => Some(5usize),
                "explanation_entities" => Some(6usize),
                "id" => Some(7usize),
                "is_anonymous" => Some(8usize),
                "is_closed" => Some(9usize),
                "media" => Some(10usize),
                "members_only" => Some(11usize),
                "open_period" => Some(12usize),
                "options" => Some(13usize),
                "question" => Some(14usize),
                "question_entities" => Some(15usize),
                "total_voter_count" => Some(16usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"allows_multiple_answers" => Some(0usize),
                b"allows_revoting" => Some(1usize),
                b"close_date" => Some(2usize),
                b"country_codes" => Some(3usize),
                b"description" => Some(4usize),
                b"description_entities" => Some(5usize),
                b"explanation_entities" => Some(6usize),
                b"id" => Some(7usize),
                b"is_anonymous" => Some(8usize),
                b"is_closed" => Some(9usize),
                b"media" => Some(10usize),
                b"members_only" => Some(11usize),
                b"open_period" => Some(12usize),
                b"options" => Some(13usize),
                b"question" => Some(14usize),
                b"question_entities" => Some(15usize),
                b"total_voter_count" => Some(16usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(
                    &mut self.allows_multiple_answers,
                    map,
                    "allows_multiple_answers",
                ),
                1usize => map_field(&mut self.allows_revoting, map, "allows_revoting"),
                2usize => map_field(&mut self.close_date, map, "close_date"),
                3usize => map_field(&mut self.country_codes, map, "country_codes"),
                4usize => map_field(&mut self.description, map, "description"),
                5usize => map_field(&mut self.description_entities, map, "description_entities"),
                6usize => map_field(&mut self.explanation_entities, map, "explanation_entities"),
                7usize => map_field(&mut self.id, map, "id"),
                8usize => map_field(&mut self.is_anonymous, map, "is_anonymous"),
                9usize => map_field(&mut self.is_closed, map, "is_closed"),
                10usize => map_field(&mut self.media, map, "media"),
                11usize => map_field(&mut self.members_only, map, "members_only"),
                12usize => map_field(&mut self.open_period, map, "open_period"),
                13usize => map_field(&mut self.options, map, "options"),
                14usize => map_field(&mut self.question, map, "question"),
                15usize => map_field(&mut self.question_entities, map, "question_entities"),
                16usize => map_field(&mut self.total_voter_count, map, "total_voter_count"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0, P1, P2> {
        meta: Meta,
        marker: PhantomData<(P0, P1, P2)>,
    }
    impl<
            'de,
            P0: Deserialize<'de> + Default,
            P1: Deserialize<'de> + Default,
            P2: Deserialize<'de> + Default,
        > Visitor<'de> for Visitor0<P0, P1, P2>
    {
        type Value = (Common, (P0, P1, P2));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let question = seq_field::<A, Box<str>>(&mut seq, 1usize, &self.meta.sequence)?;
            let question_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                2usize,
                &self.meta.sequence,
            )?;
            let options = seq_field::<A, Box<[crate::types::PollOption]>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let total_voter_count = seq_field::<A, i64>(&mut seq, 4usize, &self.meta.sequence)?;
            let is_closed = seq_field::<A, bool>(&mut seq, 5usize, &self.meta.sequence)?;
            let is_anonymous = seq_field::<A, bool>(&mut seq, 6usize, &self.meta.sequence)?;
            let allows_multiple_answers =
                seq_field::<A, bool>(&mut seq, 7usize, &self.meta.sequence)?;
            let allows_revoting = seq_field::<A, bool>(&mut seq, 8usize, &self.meta.sequence)?;
            let members_only = seq.next_element::<bool>()?.unwrap_or_default();
            let country_codes =
                seq_field::<A, Option<Box<[Box<str>]>>>(&mut seq, 10usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 11usize, &self.meta.sequence)?;
            let value_1 = seq_field::<A, P1>(&mut seq, 12usize, &self.meta.sequence)?;
            let explanation_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                13usize,
                &self.meta.sequence,
            )?;
            let value_2 = seq_field::<A, P2>(&mut seq, 14usize, &self.meta.sequence)?;
            let open_period = seq_field::<A, Option<i64>>(&mut seq, 15usize, &self.meta.sequence)?;
            let close_date = seq_field::<A, Option<i64>>(&mut seq, 16usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 17usize, &self.meta.sequence)?;
            let description_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                18usize,
                &self.meta.sequence,
            )?;
            let media = seq_field::<A, Option<crate::types::PollMedia>>(
                &mut seq,
                19usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    allows_multiple_answers,
                    allows_revoting,
                    close_date,
                    country_codes,
                    description,
                    description_entities,
                    explanation_entities,
                    id,
                    is_anonymous,
                    is_closed,
                    media,
                    members_only,
                    open_period,
                    options,
                    question,
                    question_entities,
                    total_voter_count,
                },
                (value_0, value_1, value_2),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            let mut value_1 = None;
            let mut value_2 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 17usize => common.read(index, &mut map)?,
                    17usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    18usize => {
                        map_field(&mut value_1, &mut map, self.meta.specific[1usize])?;
                    }
                    19usize => {
                        map_field(&mut value_2, &mut map, self.meta.specific[2usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let question = common
                .question
                .ok_or_else(|| A::Error::missing_field("question"))?;
            let question_entities = common.question_entities.unwrap_or(None);
            let options = common
                .options
                .ok_or_else(|| A::Error::missing_field("options"))?;
            let total_voter_count = common
                .total_voter_count
                .ok_or_else(|| A::Error::missing_field("total_voter_count"))?;
            let is_closed = common
                .is_closed
                .ok_or_else(|| A::Error::missing_field("is_closed"))?;
            let is_anonymous = common
                .is_anonymous
                .ok_or_else(|| A::Error::missing_field("is_anonymous"))?;
            let allows_multiple_answers = common
                .allows_multiple_answers
                .ok_or_else(|| A::Error::missing_field("allows_multiple_answers"))?;
            let allows_revoting = common
                .allows_revoting
                .ok_or_else(|| A::Error::missing_field("allows_revoting"))?;
            let members_only = common.members_only.unwrap_or_default();
            let country_codes = common.country_codes.unwrap_or(None);
            let value_0 = value_0.unwrap_or_default();
            let value_1 = value_1.unwrap_or_default();
            let explanation_entities = common.explanation_entities.unwrap_or(None);
            let value_2 = value_2.unwrap_or_default();
            let open_period = common.open_period.unwrap_or(None);
            let close_date = common.close_date.unwrap_or(None);
            let description = common.description.unwrap_or(None);
            let description_entities = common.description_entities.unwrap_or(None);
            let media = common.media.unwrap_or(None);
            Ok((
                Common {
                    allows_multiple_answers,
                    allows_revoting,
                    close_date,
                    country_codes,
                    description,
                    description_entities,
                    explanation_entities,
                    id,
                    is_anonymous,
                    is_closed,
                    media,
                    members_only,
                    open_period,
                    options,
                    question,
                    question_entities,
                    total_voter_count,
                },
                (value_0, value_1, value_2),
            ))
        }
    }
    pub(crate) fn read_0<
        'de,
        D: Deserializer<'de>,
        P0: Deserialize<'de> + Default,
        P1: Deserialize<'de> + Default,
        P2: Deserialize<'de> + Default,
    >(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0, P1, P2)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        (
            $name:ident, $field_0:ident : $P0:ty, $field_1:ident : $P1:ty, $field_2:ident
            : $P2:ty
        ) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0, value_1, value_2,)) = $crate::deserialization::poll::read_0:: < D,
            $P0, $P1, $P2 > (deserializer, $crate::deserialization::Meta { name :
            stringify!($name), fields : const { & ["id", "question", "question_entities",
            "options", "total_voter_count", "is_closed", "is_anonymous",
            "allows_multiple_answers", "allows_revoting", "members_only",
            "country_codes", $crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            "explanation_entities",
            $crate::deserialization::field_name(stringify!($field_2)), "open_period",
            "close_date", "description", "description_entities", "media"] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0)),
            $crate::deserialization::field_name(stringify!($field_1)),
            $crate::deserialization::field_name(stringify!($field_2))] }, sequence :
            concat!("struct ", stringify!($name), " with 20 elements"), },) ?; Ok(Self {
            allows_multiple_answers : common.allows_multiple_answers, allows_revoting :
            common.allows_revoting, close_date : common.close_date, country_codes :
            common.country_codes, description : common.description, description_entities
            : common.description_entities, explanation_entities : common
            .explanation_entities, id : common.id, is_anonymous : common.is_anonymous,
            is_closed : common.is_closed, media : common.media, members_only : common
            .members_only, open_period : common.open_period, options : common.options,
            question : common.question, question_entities : common.question_entities,
            total_voter_count : common.total_voter_count, $field_0 : value_0, $field_1 :
            value_1, $field_2 : value_2, }) } }
        };
    }
    pub(crate) use deserialize_0;
    struct Visitor1 {
        meta: Meta,
        marker: PhantomData<()>,
    }
    impl<'de> Visitor<'de> for Visitor1 {
        type Value = (Common, ());

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let question = seq_field::<A, Box<str>>(&mut seq, 1usize, &self.meta.sequence)?;
            let question_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                2usize,
                &self.meta.sequence,
            )?;
            let options = seq_field::<A, Box<[crate::types::PollOption]>>(
                &mut seq,
                3usize,
                &self.meta.sequence,
            )?;
            let total_voter_count = seq_field::<A, i64>(&mut seq, 4usize, &self.meta.sequence)?;
            let is_closed = seq_field::<A, bool>(&mut seq, 5usize, &self.meta.sequence)?;
            let is_anonymous = seq_field::<A, bool>(&mut seq, 6usize, &self.meta.sequence)?;
            let allows_multiple_answers =
                seq_field::<A, bool>(&mut seq, 7usize, &self.meta.sequence)?;
            let allows_revoting = seq_field::<A, bool>(&mut seq, 8usize, &self.meta.sequence)?;
            let members_only = seq.next_element::<bool>()?.unwrap_or_default();
            let country_codes =
                seq_field::<A, Option<Box<[Box<str>]>>>(&mut seq, 10usize, &self.meta.sequence)?;
            let explanation_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                11usize,
                &self.meta.sequence,
            )?;
            let open_period = seq_field::<A, Option<i64>>(&mut seq, 12usize, &self.meta.sequence)?;
            let close_date = seq_field::<A, Option<i64>>(&mut seq, 13usize, &self.meta.sequence)?;
            let description =
                seq_field::<A, Option<Box<str>>>(&mut seq, 14usize, &self.meta.sequence)?;
            let description_entities = seq_field::<A, Option<Box<[crate::types::MessageEntity]>>>(
                &mut seq,
                15usize,
                &self.meta.sequence,
            )?;
            let media = seq_field::<A, Option<crate::types::PollMedia>>(
                &mut seq,
                16usize,
                &self.meta.sequence,
            )?;
            Ok((
                Common {
                    allows_multiple_answers,
                    allows_revoting,
                    close_date,
                    country_codes,
                    description,
                    description_entities,
                    explanation_entities,
                    id,
                    is_anonymous,
                    is_closed,
                    media,
                    members_only,
                    open_period,
                    options,
                    question,
                    question_entities,
                    total_voter_count,
                },
                (),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 17usize => common.read(index, &mut map)?,
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let question = common
                .question
                .ok_or_else(|| A::Error::missing_field("question"))?;
            let question_entities = common.question_entities.unwrap_or(None);
            let options = common
                .options
                .ok_or_else(|| A::Error::missing_field("options"))?;
            let total_voter_count = common
                .total_voter_count
                .ok_or_else(|| A::Error::missing_field("total_voter_count"))?;
            let is_closed = common
                .is_closed
                .ok_or_else(|| A::Error::missing_field("is_closed"))?;
            let is_anonymous = common
                .is_anonymous
                .ok_or_else(|| A::Error::missing_field("is_anonymous"))?;
            let allows_multiple_answers = common
                .allows_multiple_answers
                .ok_or_else(|| A::Error::missing_field("allows_multiple_answers"))?;
            let allows_revoting = common
                .allows_revoting
                .ok_or_else(|| A::Error::missing_field("allows_revoting"))?;
            let members_only = common.members_only.unwrap_or_default();
            let country_codes = common.country_codes.unwrap_or(None);
            let explanation_entities = common.explanation_entities.unwrap_or(None);
            let open_period = common.open_period.unwrap_or(None);
            let close_date = common.close_date.unwrap_or(None);
            let description = common.description.unwrap_or(None);
            let description_entities = common.description_entities.unwrap_or(None);
            let media = common.media.unwrap_or(None);
            Ok((
                Common {
                    allows_multiple_answers,
                    allows_revoting,
                    close_date,
                    country_codes,
                    description,
                    description_entities,
                    explanation_entities,
                    id,
                    is_anonymous,
                    is_closed,
                    media,
                    members_only,
                    open_period,
                    options,
                    question,
                    question_entities,
                    total_voter_count,
                },
                (),
            ))
        }
    }
    pub(crate) fn read_1<'de, D: Deserializer<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, ()), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor1 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_1 {
        ($name:ident,) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common, ()) =
            $crate::deserialization::poll::read_1:: < D, > (deserializer,
            $crate::deserialization::Meta { name : stringify!($name), fields : const { &
            ["id", "question", "question_entities", "options", "total_voter_count",
            "is_closed", "is_anonymous", "allows_multiple_answers", "allows_revoting",
            "members_only", "country_codes", "explanation_entities", "open_period",
            "close_date", "description", "description_entities", "media"] }, specific :
            const { & [] }, sequence : concat!("struct ", stringify!($name),
            " with 17 elements"), },) ?; Ok(Self { allows_multiple_answers : common
            .allows_multiple_answers, allows_revoting : common.allows_revoting,
            close_date : common.close_date, country_codes : common.country_codes,
            description : common.description, description_entities : common
            .description_entities, explanation_entities : common.explanation_entities, id
            : common.id, is_anonymous : common.is_anonymous, is_closed : common
            .is_closed, media : common.media, members_only : common.members_only,
            open_period : common.open_period, options : common.options, question : common
            .question, question_entities : common.question_entities, total_voter_count :
            common.total_voter_count, }) } }
        };
    }
    pub(crate) use deserialize_1;
}
pub(crate) mod star_transaction {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) amount: i64,
        pub(crate) date: i64,
        pub(crate) id: Box<str>,
        pub(crate) nanostar_amount: Option<u32>,
    }
    #[derive(Default)]
    struct Slots {
        amount: Option<i64>,
        date: Option<i64>,
        id: Option<Box<str>>,
        nanostar_amount: Option<Option<u32>>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 4usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "amount" => Some(0usize),
                "date" => Some(1usize),
                "id" => Some(2usize),
                "nanostar_amount" => Some(3usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"amount" => Some(0usize),
                b"date" => Some(1usize),
                b"id" => Some(2usize),
                b"nanostar_amount" => Some(3usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.amount, map, "amount"),
                1usize => map_field(&mut self.date, map, "date"),
                2usize => map_field(&mut self.id, map, "id"),
                3usize => map_field(&mut self.nanostar_amount, map, "nanostar_amount"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let amount = seq_field::<A, i64>(&mut seq, 1usize, &self.meta.sequence)?;
            let nanostar_amount =
                seq_field::<A, Option<u32>>(&mut seq, 2usize, &self.meta.sequence)?;
            let date = seq_field::<A, i64>(&mut seq, 3usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 4usize, &self.meta.sequence)?;
            Ok((
                Common {
                    amount,
                    date,
                    id,
                    nanostar_amount,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 4usize => common.read(index, &mut map)?,
                    4usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let id = common.id.ok_or_else(|| A::Error::missing_field("id"))?;
            let amount = common
                .amount
                .ok_or_else(|| A::Error::missing_field("amount"))?;
            let nanostar_amount = common.nanostar_amount.unwrap_or(None);
            let date = common.date.ok_or_else(|| A::Error::missing_field("date"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    amount,
                    date,
                    id,
                    nanostar_amount,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::star_transaction::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["id", "amount", "nanostar_amount", "date",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 5 elements"), },) ?;
            Ok(Self { amount : common.amount, date : common.date, id : common.id,
            nanostar_amount : common.nanostar_amount, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
}
pub(crate) mod sticker {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) emoji: Option<Box<str>>,
        pub(crate) file_id: Box<str>,
        pub(crate) file_size: Option<i64>,
        pub(crate) file_unique_id: Box<str>,
        pub(crate) height: i64,
        pub(crate) is_animated: bool,
        pub(crate) is_video: bool,
        pub(crate) needs_repainting: Option<bool>,
        pub(crate) set_name: Option<Box<str>>,
        pub(crate) thumbnail: Option<crate::types::PhotoSize>,
        pub(crate) width: i64,
    }
    #[derive(Default)]
    struct Slots {
        emoji: Option<Option<Box<str>>>,
        file_id: Option<Box<str>>,
        file_size: Option<Option<i64>>,
        file_unique_id: Option<Box<str>>,
        height: Option<i64>,
        is_animated: Option<bool>,
        is_video: Option<bool>,
        needs_repainting: Option<Option<bool>>,
        set_name: Option<Option<Box<str>>>,
        thumbnail: Option<Option<crate::types::PhotoSize>>,
        width: Option<i64>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 11usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "emoji" => Some(0usize),
                "file_id" => Some(1usize),
                "file_size" => Some(2usize),
                "file_unique_id" => Some(3usize),
                "height" => Some(4usize),
                "is_animated" => Some(5usize),
                "is_video" => Some(6usize),
                "needs_repainting" => Some(7usize),
                "set_name" => Some(8usize),
                "thumbnail" => Some(9usize),
                "width" => Some(10usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"emoji" => Some(0usize),
                b"file_id" => Some(1usize),
                b"file_size" => Some(2usize),
                b"file_unique_id" => Some(3usize),
                b"height" => Some(4usize),
                b"is_animated" => Some(5usize),
                b"is_video" => Some(6usize),
                b"needs_repainting" => Some(7usize),
                b"set_name" => Some(8usize),
                b"thumbnail" => Some(9usize),
                b"width" => Some(10usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.emoji, map, "emoji"),
                1usize => map_field(&mut self.file_id, map, "file_id"),
                2usize => map_field(&mut self.file_size, map, "file_size"),
                3usize => map_field(&mut self.file_unique_id, map, "file_unique_id"),
                4usize => map_field(&mut self.height, map, "height"),
                5usize => map_field(&mut self.is_animated, map, "is_animated"),
                6usize => map_field(&mut self.is_video, map, "is_video"),
                7usize => map_field(&mut self.needs_repainting, map, "needs_repainting"),
                8usize => map_field(&mut self.set_name, map, "set_name"),
                9usize => map_field(&mut self.thumbnail, map, "thumbnail"),
                10usize => map_field(&mut self.width, map, "width"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de> + Default> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let file_id = seq_field::<A, Box<str>>(&mut seq, 0usize, &self.meta.sequence)?;
            let file_unique_id = seq_field::<A, Box<str>>(&mut seq, 1usize, &self.meta.sequence)?;
            let width = seq_field::<A, i64>(&mut seq, 2usize, &self.meta.sequence)?;
            let height = seq_field::<A, i64>(&mut seq, 3usize, &self.meta.sequence)?;
            let is_animated = seq_field::<A, bool>(&mut seq, 4usize, &self.meta.sequence)?;
            let is_video = seq_field::<A, bool>(&mut seq, 5usize, &self.meta.sequence)?;
            let thumbnail = seq_field::<A, Option<crate::types::PhotoSize>>(
                &mut seq,
                6usize,
                &self.meta.sequence,
            )?;
            let emoji = seq_field::<A, Option<Box<str>>>(&mut seq, 7usize, &self.meta.sequence)?;
            let set_name = seq_field::<A, Option<Box<str>>>(&mut seq, 8usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 9usize, &self.meta.sequence)?;
            let needs_repainting =
                seq_field::<A, Option<bool>>(&mut seq, 10usize, &self.meta.sequence)?;
            let file_size = seq_field::<A, Option<i64>>(&mut seq, 11usize, &self.meta.sequence)?;
            Ok((
                Common {
                    emoji,
                    file_id,
                    file_size,
                    file_unique_id,
                    height,
                    is_animated,
                    is_video,
                    needs_repainting,
                    set_name,
                    thumbnail,
                    width,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 11usize => common.read(index, &mut map)?,
                    11usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let file_id = common
                .file_id
                .ok_or_else(|| A::Error::missing_field("file_id"))?;
            let file_unique_id = common
                .file_unique_id
                .ok_or_else(|| A::Error::missing_field("file_unique_id"))?;
            let width = common
                .width
                .ok_or_else(|| A::Error::missing_field("width"))?;
            let height = common
                .height
                .ok_or_else(|| A::Error::missing_field("height"))?;
            let is_animated = common
                .is_animated
                .ok_or_else(|| A::Error::missing_field("is_animated"))?;
            let is_video = common
                .is_video
                .ok_or_else(|| A::Error::missing_field("is_video"))?;
            let thumbnail = common.thumbnail.unwrap_or(None);
            let emoji = common.emoji.unwrap_or(None);
            let set_name = common.set_name.unwrap_or(None);
            let value_0 = value_0.unwrap_or_default();
            let needs_repainting = common.needs_repainting.unwrap_or(None);
            let file_size = common.file_size.unwrap_or(None);
            Ok((
                Common {
                    emoji,
                    file_id,
                    file_size,
                    file_unique_id,
                    height,
                    is_animated,
                    is_video,
                    needs_repainting,
                    set_name,
                    thumbnail,
                    width,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de> + Default>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::sticker::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["file_id", "file_unique_id", "width", "height",
            "is_animated", "is_video", "thumbnail", "emoji", "set_name",
            $crate::deserialization::field_name(stringify!($field_0)),
            "needs_repainting", "file_size"] }, specific : const { &
            [$crate::deserialization::field_name(stringify!($field_0))] }, sequence :
            concat!("struct ", stringify!($name), " with 12 elements"), },) ?; Ok(Self {
            emoji : common.emoji, file_id : common.file_id, file_size : common.file_size,
            file_unique_id : common.file_unique_id, height : common.height, is_animated :
            common.is_animated, is_video : common.is_video, needs_repainting : common
            .needs_repainting, set_name : common.set_name, thumbnail : common.thumbnail,
            width : common.width, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
}
pub(crate) mod update {
    use super::{
        fmt, map_field, seq_field, CommonFields, Deserialize, Deserializer, Error, IgnoredAny, Key,
        MapAccess, Meta, PhantomData, SeqAccess, Visitor,
    };
    pub(crate) struct Common {
        pub(crate) update_id: i64,
    }
    #[derive(Default)]
    struct Slots {
        update_id: Option<i64>,
    }
    impl CommonFields for Slots {
        const COUNT: usize = 1usize;

        #[inline]
        fn field(value: &str) -> Option<usize> {
            match value {
                "update_id" => Some(0usize),
                _ => None,
            }
        }

        #[inline]
        fn field_bytes(value: &[u8]) -> Option<usize> {
            match value {
                b"update_id" => Some(0usize),
                _ => None,
            }
        }
    }
    impl Slots {
        fn read<'de, A: MapAccess<'de>>(
            &mut self,
            index: usize,
            map: &mut A,
        ) -> Result<(), A::Error> {
            match index {
                0usize => map_field(&mut self.update_id, map, "update_id"),
                _ => unreachable!(),
            }
        }
    }
    struct Visitor0<P0> {
        meta: Meta,
        marker: PhantomData<(P0,)>,
    }
    impl<'de, P0: Deserialize<'de>> Visitor<'de> for Visitor0<P0> {
        type Value = (Common, (P0,));

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("struct ")?;
            formatter.write_str(self.meta.name)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let update_id = seq_field::<A, i64>(&mut seq, 0usize, &self.meta.sequence)?;
            let value_0 = seq_field::<A, P0>(&mut seq, 1usize, &self.meta.sequence)?;
            Ok((
                Common {
                    update_id,
                },
                (value_0,),
            ))
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut common = Slots::default();
            let mut value_0 = None;
            while let Some(field) = map.next_key_seed(Key::<Slots>::new(self.meta))? {
                match field {
                    index if index < 1usize => common.read(index, &mut map)?,
                    1usize => {
                        map_field(&mut value_0, &mut map, self.meta.specific[0usize])?;
                    }
                    _ => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            let update_id = common
                .update_id
                .ok_or_else(|| A::Error::missing_field("update_id"))?;
            let value_0 =
                value_0.ok_or_else(|| A::Error::missing_field(self.meta.specific[0usize]))?;
            Ok((
                Common {
                    update_id,
                },
                (value_0,),
            ))
        }
    }
    pub(crate) fn read_0<'de, D: Deserializer<'de>, P0: Deserialize<'de>>(
        deserializer: D,
        meta: Meta,
    ) -> Result<(Common, (P0,)), D::Error> {
        deserializer.deserialize_struct(
            meta.name,
            meta.fields,
            Visitor0 {
                meta,
                marker: PhantomData,
            },
        )
    }
    #[rustfmt::skip]
    macro_rules! deserialize_0 {
        ($name:ident, $field_0:ident : $P0:ty) => {
            impl < 'de > serde::Deserialize < 'de > for $name {
            #[allow(clippy::too_many_lines)] fn deserialize < D : serde::Deserializer <
            'de >> (deserializer : D) -> Result < Self, D::Error > { let (common,
            (value_0,)) = $crate::deserialization::update::read_0:: < D, $P0 >
            (deserializer, $crate::deserialization::Meta { name : stringify!($name),
            fields : const { & ["update_id",
            $crate::deserialization::field_name(stringify!($field_0))] }, specific :
            const { & [$crate::deserialization::field_name(stringify!($field_0))] },
            sequence : concat!("struct ", stringify!($name), " with 2 elements"), },) ?;
            Ok(Self { update_id : common.update_id, $field_0 : value_0, }) } }
        };
    }
    pub(crate) use deserialize_0;
}
