//! Keep a chat action (like "typing") alive for the duration of a long-running operation.
//!
//! Telegram clears a chat action after ~5 seconds, so to show it for longer it must be re-sent
//! periodically. [`ChatActionSender`] does that in the background: configure it (directly or via a
//! convenience constructor like [`ChatActionSender::typing`]), call [`ChatActionSender::start`] to
//! spawn the re-sending task, and keep the returned [`ChatActionGuard`] alive while you work.
//! Dropping the guard stops sending — the Rust analog of aiogram's `async with ChatActionSender(...)`.
//!
//! # Example
//! ```no_run
//! # async fn example(bot: telers::Bot, chat_id: i64) {
//! use telers::utils::chat_action::ChatActionSender;
//!
//! let guard = ChatActionSender::typing(bot.clone(), chat_id).start();
//! // ...do the slow work; "typing" keeps being shown until `guard` is dropped...
//! drop(guard);
//! # }
//! ```

use std::time::Duration;

use tokio::{task::JoinHandle, time::sleep};
use tracing::{event, Level};

use crate::{
    client::{Bot, Reqwest, Session},
    methods::SendChatAction,
    types::ChatIdKind,
};

/// Default re-send interval. A chat action is cleared by Telegram after ~5 seconds, so it has to be
/// refreshed at least that often.
pub const DEFAULT_INTERVAL: Duration = Duration::from_secs(5);

/// Periodically sends a [`SendChatAction`] to keep a chat action visible while a slow operation runs.
///
/// Build it with [`ChatActionSender::new`] or one of the action-named constructors, optionally tune
/// it with the setters, then call [`ChatActionSender::start`].
#[derive(Clone)]
pub struct ChatActionSender<Client = Reqwest> {
    bot: Bot<Client>,
    chat_id: ChatIdKind,
    action: Box<str>,
    interval: Duration,
    initial_sleep: Duration,
    message_thread_id: Option<i64>,
    business_connection_id: Option<Box<str>>,
}

/// Generates the action-named convenience constructors (`typing`, `upload_photo`, ...), each a
/// shortcut for [`ChatActionSender::new`] with the matching action string.
macro_rules! action_constructors {
    ($($(#[$meta:meta])* $name:ident => $action:literal),* $(,)?) => {
        $(
            $(#[$meta])*
            #[must_use]
            pub fn $name(bot: Bot<Client>, chat_id: impl Into<ChatIdKind>) -> Self {
                Self::new(bot, chat_id, $action)
            }
        )*
    };
}

impl<Client> ChatActionSender<Client> {
    action_constructors! {
        /// `typing` — for text messages.
        typing => "typing",
        /// `upload_photo` — for photos.
        upload_photo => "upload_photo",
        /// `record_video` — for videos.
        record_video => "record_video",
        /// `upload_video` — for videos.
        upload_video => "upload_video",
        /// `record_voice` — for voice notes.
        record_voice => "record_voice",
        /// `upload_voice` — for voice notes.
        upload_voice => "upload_voice",
        /// `upload_document` — for general files.
        upload_document => "upload_document",
        /// `choose_sticker` — for stickers.
        choose_sticker => "choose_sticker",
        /// `find_location` — for location data.
        find_location => "find_location",
        /// `record_video_note` — for video notes.
        record_video_note => "record_video_note",
        /// `upload_video_note` — for video notes.
        upload_video_note => "upload_video_note",
    }

    /// Creates a sender for an arbitrary `action` string (see the action-named constructors for the
    /// known values). Defaults: interval [`DEFAULT_INTERVAL`], no initial delay, no thread/business
    /// connection.
    #[must_use]
    pub fn new(
        bot: Bot<Client>,
        chat_id: impl Into<ChatIdKind>,
        action: impl Into<Box<str>>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            action: action.into(),
            interval: DEFAULT_INTERVAL,
            initial_sleep: Duration::ZERO,
            message_thread_id: None,
            business_connection_id: None,
        }
    }

    /// Sets how often the action is re-sent (default [`DEFAULT_INTERVAL`]).
    #[must_use]
    pub const fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Delays the first send by `initial_sleep`. Useful to avoid flashing an action for operations
    /// that usually finish quickly (default: no delay, the first action is sent immediately).
    #[must_use]
    pub const fn initial_sleep(mut self, initial_sleep: Duration) -> Self {
        self.initial_sleep = initial_sleep;
        self
    }

    /// Sets the target message thread / forum topic id.
    #[must_use]
    pub const fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    /// Sets the business connection id on behalf of which the action is sent.
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<Box<str>>) -> Self {
        self.business_connection_id = Some(business_connection_id.into());
        self
    }
}

impl<Client> ChatActionSender<Client>
where
    Client: Session + Clone + 'static,
{
    /// Spawns the background re-sending task and returns a [`ChatActionGuard`] that stops it when
    /// dropped.
    ///
    /// # Panics
    /// Like [`tokio::spawn`], this must be called from within a Tokio runtime (it always is inside
    /// telers handlers/middlewares).
    pub fn start(self) -> ChatActionGuard {
        let Self {
            bot,
            chat_id,
            action,
            interval,
            initial_sleep,
            message_thread_id,
            business_connection_id,
        } = self;

        let method = SendChatAction::new(chat_id, action)
            .message_thread_id_option(message_thread_id)
            .business_connection_id_option(business_connection_id);

        let handle = tokio::spawn(async move {
            if !initial_sleep.is_zero() {
                sleep(initial_sleep).await;
            }

            loop {
                if let Err(err) = bot.send(method.clone()).await {
                    event!(Level::WARN, %err, "Failed to send chat action");
                }

                sleep(interval).await;
            }
        });

        ChatActionGuard { handle }
    }
}

/// Keeps a chat action alive while it is in scope. Dropping it aborts the background re-sending task.
#[must_use = "the chat action stops as soon as the guard is dropped"]
pub struct ChatActionGuard {
    handle: JoinHandle<()>,
}

impl Drop for ChatActionGuard {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::{ChatActionSender, DEFAULT_INTERVAL};
    use crate::{client::Reqwest, Bot};
    use std::time::Duration;

    #[test]
    fn convenience_constructors_set_action_and_defaults() {
        let bot = Bot::<Reqwest>::default();

        let sender = ChatActionSender::upload_photo(bot.clone(), 42);
        assert_eq!(&*sender.action, "upload_photo");
        assert_eq!(sender.interval, DEFAULT_INTERVAL);
        assert_eq!(sender.initial_sleep, Duration::ZERO);
        assert_eq!(sender.message_thread_id, None);
        assert!(sender.business_connection_id.is_none());

        assert_eq!(&*ChatActionSender::typing(bot, 42).action, "typing");
    }

    #[test]
    fn setters_override_defaults() {
        let sender = ChatActionSender::typing(Bot::<Reqwest>::default(), 42)
            .interval(Duration::from_secs(3))
            .initial_sleep(Duration::from_secs(1))
            .message_thread_id(7)
            .business_connection_id("bc");

        assert_eq!(sender.interval, Duration::from_secs(3));
        assert_eq!(sender.initial_sleep, Duration::from_secs(1));
        assert_eq!(sender.message_thread_id, Some(7));
        assert_eq!(sender.business_connection_id.as_deref(), Some("bc"));
    }
}
