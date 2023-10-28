use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
    fn from(id: i64) -> Self {
        Self::Id(id)
    }
}

impl From<Box<str>> for ChatIdKind {
    fn from(username: Box<str>) -> Self {
        Self::Username(username)
    }
}

impl From<&str> for ChatIdKind {
    fn from(username: &str) -> Self {
        Self::Username(username.into())
    }
}
