//! This module contains the [`ChatIdKind`] enum, which addresses a chat by its numeric identifier or by the
//! `@username` of a bot, a supergroup or a channel.

use crate::serialization::{Deserialize, Serialize};
use strum_macros::Display;

/// Chat a method is sent to, addressed either by its unique identifier or by the `@username` of the target
/// bot, supergroup or channel.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(not(feature = "deser"), serde(untagged, rename_all = "snake_case"))]
#[cfg_attr(feature = "deser", deser(untagged, rename_all = "snake_case"))]
pub enum ChatIdKind {
    Id(i64),
    Username(Box<str>),
}

impl ChatIdKind {
    #[inline]
    #[must_use]
    pub fn id(val: i64) -> Self {
        Self::Id(val)
    }

    #[inline]
    #[must_use]
    pub fn username(val: impl Into<Box<str>>) -> Self {
        Self::Username(val.into())
    }
}

impl From<i64> for ChatIdKind {
    #[inline]
    fn from(val: i64) -> Self {
        Self::id(val)
    }
}

impl From<Box<str>> for ChatIdKind {
    #[inline]
    fn from(val: Box<str>) -> Self {
        Self::username(val)
    }
}

impl From<&str> for ChatIdKind {
    #[inline]
    fn from(val: &str) -> Self {
        Self::username(val)
    }
}

impl From<String> for ChatIdKind {
    #[inline]
    fn from(val: String) -> Self {
        Self::username(val)
    }
}
