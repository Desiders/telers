use super::{
    Animation, Audio, Chat, Contact, Dice, Document, ForumTopicClosed, ForumTopicCreated,
    ForumTopicReopened, Game, InlineKeyboardMarkup, Invoice, Location,
    MessageAutoDeleteTimerChanged, MessageEntity, PassportData, PhotoSize, Poll,
    ProximityAlertTriggered, Sticker, SuccessfulPayment, Update, User, Venue, Video,
    VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted, VideoNote,
    Voice, WebAppData,
};

use serde::{Deserialize, Serialize};

/// This object represents a message.
/// <https://core.telegram.org/bots/api#message>
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,
    /// *Optional*. Unique identifier of a message thread to which the message belongs; for supergroups only
    pub message_thread_id: Option<i64>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Box<Chat>,
    /// *Optional*. Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// *Optional*. Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Box<Chat>>,
    /// *Optional*. For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// *Optional*. For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    pub forward_from_chat: Option<Box<Chat>>,
    /// *Optional*. For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i64>,
    /// *Optional*. For forwarded messages that were originally sent in channels or by an anonymous chat administrator, signature of the message sender if present
    pub forward_signature: Option<String>,
    /// *Optional*. Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    pub forward_sender_name: Option<String>,
    /// *Optional*. For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<i64>,
    /// *Optional*. `True`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// *Optional*. `True`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// *Optional*. For replies, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// *Optional*. Bot through which the message was sent
    pub via_bot: Option<User>,
    /// *Optional*. Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// *Optional*. `True`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// *Optional*. The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// *Optional*. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,
    /// *Optional*. For text messages, the actual UTF-8 text of the message
    pub text: Option<String>,
    /// *Optional*. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set
    pub animation: Option<Animation>,
    /// *Optional*. Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// *Optional*. Message is a general file, information about the file
    pub document: Option<Document>,
    /// *Optional*. Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// *Optional*. Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// *Optional*. Message is a video, information about the video
    pub video: Option<Video>,
    /// *Optional*. Message is a `video note <https://telegram.org/blog/video-messages-and-telescope>`, information about the video message
    pub video_note: Option<VideoNote>,
    /// *Optional*. Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// *Optional*. Caption for the animation, audio, document, photo, video or voice
    pub caption: Option<String>,
    /// *Optional*. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// *Optional*. Message is a dice with random value
    pub dice: Option<Dice>,
    /// *Optional*. Message is a game, information about the game. `More about games » <https://core.telegram.org/bots/api#games>`
    pub game: Option<Game>,
    /// *Optional*. Message is a native poll, information about the poll
    pub poll: Option<Poll>,
    /// *Optional*. Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set
    pub venue: Option<Venue>,
    /// *Optional*. Message is a shared location, information about the location
    pub location: Option<Location>,
    /// *Optional*. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// *Optional*. A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// *Optional*. A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// *Optional*. A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// *Optional*. Service message: the chat photo was deleted
    pub delete_chat_photo: Option<bool>,
    /// *Optional*. Service message: the group has been created
    pub group_chat_created: Option<bool>,
    /// *Optional*. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// *Optional*. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    pub channel_chat_created: Option<bool>,
    /// *Optional*. Service message: auto-delete timer settings changed in the chat
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    /// *Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// *Optional*. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// *Optional*. Specified message was pinned. Note that the Message object in this field will not contain further *reply_to_message* fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// *Optional*. Message is an invoice for a `payment <https://core.telegram.org/bots/api#payments>`, information about the invoice. `More about payments » <https://core.telegram.org/bots/api#payments>`
    pub invoice: Option<Invoice>,
    /// *Optional*. Message is a service message about a successful payment, information about the payment. `More about payments » <https://core.telegram.org/bots/api#payments>`
    pub successful_payment: Option<SuccessfulPayment>,
    /// *Optional*. The domain name of the website on which the user has logged in. `More about Telegram Login » <https://core.telegram.org/widgets/login>`
    pub connected_website: Option<String>,
    /// *Optional*. Telegram Passport data
    pub passport_data: Option<PassportData>,
    /// *Optional*. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// *Optional*. Service message: forum topic created
    pub forum_topic_created: Option<ForumTopicCreated>,
    /// *Optional*. Service message: forum topic closed
    pub forum_topic_closed: Option<ForumTopicClosed>,
    /// *Optional*. Service message: forum topic reopened
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    /// *Optional*. Service message: video chat scheduled
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    /// Optional*. Service message: video chat started
    pub video_chat_started: Option<VideoChatStarted>,
    /// *Optional*. Service message: video chat ended
    pub video_chat_ended: Option<VideoChatEnded>,
    /// *Optional*. Service message: new participants invited to a video chat
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    /// *Optional*. Service message: data sent by a Web App
    pub web_app_data: Option<WebAppData>,
    /// *Optional*. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl Message {
    /// Alias for `message_id`
    #[must_use]
    pub fn id(&self) -> i64 {
        self.message_id
    }

    #[must_use]
    pub fn get_text_or_caption(&self) -> Option<&String> {
        if let Some(ref text) = self.text {
            Some(text)
        } else if let Some(ref caption) = self.caption {
            Some(caption)
        } else {
            None
        }
    }
}

impl From<Update> for Message {
    fn from(update: Update) -> Self {
        update
            .message
            .or(update.edited_message)
            .or(update.channel_post)
            .or(update.edited_channel_post)
            .expect("Update doesn't contain a `Message`")
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Message;

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "message_id": 1,
            "date": 1,
            "chat": {
                "id": 1,
                "type": "private"
            },
            "from": {
                "id": 1,
                "is_bot": false,
                "first_name": "test"
            },
            "sender_chat": {
                "id": 1,
                "type": "private"
            },
            "forward_from": {
                "id": 1,
                "is_bot": false,
                "first_name": "test"
            },
            "forward_from_chat": {
                "id": 1,
                "type": "private"
            },
            "forward_from_message_id": 1,
            "forward_signature": "test",
            "forward_sender_name": "test",
            "forward_date": 1,
            "is_automatic_forward": true,
            "reply_to_message": {
                "message_id": 1,
                "date": 1,
                "chat": {
                    "id": 1,
                    "type": "private"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test"
                }
            },
            "via_bot": {
                "id": 1,
                "is_bot": false,
                "first_name": "test"
            },
            "edit_date": 1,
            "has_protected_content": true,
            "media_group_id": "test",
            "author_signature": "test",
            "text": "test",
            "entities": [
                {
                    "type": "mention",
                    "offset": 0,
                    "length": 1
                }
            ],
            "animation": {
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
                "duration": 1,
                "thumb": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                },
                "file_name": "test",
                "mime_type": "test",
                "file_size": 1
            },
            "audio": {
                "file_id": "test",
                "file_unique_id": "test",
                "duration": 1,
                "performer": "test",
                "title": "test",
                "file_name": "test",
                "mime_type": "test",
                "file_size": 1,
                "thumb": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                }
            },
            "document": {
                "file_id": "test",
                "file_unique_id": "test",
                "thumb": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                },
                "file_name": "test",
                "mime_type": "test",
                "file_size": 1
            },
            "photo": [
                {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                }
            ],
            "sticker": {
                "file_id": "test",
                "file_unique_id": "test",
                "type": "test",
                "width": 1,
                "height": 1,
                "is_animated": true,
                "is_video": true,
                "thumb": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                },
                "emoji": "test",
                "set_name": "test",
                "mask_position": {
                    "point": "forehead",
                    "x_shift": 1.0,
                    "y_shift": 1.0,
                    "scale": 1.0
                },
                "file_size": 1
            },
            "video": {
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
                "duration": 1,
                "thumb": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                },
                "file_name": "test",
                "mime_type": "test",
                "file_size": 1
            },
            "video_note": {
                "file_id": "test",
                "file_unique_id": "test",
                "length": 1,
                "duration": 1,
                "thumb": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                },
                "file_size": 1
            },
            "voice": {
                "file_id": "test",
                "file_unique_id": "test",
                "duration": 1,
                "mime_type": "test",
                "file_size": 1
            },
            "caption": "test",
            "caption_entities": [
                {
                    "type": "mention",
                    "offset": 0,
                    "length": 1
                }
            ],
            "contact": {
                "phone_number": "test",
                "first_name": "test",
                "last_name": "test",
                "user_id": 1,
                "vcard": "test"
            },
            "dice": {
                "value": 1,
                "emoji": "test"
            },
            "game": {
                "title": "test",
                "description": "test",
                "photo": [
                    {
                        "file_id": "test",
                        "file_unique_id": "test",
                        "width": 1,
                        "height": 1,
                        "file_size": 1
                    }
                ],
                "text": "test",
                "text_entities": [
                    {
                        "type": "mention",
                        "offset": 0,
                        "length": 1
                    }
                ],
                "animation": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "duration": 1,
                    "thumb": {
                        "file_id": "test",
                        "file_unique_id": "test",
                        "width": 1,
                        "height": 1,
                        "file_size": 1
                    },
                    "file_name": "test",
                    "mime_type": "test",
                    "file_size": 1
                }
            },
            "poll": {
                "id": "test",
                "question": "test",
                "options": [
                    {
                        "text": "test",
                        "voter_count": 1
                    }
                ],
                "total_voter_count": 1,
                "is_closed": true,
                "is_anonymous": true,
                "type": "regular",
                "allows_multiple_answers": true,
                "correct_option_id": 1,
                "explanation": "test",
                "explanation_entities": [
                    {
                        "type": "mention",
                        "offset": 0,
                        "length": 1
                    }
                ],
                "open_period": 1,
                "close_date": 1
            },
            "venue": {
                "location": {
                    "latitude": 1.0,
                    "longitude": 1.0,
                    "horizontal_accuracy": 1.0,
                    "live_period": 1,
                    "heading": 1,
                    "proximity_alert_radius": 1
                },
                "title": "test",
                "address": "test",
                "foursquare_id": "test",
                "foursquare_type": "test",
                "google_place_id": "test",
                "google_place_type": "test"
            },
            "location": {
                "latitude": 1.0,
                "longitude": 1.0,
                "horizontal_accuracy": 1.0,
                "live_period": 1,
                "heading": 1,
                "proximity_alert_radius": 1
            },
            "new_chat_members": [
                {
                    "id": 1,
                    "is_bot": true,
                    "first_name": "test",
                    "last_name": "test",
                    "username": "test",
                    "language_code": "test",
                    "can_join_groups": true,
                    "can_read_all_group_messages": true,
                    "supports_inline_queries": true
                }
            ],
            "left_chat_member": {
                "id": 1,
                "is_bot": true,
                "first_name": "test",
                "last_name": "test",
                "username": "test",
                "language_code": "test",
                "can_join_groups": true,
                "can_read_all_group_messages": true,
                "supports_inline_queries": true
            },
            "new_chat_title": "test",
            "new_chat_photo": [
                {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "file_size": 1
                }
            ],
            "delete_chat_photo": true,
            "group_chat_created": true,
            "supergroup_chat_created": true,
            "channel_chat_created": true,
            "migrate_to_chat_id": 1,
            "migrate_from_chat_id": 1,
            "pinned_message": {
                "message_id": 1,
                "date": 1,
                "chat": {
                    "id": 1,
                    "type": "private"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test"
                },
                "sender_chat": {
                    "id": 1,
                    "type": "private"
                },
                "forward_from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test"
                },
                "forward_from_chat": {
                    "id": 1,
                    "type": "private"
                },
                "forward_from_message_id": 1,
                "forward_signature": "test",
                "forward_sender_name": "test",
                "forward_date": 1,
                "is_automatic_forward": true
            },
            "invoice": {
                "title": "test",
                "description": "test",
                "start_parameter": "test",
                "currency": "test",
                "total_amount": 1
            },
            "successful_payment": {
                "currency": "test",
                "total_amount": 1,
                "invoice_payload": "test",
                "shipping_option_id": "test",
                "order_info": {
                    "name": "test",
                    "phone_number": "test",
                    "email": "test",
                    "shipping_address": {
                        "country_code": "test",
                        "state": "test",
                        "city": "test",
                        "street_line1": "test",
                        "street_line2": "test",
                        "post_code": "test"
                    }
                },
                "telegram_payment_charge_id": "test",
                "provider_payment_charge_id": "test"
            },
            "connected_website": "test",
            "passport_data": {
                "data": [
                    {
                        "type": "test",
                        "hash": "test"
                    }
                ],
                "credentials": {
                    "data": "test",
                    "hash": "test",
                    "secret": "test"
                }
            },
            "proximity_alert_triggered": {
                "traveler": {
                    "id": 1,
                    "is_bot": true,
                    "first_name": "test",
                    "last_name": "test",
                    "username": "test",
                    "language_code": "test",
                    "can_join_groups": true,
                    "can_read_all_group_messages": true,
                    "supports_inline_queries": true
                },
                "watcher": {
                    "id": 1,
                    "is_bot": true,
                    "first_name": "test",
                    "last_name": "test",
                    "username": "test",
                    "language_code": "test",
                    "can_join_groups": true,
                    "can_read_all_group_messages": true,
                    "supports_inline_queries": true
                },
                "distance": 1
            },
            "voice_chat_scheduled": {
                "start_date": 1
            },
            "voice_chat_started": {},
            "voice_chat_ended": {
                "duration": 1
            },
            "voice_chat_participants_invited": {
                "users": [
                    {
                        "id": 1,
                        "is_bot": true,
                        "first_name": "test",
                        "last_name": "test",
                        "username": "test",
                        "language_code": "test",
                        "can_join_groups": true,
                        "can_read_all_group_messages": true,
                        "supports_inline_queries": true
                    }
                ]
            },
            "web_app_data": {
                "data": "test",
                "button_text": "test"
            },
            "reply_markup": {
                "inline_keyboard": [
                    [
                        {
                            "text": "test",
                            "url": "test",
                            "login_url": {
                                "url": "test",
                                "forward_text": "test",
                                "bot_username": "test",
                                "request_write_access": true
                            },
                            "callback_data": "test",
                            "switch_inline_query": "test",
                            "switch_inline_query_current_chat": "test",
                            "callback_game": {
                                "name": "test",
                                "description": "test"
                            },
                            "pay": true
                        }
                    ]
                ],
                "keyboard": [
                    [
                        {
                            "text": "test",
                            "request_contact": true,
                            "request_location": true,
                            "request_poll": {
                                "type": "test"
                            }
                        }
                    ]
                ],
                "resize_keyboard": true,
                "one_time_keyboard": true,
                "selective": true,
                "force_reply": true
            }
        }"#;
        let deserializer = &mut serde_json::Deserializer::from_str(json);
        let result: Result<Message, _> = serde_path_to_error::deserialize(deserializer);

        if let Err(err) = result {
            println!("Path: {}", err.path());

            let _: Message = serde_json::from_str(json).unwrap(); // for traceback
        }
    }
}
