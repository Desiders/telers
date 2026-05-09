use crate::types::ExternalReplyInfo;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
/// Currently, it can be one of
/// - [`crate::types::ExternalReplyInfoAnimation`]
/// - [`crate::types::ExternalReplyInfoAudio`]
/// - [`crate::types::ExternalReplyInfoChecklist`]
/// - [`crate::types::ExternalReplyInfoContact`]
/// - [`crate::types::ExternalReplyInfoDice`]
/// - [`crate::types::ExternalReplyInfoDocument`]
/// - [`crate::types::ExternalReplyInfoGame`]
/// - [`crate::types::ExternalReplyInfoGiveaway`]
/// - [`crate::types::ExternalReplyInfoGiveawayWinners`]
/// - [`crate::types::ExternalReplyInfoInvoice`]
/// - [`crate::types::ExternalReplyInfoLivePhoto`]
/// - [`crate::types::ExternalReplyInfoLocation`]
/// - [`crate::types::ExternalReplyInfoPhoto`]
/// - [`crate::types::ExternalReplyInfoPoll`]
/// - [`crate::types::ExternalReplyInfoSticker`]
/// - [`crate::types::ExternalReplyInfoStory`]
/// - [`crate::types::ExternalReplyInfoVenue`]
/// - [`crate::types::ExternalReplyInfoVideo`]
/// - [`crate::types::ExternalReplyInfoVideoNote`]
/// - [`crate::types::ExternalReplyInfoVoice`]
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ExternalReplyInfoType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "checklist")]
    Checklist,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "dice")]
    Dice,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "giveaway")]
    Giveaway,
    #[strum(serialize = "giveaway_winners")]
    GiveawayWinners,
    #[strum(serialize = "invoice")]
    Invoice,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "story")]
    Story,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "video_note")]
    VideoNote,
    #[strum(serialize = "voice")]
    Voice,
}
impl ExternalReplyInfoType {
    #[must_use]
    pub const fn all() -> [ExternalReplyInfoType; 20usize] {
        [
            ExternalReplyInfoType::Animation,
            ExternalReplyInfoType::Audio,
            ExternalReplyInfoType::Checklist,
            ExternalReplyInfoType::Contact,
            ExternalReplyInfoType::Dice,
            ExternalReplyInfoType::Document,
            ExternalReplyInfoType::Game,
            ExternalReplyInfoType::Giveaway,
            ExternalReplyInfoType::GiveawayWinners,
            ExternalReplyInfoType::Invoice,
            ExternalReplyInfoType::LivePhoto,
            ExternalReplyInfoType::Location,
            ExternalReplyInfoType::Photo,
            ExternalReplyInfoType::Poll,
            ExternalReplyInfoType::Sticker,
            ExternalReplyInfoType::Story,
            ExternalReplyInfoType::Venue,
            ExternalReplyInfoType::Video,
            ExternalReplyInfoType::VideoNote,
            ExternalReplyInfoType::Voice,
        ]
    }
}
impl From<ExternalReplyInfoType> for Box<str> {
    fn from(val: ExternalReplyInfoType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ExternalReplyInfoType> for String {
    fn from(val: ExternalReplyInfoType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ExternalReplyInfoType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ExternalReplyInfo> for ExternalReplyInfoType {
    fn from(val: &'a ExternalReplyInfo) -> Self {
        match val {
            ExternalReplyInfo::Animation(_) => ExternalReplyInfoType::Animation,
            ExternalReplyInfo::Audio(_) => ExternalReplyInfoType::Audio,
            ExternalReplyInfo::Checklist(_) => ExternalReplyInfoType::Checklist,
            ExternalReplyInfo::Contact(_) => ExternalReplyInfoType::Contact,
            ExternalReplyInfo::Dice(_) => ExternalReplyInfoType::Dice,
            ExternalReplyInfo::Document(_) => ExternalReplyInfoType::Document,
            ExternalReplyInfo::Game(_) => ExternalReplyInfoType::Game,
            ExternalReplyInfo::Giveaway(_) => ExternalReplyInfoType::Giveaway,
            ExternalReplyInfo::GiveawayWinners(_) => ExternalReplyInfoType::GiveawayWinners,
            ExternalReplyInfo::Invoice(_) => ExternalReplyInfoType::Invoice,
            ExternalReplyInfo::LivePhoto(_) => ExternalReplyInfoType::LivePhoto,
            ExternalReplyInfo::Location(_) => ExternalReplyInfoType::Location,
            ExternalReplyInfo::Photo(_) => ExternalReplyInfoType::Photo,
            ExternalReplyInfo::Poll(_) => ExternalReplyInfoType::Poll,
            ExternalReplyInfo::Sticker(_) => ExternalReplyInfoType::Sticker,
            ExternalReplyInfo::Story(_) => ExternalReplyInfoType::Story,
            ExternalReplyInfo::Venue(_) => ExternalReplyInfoType::Venue,
            ExternalReplyInfo::Video(_) => ExternalReplyInfoType::Video,
            ExternalReplyInfo::VideoNote(_) => ExternalReplyInfoType::VideoNote,
            ExternalReplyInfo::Voice(_) => ExternalReplyInfoType::Voice,
        }
    }
}
impl From<ExternalReplyInfo> for ExternalReplyInfoType {
    fn from(val: ExternalReplyInfo) -> Self {
        ExternalReplyInfoType::from(&val)
    }
}
