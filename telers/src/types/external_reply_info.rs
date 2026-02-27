use serde::{Deserialize, Serialize};
/// This object contains information about a message that is being replied to, which may come from another chat or forum topic.
/// Currently, it can be one of
/// - [`ExternalReplyInfoAnimation`]
/// - [`ExternalReplyInfoAudio`]
/// - [`ExternalReplyInfoChecklist`]
/// - [`ExternalReplyInfoContact`]
/// - [`ExternalReplyInfoDice`]
/// - [`ExternalReplyInfoDocument`]
/// - [`ExternalReplyInfoGame`]
/// - [`ExternalReplyInfoGiveaway`]
/// - [`ExternalReplyInfoGiveawayWinners`]
/// - [`ExternalReplyInfoInvoice`]
/// - [`ExternalReplyInfoLocation`]
/// - [`ExternalReplyInfoPhoto`]
/// - [`ExternalReplyInfoPoll`]
/// - [`ExternalReplyInfoSticker`]
/// - [`ExternalReplyInfoStory`]
/// - [`ExternalReplyInfoVenue`]
/// - [`ExternalReplyInfoVideo`]
/// - [`ExternalReplyInfoVideoNote`]
/// - [`ExternalReplyInfoVoice`]
/// # Documentation
/// <https://core.telegram.org/bots/api#externalreplyinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExternalReplyInfo {
    Animation(crate::types::ExternalReplyInfoAnimation),
    Audio(crate::types::ExternalReplyInfoAudio),
    Checklist(crate::types::ExternalReplyInfoChecklist),
    Contact(crate::types::ExternalReplyInfoContact),
    Dice(crate::types::ExternalReplyInfoDice),
    Document(crate::types::ExternalReplyInfoDocument),
    Game(crate::types::ExternalReplyInfoGame),
    Giveaway(crate::types::ExternalReplyInfoGiveaway),
    GiveawayWinners(crate::types::ExternalReplyInfoGiveawayWinners),
    Invoice(crate::types::ExternalReplyInfoInvoice),
    Location(crate::types::ExternalReplyInfoLocation),
    Photo(crate::types::ExternalReplyInfoPhoto),
    Poll(crate::types::ExternalReplyInfoPoll),
    Sticker(crate::types::ExternalReplyInfoSticker),
    Story(crate::types::ExternalReplyInfoStory),
    Venue(crate::types::ExternalReplyInfoVenue),
    Video(crate::types::ExternalReplyInfoVideo),
    VideoNote(crate::types::ExternalReplyInfoVideoNote),
    Voice(crate::types::ExternalReplyInfoVoice),
}
impl ExternalReplyInfo {
    /// Helper method for field `animation`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAnimation`. Message is an animation, information about the animation
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::Animation(val) => Some(val.animation.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `audio`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAudio`. Message is an audio file, information about the file
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Audio(val) => Some(val.audio.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `chat`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAnimation`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoAudio`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoChecklist`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoContact`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoDice`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoDocument`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoGame`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoGiveaway`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoGiveawayWinners`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoInvoice`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoLocation`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoPhoto`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoPoll`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoSticker`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoStory`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVenue`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVideo`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVideoNote`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVoice`. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Animation(val) => val.chat.as_deref(),
            Self::Audio(val) => val.chat.as_deref(),
            Self::Checklist(val) => val.chat.as_deref(),
            Self::Contact(val) => val.chat.as_deref(),
            Self::Dice(val) => val.chat.as_deref(),
            Self::Document(val) => val.chat.as_deref(),
            Self::Game(val) => val.chat.as_deref(),
            Self::Giveaway(val) => val.chat.as_deref(),
            Self::GiveawayWinners(val) => val.chat.as_deref(),
            Self::Invoice(val) => val.chat.as_deref(),
            Self::Location(val) => val.chat.as_deref(),
            Self::Photo(val) => val.chat.as_deref(),
            Self::Poll(val) => val.chat.as_deref(),
            Self::Sticker(val) => val.chat.as_deref(),
            Self::Story(val) => val.chat.as_deref(),
            Self::Venue(val) => val.chat.as_deref(),
            Self::Video(val) => val.chat.as_deref(),
            Self::VideoNote(val) => val.chat.as_deref(),
            Self::Voice(val) => val.chat.as_deref(),
        }
    }

    /// Helper method for field `checklist`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoChecklist`. Message is a checklist
    #[must_use]
    pub fn checklist(&self) -> Option<&crate::types::Checklist> {
        match self {
            Self::Checklist(val) => Some(&val.checklist),
            _ => None,
        }
    }

    /// Helper method for field `contact`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoContact`. Message is a shared contact, information about the contact
    #[must_use]
    pub fn contact(&self) -> Option<&crate::types::Contact> {
        match self {
            Self::Contact(val) => Some(&val.contact),
            _ => None,
        }
    }

    /// Helper method for field `dice`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoDice`. Message is a dice with random value
    #[must_use]
    pub fn dice(&self) -> Option<&crate::types::Dice> {
        match self {
            Self::Dice(val) => Some(&val.dice),
            _ => None,
        }
    }

    /// Helper method for field `document`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoDocument`. Message is a general file, information about the file
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::Document(val) => Some(val.document.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `game`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoGame`. Message is a game, information about the game. More about games: <https://core.telegram.org/bots/api#games>
    #[must_use]
    pub fn game(&self) -> Option<&crate::types::Game> {
        match self {
            Self::Game(val) => Some(&val.game),
            _ => None,
        }
    }

    /// Helper method for field `giveaway`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoGiveaway`. Message is a scheduled giveaway, information about the giveaway
    #[must_use]
    pub fn giveaway(&self) -> Option<&crate::types::Giveaway> {
        match self {
            Self::Giveaway(val) => Some(&val.giveaway),
            _ => None,
        }
    }

    /// Helper method for field `giveaway_winners`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoGiveawayWinners`. A giveaway with public winners was completed
    #[must_use]
    pub fn giveaway_winners(&self) -> Option<&crate::types::GiveawayWinners> {
        match self {
            Self::GiveawayWinners(val) => Some(&val.giveaway_winners),
            _ => None,
        }
    }

    /// Helper method for field `has_media_spoiler`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAnimation`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoAudio`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoChecklist`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoContact`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoDice`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoDocument`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoGame`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoGiveaway`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoGiveawayWinners`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoInvoice`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoLocation`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoPhoto`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoPoll`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoSticker`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoStory`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoVenue`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoVideo`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoVideoNote`. `true`, if the message media is covered by a spoiler animation
    /// - `ExternalReplyInfoVoice`. `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_media_spoiler,
            Self::Audio(val) => val.has_media_spoiler,
            Self::Checklist(val) => val.has_media_spoiler,
            Self::Contact(val) => val.has_media_spoiler,
            Self::Dice(val) => val.has_media_spoiler,
            Self::Document(val) => val.has_media_spoiler,
            Self::Game(val) => val.has_media_spoiler,
            Self::Giveaway(val) => val.has_media_spoiler,
            Self::GiveawayWinners(val) => val.has_media_spoiler,
            Self::Invoice(val) => val.has_media_spoiler,
            Self::Location(val) => val.has_media_spoiler,
            Self::Photo(val) => val.has_media_spoiler,
            Self::Poll(val) => val.has_media_spoiler,
            Self::Sticker(val) => val.has_media_spoiler,
            Self::Story(val) => val.has_media_spoiler,
            Self::Venue(val) => val.has_media_spoiler,
            Self::Video(val) => val.has_media_spoiler,
            Self::VideoNote(val) => val.has_media_spoiler,
            Self::Voice(val) => val.has_media_spoiler,
        }
    }

    /// Helper method for field `invoice`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoInvoice`. Message is an invoice for a payment, information about the invoice. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn invoice(&self) -> Option<&crate::types::Invoice> {
        match self {
            Self::Invoice(val) => Some(&val.invoice),
            _ => None,
        }
    }

    /// Helper method for field `link_preview_options`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAnimation`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoAudio`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoChecklist`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoContact`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoDice`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoDocument`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoGame`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoGiveaway`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoGiveawayWinners`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoInvoice`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoLocation`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoPhoto`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoPoll`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoSticker`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoStory`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoVenue`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoVideo`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoVideoNote`. Options used for link preview generation for the original message, if it is a text message
    /// - `ExternalReplyInfoVoice`. Options used for link preview generation for the original message, if it is a text message
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::Animation(val) => val.link_preview_options.as_ref(),
            Self::Audio(val) => val.link_preview_options.as_ref(),
            Self::Checklist(val) => val.link_preview_options.as_ref(),
            Self::Contact(val) => val.link_preview_options.as_ref(),
            Self::Dice(val) => val.link_preview_options.as_ref(),
            Self::Document(val) => val.link_preview_options.as_ref(),
            Self::Game(val) => val.link_preview_options.as_ref(),
            Self::Giveaway(val) => val.link_preview_options.as_ref(),
            Self::GiveawayWinners(val) => val.link_preview_options.as_ref(),
            Self::Invoice(val) => val.link_preview_options.as_ref(),
            Self::Location(val) => val.link_preview_options.as_ref(),
            Self::Photo(val) => val.link_preview_options.as_ref(),
            Self::Poll(val) => val.link_preview_options.as_ref(),
            Self::Sticker(val) => val.link_preview_options.as_ref(),
            Self::Story(val) => val.link_preview_options.as_ref(),
            Self::Venue(val) => val.link_preview_options.as_ref(),
            Self::Video(val) => val.link_preview_options.as_ref(),
            Self::VideoNote(val) => val.link_preview_options.as_ref(),
            Self::Voice(val) => val.link_preview_options.as_ref(),
        }
    }

    /// Helper method for field `location`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoLocation`. Message is a shared location, information about the location
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::Location> {
        match self {
            Self::Location(val) => Some(&val.location),
            _ => None,
        }
    }

    /// Helper method for field `message_id`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAnimation`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoAudio`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoChecklist`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoContact`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoDice`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoDocument`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoGame`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoGiveaway`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoGiveawayWinners`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoInvoice`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoLocation`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoPhoto`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoPoll`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoSticker`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoStory`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVenue`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVideo`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVideoNote`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    /// - `ExternalReplyInfoVoice`. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[must_use]
    pub fn message_id(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.message_id,
            Self::Audio(val) => val.message_id,
            Self::Checklist(val) => val.message_id,
            Self::Contact(val) => val.message_id,
            Self::Dice(val) => val.message_id,
            Self::Document(val) => val.message_id,
            Self::Game(val) => val.message_id,
            Self::Giveaway(val) => val.message_id,
            Self::GiveawayWinners(val) => val.message_id,
            Self::Invoice(val) => val.message_id,
            Self::Location(val) => val.message_id,
            Self::Photo(val) => val.message_id,
            Self::Poll(val) => val.message_id,
            Self::Sticker(val) => val.message_id,
            Self::Story(val) => val.message_id,
            Self::Venue(val) => val.message_id,
            Self::Video(val) => val.message_id,
            Self::VideoNote(val) => val.message_id,
            Self::Voice(val) => val.message_id,
        }
    }

    /// Helper method for field `origin`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAnimation`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoAudio`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoChecklist`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoContact`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoDice`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoDocument`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoGame`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoGiveaway`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoGiveawayWinners`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoInvoice`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoLocation`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoPhoto`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoPoll`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoSticker`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoStory`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoVenue`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoVideo`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoVideoNote`. Origin of the message replied to by the given message
    /// - `ExternalReplyInfoVoice`. Origin of the message replied to by the given message
    #[must_use]
    pub fn origin(&self) -> &crate::types::MessageOrigin {
        match self {
            Self::Animation(val) => &val.origin,
            Self::Audio(val) => &val.origin,
            Self::Checklist(val) => &val.origin,
            Self::Contact(val) => &val.origin,
            Self::Dice(val) => &val.origin,
            Self::Document(val) => &val.origin,
            Self::Game(val) => &val.origin,
            Self::Giveaway(val) => &val.origin,
            Self::GiveawayWinners(val) => &val.origin,
            Self::Invoice(val) => &val.origin,
            Self::Location(val) => &val.origin,
            Self::Photo(val) => &val.origin,
            Self::Poll(val) => &val.origin,
            Self::Sticker(val) => &val.origin,
            Self::Story(val) => &val.origin,
            Self::Venue(val) => &val.origin,
            Self::Video(val) => &val.origin,
            Self::VideoNote(val) => &val.origin,
            Self::Voice(val) => &val.origin,
        }
    }

    /// Helper method for field `paid_media`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoAnimation`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoAudio`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoChecklist`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoContact`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoDice`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoDocument`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoGame`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoGiveaway`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoGiveawayWinners`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoInvoice`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoLocation`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoPhoto`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoPoll`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoSticker`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoStory`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoVenue`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoVideo`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoVideoNote`. Message contains paid media; information about the paid media
    /// - `ExternalReplyInfoVoice`. Message contains paid media; information about the paid media
    #[must_use]
    pub fn paid_media(&self) -> Option<&crate::types::PaidMediaInfo> {
        match self {
            Self::Animation(val) => val.paid_media.as_ref(),
            Self::Audio(val) => val.paid_media.as_ref(),
            Self::Checklist(val) => val.paid_media.as_ref(),
            Self::Contact(val) => val.paid_media.as_ref(),
            Self::Dice(val) => val.paid_media.as_ref(),
            Self::Document(val) => val.paid_media.as_ref(),
            Self::Game(val) => val.paid_media.as_ref(),
            Self::Giveaway(val) => val.paid_media.as_ref(),
            Self::GiveawayWinners(val) => val.paid_media.as_ref(),
            Self::Invoice(val) => val.paid_media.as_ref(),
            Self::Location(val) => val.paid_media.as_ref(),
            Self::Photo(val) => val.paid_media.as_ref(),
            Self::Poll(val) => val.paid_media.as_ref(),
            Self::Sticker(val) => val.paid_media.as_ref(),
            Self::Story(val) => val.paid_media.as_ref(),
            Self::Venue(val) => val.paid_media.as_ref(),
            Self::Video(val) => val.paid_media.as_ref(),
            Self::VideoNote(val) => val.paid_media.as_ref(),
            Self::Voice(val) => val.paid_media.as_ref(),
        }
    }

    /// Helper method for field `photo`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoPhoto`. Message is a photo, available sizes of the photo
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Photo(val) => Some(val.photo.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `poll`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoPoll`. Message is a native poll, information about the poll
    #[must_use]
    pub fn poll(&self) -> Option<&crate::types::Poll> {
        match self {
            Self::Poll(val) => Some(val.poll.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `sticker`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoSticker`. Message is a sticker, information about the sticker
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::Sticker(val) => Some(val.sticker.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `story`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoStory`. Message is a forwarded story
    #[must_use]
    pub fn story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Story(val) => Some(&val.story),
            _ => None,
        }
    }

    /// Helper method for field `venue`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoVenue`. Message is a venue, information about the venue
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        match self {
            Self::Venue(val) => Some(val.venue.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `video`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoVideo`. Message is a video, information about the video
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::Video(val) => Some(val.video.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `video_note`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoVideoNote`. Message is a video note, information about the video message
    #[must_use]
    pub fn video_note(&self) -> Option<&crate::types::VideoNote> {
        match self {
            Self::VideoNote(val) => Some(val.video_note.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `voice`.
    ///
    /// # Variants
    /// - `ExternalReplyInfoVoice`. Message is a voice message, information about the file
    #[must_use]
    pub fn voice(&self) -> Option<&crate::types::Voice> {
        match self {
            Self::Voice(val) => Some(&val.voice),
            _ => None,
        }
    }

    /// Helper method for nested field `additional_chat_count`.
    #[must_use]
    pub fn additional_chat_count(&self) -> Option<i64> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::additional_chat_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `address`.
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                Some(inner.address.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `allows_multiple_answers`.
    #[must_use]
    pub fn allows_multiple_answers(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::allows_multiple_answers(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `author_signature`.
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        crate::types::MessageOrigin::author_signature(self.origin())
    }

    /// Helper method for nested field `chats`.
    #[must_use]
    pub fn chats(&self) -> Option<&[crate::types::Chat]> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                Some(crate::types::Giveaway::chats(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `close_date`.
    #[must_use]
    pub fn close_date(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::close_date(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `correct_option_id`.
    #[must_use]
    pub fn correct_option_id(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::correct_option_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `country_codes`.
    #[must_use]
    pub fn country_codes(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::country_codes(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `cover`.
    #[must_use]
    pub fn cover(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.cover.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `currency`.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        match self {
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.currency.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `custom_emoji_id`.
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::custom_emoji_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `date`.
    #[must_use]
    pub fn date(&self) -> i64 {
        crate::types::MessageOrigin::date(self.origin())
    }

    /// Helper method for nested field `description`.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Game(val) => {
                let inner = &val.game;
                Some(inner.description.as_ref())
            }
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.description.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `duration`.
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.duration)
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.duration)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.duration)
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.duration)
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                Some(inner.duration)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `emoji`.
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Dice(val) => {
                let inner = &val.dice;
                Some(inner.emoji.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::emoji(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation`.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation_entities`.
    #[must_use]
    pub fn explanation_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_id`.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::file_id(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                Some(inner.file_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_name`.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_name.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_name.as_deref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.file_name.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_size`.
    #[must_use]
    pub fn file_size(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_size
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_size
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.file_size
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::file_size(inner)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_size
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                inner.file_size
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                inner.file_size
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_unique_id`.
    #[must_use]
    pub fn file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::file_unique_id(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                Some(inner.file_unique_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `foursquare_id`.
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.foursquare_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `foursquare_type`.
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.foursquare_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway_message_id`.
    #[must_use]
    pub fn giveaway_message_id(&self) -> Option<i64> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::giveaway_message_id(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `google_place_id`.
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.google_place_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `google_place_type`.
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.google_place_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_public_winners`.
    #[must_use]
    pub fn has_public_winners(&self) -> Option<bool> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::has_public_winners(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `heading`.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.heading
            }
            _ => None,
        }
    }

    /// Helper method for nested field `height`.
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.height)
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::height(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.height)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `horizontal_accuracy`.
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.horizontal_accuracy
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_animated`.
    #[must_use]
    pub fn is_animated(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::is_animated(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_anonymous`.
    #[must_use]
    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::is_anonymous(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_closed`.
    #[must_use]
    pub fn is_closed(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::is_closed(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_direct_messages`.
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        self.chat().and_then(crate::types::Chat::is_direct_messages)
    }

    /// Helper method for nested field `is_disabled`.
    #[must_use]
    pub fn is_disabled(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.is_disabled)
    }

    /// Helper method for nested field `is_forum`.
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        self.chat().and_then(crate::types::Chat::is_forum)
    }

    /// Helper method for nested field `is_video`.
    #[must_use]
    pub fn is_video(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::is_video(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `latitude`.
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                Some(inner.latitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `length`.
    #[must_use]
    pub fn length(&self) -> Option<i64> {
        match self {
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.length)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `live_period`.
    #[must_use]
    pub fn live_period(&self) -> Option<i64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.live_period
            }
            _ => None,
        }
    }

    /// Helper method for nested field `longitude`.
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                Some(inner.longitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mask_position`.
    #[must_use]
    pub fn mask_position(&self) -> Option<&crate::types::MaskPosition> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::mask_position(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mime_type`.
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                inner.mime_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `needs_repainting`.
    #[must_use]
    pub fn needs_repainting(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::needs_repainting(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `only_new_members`.
    #[must_use]
    pub fn only_new_members(&self) -> Option<bool> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::only_new_members(inner)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::only_new_members(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `open_period`.
    #[must_use]
    pub fn open_period(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::open_period(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `options`.
    #[must_use]
    pub fn options(&self) -> Option<&[crate::types::PollOption]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::options(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `others_can_add_tasks`.
    #[must_use]
    pub fn others_can_add_tasks(&self) -> Option<bool> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                inner.others_can_add_tasks
            }
            _ => None,
        }
    }

    /// Helper method for nested field `others_can_mark_tasks_as_done`.
    #[must_use]
    pub fn others_can_mark_tasks_as_done(&self) -> Option<bool> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                inner.others_can_mark_tasks_as_done
            }
            _ => None,
        }
    }

    /// Helper method for nested field `performer`.
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.performer.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `phone_number`.
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => {
                let inner = &val.contact;
                Some(inner.phone_number.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `prefer_large_media`.
    #[must_use]
    pub fn prefer_large_media(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.prefer_large_media)
    }

    /// Helper method for nested field `prefer_small_media`.
    #[must_use]
    pub fn prefer_small_media(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.prefer_small_media)
    }

    /// Helper method for nested field `premium_animation`.
    #[must_use]
    pub fn premium_animation(&self) -> Option<&crate::types::File> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::premium_animation(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `premium_subscription_month_count`.
    #[must_use]
    pub fn premium_subscription_month_count(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::premium_subscription_month_count(inner)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::premium_subscription_month_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `prize_description`.
    #[must_use]
    pub fn prize_description(&self) -> Option<&str> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::prize_description(inner)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::prize_description(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `prize_star_count`.
    #[must_use]
    pub fn prize_star_count(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::prize_star_count(inner)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::prize_star_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `proximity_alert_radius`.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<i64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.proximity_alert_radius
            }
            _ => None,
        }
    }

    /// Helper method for nested field `qualities`.
    #[must_use]
    pub fn qualities(&self) -> Option<&[crate::types::VideoQuality]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.qualities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `question`.
    #[must_use]
    pub fn question(&self) -> Option<&str> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::question(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `question_entities`.
    #[must_use]
    pub fn question_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::question_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sender_chat`.
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        crate::types::MessageOrigin::sender_chat(self.origin())
    }

    /// Helper method for nested field `sender_user`.
    #[must_use]
    pub fn sender_user(&self) -> Option<&crate::types::User> {
        crate::types::MessageOrigin::sender_user(self.origin())
    }

    /// Helper method for nested field `sender_user_name`.
    #[must_use]
    pub fn sender_user_name(&self) -> Option<&str> {
        crate::types::MessageOrigin::sender_user_name(self.origin())
    }

    /// Helper method for nested field `set_name`.
    #[must_use]
    pub fn set_name(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::set_name(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `show_above_text`.
    #[must_use]
    pub fn show_above_text(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.show_above_text)
    }

    /// Helper method for nested field `star_count`.
    #[must_use]
    pub fn star_count(&self) -> Option<i64> {
        self.paid_media().map(|inner| inner.star_count)
    }

    /// Helper method for nested field `start_parameter`.
    #[must_use]
    pub fn start_parameter(&self) -> Option<&str> {
        match self {
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.start_parameter.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `start_timestamp`.
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.start_timestamp
            }
            _ => None,
        }
    }

    /// Helper method for nested field `tasks`.
    #[must_use]
    pub fn tasks(&self) -> Option<&[crate::types::ChecklistTask]> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                Some(inner.tasks.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Game(val) => {
                let inner = &val.game;
                inner.text.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `text_entities`.
    #[must_use]
    pub fn text_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Game(val) => {
                let inner = &val.game;
                inner.text_entities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `thumbnail`.
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::PhotoSize> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::thumbnail(inner)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                inner.thumbnail.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `title_entities`.
    #[must_use]
    pub fn title_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                inner.title_entities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `total_amount`.
    #[must_use]
    pub fn total_amount(&self) -> Option<i64> {
        match self {
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.total_amount)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `total_voter_count`.
    #[must_use]
    pub fn total_voter_count(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::total_voter_count(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `unclaimed_prize_count`.
    #[must_use]
    pub fn unclaimed_prize_count(&self) -> Option<i64> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::unclaimed_prize_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `url`.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        self.link_preview_options()
            .and_then(|inner| inner.url.as_deref())
    }

    /// Helper method for nested field `user_id`.
    #[must_use]
    pub fn user_id(&self) -> Option<i64> {
        match self {
            Self::Contact(val) => {
                let inner = &val.contact;
                inner.user_id
            }
            _ => None,
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        self.chat().and_then(crate::types::Chat::username)
    }

    /// Helper method for nested field `value`.
    #[must_use]
    pub fn value(&self) -> Option<u8> {
        match self {
            Self::Dice(val) => {
                let inner = &val.dice;
                Some(inner.value)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `vcard`.
    #[must_use]
    pub fn vcard(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => {
                let inner = &val.contact;
                inner.vcard.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `was_refunded`.
    #[must_use]
    pub fn was_refunded(&self) -> Option<bool> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::was_refunded(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `width`.
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.width)
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::width(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.width)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `winner_count`.
    #[must_use]
    pub fn winner_count(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                Some(crate::types::Giveaway::winner_count(inner))
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::winner_count(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `winners`.
    #[must_use]
    pub fn winners(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::winners(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `winners_selection_date`.
    #[must_use]
    pub fn winners_selection_date(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                Some(crate::types::Giveaway::winners_selection_date(inner))
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::winners_selection_date(inner))
            }
            _ => None,
        }
    }
}
impl From<crate::types::ExternalReplyInfoAnimation> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoAnimation) -> Self {
        Self::Animation(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Animation(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoAnimation),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoAudio> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoAudio) -> Self {
        Self::Audio(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Audio(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoAudio),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoChecklist> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoChecklist) -> Self {
        Self::Checklist(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoChecklist {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Checklist(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoChecklist),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoContact> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoContact) -> Self {
        Self::Contact(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoContact {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Contact(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoContact),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoDice> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoDice) -> Self {
        Self::Dice(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoDice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Dice(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoDice),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoDocument> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoDocument) -> Self {
        Self::Document(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Document(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoDocument),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoGame> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoGame) -> Self {
        Self::Game(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoGame {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Game(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoGame),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoGiveaway> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoGiveaway) -> Self {
        Self::Giveaway(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoGiveaway {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Giveaway(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoGiveaway),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoGiveawayWinners> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoGiveawayWinners) -> Self {
        Self::GiveawayWinners(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoGiveawayWinners {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::GiveawayWinners(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoGiveawayWinners),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoInvoice> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoInvoice) -> Self {
        Self::Invoice(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoInvoice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Invoice(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoInvoice),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoLocation> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoLocation) -> Self {
        Self::Location(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Location(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoLocation),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoPhoto> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoPhoto),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoPoll> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoPoll) -> Self {
        Self::Poll(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoPoll {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Poll(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoPoll),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoSticker> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoSticker) -> Self {
        Self::Sticker(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Sticker(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoSticker),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoStory> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoStory) -> Self {
        Self::Story(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoStory {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Story(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoStory),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoVenue> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoVenue) -> Self {
        Self::Venue(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Venue(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoVenue),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoVideo> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoVideo),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoVideoNote> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoVideoNote) -> Self {
        Self::VideoNote(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoVideoNote {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::VideoNote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoVideoNote),
            ))
        }
    }
}
impl From<crate::types::ExternalReplyInfoVoice> for ExternalReplyInfo {
    fn from(val: crate::types::ExternalReplyInfoVoice) -> Self {
        Self::Voice(val)
    }
}
impl TryFrom<ExternalReplyInfo> for crate::types::ExternalReplyInfoVoice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ExternalReplyInfo) -> Result<Self, Self::Error> {
        if let ExternalReplyInfo::Voice(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ExternalReplyInfo),
                stringify!(ExternalReplyInfoVoice),
            ))
        }
    }
}
