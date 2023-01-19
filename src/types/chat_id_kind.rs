use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ChatIdKind {
    Id(i64),
    Username(String),
}

impl ChatIdKind {
    #[must_use]
    pub fn id(id: i64) -> Self {
        Self::Id(id)
    }

    #[must_use]
    pub fn username<T: Into<String>>(username: T) -> Self {
        Self::Username(username.into())
    }
}

impl From<i64> for ChatIdKind {
    fn from(id: i64) -> Self {
        Self::Id(id)
    }
}

impl From<String> for ChatIdKind {
    fn from(username: String) -> Self {
        Self::Username(username)
    }
}

impl From<&str> for ChatIdKind {
    fn from(username: &str) -> Self {
        Self::Username(username.to_string())
    }
}
