use crate::types::WidgetQueryType;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the type of a query from a widget. Currently, it can be one of
/// - [`crate::types::WidgetQueryTypeData`]
/// - [`crate::types::WidgetQueryTypeSignData`]
/// - [`crate::types::WidgetQueryTypeSentTransaction`]
/// # Documentation
/// <https://core.telegram.org/bots/api#widgetquerytype>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum WidgetQueryTypeType {
    #[strum(serialize = "data")]
    Data,
    #[strum(serialize = "sign_data")]
    SignData,
    #[strum(serialize = "sent_transaction")]
    SentTransaction,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl WidgetQueryTypeType {
    #[must_use]
    pub const fn all() -> [WidgetQueryTypeType; 4usize] {
        [
            WidgetQueryTypeType::Data,
            WidgetQueryTypeType::SignData,
            WidgetQueryTypeType::SentTransaction,
            WidgetQueryTypeType::Unknown,
        ]
    }
}
impl From<WidgetQueryTypeType> for Box<str> {
    fn from(val: WidgetQueryTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<WidgetQueryTypeType> for String {
    fn from(val: WidgetQueryTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for WidgetQueryTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a WidgetQueryType> for WidgetQueryTypeType {
    fn from(val: &'a WidgetQueryType) -> Self {
        match val {
            WidgetQueryType::Data(_) => WidgetQueryTypeType::Data,
            WidgetQueryType::SignData(_) => WidgetQueryTypeType::SignData,
            WidgetQueryType::SentTransaction(_) => WidgetQueryTypeType::SentTransaction,
            WidgetQueryType::Unknown(_) => WidgetQueryTypeType::Unknown,
        }
    }
}
impl From<WidgetQueryType> for WidgetQueryTypeType {
    fn from(val: WidgetQueryType) -> Self {
        WidgetQueryTypeType::from(&val)
    }
}
