use serde::{Deserialize, Serialize};
/// This object describes a message that can be inaccessible to the bot. It can be one of
/// - Message
/// - [`crate::types::InaccessibleMessage`]
/// # Documentation
/// <https://core.telegram.org/bots/api#maybeinaccessiblemessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    InaccessibleMessage(crate::types::InaccessibleMessage),
    Message(crate::types::Message),
}
impl MaybeInaccessibleMessage {
    /// Helper method for field `animation`.
    ///
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::Message(val) => crate::types::Message::animation(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `audio`.
    ///
    /// Message is an audio file, information about the file
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Message(val) => crate::types::Message::audio(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `author_signature`.
    ///
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::author_signature(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `boost_added`.
    ///
    /// Service message: user boosted the chat
    #[must_use]
    pub fn boost_added(&self) -> Option<&crate::types::ChatBoostAdded> {
        match self {
            Self::Message(val) => crate::types::Message::boost_added(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `business_connection_id`.
    ///
    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[must_use]
    pub fn business_connection_id(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::business_connection_id(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `caption`.
    ///
    /// Caption for the animation, audio, document, paid media, photo, video or voice
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::caption(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `caption_entities`.
    ///
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Message(val) => crate::types::Message::caption_entities(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `channel_chat_created`.
    ///
    /// Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in `reply_to_message` if someone replies to a very first message in a channel.
    #[must_use]
    pub fn channel_chat_created(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::channel_chat_created(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `chat`.
    ///
    /// # Variants
    /// - `InaccessibleMessage`. Chat the message belonged to
    /// - `Message`. Chat the message belongs to
    #[must_use]
    pub fn chat(&self) -> &crate::types::Chat {
        match self {
            Self::InaccessibleMessage(val) => val.chat.as_ref(),
            Self::Message(val) => crate::types::Message::chat(val),
        }
    }

    /// Helper method for field `chat_background_set`.
    ///
    /// Service message: chat background set
    #[must_use]
    pub fn chat_background_set(&self) -> Option<&crate::types::ChatBackground> {
        match self {
            Self::Message(val) => crate::types::Message::chat_background_set(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `chat_owner_changed`.
    ///
    /// Service message: chat owner has changed
    #[must_use]
    pub fn chat_owner_changed(&self) -> Option<&crate::types::ChatOwnerChanged> {
        match self {
            Self::Message(val) => crate::types::Message::chat_owner_changed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `chat_owner_left`.
    ///
    /// Service message: chat owner has left
    #[must_use]
    pub fn chat_owner_left(&self) -> Option<&crate::types::ChatOwnerLeft> {
        match self {
            Self::Message(val) => crate::types::Message::chat_owner_left(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `chat_shared`.
    ///
    /// Service message: a chat was shared with the bot
    #[must_use]
    pub fn chat_shared(&self) -> Option<&crate::types::ChatShared> {
        match self {
            Self::Message(val) => crate::types::Message::chat_shared(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `checklist`.
    ///
    /// Message is a checklist
    #[must_use]
    pub fn checklist(&self) -> Option<&crate::types::Checklist> {
        match self {
            Self::Message(val) => crate::types::Message::checklist(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `checklist_tasks_added`.
    ///
    /// Service message: tasks were added to a checklist
    #[must_use]
    pub fn checklist_tasks_added(&self) -> Option<&crate::types::ChecklistTasksAdded> {
        match self {
            Self::Message(val) => crate::types::Message::checklist_tasks_added(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `checklist_tasks_done`.
    ///
    /// Service message: some tasks in a checklist were marked as done or not done
    #[must_use]
    pub fn checklist_tasks_done(&self) -> Option<&crate::types::ChecklistTasksDone> {
        match self {
            Self::Message(val) => crate::types::Message::checklist_tasks_done(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `connected_website`.
    ///
    /// The domain name of the website on which the user has logged in. More about Telegram Login: <https://core.telegram.org/widgets/login>
    #[must_use]
    pub fn connected_website(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::connected_website(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `contact`.
    ///
    /// Message is a shared contact, information about the contact
    #[must_use]
    pub fn contact(&self) -> Option<&crate::types::Contact> {
        match self {
            Self::Message(val) => crate::types::Message::contact(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `date`.
    ///
    /// # Variants
    /// - `InaccessibleMessage`. Always 0. The field can be used to differentiate regular and inaccessible messages.
    /// - `Message`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    #[must_use]
    pub fn date(&self) -> i64 {
        match self {
            Self::InaccessibleMessage(val) => val.date,
            Self::Message(val) => crate::types::Message::date(val),
        }
    }

    /// Helper method for field `delete_chat_photo`.
    ///
    /// Service message: the chat photo was deleted
    #[must_use]
    pub fn delete_chat_photo(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::delete_chat_photo(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `dice`.
    ///
    /// Message is a dice with random value
    #[must_use]
    pub fn dice(&self) -> Option<&crate::types::Dice> {
        match self {
            Self::Message(val) => crate::types::Message::dice(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `direct_message_price_changed`.
    ///
    /// Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed
    #[must_use]
    pub fn direct_message_price_changed(&self) -> Option<&crate::types::DirectMessagePriceChanged> {
        match self {
            Self::Message(val) => crate::types::Message::direct_message_price_changed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `direct_messages_topic`.
    ///
    /// Information about the direct messages chat topic that contains the message
    #[must_use]
    pub fn direct_messages_topic(&self) -> Option<&crate::types::DirectMessagesTopic> {
        match self {
            Self::Message(val) => crate::types::Message::direct_messages_topic(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `document`.
    ///
    /// Message is a general file, information about the file
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::Message(val) => crate::types::Message::document(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `edit_date`.
    ///
    /// Date the message was last edited in Unix time
    #[must_use]
    pub fn edit_date(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::edit_date(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `effect_id`.
    ///
    /// Unique identifier of the message effect added to the message
    #[must_use]
    pub fn effect_id(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::effect_id(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `entities`.
    ///
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Message(val) => crate::types::Message::entities(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `external_reply`.
    ///
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    #[must_use]
    pub fn external_reply(&self) -> Option<&crate::types::ExternalReplyInfo> {
        match self {
            Self::Message(val) => crate::types::Message::external_reply(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `forum_topic_closed`.
    ///
    /// Service message: forum topic closed
    #[must_use]
    pub fn forum_topic_closed(&self) -> Option<&crate::types::ForumTopicClosed> {
        match self {
            Self::Message(val) => crate::types::Message::forum_topic_closed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `forum_topic_created`.
    ///
    /// Service message: forum topic created
    #[must_use]
    pub fn forum_topic_created(&self) -> Option<&crate::types::ForumTopicCreated> {
        match self {
            Self::Message(val) => crate::types::Message::forum_topic_created(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `forum_topic_edited`.
    ///
    /// Service message: forum topic edited
    #[must_use]
    pub fn forum_topic_edited(&self) -> Option<&crate::types::ForumTopicEdited> {
        match self {
            Self::Message(val) => crate::types::Message::forum_topic_edited(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `forum_topic_reopened`.
    ///
    /// Service message: forum topic reopened
    #[must_use]
    pub fn forum_topic_reopened(&self) -> Option<&crate::types::ForumTopicReopened> {
        match self {
            Self::Message(val) => crate::types::Message::forum_topic_reopened(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `forward_origin`.
    ///
    /// Information about the original message for forwarded messages
    #[must_use]
    pub fn forward_origin(&self) -> Option<&crate::types::MessageOrigin> {
        match self {
            Self::Message(val) => crate::types::Message::forward_origin(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `from`.
    ///
    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    #[must_use]
    pub fn from(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::from(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `game`.
    ///
    /// Message is a game, information about the game. More about games: <https://core.telegram.org/bots/api#games>
    #[must_use]
    pub fn game(&self) -> Option<&crate::types::Game> {
        match self {
            Self::Message(val) => crate::types::Message::game(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `general_forum_topic_hidden`.
    ///
    /// Service message: the 'General' forum topic hidden
    #[must_use]
    pub fn general_forum_topic_hidden(&self) -> Option<&crate::types::GeneralForumTopicHidden> {
        match self {
            Self::Message(val) => crate::types::Message::general_forum_topic_hidden(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `general_forum_topic_unhidden`.
    ///
    /// Service message: the 'General' forum topic unhidden
    #[must_use]
    pub fn general_forum_topic_unhidden(&self) -> Option<&crate::types::GeneralForumTopicUnhidden> {
        match self {
            Self::Message(val) => crate::types::Message::general_forum_topic_unhidden(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `gift`.
    ///
    /// Service message: a regular gift was sent or received
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::Message(val) => crate::types::Message::gift(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `gift_upgrade_sent`.
    ///
    /// Service message: upgrade of a gift was purchased after the gift was sent
    #[must_use]
    pub fn gift_upgrade_sent(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::Message(val) => crate::types::Message::gift_upgrade_sent(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `giveaway`.
    ///
    /// The message is a scheduled giveaway message
    #[must_use]
    pub fn giveaway(&self) -> Option<&crate::types::Giveaway> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `giveaway_completed`.
    ///
    /// Service message: a giveaway without public winners was completed
    #[must_use]
    pub fn giveaway_completed(&self) -> Option<&crate::types::GiveawayCompleted> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_completed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `giveaway_created`.
    ///
    /// Service message: a scheduled giveaway was created
    #[must_use]
    pub fn giveaway_created(&self) -> Option<&crate::types::GiveawayCreated> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_created(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `giveaway_winners`.
    ///
    /// A giveaway with public winners was completed
    #[must_use]
    pub fn giveaway_winners(&self) -> Option<&crate::types::GiveawayWinners> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_winners(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `group_chat_created`.
    ///
    /// Service message: the group has been created
    #[must_use]
    pub fn group_chat_created(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::group_chat_created(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `has_media_spoiler`.
    ///
    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::has_media_spoiler(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `has_protected_content`.
    ///
    /// `true`, if the message can't be forwarded
    #[must_use]
    pub fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::has_protected_content(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `invoice`.
    ///
    /// Message is an invoice for a payment, information about the invoice. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn invoice(&self) -> Option<&crate::types::Invoice> {
        match self {
            Self::Message(val) => crate::types::Message::invoice(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `is_automatic_forward`.
    ///
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[must_use]
    pub fn is_automatic_forward(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::is_automatic_forward(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `is_from_offline`.
    ///
    /// `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[must_use]
    pub fn is_from_offline(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::is_from_offline(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `is_paid_post`.
    ///
    /// `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[must_use]
    pub fn is_paid_post(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::is_paid_post(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `is_topic_message`.
    ///
    /// `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[must_use]
    pub fn is_topic_message(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::is_topic_message(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `left_chat_member`.
    ///
    /// A member was removed from the group, information about them (this member may be the bot itself)
    #[must_use]
    pub fn left_chat_member(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::left_chat_member(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `link_preview_options`.
    ///
    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::Message(val) => crate::types::Message::link_preview_options(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `location`.
    ///
    /// Message is a shared location, information about the location
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::Location> {
        match self {
            Self::Message(val) => crate::types::Message::location(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `media_group_id`.
    ///
    /// The unique identifier inside this chat of a media message group this message belongs to
    #[must_use]
    pub fn media_group_id(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::media_group_id(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `message_auto_delete_timer_changed`.
    ///
    /// Service message: auto-delete timer settings changed in the chat
    #[must_use]
    pub fn message_auto_delete_timer_changed(
        &self,
    ) -> Option<&crate::types::MessageAutoDeleteTimerChanged> {
        match self {
            Self::Message(val) => crate::types::Message::message_auto_delete_timer_changed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `message_id`.
    ///
    /// # Variants
    /// - `InaccessibleMessage`. Unique message identifier inside the chat
    /// - `Message`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    #[must_use]
    pub fn message_id(&self) -> i64 {
        match self {
            Self::InaccessibleMessage(val) => val.message_id,
            Self::Message(val) => crate::types::Message::message_id(val),
        }
    }

    /// Helper method for field `message_thread_id`.
    ///
    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[must_use]
    pub fn message_thread_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::message_thread_id(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `migrate_from_chat_id`.
    ///
    /// The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_from_chat_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::migrate_from_chat_id(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `migrate_to_chat_id`.
    ///
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::migrate_to_chat_id(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `new_chat_members`.
    ///
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[must_use]
    pub fn new_chat_members(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::Message(val) => crate::types::Message::new_chat_members(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `new_chat_photo`.
    ///
    /// A chat photo was change to this value
    #[must_use]
    pub fn new_chat_photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Message(val) => crate::types::Message::new_chat_photo(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `new_chat_title`.
    ///
    /// A chat title was changed to this value
    #[must_use]
    pub fn new_chat_title(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::new_chat_title(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `paid_media`.
    ///
    /// Message contains paid media; information about the paid media
    #[must_use]
    pub fn paid_media(&self) -> Option<&crate::types::PaidMediaInfo> {
        match self {
            Self::Message(val) => crate::types::Message::paid_media(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `paid_message_price_changed`.
    ///
    /// Service message: the price for paid messages has changed in the chat
    #[must_use]
    pub fn paid_message_price_changed(&self) -> Option<&crate::types::PaidMessagePriceChanged> {
        match self {
            Self::Message(val) => crate::types::Message::paid_message_price_changed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `paid_star_count`.
    ///
    /// The number of Telegram Stars that were paid by the sender of the message to send it
    #[must_use]
    pub fn paid_star_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::paid_star_count(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `passport_data`.
    ///
    /// Telegram Passport data
    #[must_use]
    pub fn passport_data(&self) -> Option<&crate::types::PassportData> {
        match self {
            Self::Message(val) => crate::types::Message::passport_data(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// Message is a photo, available sizes of the photo
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Message(val) => crate::types::Message::photo(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `pinned_message`.
    ///
    /// Specified message was pinned. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn pinned_message(&self) -> Option<&crate::types::MaybeInaccessibleMessage> {
        match self {
            Self::Message(val) => crate::types::Message::pinned_message(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `poll`.
    ///
    /// Message is a native poll, information about the poll
    #[must_use]
    pub fn poll(&self) -> Option<&crate::types::Poll> {
        match self {
            Self::Message(val) => crate::types::Message::poll(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `proximity_alert_triggered`.
    ///
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[must_use]
    pub fn proximity_alert_triggered(&self) -> Option<&crate::types::ProximityAlertTriggered> {
        match self {
            Self::Message(val) => crate::types::Message::proximity_alert_triggered(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `quote`.
    ///
    /// For replies that quote part of the original message, the quoted part of the message
    #[must_use]
    pub fn quote(&self) -> Option<&crate::types::TextQuote> {
        match self {
            Self::Message(val) => crate::types::Message::quote(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `refunded_payment`.
    ///
    /// Message is a service message about a refunded payment, information about the payment. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn refunded_payment(&self) -> Option<&crate::types::RefundedPayment> {
        match self {
            Self::Message(val) => crate::types::Message::refunded_payment(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `reply_markup`.
    ///
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::Message(val) => crate::types::Message::reply_markup(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `reply_to_checklist_task_id`.
    ///
    /// Identifier of the specific checklist task that is being replied to
    #[must_use]
    pub fn reply_to_checklist_task_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::reply_to_checklist_task_id(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `reply_to_message`.
    ///
    /// For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn reply_to_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Message(val) => crate::types::Message::reply_to_message(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `reply_to_story`.
    ///
    /// For replies to a story, the original story
    #[must_use]
    pub fn reply_to_story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Message(val) => crate::types::Message::reply_to_story(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `sender_boost_count`.
    ///
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    #[must_use]
    pub fn sender_boost_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::sender_boost_count(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `sender_business_bot`.
    ///
    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[must_use]
    pub fn sender_business_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::sender_business_bot(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `sender_chat`.
    ///
    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Message(val) => crate::types::Message::sender_chat(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `sender_tag`.
    ///
    /// Tag or custom title of the sender of the message; for supergroups only
    #[must_use]
    pub fn sender_tag(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::sender_tag(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `show_caption_above_media`.
    ///
    /// `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::show_caption_above_media(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `sticker`.
    ///
    /// Message is a sticker, information about the sticker
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::Message(val) => crate::types::Message::sticker(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `story`.
    ///
    /// Message is a forwarded story
    #[must_use]
    pub fn story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Message(val) => crate::types::Message::story(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `successful_payment`.
    ///
    /// Message is a service message about a successful payment, information about the payment. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn successful_payment(&self) -> Option<&crate::types::SuccessfulPayment> {
        match self {
            Self::Message(val) => crate::types::Message::successful_payment(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `suggested_post_approval_failed`.
    ///
    /// Service message: approval of a suggested post has failed
    #[must_use]
    pub fn suggested_post_approval_failed(
        &self,
    ) -> Option<&crate::types::SuggestedPostApprovalFailed> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_approval_failed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `suggested_post_approved`.
    ///
    /// Service message: a suggested post was approved
    #[must_use]
    pub fn suggested_post_approved(&self) -> Option<&crate::types::SuggestedPostApproved> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_approved(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `suggested_post_declined`.
    ///
    /// Service message: a suggested post was declined
    #[must_use]
    pub fn suggested_post_declined(&self) -> Option<&crate::types::SuggestedPostDeclined> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_declined(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `suggested_post_info`.
    ///
    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[must_use]
    pub fn suggested_post_info(&self) -> Option<&crate::types::SuggestedPostInfo> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_info(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `suggested_post_paid`.
    ///
    /// Service message: payment for a suggested post was received
    #[must_use]
    pub fn suggested_post_paid(&self) -> Option<&crate::types::SuggestedPostPaid> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_paid(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `suggested_post_refunded`.
    ///
    /// Service message: payment for a suggested post was refunded
    #[must_use]
    pub fn suggested_post_refunded(&self) -> Option<&crate::types::SuggestedPostRefunded> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_refunded(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `supergroup_chat_created`.
    ///
    /// Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in `reply_to_message` if someone replies to a very first message in a directly created supergroup.
    #[must_use]
    pub fn supergroup_chat_created(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::supergroup_chat_created(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `text`.
    ///
    /// For text messages, the actual UTF-8 text of the message
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::text(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `unique_gift`.
    ///
    /// Service message: a unique gift was sent or received
    #[must_use]
    pub fn unique_gift(&self) -> Option<&crate::types::UniqueGiftInfo> {
        match self {
            Self::Message(val) => crate::types::Message::unique_gift(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `users_shared`.
    ///
    /// Service message: users were shared with the bot
    #[must_use]
    pub fn users_shared(&self) -> Option<&crate::types::UsersShared> {
        match self {
            Self::Message(val) => crate::types::Message::users_shared(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `venue`.
    ///
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        match self {
            Self::Message(val) => crate::types::Message::venue(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `via_bot`.
    ///
    /// Bot through which the message was sent
    #[must_use]
    pub fn via_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::via_bot(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `video`.
    ///
    /// Message is a video, information about the video
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::Message(val) => crate::types::Message::video(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `video_chat_ended`.
    ///
    /// Service message: video chat ended
    #[must_use]
    pub fn video_chat_ended(&self) -> Option<&crate::types::VideoChatEnded> {
        match self {
            Self::Message(val) => crate::types::Message::video_chat_ended(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `video_chat_participants_invited`.
    ///
    /// Service message: new participants invited to a video chat
    #[must_use]
    pub fn video_chat_participants_invited(
        &self,
    ) -> Option<&crate::types::VideoChatParticipantsInvited> {
        match self {
            Self::Message(val) => crate::types::Message::video_chat_participants_invited(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `video_chat_scheduled`.
    ///
    /// Service message: video chat scheduled
    #[must_use]
    pub fn video_chat_scheduled(&self) -> Option<&crate::types::VideoChatScheduled> {
        match self {
            Self::Message(val) => crate::types::Message::video_chat_scheduled(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `video_chat_started`.
    ///
    /// Service message: video chat started
    #[must_use]
    pub fn video_chat_started(&self) -> Option<&crate::types::VideoChatStarted> {
        match self {
            Self::Message(val) => crate::types::Message::video_chat_started(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `video_note`.
    ///
    /// Message is a video note, information about the video message
    #[must_use]
    pub fn video_note(&self) -> Option<&crate::types::VideoNote> {
        match self {
            Self::Message(val) => crate::types::Message::video_note(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `voice`.
    ///
    /// Message is a voice message, information about the file
    #[must_use]
    pub fn voice(&self) -> Option<&crate::types::Voice> {
        match self {
            Self::Message(val) => crate::types::Message::voice(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `web_app_data`.
    ///
    /// Service message: data sent by a Web App
    #[must_use]
    pub fn web_app_data(&self) -> Option<&crate::types::WebAppData> {
        match self {
            Self::Message(val) => crate::types::Message::web_app_data(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for field `write_access_allowed`.
    ///
    /// Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess
    #[must_use]
    pub fn write_access_allowed(&self) -> Option<&crate::types::WriteAccessAllowed> {
        match self {
            Self::Message(val) => crate::types::Message::write_access_allowed(val),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `additional_chat_count`.
    #[must_use]
    pub fn additional_chat_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_winners(val)
                .and_then(crate::types::GiveawayWinners::additional_chat_count),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `address`.
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::venue(val).map(|inner| inner.address.as_ref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `allows_multiple_answers`.
    #[must_use]
    pub fn allows_multiple_answers(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).map(crate::types::Poll::allows_multiple_answers)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `amount`.
    #[must_use]
    pub fn amount(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::suggested_post_paid(val).and_then(|inner| inner.amount)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `are_direct_messages_enabled`.
    #[must_use]
    pub fn are_direct_messages_enabled(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::direct_message_price_changed(val)
                .map(|inner| inner.are_direct_messages_enabled),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `boost_count`.
    #[must_use]
    pub fn boost_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::boost_added(val).map(|inner| inner.boost_count)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `button_text`.
    #[must_use]
    pub fn button_text(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::web_app_data(val).map(|inner| inner.button_text.as_ref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `chat_id`.
    #[must_use]
    pub fn chat_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::chat_shared(val).map(|inner| inner.chat_id)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `chats`.
    #[must_use]
    pub fn chats(&self) -> Option<&[crate::types::Chat]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::giveaway(val).map(crate::types::Giveaway::chats)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `close_date`.
    #[must_use]
    pub fn close_date(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).and_then(crate::types::Poll::close_date)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `comment`.
    #[must_use]
    pub fn comment(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_declined(val)
                .and_then(|inner| inner.comment.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `correct_option_id`.
    #[must_use]
    pub fn correct_option_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).and_then(crate::types::Poll::correct_option_id)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `country_codes`.
    #[must_use]
    pub fn country_codes(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::giveaway(val).and_then(crate::types::Giveaway::country_codes)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `cover`.
    #[must_use]
    pub fn cover(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::video(val).and_then(|inner| inner.cover.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `credentials`.
    #[must_use]
    pub fn credentials(&self) -> Option<&crate::types::EncryptedCredentials> {
        match self {
            Self::Message(val) => {
                crate::types::Message::passport_data(val).map(|inner| &inner.credentials)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `custom_emoji_id`.
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::sticker(val).and_then(crate::types::Sticker::custom_emoji_id)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `direct_message_star_count`.
    #[must_use]
    pub fn direct_message_star_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::direct_message_price_changed(val)
                .and_then(|inner| inner.direct_message_star_count),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `distance`.
    #[must_use]
    pub fn distance(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::proximity_alert_triggered(val).map(|inner| inner.distance)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `explanation`.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).and_then(crate::types::Poll::explanation)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `explanation_entities`.
    #[must_use]
    pub fn explanation_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).and_then(crate::types::Poll::explanation_entities)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `foursquare_id`.
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::venue(val).and_then(|inner| inner.foursquare_id.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `foursquare_type`.
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::venue(val).and_then(|inner| inner.foursquare_type.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `from_attachment_menu`.
    #[must_use]
    pub fn from_attachment_menu(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::write_access_allowed(val)
                .and_then(|inner| inner.from_attachment_menu),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `from_request`.
    #[must_use]
    pub fn from_request(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::write_access_allowed(val)
                .and_then(|inner| inner.from_request),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `giveaway_message`.
    #[must_use]
    pub fn giveaway_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_completed(val)
                .and_then(|inner| inner.giveaway_message.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `giveaway_message_id`.
    #[must_use]
    pub fn giveaway_message_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_winners(val)
                .map(crate::types::GiveawayWinners::giveaway_message_id),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `google_place_id`.
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::venue(val).and_then(|inner| inner.google_place_id.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `google_place_type`.
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::venue(val)
                .and_then(|inner| inner.google_place_type.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `has_public_winners`.
    #[must_use]
    pub fn has_public_winners(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway(val)
                .and_then(crate::types::Giveaway::has_public_winners),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `heading`.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::Message(val) => {
                crate::types::Message::location(val).and_then(|inner| inner.heading)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `horizontal_accuracy`.
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::location(val).and_then(|inner| inner.horizontal_accuracy)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `icon_color`.
    #[must_use]
    pub fn icon_color(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::forum_topic_created(val).map(|inner| inner.icon_color)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `inline_keyboard`.
    #[must_use]
    pub fn inline_keyboard(&self) -> Option<&[Box<[crate::types::InlineKeyboardButton]>]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::reply_markup(val).map(|inner| inner.inline_keyboard.as_ref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_animated`.
    #[must_use]
    pub fn is_animated(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::sticker(val).map(crate::types::Sticker::is_animated)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_anonymous`.
    #[must_use]
    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).map(crate::types::Poll::is_anonymous)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_closed`.
    #[must_use]
    pub fn is_closed(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).map(crate::types::Poll::is_closed)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_disabled`.
    #[must_use]
    pub fn is_disabled(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::link_preview_options(val).and_then(|inner| inner.is_disabled)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_first_recurring`.
    #[must_use]
    pub fn is_first_recurring(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::successful_payment(val)
                .and_then(|inner| inner.is_first_recurring),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_manual`.
    #[must_use]
    pub fn is_manual(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::quote(val).and_then(|inner| inner.is_manual)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_name_implicit`.
    #[must_use]
    pub fn is_name_implicit(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::forum_topic_created(val)
                .and_then(|inner| inner.is_name_implicit),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_recurring`.
    #[must_use]
    pub fn is_recurring(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::successful_payment(val).and_then(|inner| inner.is_recurring)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_star_giveaway`.
    #[must_use]
    pub fn is_star_giveaway(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_completed(val)
                .and_then(|inner| inner.is_star_giveaway),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `is_video`.
    #[must_use]
    pub fn is_video(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::sticker(val).map(crate::types::Sticker::is_video)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `last_resale_amount`.
    #[must_use]
    pub fn last_resale_amount(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::unique_gift(val).and_then(|inner| inner.last_resale_amount)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `last_resale_currency`.
    #[must_use]
    pub fn last_resale_currency(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::unique_gift(val)
                .and_then(|inner| inner.last_resale_currency.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `latitude`.
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Message(val) => crate::types::Message::location(val).map(|inner| inner.latitude),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `length`.
    #[must_use]
    pub fn length(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::video_note(val).map(|inner| inner.length),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `live_period`.
    #[must_use]
    pub fn live_period(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::location(val).and_then(|inner| inner.live_period)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `longitude`.
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Message(val) => crate::types::Message::location(val).map(|inner| inner.longitude),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `marked_as_done_task_ids`.
    #[must_use]
    pub fn marked_as_done_task_ids(&self) -> Option<&[i64]> {
        match self {
            Self::Message(val) => crate::types::Message::checklist_tasks_done(val)
                .and_then(|inner| inner.marked_as_done_task_ids.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `marked_as_not_done_task_ids`.
    #[must_use]
    pub fn marked_as_not_done_task_ids(&self) -> Option<&[i64]> {
        match self {
            Self::Message(val) => crate::types::Message::checklist_tasks_done(val)
                .and_then(|inner| inner.marked_as_not_done_task_ids.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `mask_position`.
    #[must_use]
    pub fn mask_position(&self) -> Option<&crate::types::MaskPosition> {
        match self {
            Self::Message(val) => {
                crate::types::Message::sticker(val).and_then(crate::types::Sticker::mask_position)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `message_auto_delete_time`.
    #[must_use]
    pub fn message_auto_delete_time(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::message_auto_delete_timer_changed(val)
                .map(|inner| inner.message_auto_delete_time),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `needs_repainting`.
    #[must_use]
    pub fn needs_repainting(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::sticker(val)
                .and_then(crate::types::Sticker::needs_repainting),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `next_transfer_date`.
    #[must_use]
    pub fn next_transfer_date(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::unique_gift(val).and_then(|inner| inner.next_transfer_date)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `open_period`.
    #[must_use]
    pub fn open_period(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).and_then(crate::types::Poll::open_period)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `options`.
    #[must_use]
    pub fn options(&self) -> Option<&[crate::types::PollOption]> {
        match self {
            Self::Message(val) => crate::types::Message::poll(val).map(crate::types::Poll::options),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `order_info`.
    #[must_use]
    pub fn order_info(&self) -> Option<&crate::types::OrderInfo> {
        match self {
            Self::Message(val) => crate::types::Message::successful_payment(val)
                .and_then(|inner| inner.order_info.as_ref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `others_can_add_tasks`.
    #[must_use]
    pub fn others_can_add_tasks(&self) -> Option<bool> {
        match self {
            Self::Message(val) => {
                crate::types::Message::checklist(val).and_then(|inner| inner.others_can_add_tasks)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `others_can_mark_tasks_as_done`.
    #[must_use]
    pub fn others_can_mark_tasks_as_done(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::checklist(val)
                .and_then(|inner| inner.others_can_mark_tasks_as_done),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `paid_message_star_count`.
    #[must_use]
    pub fn paid_message_star_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::paid_message_price_changed(val)
                .map(|inner| inner.paid_message_star_count),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `performer`.
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::audio(val).and_then(|inner| inner.performer.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `phone_number`.
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::contact(val).map(|inner| inner.phone_number.as_ref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `position`.
    #[must_use]
    pub fn position(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::quote(val).map(|inner| inner.position),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `prefer_large_media`.
    #[must_use]
    pub fn prefer_large_media(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::link_preview_options(val)
                .and_then(|inner| inner.prefer_large_media),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `prefer_small_media`.
    #[must_use]
    pub fn prefer_small_media(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::link_preview_options(val)
                .and_then(|inner| inner.prefer_small_media),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `premium_animation`.
    #[must_use]
    pub fn premium_animation(&self) -> Option<&crate::types::File> {
        match self {
            Self::Message(val) => crate::types::Message::sticker(val)
                .and_then(crate::types::Sticker::premium_animation),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `proximity_alert_radius`.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::location(val).and_then(|inner| inner.proximity_alert_radius)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `qualities`.
    #[must_use]
    pub fn qualities(&self) -> Option<&[crate::types::VideoQuality]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::video(val).and_then(|inner| inner.qualities.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `question`.
    #[must_use]
    pub fn question(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).map(crate::types::Poll::question)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `question_entities`.
    #[must_use]
    pub fn question_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).and_then(crate::types::Poll::question_entities)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `reason`.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_refunded(val)
                .map(|inner| inner.reason.as_ref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `sender_user`.
    #[must_use]
    pub fn sender_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::forward_origin(val)
                .and_then(crate::types::MessageOrigin::sender_user),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `sender_user_name`.
    #[must_use]
    pub fn sender_user_name(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::forward_origin(val)
                .and_then(crate::types::MessageOrigin::sender_user_name),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `set_name`.
    #[must_use]
    pub fn set_name(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::sticker(val).and_then(crate::types::Sticker::set_name)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `shipping_option_id`.
    #[must_use]
    pub fn shipping_option_id(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::successful_payment(val)
                .and_then(|inner| inner.shipping_option_id.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `show_above_text`.
    #[must_use]
    pub fn show_above_text(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::link_preview_options(val)
                .and_then(|inner| inner.show_above_text),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `star_amount`.
    #[must_use]
    pub fn star_amount(&self) -> Option<&crate::types::StarAmount> {
        match self {
            Self::Message(val) => crate::types::Message::suggested_post_paid(val)
                .and_then(|inner| inner.star_amount.as_ref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `star_count`.
    #[must_use]
    pub fn star_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::paid_media(val).map(|inner| inner.star_count)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `start_date`.
    #[must_use]
    pub fn start_date(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::video_chat_scheduled(val).map(|inner| inner.start_date)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `start_parameter`.
    #[must_use]
    pub fn start_parameter(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::invoice(val).map(|inner| inner.start_parameter.as_ref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `start_timestamp`.
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::video(val).and_then(|inner| inner.start_timestamp)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `state`.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::suggested_post_info(val).map(|inner| inner.state.as_ref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `subscription_expiration_date`.
    #[must_use]
    pub fn subscription_expiration_date(&self) -> Option<i64> {
        match self {
            Self::Message(val) => crate::types::Message::successful_payment(val)
                .and_then(|inner| inner.subscription_expiration_date),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `text_entities`.
    #[must_use]
    pub fn text_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Message(val) => {
                crate::types::Message::game(val).and_then(|inner| inner.text_entities.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `title_entities`.
    #[must_use]
    pub fn title_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Message(val) => crate::types::Message::checklist(val)
                .and_then(|inner| inner.title_entities.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `topic_id`.
    #[must_use]
    pub fn topic_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::direct_messages_topic(val).map(|inner| inner.topic_id)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `total_voter_count`.
    #[must_use]
    pub fn total_voter_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::poll(val).map(crate::types::Poll::total_voter_count)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `transfer_star_count`.
    #[must_use]
    pub fn transfer_star_count(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::unique_gift(val).and_then(|inner| inner.transfer_star_count)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `traveler`.
    #[must_use]
    pub fn traveler(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::proximity_alert_triggered(val)
                .map(|inner| inner.traveler.as_ref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `type`.
    #[must_use]
    pub fn r#type(&self) -> Option<&crate::types::BackgroundType> {
        match self {
            Self::Message(val) => {
                crate::types::Message::chat_background_set(val).map(|inner| inner.r#type.as_ref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `url`.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::link_preview_options(val)
                .and_then(|inner| inner.url.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `user`.
    #[must_use]
    pub fn user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::direct_messages_topic(val)
                .and_then(|inner| inner.user.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `user_id`.
    #[must_use]
    pub fn user_id(&self) -> Option<i64> {
        match self {
            Self::Message(val) => {
                crate::types::Message::contact(val).and_then(|inner| inner.user_id)
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `value`.
    #[must_use]
    pub fn value(&self) -> Option<u8> {
        match self {
            Self::Message(val) => crate::types::Message::dice(val).map(|inner| inner.value),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `vcard`.
    #[must_use]
    pub fn vcard(&self) -> Option<&str> {
        match self {
            Self::Message(val) => {
                crate::types::Message::contact(val).and_then(|inner| inner.vcard.as_deref())
            }
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `was_refunded`.
    #[must_use]
    pub fn was_refunded(&self) -> Option<bool> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_winners(val)
                .and_then(crate::types::GiveawayWinners::was_refunded),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `watcher`.
    #[must_use]
    pub fn watcher(&self) -> Option<&crate::types::User> {
        match self {
            Self::Message(val) => crate::types::Message::proximity_alert_triggered(val)
                .map(|inner| inner.watcher.as_ref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `web_app_name`.
    #[must_use]
    pub fn web_app_name(&self) -> Option<&str> {
        match self {
            Self::Message(val) => crate::types::Message::write_access_allowed(val)
                .and_then(|inner| inner.web_app_name.as_deref()),
            Self::InaccessibleMessage(_) => None,
        }
    }

    /// Helper method for nested field `winners`.
    #[must_use]
    pub fn winners(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::Message(val) => crate::types::Message::giveaway_winners(val)
                .map(crate::types::GiveawayWinners::winners),
            Self::InaccessibleMessage(_) => None,
        }
    }
}
impl From<crate::types::InaccessibleMessage> for MaybeInaccessibleMessage {
    fn from(val: crate::types::InaccessibleMessage) -> Self {
        Self::InaccessibleMessage(val)
    }
}
impl TryFrom<MaybeInaccessibleMessage> for crate::types::InaccessibleMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MaybeInaccessibleMessage) -> Result<Self, Self::Error> {
        match val {
            MaybeInaccessibleMessage::InaccessibleMessage(inner) => Ok(inner),
            MaybeInaccessibleMessage::Message(_) => Err(Self::Error::new(
                stringify!(MaybeInaccessibleMessage),
                stringify!(InaccessibleMessage),
            )),
        }
    }
}
impl From<crate::types::Message> for MaybeInaccessibleMessage {
    fn from(val: crate::types::Message) -> Self {
        Self::Message(val)
    }
}
impl TryFrom<MaybeInaccessibleMessage> for crate::types::Message {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MaybeInaccessibleMessage) -> Result<Self, Self::Error> {
        match val {
            MaybeInaccessibleMessage::Message(inner) => Ok(inner),
            MaybeInaccessibleMessage::InaccessibleMessage(_) => Err(Self::Error::new(
                stringify!(MaybeInaccessibleMessage),
                stringify!(Message),
            )),
        }
    }
}
