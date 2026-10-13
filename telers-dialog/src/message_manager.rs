use crate::{
    entities::{NewMessage, OldMessage, ShowMode},
    errors::DialogError,
    widgets::media::{MediaAttachment, MediaContentType},
};
use telers::{
    client::Session,
    enums::{MessageType, ReplyMarkupType},
    errors::{SessionErrorKind, TelegramErrorKind},
    methods::{
        DeleteMessage, EditMessageCaption, EditMessageMedia, EditMessageReplyMarkup,
        EditMessageText, SendAnimation, SendAudio, SendDocument, SendMessage, SendPhoto, SendVideo,
        SendDice, SendSticker, SendVideoNote, SendVoice,
    },
    serialization::Serialize,
    types::{
        InputFile, InputMedia, InputMediaAnimation, InputMediaAudio, InputMediaDocument,
        InputMediaPhoto, InputMediaVideo, Message, ReplyKeyboardRemove, ReplyMarkup,
    },
    Bot, Either,
};
use tracing::{debug, trace};

pub struct MessageManager;

impl MessageManager {
    /// Remove or clean up the last dialog message when the dialog is closed.
    ///
    /// # Errors
    /// Returns telegram errors as `DialogError`.
    pub async fn close_message<Client: Session>(
        bot: &Bot<Client>,
        show_mode: ShowMode,
        old_message: Option<&OldMessage>,
    ) -> Result<(), DialogError> {
        Self::remove_kbd(bot, show_mode, old_message, None).await
    }

    /// Show message with edit/delete/send logic.
    ///
    /// # Errors
    /// Returns telegram errors as `DialogError`.
    pub async fn show_message<Client: Session>(
        bot: &Bot<Client>,
        mut new_message: NewMessage,
        old_message: Option<OldMessage>,
    ) -> Result<OldMessage, DialogError> {
        if new_message.media.as_ref().is_some_and(|media| {
            media.content_type != MediaContentType::Dice && media.to_input_file().is_none()
        }) {
            new_message.media = None;
        }
        if let Some(media) = &new_message.media {
            if matches!(
                media.content_type,
                MediaContentType::Sticker | MediaContentType::Dice | MediaContentType::VideoNote
            ) {
                new_message.text = "".into();
                new_message.parse_mode = None;
            } else {
                if let Some(caption) = &media.caption {
                    new_message.text = caption.as_ref().into();
                }
                if let Some(parse_mode) = &media.parse_mode {
                    new_message.parse_mode = Some(parse_mode.as_ref().into());
                }
            }
        }
        debug!(
            chat_id = new_message.chat.id(),
            show_mode = ?new_message.show_mode,
            has_previous_message = old_message.is_some(),
            "Show dialog message"
        );
        if new_message.show_mode == ShowMode::NoUpdate {
            trace!("Skip message update");
            return old_message.ok_or(DialogError::DialogNotFound);
        }
        if new_message.show_mode == ShowMode::Send {
            trace!("Sending new dialog message");
            Self::remove_kbd(
                bot, new_message.show_mode, old_message.as_ref(), Some(&new_message),
            ).await?;
            let sent = Self::send_message(bot, new_message.clone()).await?;
            return Ok(Self::combine(&new_message, &sent));
        }
        let Some(old) = old_message.as_ref() else {
            trace!("Sending new dialog message");
            Self::remove_kbd(bot, new_message.show_mode, None, Some(&new_message)).await?;
            let sent = Self::send_message(bot, new_message.clone()).await?;
            return Ok(Self::combine(&new_message, &sent));
        };
        if new_message.show_mode == ShowMode::DeleteAndSend {
            trace!(
                message_id = old.message_id,
                "Delete and resend dialog message"
            );
            if Self::need_reply_keyboard(&new_message) {
                let sent = Self::send_message(bot, new_message.clone()).await?;
                Self::remove_message_safe(bot, old, Some(&new_message)).await?;
                return Ok(Self::combine(&new_message, &sent));
            }
            Self::remove_message_safe(bot, old, Some(&new_message)).await?;
            let sent = Self::send_message(bot, new_message.clone()).await?;
            return Ok(Self::combine(&new_message, &sent));
        }
        if !Self::message_changed(&new_message, old) {
            trace!(message_id = old.message_id, "Dialog message did not change");
            return Ok(old.clone());
        }
        if !Self::can_edit(&new_message, old) {
            trace!(
                message_id = old.message_id,
                "Dialog message cannot be edited, recreating"
            );
            Self::remove_message_safe(bot, old, Some(&new_message)).await?;
            let sent = Self::send_message(bot, new_message.clone()).await?;
            return Ok(Self::combine(&new_message, &sent));
        }
        trace!(
            message_id = old.message_id,
            "Editing existing dialog message"
        );
        match Self::edit_message(bot, new_message.clone(), old).await {
            Ok(edited) => Ok(Self::combine(&new_message, &edited)),
            Err(err) if bad_request_contains(&err, &["message is not modified"]) => Ok(old.clone()),
            Err(err) if bad_request_contains(&err, &[
                "message can't be edited", "message to edit not found", "message_id_invalid",
            ]) => {
                let sent = Self::send_message(bot, new_message.clone()).await?;
                Ok(Self::combine(&new_message, &sent))
            }
            Err(err) => Err(err),
        }
    }

    /// Combine sent result with metadata to build `OldMessage`.
    #[must_use]
    fn combine(sent_message: &NewMessage, message_result: &Message) -> OldMessage {
        let reply_markup_type = sent_message
            .reply_markup
            .as_ref()
            .map(ReplyMarkupType::from);

        let message_type = MessageType::from(message_result);
        let media_content_type = match message_type {
            MessageType::Animation
            | MessageType::Audio
            | MessageType::Document
            | MessageType::Photo
            | MessageType::Video
            | MessageType::VideoNote
            | MessageType::Voice => Some(message_type),
            MessageType::Sticker | MessageType::Dice => Some(message_type),
            _ => None,
        };

        let (media_file_id, media_unique_id) = Self::media_ids(message_result);

        OldMessage::new(
            message_result.chat().clone(),
            message_result.message_id(),
            Some(sent_message.text.clone()),
            message_result.has_protected_content(),
            reply_markup_type,
            serialize_option(sent_message.reply_markup.as_ref()),
            message_result
                .business_connection_id()
                .map(Box::<str>::from)
                .or_else(|| sent_message.business_connection_id.clone()),
            Some(message_type),
            serialize_option(sent_message.link_preview_options.as_ref()),
        )
        .with_media(media_file_id, media_unique_id, media_content_type)
        .with_render_options(
            sent_message.parse_mode.clone(), Self::media_options(sent_message.media.as_ref()),
        )
        .with_message_thread_id(sent_message.message_thread_id)
    }

    fn media_ids(message: &Message) -> (Option<Box<str>>, Option<Box<str>>) {
        if let Some(file_id) = message.file_id() {
            return (
                Some(file_id.into()),
                message.file_unique_id().map(Into::into),
            );
        }
        if let Some(size) = message.photo().and_then(<[_]>::last) {
            return (
                Some(size.file_id.clone()),
                Some(size.file_unique_id.clone()),
            );
        }
        (None, None)
    }

    /// Returns true if old message had reply keyboard.
    fn had_reply_keyboard(old: &OldMessage) -> bool {
        matches!(
            old.reply_markup_type,
            Some(ReplyMarkupType::ReplyKeyboardMarkup)
        )
    }

    /// Returns true if old message had inline keyboard.
    fn had_inline_keyboard(old: &OldMessage) -> bool {
        matches!(
            old.reply_markup_type,
            Some(ReplyMarkupType::InlineKeyboardMarkup)
        )
    }

    /// Returns true if new message requires reply keyboard.
    fn need_reply_keyboard(new: &NewMessage) -> bool {
        matches!(new.reply_markup, Some(ReplyMarkup::ReplyKeyboardMarkup(_)))
    }

    fn replaces_reply_keyboard(new: &NewMessage, old: &OldMessage) -> bool {
        Self::need_reply_keyboard(new)
            && new.chat.id() == old.chat.id()
            && new.business_connection_id == old.business_connection_id
    }

    /// Check if message content or protection flags changed.
    fn message_changed(new: &NewMessage, old: &OldMessage) -> bool {
        let text_changed = new.text.as_ref() != old.text.as_deref().unwrap_or("");
        let markup_changed = serialize_option(new.reply_markup.as_ref()) != old.reply_markup_value
            || new.reply_markup.as_ref().map(ReplyMarkupType::from) != old.reply_markup_type;
        let protect_changed =
            new.protect_content.unwrap_or(false) != old.has_protected_content.unwrap_or(false);
        let parse_mode_changed = new.parse_mode != old.parse_mode;
        let link_preview_changed =
            serialize_option(new.link_preview_options.as_ref()) != old.link_preview_options_value;
        let media_changed = Self::media_changed(new, old);
        let route_changed = Self::route_changed(new, old);

        let changed = text_changed
            || markup_changed
            || protect_changed
            || parse_mode_changed
            || link_preview_changed
            || media_changed
            || route_changed;
        trace!(
            message_id = old.message_id,
            changed,
            text_changed,
            markup_changed,
            media_changed,
            route_changed,
            "Compared dialog message snapshots"
        );
        changed
    }

    fn route_changed(new: &NewMessage, old: &OldMessage) -> bool {
        new.chat.id() != old.chat.id()
            || new.message_thread_id != old.message_thread_id
            || new.business_connection_id != old.business_connection_id
    }

    /// Check if media content changed.
    fn media_changed(new: &NewMessage, old: &OldMessage) -> bool {
        if Self::media_type_changed(new, old)
            || Self::media_options(new.media.as_ref()) != old.media_options_value {
            return true;
        }
        match (&new.media, &old.media_file_id) {
            (None, None) => false,
            (Some(_), None) | (None, Some(_)) => true,
            (Some(new_media), Some(old_file_id)) => match new_media.get_file_id() {
                Some(new_file_id) => new_file_id != old_file_id.as_ref(),
                None => true,
            },
        }
    }

    fn media_options(media: Option<&MediaAttachment>) -> Option<telers::serialization::Value> {
        media.map(|media| telers::serialization::json!({
            "has_spoiler": media.has_spoiler,
            "show_caption_above_media": media.show_caption_above_media,
            "width": media.width,
            "height": media.height,
            "duration": media.duration,
            "performer": media.performer.as_deref(),
            "title": media.title.as_deref(),
            "supports_streaming": media.supports_streaming,
            "emoji": media.emoji.as_deref(),
        }))
    }

    /// Check if the rendered media type changed.
    fn media_type_changed(new: &NewMessage, old: &OldMessage) -> bool {
        match (&new.media, &old.media_content_type) {
            (None, None) => false,
            (Some(_), None) | (None, Some(_)) => true,
            (Some(new_media), Some(old_type)) => !matches!(
                (new_media.content_type, old_type),
                (MediaContentType::Animation, MessageType::Animation)
                    | (MediaContentType::Audio, MessageType::Audio)
                    | (MediaContentType::Document, MessageType::Document)
                    | (MediaContentType::Photo, MessageType::Photo)
                    | (MediaContentType::Video, MessageType::Video)
                    | (MediaContentType::Voice, MessageType::Voice)
                    | (MediaContentType::VideoNote, MessageType::VideoNote)
                    | (MediaContentType::Sticker, MessageType::Sticker)
                    | (MediaContentType::Dice, MessageType::Dice)
            ),
        }
    }

    /// Check if edit is possible without deleting message.
    fn can_edit(new: &NewMessage, old: &OldMessage) -> bool {
        if Self::route_changed(new, old) {
            return false;
        }
        // Telegram edit methods only accept inline markup.
        if matches!(old.reply_markup_type,
            Some(ReplyMarkupType::ReplyKeyboardMarkup | ReplyMarkupType::ForceReply)
        ) || matches!(new.reply_markup,
            Some(ReplyMarkup::ReplyKeyboardMarkup(_) | ReplyMarkup::ForceReply(_) | ReplyMarkup::ReplyKeyboardRemove(_))
        ) {
            return false;
        }
        // These media types have no corresponding InputMedia variant.
        if matches!(
            old.media_content_type,
            Some(MessageType::Voice | MessageType::VideoNote | MessageType::Sticker | MessageType::Dice)
        ) {
            return false;
        }
        if let Some(media) = &new.media {
            if !media.content_type.can_edit_in_place() {
                return false;
            }
        }
        if old.has_media() && !new.has_media() {
            // Cannot turn a media message back into text: `editMessageText` only edits
            // text/rich/game messages. The reverse, text -> media, *is* editable: since Bot
            // API 10.1 `editMessageMedia` can replace a text message with media.
            return false;
        }
        true
    }

    /// Remove reply or inline keyboard depending on mode.
    async fn remove_kbd<Client: Session>(
        bot: &Bot<Client>,
        show_mode: ShowMode,
        old_message: Option<&OldMessage>,
        new_message: Option<&NewMessage>,
    ) -> Result<(), DialogError> {
        if show_mode == ShowMode::NoUpdate {
            return Ok(());
        }
        if show_mode == ShowMode::DeleteAndSend {
            if let Some(old_message) = old_message {
                trace!(
                    message_id = old_message.message_id,
                    "Remove old message before resend"
                );
                Self::remove_message_safe(bot, old_message, new_message).await?;
                return Ok(());
            }
        }
        Self::remove_markup(bot, old_message, new_message).await
    }

    async fn remove_markup<Client: Session>(
        bot: &Bot<Client>,
        old_message: Option<&OldMessage>,
        new_message: Option<&NewMessage>,
    ) -> Result<(), DialogError> {
        if let Some(old) = old_message {
            if Self::had_reply_keyboard(old) {
                if new_message.is_some_and(|new| Self::replaces_reply_keyboard(new, old)) {
                    return Ok(());
                }
                trace!(
                    message_id = old.message_id,
                    "Remove reply keyboard from old message"
                );
                Self::remove_reply_kbd(bot, old).await?;
            } else if Self::had_inline_keyboard(old) {
                trace!(
                    message_id = old.message_id,
                    "Remove inline keyboard from old message"
                );
                Self::remove_inline_kbd(bot, old).await?;
            }
        }
        Ok(())
    }

    /// Remove inline keyboard via edit reply markup.
    async fn remove_inline_kbd<Client: Session>(
        bot: &Bot<Client>,
        old: &OldMessage,
    ) -> Result<(), DialogError> {
        trace!(
            message_id = old.message_id,
            "Editing message to remove inline keyboard"
        );
        match bot
            .send(
                EditMessageReplyMarkup::new()
                    .chat_id(old.chat.id())
                    .message_id(old.message_id)
                    .business_connection_id_option(old.business_connection_id.clone()),
            )
            .await {
            Ok(_) => Ok(()),
            Err(err) if session_bad_request_contains(&err, &[
                "message is not modified", "message can't be edited",
                "message to edit not found", "message_id_invalid",
            ]) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    /// Remove reply keyboard by sending keyboard remove.
    async fn remove_reply_kbd<Client: Session>(
        bot: &Bot<Client>,
        old: &OldMessage,
    ) -> Result<(), DialogError> {
        trace!(chat_id = old.chat.id(), "Sending reply keyboard removal");
        let m = SendMessage::new(old.chat.id(), "...")
            .reply_markup(ReplyKeyboardRemove::new(true))
            .message_thread_id_option(old.message_thread_id)
            .business_connection_id_option(old.business_connection_id.clone());
        let _ = bot.send(m).await?;
        Ok(())
    }

    /// Delete message, ignoring inability to delete.
    async fn remove_message_safe<Client: Session>(
        bot: &Bot<Client>,
        old: &OldMessage,
        new: Option<&NewMessage>,
    ) -> Result<(), DialogError> {
        if old.business_connection_id.is_some() {
            return Self::remove_markup(bot, Some(old), new).await;
        }
        trace!(message_id = old.message_id, "Deleting dialog message");
        match bot
            .send(DeleteMessage::new(old.chat.id(), old.message_id))
            .await {
            Ok(_) => {
                if Self::had_reply_keyboard(old) && !new.is_some_and(|new| Self::replaces_reply_keyboard(new, old)) {
                    Self::remove_reply_kbd(bot, old).await?;
                }
                Ok(())
            }
            Err(err) if session_bad_request_contains(&err, &["message to delete not found", "message_id_invalid"]) => {
                if Self::had_reply_keyboard(old) && !new.is_some_and(|new| Self::replaces_reply_keyboard(new, old)) {
                    Self::remove_reply_kbd(bot, old).await?;
                }
                Ok(())
            }
            Err(err) if session_bad_request_contains(&err, &["message can't be deleted"]) => {
                Self::remove_markup(bot, Some(old), new).await
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Edit message text or media (fallbacks to send if needed).
    async fn edit_message<Client: Session>(
        bot: &Bot<Client>,
        new: NewMessage,
        old: &OldMessage,
    ) -> Result<telers::types::Message, DialogError> {
        if new.protect_content.unwrap_or(false) != old.has_protected_content.unwrap_or(false) {
            trace!(
                message_id = old.message_id,
                "Protect content changed, recreate message"
            );
            Self::remove_message_safe(bot, old, Some(&new)).await?;
            return Self::send_message(bot, new).await;
        }

        if let Some(ref media) = new.media {
            if Self::media_unchanged(media, old)
                && !Self::media_type_changed(&new, old)
                && Self::media_options(Some(media)) == old.media_options_value {
                return Self::edit_caption_message(bot, &new, old).await;
            }
            return Self::edit_media_message(bot, &new, media, old).await;
        }

        // Edit text message
        let mut m = EditMessageText::new()
            .text(new.text.clone())
            .chat_id(old.chat.id())
            .message_id(old.message_id)
            .business_connection_id_option(old.business_connection_id.clone())
            .parse_mode_option(new.parse_mode.clone())
            .link_preview_options_option(new.link_preview_options.clone());
        if let Some(ReplyMarkup::InlineKeyboardMarkup(kb)) = new.reply_markup {
            m = m.reply_markup(kb);
        }
        match bot.send(m).await? {
            Either::Left(msg) => Ok(msg),
            Either::Right(_) => {
                unreachable!("EditMessageText should return Message")
            }
        }
    }

    fn media_unchanged(media: &MediaAttachment, old: &OldMessage) -> bool {
        match (media.get_file_id(), old.media_file_id.as_deref()) {
            (Some(new_id), Some(old_id)) => new_id == old_id,
            _ => false,
        }
    }

    async fn edit_caption_message<Client: Session>(
        bot: &Bot<Client>,
        new: &NewMessage,
        old: &OldMessage,
    ) -> Result<telers::types::Message, DialogError> {
        let mut m = EditMessageCaption::new()
            .chat_id(old.chat.id())
            .message_id(old.message_id)
            .business_connection_id_option(old.business_connection_id.clone())
            .parse_mode_option(new.parse_mode.clone())
            .show_caption_above_media_option(
                new.media.as_ref().and_then(|media| media.show_caption_above_media),
            );
        if !new.text.is_empty() {
            m = m.caption(new.text.clone());
        }
        if let Some(ReplyMarkup::InlineKeyboardMarkup(ref kb)) = new.reply_markup {
            m = m.reply_markup(kb.clone());
        }
        match bot.send(m).await? {
            Either::Left(msg) => Ok(msg),
            Either::Right(_) => unreachable!("EditMessageCaption should return Message"),
        }
    }

    /// Edit a media message using `editMessageMedia`.
    async fn edit_media_message<Client: Session>(
        bot: &Bot<Client>,
        new: &NewMessage,
        media: &MediaAttachment,
        old: &OldMessage,
    ) -> Result<telers::types::Message, DialogError> {
        let Some(input_file) = media.to_input_file() else {
            trace!(
                message_id = old.message_id,
                "No valid media source, falling back to text edit"
            );
            // Fall back to text edit if no valid media
            let mut m = EditMessageText::new()
                .text(new.text.clone())
                .chat_id(old.chat.id())
                .message_id(old.message_id)
                .business_connection_id_option(old.business_connection_id.clone())
                .parse_mode_option(new.parse_mode.clone());
            if let Some(ReplyMarkup::InlineKeyboardMarkup(ref kb)) = new.reply_markup {
                m = m.reply_markup(kb.clone());
            }
            return match bot.send(m).await? {
                Either::Left(msg) => Ok(msg),
                Either::Right(_) => unreachable!("EditMessageText should return Message"),
            };
        };

        let input_media =
            Self::build_input_media(media, input_file, &new.text, new.parse_mode.as_deref());

        let mut m = EditMessageMedia::new(input_media)
            .chat_id(old.chat.id())
            .message_id(old.message_id)
            .business_connection_id_option(old.business_connection_id.clone());

        if let Some(ReplyMarkup::InlineKeyboardMarkup(ref kb)) = new.reply_markup {
            m = m.reply_markup(kb.clone());
        }

        match bot.send(m).await? {
            Either::Left(msg) => Ok(msg),
            Either::Right(_) => unreachable!("EditMessageMedia should return Message"),
        }
    }

    /// Build an `InputMedia` from a `MediaAttachment`.
    fn build_input_media(
        media: &MediaAttachment,
        input_file: InputFile,
        caption: &str,
        parse_mode: Option<&str>,
    ) -> InputMedia {
        let caption_opt = (!caption.is_empty()).then_some(caption);

        match media.content_type {
            MediaContentType::Photo => {
                let m = InputMediaPhoto::new(input_file)
                    .caption_option(caption_opt)
                    .parse_mode_option(parse_mode)
                    .has_spoiler_option(media.has_spoiler)
                    .show_caption_above_media_option(media.show_caption_above_media);
                InputMedia::Photo(m)
            }
            MediaContentType::Video => {
                let m = InputMediaVideo::new(input_file)
                    .caption_option(caption_opt)
                    .parse_mode_option(parse_mode)
                    .has_spoiler_option(media.has_spoiler)
                    .show_caption_above_media_option(media.show_caption_above_media)
                    .width_option(media.width)
                    .height_option(media.height)
                    .duration_option(media.duration)
                    .supports_streaming_option(media.supports_streaming);
                InputMedia::Video(m)
            }
            MediaContentType::Audio => {
                let m = InputMediaAudio::new(input_file)
                    .caption_option(caption_opt)
                    .parse_mode_option(parse_mode)
                    .duration_option(media.duration)
                    .performer_option(media.performer.as_deref())
                    .title_option(media.title.as_deref());
                InputMedia::Audio(m)
            }
            MediaContentType::Document => {
                let m = InputMediaDocument::new(input_file)
                    .caption_option(caption_opt)
                    .parse_mode_option(parse_mode);
                InputMedia::Document(m)
            }
            MediaContentType::Animation => {
                let m = InputMediaAnimation::new(input_file)
                    .caption_option(caption_opt)
                    .parse_mode_option(parse_mode)
                    .has_spoiler_option(media.has_spoiler)
                    .show_caption_above_media_option(media.show_caption_above_media)
                    .width_option(media.width)
                    .height_option(media.height)
                    .duration_option(media.duration);
                InputMedia::Animation(m)
            }
            // Guarded by `can_edit` before editing.
            MediaContentType::Voice | MediaContentType::VideoNote
            | MediaContentType::Sticker | MediaContentType::Dice => {
                unreachable!("noneditable media must be sent as a new message")
            }
        }
    }

    /// Send text or media message.
    async fn send_message<Client: Session>(
        bot: &Bot<Client>,
        msg: NewMessage,
    ) -> Result<telers::types::Message, DialogError> {
        // Send media message if media is present
        if let Some(ref media) = msg.media {
            return Self::send_media_message(bot, &msg, media).await;
        }

        // Send text message
        trace!(
            chat_id = msg.chat.id(),
            text_len = msg.text.len(),
            show_mode = ?msg.show_mode,
            "Sending dialog text message"
        );
        Ok(bot
            .send(
                SendMessage::new(msg.chat.id(), msg.text)
                    .reply_markup_option(msg.reply_markup)
                    .parse_mode_option(msg.parse_mode)
                    .link_preview_options_option(msg.link_preview_options)
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id)
                    .message_thread_id_option(msg.message_thread_id),
            )
            .await?)
    }

    /// Send a media message.
    async fn send_media_message<Client: Session>(
        bot: &Bot<Client>,
        msg: &NewMessage,
        media: &MediaAttachment,
    ) -> Result<telers::types::Message, DialogError> {
        if media.content_type == MediaContentType::Dice {
            return Ok(bot.send(SendDice::new(msg.chat.id())
                .emoji_option(media.emoji.as_deref())
                .reply_markup_option(msg.reply_markup.clone())
                .protect_content_option(msg.protect_content)
                .business_connection_id_option(msg.business_connection_id.clone())
                .message_thread_id_option(msg.message_thread_id)).await?);
        }
        let Some(input_file) = media.to_input_file() else {
            trace!(
                chat_id = msg.chat.id(),
                "No valid media source, falling back to text message"
            );
            return Ok(bot
                .send(
                    SendMessage::new(msg.chat.id(), msg.text.clone())
                        .reply_markup_option(msg.reply_markup.clone())
                        .parse_mode_option(msg.parse_mode.clone())
                        .link_preview_options_option(msg.link_preview_options.clone())
                        .protect_content_option(msg.protect_content)
                        .business_connection_id_option(msg.business_connection_id.clone())
                        .message_thread_id_option(msg.message_thread_id),
                )
                .await?);
        };

        let caption = (!msg.text.is_empty()).then_some(msg.text.as_ref());

        trace!(
            chat_id = msg.chat.id(),
            media_type = ?media.content_type,
            has_caption = caption.is_some(),
            "Sending dialog media message"
        );

        match media.content_type {
            MediaContentType::Photo => {
                let m = SendPhoto::new(msg.chat.id(), input_file)
                    .caption_option(caption)
                    .parse_mode_option(msg.parse_mode.clone())
                    .has_spoiler_option(media.has_spoiler)
                    .show_caption_above_media_option(media.show_caption_above_media)
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Video => {
                let m = SendVideo::new(msg.chat.id(), input_file)
                    .caption_option(caption)
                    .parse_mode_option(msg.parse_mode.clone())
                    .has_spoiler_option(media.has_spoiler)
                    .show_caption_above_media_option(media.show_caption_above_media)
                    .width_option(media.width)
                    .height_option(media.height)
                    .duration_option(media.duration)
                    .supports_streaming_option(media.supports_streaming)
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Audio => {
                let m = SendAudio::new(msg.chat.id(), input_file)
                    .caption_option(caption)
                    .parse_mode_option(msg.parse_mode.clone())
                    .duration_option(media.duration)
                    .performer_option(media.performer.as_deref())
                    .title_option(media.title.as_deref())
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Document => {
                let m = SendDocument::new(msg.chat.id(), input_file)
                    .caption_option(caption)
                    .parse_mode_option(msg.parse_mode.clone())
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Voice => {
                let m = SendVoice::new(msg.chat.id(), input_file)
                    .caption_option(caption)
                    .parse_mode_option(msg.parse_mode.clone())
                    .duration_option(media.duration)
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::VideoNote => {
                let m = SendVideoNote::new(msg.chat.id(), input_file)
                    .duration_option(media.duration)
                    .length_option(media.width)
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Sticker => {
                let m = SendSticker::new(msg.chat.id(), input_file)
                    .emoji_option(media.emoji.as_deref())
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Dice => unreachable!("dice is sent without a media source"),
            MediaContentType::Animation => {
                let m = SendAnimation::new(msg.chat.id(), input_file)
                    .caption_option(caption)
                    .parse_mode_option(msg.parse_mode.clone())
                    .has_spoiler_option(media.has_spoiler)
                    .show_caption_above_media_option(media.show_caption_above_media)
                    .width_option(media.width)
                    .height_option(media.height)
                    .duration_option(media.duration)
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
        }
    }
}

fn bad_request_contains(error: &DialogError, descriptions: &[&str]) -> bool {
    matches!(error, DialogError::SessionErrorKind(error) if session_bad_request_contains(error, descriptions))
}

fn session_bad_request_contains(error: &SessionErrorKind, descriptions: &[&str]) -> bool {
    matches!(error, SessionErrorKind::Telegram(TelegramErrorKind::BadRequest { message })
        if descriptions.iter().any(|description| message.to_ascii_lowercase().contains(description)))
}

fn serialize_option<T>(value: Option<&T>) -> Option<telers::serialization::Value>
where
    T: Serialize,
{
    value.and_then(|value| telers::serialization::to_value(value).ok())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::MessageManager;
    use crate::{
        entities::{NewMessage, OldMessage, ShowMode},
        widgets::media::{MediaAttachment, MediaContentType, MediaId},
        DialogError,
    };
    use telers::{
        client::{
            session::{ClientResponse, ClientStreamResponse},
            telegram::APIServer,
            Reqwest, Session,
        },
        enums::{MessageType, ReplyMarkupType},
        methods::TelegramMethod,
        serialization::{json, Value},
        types::{
            ChatPrivate, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton,
            LinkPreviewOptions, Message, MessagePhoto, MessageText, PhotoSize, ReplyKeyboardMarkup,
            ReplyMarkup, ForceReply,
        },
        Bot,
    };
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex, OnceLock},
    };

    const TEXT_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":1,"type":"private"},"text":"updated"}}"#;
    const OTHER_CHAT_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":2,"type":"private"},"text":"updated"}}"#;
    const PHOTO_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":1,"type":"private"},"photo":[{"file_id":"file-1","file_unique_id":"unique-1","width":50,"height":50}]}}"#;
    const VIDEO_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":1,"type":"private"},"video":{"file_id":"file-1","file_unique_id":"unique-1","width":50,"height":50,"duration":2}}}"#;
    const VOICE_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":1,"type":"private"},"voice":{"file_id":"voice-1","file_unique_id":"unique-1","duration":2}}}"#;
    const VIDEO_NOTE_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":1,"type":"private"},"video_note":{"file_id":"note-1","file_unique_id":"unique-1","duration":2,"length":50}}}"#;
    const STICKER_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":1,"type":"private"},"sticker":{"file_id":"sticker-1","file_unique_id":"unique-1","type":"regular","width":50,"height":50,"is_animated":false,"is_video":false}}}"#;
    const DICE_RESPONSE: &str = r#"{"ok":true,"result":{"message_id":20,"date":1,"chat":{"id":1,"type":"private"},"dice":{"emoji":"🎯","value":4}}}"#;
    const TRUE_RESPONSE: &str = r#"{"ok":true,"result":true}"#;
    const MISSING_EDIT_RESPONSE: &str = r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#;
    const UNCHANGED_RESPONSE: &str = r#"{"ok":false,"error_code":400,"description":"Bad Request: message is not modified"}"#;
    const CANNOT_DELETE_RESPONSE: &str = r#"{"ok":false,"error_code":400,"description":"Bad Request: message can't be deleted"}"#;
    const INVALID_RESPONSE: &str = r#"{"ok":false,"error_code":400,"description":"Bad Request: can't parse entities"}"#;

    #[derive(Clone)]
    pub(crate) struct MockSession {
        responses: Arc<Mutex<VecDeque<ClientResponse>>>,
        requests: Arc<Mutex<Vec<(&'static str, Value)>>>,
    }

    impl Session for MockSession {
        fn api(&self) -> &APIServer {
            static API: OnceLock<APIServer> = OnceLock::new();
            API.get_or_init(APIServer::default)
        }

        async fn send_request<Client, T>(
            &self,
            bot: &Bot<Client>,
            method: T,
            _timeout: Option<f32>,
        ) -> Result<ClientResponse, anyhow::Error>
        where
            Client: Session,
            T: TelegramMethod + Send + Sync,
            T::Method: Send + Sync,
        {
            let request = method.build_request(bot);
            self.requests.lock().expect("requests").push((
                request.method_name,
                telers::serialization::to_value(&request.data).expect("serialize request"),
            ));
            Ok(self
                .responses
                .lock()
                .expect("responses")
                .pop_front()
                .expect("prepared response"))
        }

        async fn stream_content(
            &self,
            _url: &str,
            _timeout: Option<f32>,
        ) -> Result<ClientStreamResponse, anyhow::Error> {
            panic!("dialog tests do not download media")
        }
    }

    pub(crate) struct MockBot {
        bot: Bot<MockSession>,
        requests: Arc<Mutex<Vec<(&'static str, Value)>>>,
    }

    impl std::ops::Deref for MockBot {
        type Target = Bot<MockSession>;

        fn deref(&self) -> &Self::Target {
            &self.bot
        }
    }

    pub(crate) fn mock_bot(responses: &[(u16, &str)]) -> MockBot {
        let session = MockSession {
            responses: Arc::new(Mutex::new(
                responses
                    .iter()
                    .map(|(status, content)| ClientResponse::new(*status, *content))
                    .collect(),
            )),
            requests: Arc::new(Mutex::new(Vec::new())),
        };
        MockBot {
            requests: session.requests.clone(),
            bot: Bot::with_client("1:test", session),
        }
    }

    pub(crate) fn requests(bot: &MockBot) -> Vec<(&'static str, Value)> {
        bot.requests.lock().expect("requests").clone()
    }

    fn old_message(reply_markup_type: Option<ReplyMarkupType>) -> OldMessage {
        OldMessage::new(
            ChatPrivate::new(1),
            10,
            Some("hello"),
            None::<bool>,
            reply_markup_type,
            None,
            None::<Box<str>>,
            None,
            None,
        )
    }

    fn new_message(reply_markup: Option<ReplyMarkup>) -> NewMessage {
        NewMessage::new(
            ChatPrivate::new(1),
            None,
            None::<Box<str>>,
            "hello",
            reply_markup,
            None::<Box<str>>,
            None,
            ShowMode::Edit,
            None,
        )
    }

    fn old_media_message(media: &MediaAttachment) -> OldMessage {
        let message_type = match media.content_type {
            MediaContentType::Animation => MessageType::Animation,
            MediaContentType::Audio => MessageType::Audio,
            MediaContentType::Document => MessageType::Document,
            MediaContentType::Photo => MessageType::Photo,
            MediaContentType::Video => MessageType::Video,
            MediaContentType::VideoNote => MessageType::VideoNote,
            MediaContentType::Voice => MessageType::Voice,
            MediaContentType::Sticker => MessageType::Sticker,
            MediaContentType::Dice => MessageType::Dice,
        };
        old_message(None).with_media(
            media.get_file_id().map(ToOwned::to_owned),
            media.get_file_unique_id().map(ToOwned::to_owned),
            Some(message_type),
        )
        .with_render_options(None, MessageManager::media_options(Some(media)))
    }

    #[tokio::test]
    async fn detects_reply_and_inline_keyboards_separately() {
        let no_markup = old_message(None);
        let reply_keyboard = old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup));
        let inline_keyboard = old_message(Some(ReplyMarkupType::InlineKeyboardMarkup));
        let force_reply = old_message(Some(ReplyMarkupType::ForceReply));

        assert!(!MessageManager::had_reply_keyboard(&no_markup));
        assert!(!MessageManager::had_inline_keyboard(&no_markup));

        assert!(MessageManager::had_reply_keyboard(&reply_keyboard));
        assert!(!MessageManager::had_inline_keyboard(&reply_keyboard));

        assert!(!MessageManager::had_reply_keyboard(&inline_keyboard));
        assert!(MessageManager::had_inline_keyboard(&inline_keyboard));

        // A `ForceReply` message has no attached keyboard to clear, so it must
        // be treated as neither a reply nor an inline keyboard (otherwise a
        // transition would try a bogus `EditMessageReplyMarkup`).
        assert!(!MessageManager::had_reply_keyboard(&force_reply));
        assert!(!MessageManager::had_inline_keyboard(&force_reply));
    }

    #[tokio::test]
    async fn detects_when_new_message_needs_reply_keyboard() {
        let reply_keyboard = new_message(Some(
            ReplyKeyboardMarkup::new([[KeyboardButton::new("OK")]]).into(),
        ));
        let inline_keyboard = new_message(Some(
            InlineKeyboardMarkup::new([[
                InlineKeyboardButton::new("Open").callback_data("td:intent:open")
            ]])
            .into(),
        ));
        let no_markup = new_message(None);

        assert!(MessageManager::need_reply_keyboard(&reply_keyboard));
        assert!(!MessageManager::need_reply_keyboard(&inline_keyboard));
        assert!(!MessageManager::need_reply_keyboard(&no_markup));
    }

    #[tokio::test]
    async fn message_changed_detects_protect_content_and_link_preview_updates() {
        let mut protected = new_message(None);
        protected.protect_content = Some(true);
        let old = old_message(None);
        assert!(MessageManager::message_changed(&protected, &old));

        let mut linked = new_message(None);
        linked.link_preview_options = Some(LinkPreviewOptions::new().show_above_text(true));
        assert!(MessageManager::message_changed(&linked, &old));
    }

    #[tokio::test]
    async fn message_changed_detects_media_file_change() {
        let old_media = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-1"))
            .build();
        let new_media = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-2"))
            .build();
        let new = new_message(None).with_media(Some(new_media));

        assert!(MessageManager::message_changed(
            &new,
            &old_media_message(&old_media)
        ));
    }

    #[tokio::test]
    async fn can_edit_allows_text_to_media_but_not_media_to_text() {
        let photo = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-1"))
            .build();

        // text (old) -> media (new): editable via editMessageMedia (Bot API 10.1).
        let to_media = new_message(None).with_media(Some(photo.clone()));
        assert!(MessageManager::can_edit(&to_media, &old_message(None)));

        // media (old) -> text (new): not editable, must resend.
        let to_text = new_message(None);
        assert!(!MessageManager::can_edit(
            &to_text,
            &old_media_message(&photo)
        ));

        // media -> media of the same type stays editable.
        assert!(MessageManager::can_edit(
            &to_media,
            &old_media_message(&photo)
        ));
    }

    #[tokio::test]
    async fn can_edit_forces_resend_for_voice_and_video_note() {
        // `editMessageMedia` cannot produce a voice/video note (it would fall back to a photo), so
        // any transition where the new *or* old message is one must resend rather than edit.
        let voice = MediaAttachment::builder(MediaContentType::Voice)
            .file_id(MediaId::new("v-1"))
            .build();
        let video_note = MediaAttachment::builder(MediaContentType::VideoNote)
            .file_id(MediaId::new("vn-1"))
            .build();

        // text (old) -> voice / video note (new): must resend, even though the old message has no
        // media (this is the case the missing guard used to let through into an edit -> photo).
        assert!(!MessageManager::can_edit(
            &new_message(None).with_media(Some(voice.clone())),
            &old_message(None),
        ));
        assert!(!MessageManager::can_edit(
            &new_message(None).with_media(Some(video_note.clone())),
            &old_message(None),
        ));

        // voice / video note (old) -> same type (new): still must resend.
        assert!(!MessageManager::can_edit(
            &new_message(None).with_media(Some(voice.clone())),
            &old_media_message(&voice),
        ));
        assert!(!MessageManager::can_edit(
            &new_message(None).with_media(Some(video_note.clone())),
            &old_media_message(&video_note),
        ));
    }

    #[tokio::test]
    async fn media_unchanged_matches_only_equal_file_ids() {
        let media = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-1"))
            .build();
        let old = old_media_message(&media);
        assert!(MessageManager::media_unchanged(&media, &old));

        let other = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-2"))
            .build();
        assert!(!MessageManager::media_unchanged(&other, &old));

        let url_media = MediaAttachment::builder(MediaContentType::Photo)
            .url("http://x/a.jpg")
            .build();
        assert!(!MessageManager::media_unchanged(&url_media, &old));
    }

    #[tokio::test]
    async fn can_edit_rejects_old_or_new_reply_keyboard() {
        let old_reply = old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup));
        let old_inline = old_message(Some(ReplyMarkupType::InlineKeyboardMarkup));
        let new_reply = new_message(Some(
            ReplyKeyboardMarkup::new([[KeyboardButton::new("OK")]]).into(),
        ));
        let new_inline = new_message(Some(
            InlineKeyboardMarkup::new([[
                InlineKeyboardButton::new("Open").callback_data("td:intent:open")
            ]])
            .into(),
        ));

        assert!(!MessageManager::can_edit(&new_inline, &old_reply));
        assert!(!MessageManager::can_edit(&new_reply, &old_inline));
        assert!(MessageManager::can_edit(&new_inline, &old_inline));
    }

    #[tokio::test]
    async fn show_message_returns_existing_snapshot_for_no_update() {
        let bot = Bot::<Reqwest>::default();
        let old = old_message(Some(ReplyMarkupType::InlineKeyboardMarkup));
        let mut new = new_message(None);
        new.show_mode = ShowMode::NoUpdate;

        let shown = MessageManager::show_message(&bot, new, Some(old.clone()))
            .await
            .expect("reuse previous snapshot");

        assert_eq!(shown.message_id, old.message_id);
        assert_eq!(shown.text, old.text);
    }

    #[tokio::test]
    async fn show_message_without_snapshot_fails_for_no_update() {
        let bot = Bot::<Reqwest>::default();
        let mut new = new_message(None);
        new.show_mode = ShowMode::NoUpdate;

        let err = MessageManager::show_message(&bot, new, None)
            .await
            .expect_err("NoUpdate without old message must fail");

        assert!(matches!(err, DialogError::DialogNotFound));
    }

    #[test]
    fn media_ids_uses_largest_photo_size() {
        let photo: Message = MessagePhoto::new(
            1,
            0,
            ChatPrivate::new(1),
            [
                PhotoSize::new("small-id", "small-uniq", 90, 90),
                PhotoSize::new("large-id", "large-uniq", 600, 400),
            ],
        )
        .into();

        let (file_id, unique_id) = MessageManager::media_ids(&photo);

        assert_eq!(file_id.as_deref(), Some("large-id"));
        assert_eq!(unique_id.as_deref(), Some("large-uniq"));
    }

    #[test]
    fn media_ids_none_for_text_message() {
        let text: Message = MessageText::new(1, 0, ChatPrivate::new(1), "hi").into();

        let (file_id, unique_id) = MessageManager::media_ids(&text);

        assert!(file_id.is_none());
        assert!(unique_id.is_none());
    }

    #[test]
    fn explicit_false_protection_matches_omitted_protection() {
        let mut new = new_message(None);
        new.protect_content = Some(false);
        assert!(!MessageManager::message_changed(&new, &old_message(None)));
        let mut old = old_message(None);
        old.has_protected_content = Some(false);
        new.protect_content = None;
        assert!(!MessageManager::message_changed(&new, &old));
    }

    #[tokio::test]
    async fn missing_edited_message_is_replaced() {
        let bot = mock_bot(&[(400, MISSING_EDIT_RESPONSE), (200, TEXT_RESPONSE)]);
        let mut new = new_message(None);
        new.text = "updated".into();
        let shown = MessageManager::show_message(&bot, new, Some(old_message(None)))
            .await
            .expect("replacement");
        assert_eq!(shown.message_id, 20);
        assert_eq!(
            requests(&bot)
                .iter()
                .map(|request| request.0)
                .collect::<Vec<_>>(),
            ["editMessageText", "sendMessage"],
        );
    }

    #[tokio::test]
    async fn telegram_not_modified_preserves_existing_snapshot() {
        let bot = mock_bot(&[(400, UNCHANGED_RESPONSE)]);
        let mut new = new_message(None);
        new.text = "updated".into();
        let shown = MessageManager::show_message(&bot, new, Some(old_message(None)))
            .await
            .expect("unchanged");
        assert_eq!(shown.message_id, 10);
        assert_eq!(requests(&bot).len(), 1);
    }

    #[tokio::test]
    async fn unrelated_edit_errors_remain_visible() {
        let bot = mock_bot(&[(400, INVALID_RESPONSE)]);
        let mut new = new_message(None);
        new.text = "updated".into();
        assert!(MessageManager::show_message(&bot, new, Some(old_message(None)))
            .await
            .is_err());
        assert_eq!(requests(&bot).len(), 1);
    }

    #[tokio::test]
    async fn undeletable_message_is_cleaned_before_resending() {
        let bot = mock_bot(&[
            (400, CANNOT_DELETE_RESPONSE),
            (400, MISSING_EDIT_RESPONSE),
            (200, TEXT_RESPONSE),
        ]);
        let mut new = new_message(None);
        new.show_mode = ShowMode::DeleteAndSend;
        MessageManager::show_message(
            &bot,
            new,
            Some(old_message(Some(ReplyMarkupType::InlineKeyboardMarkup))),
        )
        .await
        .expect("replacement");
        assert_eq!(
            requests(&bot)
                .iter()
                .map(|request| request.0)
                .collect::<Vec<_>>(),
            ["deleteMessage", "editMessageReplyMarkup", "sendMessage"],
        );
    }

    #[tokio::test]
    async fn leaving_reply_keyboard_removes_it_even_after_deleting_message() {
        let bot = mock_bot(&[
            (200, TRUE_RESPONSE),
            (200, TEXT_RESPONSE),
            (200, TEXT_RESPONSE),
        ]);
        MessageManager::show_message(
            &bot,
            new_message(None),
            Some(old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup))),
        )
        .await
        .expect("replacement");
        let sent = requests(&bot);
        assert_eq!(
            sent.iter().map(|request| request.0).collect::<Vec<_>>(),
            ["deleteMessage", "sendMessage", "sendMessage"],
        );
        assert_eq!(
            sent[1]
                .1
                .get("reply_markup")
                .and_then(|markup| markup.get("remove_keyboard")),
            Some(&json!(true)),
        );
    }

    #[tokio::test]
    async fn replacing_reply_keyboard_does_not_send_removal_message() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE)]);
        let mut new = new_message(Some(
            ReplyKeyboardMarkup::new([[KeyboardButton::new("New")]]).into(),
        ));
        new.show_mode = ShowMode::Send;
        MessageManager::show_message(
            &bot,
            new,
            Some(old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup))),
        )
        .await
        .expect("send");
        assert_eq!(
            requests(&bot)
                .iter()
                .map(|request| request.0)
                .collect::<Vec<_>>(),
            ["sendMessage"],
        );
    }

    #[tokio::test]
    async fn closing_business_message_clears_markup_without_ordinary_delete() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE)]);
        let mut old = old_message(Some(ReplyMarkupType::InlineKeyboardMarkup));
        old.business_connection_id = Some("business".into());
        MessageManager::close_message(&bot, ShowMode::DeleteAndSend, Some(&old))
            .await
            .expect("close");
        let sent = requests(&bot);
        assert_eq!(sent[0].0, "editMessageReplyMarkup");
        assert_eq!(
            sent[0].1.get("business_connection_id"),
            Some(&json!("business")),
        );
        assert_eq!(sent.len(), 1);
    }

    #[test]
    fn force_reply_and_keyboard_removal_require_new_message() {
        let force_reply = new_message(Some(ForceReply::new(true).into()));
        assert!(!MessageManager::can_edit(&force_reply, &old_message(None)));
        assert!(!MessageManager::can_edit(
            &new_message(None),
            &old_message(Some(ReplyMarkupType::ForceReply)),
        ));
    }

    #[tokio::test]
    async fn voice_and_video_notes_use_their_telegram_methods() {
        for (content_type, response, method) in [
            (MediaContentType::Voice, VOICE_RESPONSE, "sendVoice"),
            (
                MediaContentType::VideoNote,
                VIDEO_NOTE_RESPONSE,
                "sendVideoNote",
            ),
        ] {
            let bot = mock_bot(&[(200, response)]);
            let media = MediaAttachment::builder(content_type)
                .file_id(MediaId::new("media-1"))
                .duration(2)
                .width(50)
                .build();
            let shown = MessageManager::show_message(
                &bot,
                new_message(None).with_media(Some(media)),
                None,
            )
            .await
            .expect("send media");
            assert_eq!(requests(&bot)[0].0, method);
            assert!(shown.has_media());
        }
    }

    #[tokio::test]
    async fn stickers_and_dice_use_native_methods_and_keep_media_snapshots() {
        for (content_type, response, method) in [
            (MediaContentType::Sticker, STICKER_RESPONSE, "sendSticker"),
            (MediaContentType::Dice, DICE_RESPONSE, "sendDice"),
        ] {
            let bot = mock_bot(&[(200, response)]);
            let media = if content_type == MediaContentType::Sticker {
                MediaAttachment::builder(content_type)
                    .file_id(MediaId::new("sticker-1"))
                    .build()
            } else {
                MediaAttachment::builder(content_type).emoji("🎯").build()
            };
            let shown = MessageManager::show_message(
                &bot,
                new_message(None).with_media(Some(media)),
                None,
            )
            .await
            .expect("native media");
            assert_eq!(requests(&bot)[0].0, method);
            assert!(shown.has_media());
            assert!(shown.media_options_value.is_some());
        }
    }

    #[tokio::test]
    async fn invalid_media_source_is_rendered_as_text() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE)]);
        let invalid = MediaAttachment::builder(MediaContentType::Photo).build();
        let shown = MessageManager::show_message(
            &bot,
            new_message(None).with_media(Some(invalid)),
            None,
        )
        .await
        .expect("text fallback");
        assert_eq!(requests(&bot)[0].0, "sendMessage");
        assert!(!shown.has_media());
        assert!(shown.media_options_value.is_none());
    }

    #[tokio::test]
    async fn media_caption_and_parse_mode_override_window_defaults() {
        let bot = mock_bot(&[(200, PHOTO_RESPONSE)]);
        let media = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-1"))
            .caption("<b>Media</b>")
            .parse_mode("HTML")
            .build();
        let shown = MessageManager::show_message(
            &bot,
            new_message(None).with_media(Some(media)),
            None,
        )
        .await
        .expect("send media");
        let sent = requests(&bot);
        assert_eq!(sent[0].1.get("caption"), Some(&json!("<b>Media</b>")));
        assert_eq!(sent[0].1.get("parse_mode"), Some(&json!("HTML")));
        assert_eq!(shown.text.as_deref(), Some("<b>Media</b>"));
        assert_eq!(shown.parse_mode.as_deref(), Some("HTML"));
    }

    #[tokio::test]
    async fn parse_mode_change_updates_equal_text() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE)]);
        let mut new = new_message(None);
        new.parse_mode = Some("HTML".into());
        MessageManager::show_message(&bot, new, Some(old_message(None)))
            .await
            .expect("update parse mode");
        assert_eq!(requests(&bot)[0].0, "editMessageText");
    }

    #[tokio::test]
    async fn media_option_changes_edit_media_with_same_file_id() {
        let bot = mock_bot(&[(200, PHOTO_RESPONSE)]);
        let media = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-1"))
            .build();
        let old = old_media_message(&media);
        let mut changed = media;
        changed.has_spoiler = Some(true);
        MessageManager::show_message(
            &bot,
            new_message(None).with_media(Some(changed)),
            Some(old),
        )
        .await
        .expect("update media options");
        let sent = requests(&bot);
        assert_eq!(sent[0].0, "editMessageMedia");
        assert_eq!(
            sent[0].1.get("media").and_then(|media| media.get("has_spoiler")),
            Some(&json!(true)),
        );
    }

    #[test]
    fn same_file_id_with_different_media_type_is_changed_and_editable() {
        let photo = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-1"))
            .build();
        let video = MediaAttachment::builder(MediaContentType::Video)
            .file_id(MediaId::new("file-1"))
            .build();
        let old = old_media_message(&photo);
        let new = new_message(None).with_media(Some(video));
        assert!(MessageManager::message_changed(&new, &old));
        assert!(MessageManager::can_edit(&new, &old));
    }

    #[tokio::test]
    async fn media_type_change_uses_media_edit_instead_of_caption_edit() {
        let bot = mock_bot(&[(200, VIDEO_RESPONSE)]);
        let photo = MediaAttachment::builder(MediaContentType::Photo)
            .file_id(MediaId::new("file-1"))
            .build();
        let video = MediaAttachment::builder(MediaContentType::Video)
            .file_id(MediaId::new("file-1"))
            .build();
        let shown = MessageManager::show_message(
            &bot,
            new_message(None).with_media(Some(video)),
            Some(old_media_message(&photo)),
        )
        .await
        .expect("change media type");
        let sent = requests(&bot);
        assert_eq!(sent[0].0, "editMessageMedia");
        assert_eq!(
            sent[0].1.get("media").and_then(|media| media.get("type")),
            Some(&json!("video")),
        );
        assert_eq!(shown.media_content_type, Some(MessageType::Video));
    }

    #[test]
    fn unchanged_noneditable_media_does_not_need_resending() {
        for content_type in [
            MediaContentType::Voice,
            MediaContentType::VideoNote,
            MediaContentType::Sticker,
        ] {
            let media = MediaAttachment::builder(content_type)
                .file_id(MediaId::new("file-1"))
                .build();
            let new = new_message(None).with_media(Some(media.clone()));
            assert!(!MessageManager::message_changed(
                &new,
                &old_media_message(&media),
            ));
            assert!(!MessageManager::can_edit(&new, &old_media_message(&media)));
        }
    }

    #[tokio::test]
    async fn equal_content_in_another_chat_is_sent_to_the_new_chat() {
        let bot = mock_bot(&[(200, TRUE_RESPONSE), (200, OTHER_CHAT_RESPONSE)]);
        let mut new = new_message(None);
        new.chat = ChatPrivate::new(2).into();
        let shown = MessageManager::show_message(&bot, new, Some(old_message(None)))
            .await
            .expect("new chat");
        let sent = requests(&bot);
        assert_eq!(sent[0].0, "deleteMessage");
        assert_eq!(sent[0].1.get("chat_id"), Some(&json!(1)));
        assert_eq!(sent[1].0, "sendMessage");
        assert_eq!(sent[1].1.get("chat_id"), Some(&json!(2)));
        assert_eq!(shown.chat.id(), 2);
    }

    #[tokio::test]
    async fn equal_content_in_another_thread_is_resent_and_snapshotted() {
        let bot = mock_bot(&[(200, TRUE_RESPONSE), (200, TEXT_RESPONSE)]);
        let old = old_message(None).with_message_thread_id(Some(50));
        let mut new = new_message(None);
        new.message_thread_id = Some(100);
        let shown = MessageManager::show_message(&bot, new, Some(old))
            .await
            .expect("new thread");
        let sent = requests(&bot);
        assert_eq!(sent[1].0, "sendMessage");
        assert_eq!(sent[1].1.get("message_thread_id"), Some(&json!(100)));
        assert_eq!(shown.message_thread_id, Some(100));
    }

    #[tokio::test]
    async fn business_route_change_cleans_the_old_connection_and_sends_on_the_new_one() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE), (200, TEXT_RESPONSE)]);
        let mut old = old_message(Some(ReplyMarkupType::InlineKeyboardMarkup));
        old.business_connection_id = Some("old-business".into());
        let mut new = new_message(None);
        new.business_connection_id = Some("new-business".into());
        let shown = MessageManager::show_message(&bot, new, Some(old))
            .await
            .expect("new business connection");
        let sent = requests(&bot);
        assert_eq!(sent[0].0, "editMessageReplyMarkup");
        assert_eq!(
            sent[0].1.get("business_connection_id"),
            Some(&json!("old-business")),
        );
        assert_eq!(sent[1].0, "sendMessage");
        assert_eq!(
            sent[1].1.get("business_connection_id"),
            Some(&json!("new-business")),
        );
        assert_eq!(shown.business_connection_id.as_deref(), Some("new-business"));
    }

    #[tokio::test]
    async fn reply_keyboard_cleanup_uses_its_original_thread() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE)]);
        let old = old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup))
            .with_message_thread_id(Some(50));
        MessageManager::close_message(&bot, ShowMode::Edit, Some(&old))
            .await
            .expect("cleanup");
        let sent = requests(&bot);
        assert_eq!(sent[0].1.get("message_thread_id"), Some(&json!(50)));
    }

    #[tokio::test]
    async fn sending_a_reply_keyboard_to_another_chat_removes_the_old_chat_keyboard() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE), (200, OTHER_CHAT_RESPONSE)]);
        let old = old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup));
        let mut new = new_message(Some(
            ReplyKeyboardMarkup::new([[KeyboardButton::new("New")]]).into(),
        ));
        new.chat = ChatPrivate::new(2).into();
        new.show_mode = ShowMode::Send;
        MessageManager::show_message(&bot, new, Some(old))
            .await
            .expect("move reply keyboard");

        let sent = requests(&bot);
        assert_eq!(
            sent.iter().map(|request| request.0).collect::<Vec<_>>(),
            ["sendMessage", "sendMessage"],
        );
        assert_eq!(sent[0].1.get("chat_id"), Some(&json!(1)));
        assert_eq!(
            sent[0]
                .1
                .get("reply_markup")
                .and_then(|markup| markup.get("remove_keyboard")),
            Some(&json!(true)),
        );
        assert_eq!(sent[1].1.get("chat_id"), Some(&json!(2)));
        assert!(sent[1]
            .1
            .get("reply_markup")
            .and_then(|markup| markup.get("keyboard"))
            .is_some());
    }

    #[tokio::test]
    async fn recreating_in_another_chat_removes_its_previous_reply_keyboard() {
        let bot = mock_bot(&[
            (200, OTHER_CHAT_RESPONSE),
            (200, TRUE_RESPONSE),
            (200, TEXT_RESPONSE),
        ]);
        let old = old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup));
        let mut new = new_message(Some(
            ReplyKeyboardMarkup::new([[KeyboardButton::new("New")]]).into(),
        ));
        new.chat = ChatPrivate::new(2).into();
        new.show_mode = ShowMode::DeleteAndSend;
        MessageManager::show_message(&bot, new, Some(old))
            .await
            .expect("move reply keyboard");

        let sent = requests(&bot);
        assert_eq!(
            sent.iter().map(|request| request.0).collect::<Vec<_>>(),
            ["sendMessage", "deleteMessage", "sendMessage"],
        );
        assert_eq!(sent[0].1.get("chat_id"), Some(&json!(2)));
        assert_eq!(sent[2].1.get("chat_id"), Some(&json!(1)));
        assert_eq!(
            sent[2]
                .1
                .get("reply_markup")
                .and_then(|markup| markup.get("remove_keyboard")),
            Some(&json!(true)),
        );
    }

    #[tokio::test]
    async fn reply_keyboards_are_shared_across_threads_of_the_same_chat() {
        let bot = mock_bot(&[(200, TEXT_RESPONSE), (200, TRUE_RESPONSE)]);
        let old = old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup))
            .with_message_thread_id(Some(50));
        let mut new = new_message(Some(
            ReplyKeyboardMarkup::new([[KeyboardButton::new("New")]]).into(),
        ));
        new.message_thread_id = Some(100);
        new.show_mode = ShowMode::DeleteAndSend;
        MessageManager::show_message(&bot, new, Some(old))
            .await
            .expect("replace reply keyboard in another thread");
        assert_eq!(
            requests(&bot)
                .iter()
                .map(|request| request.0)
                .collect::<Vec<_>>(),
            ["sendMessage", "deleteMessage"],
        );
    }
}
