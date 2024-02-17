use super::{Chat, LinkPreviewOptions, MessageOrigin};

use crate::types;

use serde::Deserialize;

/// This object contains information about a message that is being replied to, which may come from another chat or forum topic
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ExternalReplyInfo {
    Animation(Animation),
    Audio(Audio),
    Document(Document),
    Photo(Photo),
    Sticker(Sticker),
    Story(Story),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
    Contact(Contact),
    Dice(Dice),
    Game(Game),
    Giveaway(Giveaway),
    GiveawayWinners(GiveawayWinners),
    Invoice(Invoice),
    Venue(Venue),
    Location(Location),
    Poll(Poll),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Story {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a forwarded story
    pub story: types::Story,
    /// `true`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Video {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    pub message_id: Option<i64>,
    /// Message is a video, information about the video
    pub video: types::Video,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
            let external_reply_info_kind: Text = serde_json::from_value(json.clone()).unwrap();
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
            "story": {},
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
