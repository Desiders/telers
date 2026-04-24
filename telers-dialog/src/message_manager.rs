use crate::{
    entities::{NewMessage, OldMessage, ShowMode},
    errors::DialogError,
    widgets::media::{MediaAttachment, MediaContentType},
};
use serde::Serialize;
use telers::{
    client::Session,
    enums::ReplyMarkupType,
    methods::{
        DeleteMessage, EditMessageMedia, EditMessageReplyMarkup, EditMessageText, SendAnimation,
        SendAudio, SendDocument, SendMessage, SendPhoto, SendVideo,
    },
    types::{
        InputFile, InputMedia, InputMediaAnimation, InputMediaAudio, InputMediaDocument,
        InputMediaPhoto, InputMediaVideo, ReplyKeyboardRemove, ReplyMarkup,
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
        Self::remove_kbd(bot, show_mode, old_message).await
    }

    /// Show message with edit/delete/send logic.
    ///
    /// # Errors
    /// Returns telegram errors as `DialogError`.
    pub async fn show_message<Client: Session>(
        bot: &Bot<Client>,
        new_message: NewMessage,
        old_message: Option<OldMessage>,
    ) -> Result<OldMessage, DialogError> {
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
            Self::remove_kbd(bot, new_message.show_mode, old_message.as_ref()).await?;
            let sent = Self::send_message(bot, new_message.clone()).await?;
            return Ok(Self::combine(&new_message, &sent));
        }
        let Some(old) = old_message.as_ref() else {
            trace!("Sending new dialog message");
            Self::remove_kbd(bot, new_message.show_mode, None).await?;
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
                Self::remove_message_safe(bot, old).await?;
                return Ok(Self::combine(&new_message, &sent));
            }
            Self::remove_message_safe(bot, old).await?;
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
            Self::remove_message_safe(bot, old).await?;
            let sent = Self::send_message(bot, new_message.clone()).await?;
            return Ok(Self::combine(&new_message, &sent));
        }
        trace!(
            message_id = old.message_id,
            "Editing existing dialog message"
        );
        let edited = Self::edit_message(bot, new_message.clone(), old).await?;
        Ok(Self::combine(&new_message, &edited))
    }

    /// Combine sent result with metadata to build `OldMessage`.
    #[must_use]
    fn combine(sent_message: &NewMessage, message_result: &telers::types::Message) -> OldMessage {
        let reply_markup_type = sent_message
            .reply_markup
            .as_ref()
            .map(ReplyMarkupType::from);

        // Extract media info from the new message
        let (media_file_id, media_unique_id, media_content_type) =
            if let Some(ref media) = sent_message.media {
                (
                    media.get_file_id().map(|s| s.to_string()),
                    media.get_file_unique_id().map(|s| s.to_string()),
                    Some(media.content_type),
                )
            } else {
                (None, None, None)
            };

        OldMessage::new(
            message_result.chat().clone(),
            message_result.message_id(),
            Some(sent_message.text.clone()),
            message_result.has_protected_content(),
            reply_markup_type,
            serialize_option(sent_message.reply_markup.as_ref()),
            message_result
                .business_connection_id()
                .map(ToOwned::to_owned),
            None,
            serialize_option(sent_message.link_preview_options.as_ref()),
        )
        .with_media(media_file_id, media_unique_id, media_content_type)
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

    /// Check if message content or protection flags changed.
    fn message_changed(new: &NewMessage, old: &OldMessage) -> bool {
        let text_changed = new.text.as_ref() != old.text.as_deref().unwrap_or("");
        let markup_changed = serialize_option(new.reply_markup.as_ref()) != old.reply_markup_value;
        let protect_changed = new.protect_content != old.has_protected_content;
        let link_preview_changed =
            serialize_option(new.link_preview_options.as_ref()) != old.link_preview_options_value;
        let media_changed = Self::media_changed(new, old);

        let changed = text_changed
            || markup_changed
            || protect_changed
            || link_preview_changed
            || media_changed;
        trace!(
            message_id = old.message_id,
            changed,
            text_changed,
            markup_changed,
            media_changed,
            "Compared dialog message snapshots"
        );
        changed
    }

    /// Check if media content changed.
    fn media_changed(new: &NewMessage, old: &OldMessage) -> bool {
        match (&new.media, &old.media_file_id) {
            (None, None) => false,
            (Some(_), None) | (None, Some(_)) => true,
            (Some(new_media), Some(old_file_id)) => {
                // Check if file ID changed
                if let Some(new_file_id) = new_media.get_file_id() {
                    new_file_id != old_file_id.as_ref()
                } else {
                    // New media is URL/path based, always consider changed
                    true
                }
            }
        }
    }

    /// Check if media type changed (requires resend rather than edit).
    fn media_type_changed(new: &NewMessage, old: &OldMessage) -> bool {
        match (&new.media, &old.media_content_type) {
            (None, None) => false,
            (Some(_), None) | (None, Some(_)) => true,
            (Some(new_media), Some(old_type)) => new_media.content_type != *old_type,
        }
    }

    /// Check if edit is possible without deleting message.
    fn can_edit(new: &NewMessage, old: &OldMessage) -> bool {
        // Cannot edit reply keyboards
        if Self::had_reply_keyboard(old) || Self::need_reply_keyboard(new) {
            return false;
        }
        // Cannot edit if switching between text and media messages
        if new.has_media() != old.has_media() {
            return false;
        }
        // Cannot edit if media type changed (need to resend)
        if Self::media_type_changed(new, old) {
            return false;
        }
        true
    }

    /// Remove reply or inline keyboard depending on mode.
    async fn remove_kbd<Client: Session>(
        bot: &Bot<Client>,
        show_mode: ShowMode,
        old_message: Option<&OldMessage>,
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
                Self::remove_message_safe(bot, old_message).await?;
                return Ok(());
            }
        }
        if let Some(old) = old_message {
            if Self::had_reply_keyboard(old) {
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
        let _ = bot
            .send(
                EditMessageReplyMarkup::new()
                    .chat_id(old.chat.id())
                    .message_id(old.message_id)
                    .business_connection_id_option(old.business_connection_id.clone()),
            )
            .await?;
        Ok(())
    }

    /// Remove reply keyboard by sending keyboard remove.
    async fn remove_reply_kbd<Client: Session>(
        bot: &Bot<Client>,
        old: &OldMessage,
    ) -> Result<(), DialogError> {
        trace!(chat_id = old.chat.id(), "Sending reply keyboard removal");
        let m = SendMessage::new(old.chat.id(), "...")
            .reply_markup(ReplyKeyboardRemove::new(true))
            .business_connection_id_option(old.business_connection_id.clone());
        let _ = bot.send(m).await?;
        Ok(())
    }

    /// Delete message, ignoring inability to delete.
    async fn remove_message_safe<Client: Session>(
        bot: &Bot<Client>,
        old: &OldMessage,
    ) -> Result<(), DialogError> {
        trace!(message_id = old.message_id, "Deleting dialog message");
        let _ = bot
            .send(DeleteMessage::new(old.chat.id(), old.message_id))
            .await?;
        Ok(())
    }

    /// Edit message text or media (fallbacks to send if needed).
    async fn edit_message<Client: Session>(
        bot: &Bot<Client>,
        new: NewMessage,
        old: &OldMessage,
    ) -> Result<telers::types::Message, DialogError> {
        if new.protect_content != old.has_protected_content {
            trace!(
                message_id = old.message_id,
                "Protect content changed, recreate message"
            );
            Self::remove_message_safe(bot, old).await?;
            return Self::send_message(bot, new).await;
        }

        // Edit media message
        if let Some(ref media) = new.media {
            return Self::edit_media_message(bot, &new, media, old).await;
        }

        // Edit text message
        let mut m = EditMessageText::new(new.text.clone())
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
            let mut m = EditMessageText::new(new.text.clone())
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

        let input_media = Self::build_input_media(media, input_file, &new.text, &new.parse_mode);

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
        parse_mode: &Option<Box<str>>,
    ) -> InputMedia {
        let caption_opt = if caption.is_empty() {
            None
        } else {
            Some(caption)
        };

        match media.content_type {
            MediaContentType::Photo => {
                let mut m = InputMediaPhoto::new(input_file);
                if let Some(caption) = caption_opt {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(spoiler) = media.has_spoiler {
                    m = m.has_spoiler(spoiler);
                }
                if let Some(show_above) = media.show_caption_above_media {
                    m = m.show_caption_above_media(show_above);
                }
                InputMedia::Photo(m)
            }
            MediaContentType::Video => {
                let mut m = InputMediaVideo::new(input_file);
                if let Some(caption) = caption_opt {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(spoiler) = media.has_spoiler {
                    m = m.has_spoiler(spoiler);
                }
                if let Some(show_above) = media.show_caption_above_media {
                    m = m.show_caption_above_media(show_above);
                }
                if let Some(w) = media.width {
                    m = m.width(w);
                }
                if let Some(h) = media.height {
                    m = m.height(h);
                }
                if let Some(d) = media.duration {
                    m = m.duration(d);
                }
                if let Some(streaming) = media.supports_streaming {
                    m = m.supports_streaming(streaming);
                }
                InputMedia::Video(m)
            }
            MediaContentType::Audio => {
                let mut m = InputMediaAudio::new(input_file);
                if let Some(caption) = caption_opt {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(d) = media.duration {
                    m = m.duration(d);
                }
                if let Some(ref performer) = media.performer {
                    m = m.performer(performer.as_ref());
                }
                if let Some(ref title) = media.title {
                    m = m.title(title.as_ref());
                }
                InputMedia::Audio(m)
            }
            MediaContentType::Document => {
                let mut m = InputMediaDocument::new(input_file);
                if let Some(caption) = caption_opt {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                InputMedia::Document(m)
            }
            MediaContentType::Animation => {
                let mut m = InputMediaAnimation::new(input_file);
                if let Some(caption) = caption_opt {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(spoiler) = media.has_spoiler {
                    m = m.has_spoiler(spoiler);
                }
                if let Some(show_above) = media.show_caption_above_media {
                    m = m.show_caption_above_media(show_above);
                }
                if let Some(w) = media.width {
                    m = m.width(w);
                }
                if let Some(h) = media.height {
                    m = m.height(h);
                }
                if let Some(d) = media.duration {
                    m = m.duration(d);
                }
                InputMedia::Animation(m)
            }
            // Voice and VideoNote cannot be edited via editMessageMedia, fall back to photo
            MediaContentType::Voice | MediaContentType::VideoNote => {
                let mut m = InputMediaPhoto::new(input_file);
                if let Some(caption) = caption_opt {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                InputMedia::Photo(m)
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

        let caption = if msg.text.is_empty() {
            None
        } else {
            Some(msg.text.as_ref())
        };

        trace!(
            chat_id = msg.chat.id(),
            media_type = ?media.content_type,
            has_caption = caption.is_some(),
            "Sending dialog media message"
        );

        match media.content_type {
            MediaContentType::Photo => {
                let mut m = SendPhoto::new(msg.chat.id(), input_file);
                if let Some(caption) = caption {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = msg.parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(spoiler) = media.has_spoiler {
                    m = m.has_spoiler(spoiler);
                }
                if let Some(show_above) = media.show_caption_above_media {
                    m = m.show_caption_above_media(show_above);
                }
                m = m
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Video => {
                let mut m = SendVideo::new(msg.chat.id(), input_file);
                if let Some(caption) = caption {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = msg.parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(spoiler) = media.has_spoiler {
                    m = m.has_spoiler(spoiler);
                }
                if let Some(show_above) = media.show_caption_above_media {
                    m = m.show_caption_above_media(show_above);
                }
                if let Some(w) = media.width {
                    m = m.width(w);
                }
                if let Some(h) = media.height {
                    m = m.height(h);
                }
                if let Some(d) = media.duration {
                    m = m.duration(d);
                }
                if let Some(streaming) = media.supports_streaming {
                    m = m.supports_streaming(streaming);
                }
                m = m
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Audio => {
                let mut m = SendAudio::new(msg.chat.id(), input_file);
                if let Some(caption) = caption {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = msg.parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(d) = media.duration {
                    m = m.duration(d);
                }
                if let Some(ref performer) = media.performer {
                    m = m.performer(performer.as_ref());
                }
                if let Some(ref title) = media.title {
                    m = m.title(title.as_ref());
                }
                m = m
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Document => {
                let mut m = SendDocument::new(msg.chat.id(), input_file);
                if let Some(caption) = caption {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = msg.parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                m = m
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            MediaContentType::Animation => {
                let mut m = SendAnimation::new(msg.chat.id(), input_file);
                if let Some(caption) = caption {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = msg.parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                if let Some(spoiler) = media.has_spoiler {
                    m = m.has_spoiler(spoiler);
                }
                if let Some(show_above) = media.show_caption_above_media {
                    m = m.show_caption_above_media(show_above);
                }
                if let Some(w) = media.width {
                    m = m.width(w);
                }
                if let Some(h) = media.height {
                    m = m.height(h);
                }
                if let Some(d) = media.duration {
                    m = m.duration(d);
                }
                m = m
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
            // Voice and VideoNote are not typically used in dialogs, fall back to document
            MediaContentType::Voice | MediaContentType::VideoNote => {
                let mut m = SendDocument::new(msg.chat.id(), input_file);
                if let Some(caption) = caption {
                    m = m.caption(caption);
                }
                if let Some(ref pm) = msg.parse_mode {
                    m = m.parse_mode(pm.as_ref());
                }
                m = m
                    .reply_markup_option(msg.reply_markup.clone())
                    .protect_content_option(msg.protect_content)
                    .business_connection_id_option(msg.business_connection_id.clone())
                    .message_thread_id_option(msg.message_thread_id);
                Ok(bot.send(m).await?)
            }
        }
    }
}

fn serialize_option<T>(value: Option<&T>) -> Option<serde_json::Value>
where
    T: Serialize,
{
    value.and_then(|value| serde_json::to_value(value).ok())
}

#[cfg(test)]
mod tests {
    use super::MessageManager;
    use crate::{
        entities::{NewMessage, OldMessage, ShowMode},
        DialogError,
    };
    use telers::{
        client::Reqwest,
        enums::ReplyMarkupType,
        types::{
            ChatPrivate, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton,
            LinkPreviewOptions, ReplyKeyboardMarkup, ReplyMarkup,
        },
        Bot,
    };

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

    #[tokio::test]
    async fn detects_reply_and_inline_keyboards_separately() {
        let no_markup = old_message(None);
        let reply_keyboard = old_message(Some(ReplyMarkupType::ReplyKeyboardMarkup));
        let inline_keyboard = old_message(Some(ReplyMarkupType::InlineKeyboardMarkup));

        assert!(!MessageManager::had_reply_keyboard(&no_markup));
        assert!(!MessageManager::had_inline_keyboard(&no_markup));

        assert!(MessageManager::had_reply_keyboard(&reply_keyboard));
        assert!(!MessageManager::had_inline_keyboard(&reply_keyboard));

        assert!(!MessageManager::had_reply_keyboard(&inline_keyboard));
        assert!(MessageManager::had_inline_keyboard(&inline_keyboard));
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
}
