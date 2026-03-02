use crate::types::BackgroundType;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the type of a background. Currently, it can be one of
/// - [`crate::types::BackgroundTypeFill`]
/// - [`crate::types::BackgroundTypeWallpaper`]
/// - [`crate::types::BackgroundTypePattern`]
/// - [`crate::types::BackgroundTypeChatTheme`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtype>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum BackgroundTypeType {
    #[strum(serialize = "fill")]
    Fill,
    #[strum(serialize = "wallpaper")]
    Wallpaper,
    #[strum(serialize = "pattern")]
    Pattern,
    #[strum(serialize = "chat_theme")]
    ChatTheme,
}
impl BackgroundTypeType {
    #[must_use]
    pub const fn all() -> [BackgroundTypeType; 4usize] {
        [
            BackgroundTypeType::Fill,
            BackgroundTypeType::Wallpaper,
            BackgroundTypeType::Pattern,
            BackgroundTypeType::ChatTheme,
        ]
    }
}
impl From<BackgroundTypeType> for Box<str> {
    fn from(val: BackgroundTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<BackgroundTypeType> for String {
    fn from(val: BackgroundTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for BackgroundTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a BackgroundType> for BackgroundTypeType {
    fn from(val: &'a BackgroundType) -> Self {
        match val {
            BackgroundType::Fill(_) => BackgroundTypeType::Fill,
            BackgroundType::Wallpaper(_) => BackgroundTypeType::Wallpaper,
            BackgroundType::Pattern(_) => BackgroundTypeType::Pattern,
            BackgroundType::ChatTheme(_) => BackgroundTypeType::ChatTheme,
        }
    }
}
impl From<BackgroundType> for BackgroundTypeType {
    fn from(val: BackgroundType) -> Self {
        BackgroundTypeType::from(&val)
    }
}
