use crate::entities::update_event::DialogUpdateEvent;
use telers::{
    client::Reqwest,
    types::{
        CallbackQuery, Chat, ChatJoinRequest, ChatMemberUpdated, MaybeInaccessibleMessage, Message,
        Update, User,
    },
    Bot,
};

pub const EVENT_CONTEXT_KEY: &str = "td_event_context";
pub const CHAT_EVENT_KEY: &str = "td_chat_event";

/// Chat-scoped event variants that can drive dialog updates.
#[derive(Clone, Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "Chat events intentionally mirror telers update payloads without extra boxing at \
              call sites."
)]
pub enum ChatEvent {
    /// Telegram callback query event.
    CallbackQuery(CallbackQuery),
    /// Telegram chat join request event.
    ChatJoinRequest(ChatJoinRequest),
    /// Telegram chat member update event.
    ChatMember(ChatMemberUpdated),
    /// Synthetic dialog event injected by the library.
    DialogUpdateEvent(DialogUpdateEvent),
    /// Telegram message-like event.
    Message(Message),
}

/// Convert a Telegram update into a dialog-relevant chat event.
#[must_use]
pub fn chat_event_from_update(update: &Update) -> Option<ChatEvent> {
    if let Some(callback_query) = update.callback_query() {
        return Some(ChatEvent::CallbackQuery(callback_query.clone()));
    }
    if let Some(chat_join_request) = update.chat_join_request() {
        return Some(ChatEvent::ChatJoinRequest(chat_join_request.clone()));
    }
    if let Some(chat_member) = update.chat_member() {
        return Some(ChatEvent::ChatMember(chat_member.clone()));
    }
    if let Some(chat_member) = update.my_chat_member() {
        return Some(ChatEvent::ChatMember(chat_member.clone()));
    }
    if let Some(message) = update.message() {
        return Some(ChatEvent::Message(message.clone()));
    }
    if let Some(message) = update.edited_message() {
        return Some(ChatEvent::Message(message.clone()));
    }
    if let Some(message) = update.business_message() {
        return Some(ChatEvent::Message(message.clone()));
    }
    if let Some(message) = update.edited_business_message() {
        return Some(ChatEvent::Message(message.clone()));
    }
    None
}

/// Normalized event context extracted from the current update.
#[derive(Clone, Debug)]
pub struct EventContext<Client = Reqwest> {
    /// Bot instance bound to the current request.
    pub bot: Bot<Client>,
    /// Event chat.
    pub chat: Chat,
    /// Event author.
    pub user: User,
    /// Forum topic / thread id when available.
    pub thread_id: Option<i64>,
    /// Business connection id when available.
    pub business_connection_id: Option<String>,
}

impl<Client> EventContext<Client> {
    /// # Panics
    /// - If the event is callback query and the message is not available
    #[must_use]
    pub fn new(bot: Bot<Client>, event: ChatEvent) -> Self {
        match event {
            ChatEvent::CallbackQuery(callback_query) => {
                let user = *callback_query.from;
                let (chat, thread_id, business_connection_id) = match callback_query.message {
                    Some(message) => match *message {
                        MaybeInaccessibleMessage::InaccessibleMessage(message) => {
                            (*message.chat, None, None)
                        }
                        MaybeInaccessibleMessage::Message(message) => (
                            message.chat().to_owned(),
                            message.message_thread_id(),
                            message.business_connection_id().map(ToOwned::to_owned),
                        ),
                    },
                    None => unreachable!("CallbackQuery must have a message"),
                };
                Self {
                    bot,
                    chat,
                    user,
                    thread_id,
                    business_connection_id,
                }
            }
            ChatEvent::ChatJoinRequest(chat_join_request) => {
                let user = *chat_join_request.from;
                let chat = *chat_join_request.chat;
                Self {
                    bot,
                    chat,
                    user,
                    thread_id: None,
                    business_connection_id: None,
                }
            }
            ChatEvent::ChatMember(chat_member_updated) => {
                let user = *chat_member_updated.from;
                let chat = *chat_member_updated.chat;
                Self {
                    bot,
                    chat,
                    user,
                    thread_id: None,
                    business_connection_id: None,
                }
            }
            ChatEvent::DialogUpdateEvent(DialogUpdateEvent {
                user,
                chat,
                thread_id,
                business_connection_id,
                ..
            }) => Self {
                bot,
                chat,
                user,
                thread_id,
                business_connection_id,
            },
            ChatEvent::Message(message) => {
                let user = match message.from() {
                    Some(user) => user.to_owned(),
                    None => unreachable!("Message must have a from"),
                };
                let (chat, thread_id, business_connection_id) = (
                    message.chat().to_owned(),
                    message.message_thread_id(),
                    message.business_connection_id().map(ToOwned::to_owned),
                );
                Self {
                    bot,
                    chat,
                    user,
                    thread_id,
                    business_connection_id,
                }
            }
        }
    }
}
