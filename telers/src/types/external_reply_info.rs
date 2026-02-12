use super::{Chat, LinkPreviewOptions, MessageOrigin};

use crate::types;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object contains information about a message that is being replied to, which may come from another chat or forum topic
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExternalReplyInfo {
    Animation(Box<Animation>),
    Audio(Box<Audio>),
    Document(Box<Document>),
    PaidMedia(Box<PaidMedia>),
    Photo(Box<Photo>),
    Sticker(Box<Sticker>),
    Story(Box<Story>),
    Video(Box<Video>),
    VideoNote(Box<VideoNote>),
    Voice(Box<Voice>),
    Checklist(Box<Checklist>),
    Contact(Box<Contact>),
    Dice(Box<Dice>),
    Game(Box<Game>),
    Giveaway(Box<Giveaway>),
    GiveawayWinners(Box<GiveawayWinners>),
    Invoice(Box<Invoice>),
    Venue(Box<Venue>),
    Location(Box<Location>),
    Poll(Box<Poll>),
    Text(Box<Text>),
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Text {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Options used for link preview generation for the original message
    pub link_preview_options: Option<LinkPreviewOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Animation {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    pub animation: types::Animation,
    /// `true`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Audio {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is an audio file, information about the file
    pub audio: types::Audio,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Document {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a general file, information about the file
    pub document: types::Document,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaidMedia {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message contains paid media; information about the paid media
    pub paid_media: types::PaidMedia,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Photo {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a photo, available sizes of the photo
    pub photo: Vec<types::PhotoSize>,
    /// `true`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sticker {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a sticker, information about the sticker
    pub sticker: types::Sticker,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Story {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a forwarded story
    pub story: types::Story,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Video {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a video, information about the video
    pub video: types::Video,
    /// `true`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VideoNote {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message
    pub video_note: types::VideoNote,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Voice {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a voice message, information about the file
    pub voice: types::Voice,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Checklist {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a checklist
    pub checklist: types::Checklist,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Contact {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a shared contact, information about the contact
    pub contact: types::Contact,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Dice {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a dice with random value
    pub dice: types::Dice,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Game {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a game, information about the game. [More about games](https://core.telegram.org/bots/api#games)
    pub game: types::Game,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Giveaway {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a scheduled giveaway, information about the giveaway
    pub giveaway: types::Giveaway,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GiveawayWinners {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// A giveaway with public winners was completed
    pub giveaway_winners: types::GiveawayWinners,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Invoice {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is an invoice for a [payment](https://core.telegram.org/bots/api#payments), information about the invoice. [More about payments](https://core.telegram.org/bots/api#payments)
    pub invoice: types::Invoice,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Location {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a shared location, information about the location
    pub location: types::Location,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Poll {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a native poll, information about the poll
    pub poll: types::Poll,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Venue {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a venue, information about the venue
    pub venue: types::Venue,
}

impl ExternalReplyInfo {
    #[must_use]
    pub const fn origin(&self) -> &MessageOrigin {
        match self {
            ExternalReplyInfo::Animation(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Audio(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Document(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::PaidMedia(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Photo(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Sticker(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Story(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Video(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::VideoNote(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Voice(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Checklist(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Contact(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Dice(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Game(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Giveaway(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::GiveawayWinners(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Invoice(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Venue(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Location(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Poll(external_reply_info) => &external_reply_info.origin,
            ExternalReplyInfo::Text(external_reply_info) => &external_reply_info.origin,
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn chat(&self) -> Option<&Chat> {
        match self {
            ExternalReplyInfo::Animation(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Audio(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Document(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::PaidMedia(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Photo(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Sticker(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Story(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Video(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::VideoNote(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Voice(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Checklist(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Contact(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Dice(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Game(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Giveaway(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::GiveawayWinners(external_reply_info) => {
                match external_reply_info.chat {
                    Some(ref chat) => Some(chat),
                    None => None,
                }
            }
            ExternalReplyInfo::Invoice(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Venue(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Location(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Poll(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
            ExternalReplyInfo::Text(external_reply_info) => match external_reply_info.chat {
                Some(ref chat) => Some(chat),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn message_id(&self) -> Option<i64> {
        match self {
            ExternalReplyInfo::Animation(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Audio(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Document(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::PaidMedia(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Photo(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Sticker(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Story(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Video(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::VideoNote(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Voice(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Checklist(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Contact(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Dice(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Game(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Giveaway(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::GiveawayWinners(external_reply_info) => {
                external_reply_info.message_id
            }
            ExternalReplyInfo::Invoice(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Venue(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Location(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Poll(external_reply_info) => external_reply_info.message_id,
            ExternalReplyInfo::Text(external_reply_info) => external_reply_info.message_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_text() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Text(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_animation() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "animation": {
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
                "duration": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Animation(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_audio() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "audio": {
                "file_id": "test",
                "file_unique_id": "test",
                "duration": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Audio(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_document() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "document": {
                "file_id": "test",
                "file_unique_id": "test",
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Document(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_paid_media() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "paid_media": {
                "type": "preview",
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::PaidMedia(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_photo() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "photo": [{
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
            }],
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Photo(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_sticker() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "sticker": {
                "file_id": "test",
                "file_unique_id": "test",
                "type": "regular",
                "width": 1,
                "height": 1,
                "is_animated": false,
                "is_video": false,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Sticker(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_story() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "story": {
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "id": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Story(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "video": {
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
                "duration": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Video(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video_note() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "video_note": {
                "file_id": "test",
                "file_unique_id": "test",
                "length": 1,
                "duration": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::VideoNote(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_voice() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "voice": {
                "file_id": "test",
                "file_unique_id": "test",
                "duration": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Voice(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_checklist() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "checklist": {
                "title": "test",
                "tasks": [
                    {
                        "id": 1,
                        "text": "test",
                    }
                ],
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Checklist(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_contact() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "contact": {
                "phone_number": "test",
                "first_name": "test",
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Contact(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_dice() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "dice": {
                "emoji": "🎲",
                "value": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Dice(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_game() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "game": {
                "title": "test",
                "description": "test",
                "photo": [{
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                }],
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Game(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_giveaway() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "giveaway": {
                "chats": [{
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                }],
                "winners_selection_date": 0,
                "winner_count": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Giveaway(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_giveaway_winners() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "giveaway_winners": {
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "giveaway_message_id": 1,
                "winners_selection_date": 0,
                "winner_count": 1,
                "winners": [{
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                }],
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::GiveawayWinners(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_invoice() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "invoice": {
                "title": "test",
                "description": "test",
                "start_parameter": "test",
                "currency": "test",
                "total_amount": 1,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Invoice(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_venue() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "venue": {
                "location": {
                    "latitude": 1.0,
                    "longitude": 1.0,
                },
                "title": "test",
                "address": "test",
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Venue(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_location() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "location": {
                "latitude": 1.0,
                "longitude": 1.0,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Location(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }

    #[test]
    fn deserialize_poll() {
        let jsons = [serde_json::json!({
            "origin": {
                "type": "user",
                "date": 0,
                "sender_user": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
            },
            "poll": {
                "id": "test",
                "question": "test",
                "options": [
                    {
                        "text": "test",
                        "voter_count": 1,
                    },
                    {
                        "text": "test",
                        "voter_count": 1,
                    },
                ],
                "total_voter_count": 2,
                "is_closed": false,
                "is_anonymous": false,
                "type": "regular",
                "allows_multiple_answers": false,
            },
        })];

        for json in jsons {
            let external_reply_info_kind = serde_json::from_value(json.clone()).unwrap();
            let external_reply_info: ExternalReplyInfo = serde_json::from_value(json).unwrap();

            match external_reply_info {
                ExternalReplyInfo::Poll(external_reply_info) => {
                    assert_eq!(external_reply_info, external_reply_info_kind)
                }
                _ => panic!("Unexpected external reply info type: {external_reply_info:?}"),
            }
        }
    }
}
