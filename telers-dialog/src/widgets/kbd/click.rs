use telers::types::CallbackQuery;

use crate::entities::{ChatEvent, Context, DataMap, EventContext};

/// Runtime data available while resolving a keyboard callback.
///
/// It exposes stored dialog state, the current event, and request-scoped
/// runtime data inserted by middlewares.
#[derive(Clone, Copy, Debug)]
pub struct ClickContext<'a> {
    /// Stored dialog context for the active intent.
    pub context: &'a Context,
    /// Raw callback data received from Telegram.
    pub callback_data: &'a str,
    /// Current event that triggered this callback.
    pub event: &'a ChatEvent,
    /// Normalized event metadata.
    pub event_context: &'a EventContext,
    /// Request-scoped context populated by telers middlewares and filters.
    pub runtime_context: &'a telers::Context,
}

impl<'a> ClickContext<'a> {
    #[inline]
    #[must_use]
    pub const fn new(
        context: &'a Context,
        callback_data: &'a str,
        event: &'a ChatEvent,
        event_context: &'a EventContext,
        runtime_context: &'a telers::Context,
    ) -> Self {
        Self {
            context,
            callback_data,
            event,
            event_context,
            runtime_context,
        }
    }

    #[inline]
    #[must_use]
    pub fn callback_query(&self) -> Option<&'a CallbackQuery> {
        match self.event {
            ChatEvent::CallbackQuery(callback_query) => Some(callback_query),
            ChatEvent::ChatJoinRequest(_)
            | ChatEvent::ChatMember(_)
            | ChatEvent::DialogUpdateEvent(_)
            | ChatEvent::Message(_) => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn runtime_data<T: 'static>(&self, key: &'static str) -> Option<&'a T> {
        self.runtime_context.get(key)
    }

    #[inline]
    #[must_use]
    pub const fn dialog_data(&self) -> &'a DataMap {
        &self.context.dialog_data
    }

    #[inline]
    #[must_use]
    pub const fn widget_data(&self) -> &'a DataMap {
        &self.context.widget_data
    }
}

#[cfg(test)]
impl ClickContext<'_> {
    pub(crate) fn with_test<R>(
        ctx: &Context,
        callback_data: &str,
        handle: impl FnOnce(&ClickContext<'_>) -> R,
    ) -> R {
        use telers::{
            client::Reqwest,
            types::{ChatPrivate, MessageText, User},
            Bot,
        };

        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatPrivate::new(10), "/test")
                .from(User::new(10, false, "tester"))
                .into(),
        );
        let event_context = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        let runtime_context = telers::Context::default();
        let click = ClickContext {
            context: ctx,
            callback_data,
            event: &event,
            event_context: &event_context,
            runtime_context: &runtime_context,
        };

        handle(&click)
    }
}
