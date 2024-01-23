use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ChatIdKind {
    Id(i64),
    Username(Box<str>),
}

impl ChatIdKind {
    #[must_use]
    pub fn id(val: i64) -> Self {
        Self::Id(val)
    }

    #[must_use]
    pub fn username(val: impl Into<Box<str>>) -> Self {
        Self::Username(val.into())
    }
}

impl From<i64> for ChatIdKind {
    fn from(val: i64) -> Self {
        Self::id(val)
    }
}

impl From<Box<str>> for ChatIdKind {
    fn from(val: Box<str>) -> Self {
        Self::username(val)
    }
}

impl From<&str> for ChatIdKind {
    fn from(val: &str) -> Self {
        Self::username(val)
    }
}
