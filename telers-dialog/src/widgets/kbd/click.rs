use std::sync::Arc;

use telers::types::CallbackQuery;

use crate::entities::{ChatEvent, Context, DataMap, EventContext};

/// Runtime data available while resolving a keyboard callback.
///
/// It exposes stored dialog state, the current event, and request-scoped
/// runtime data inserted by middlewares.
#[derive(Clone, Debug)]
pub struct ClickContext {
    /// Stored dialog context for the active intent.
    pub context: Arc<Context>,
    /// Raw callback data received from Telegram.
    pub callback_data: String,
    /// Current event that triggered this callback.
    pub event: Arc<ChatEvent>,
    /// Normalized event metadata.
    pub event_context: Arc<EventContext>,
    /// Request-scoped context populated by telers middlewares and filters.
    pub runtime_context: Arc<telers::Context>,
}

impl ClickContext {
    #[inline]
    #[must_use]
    pub fn new(
        context: &Context,
        callback_data: &str,
        event: &ChatEvent,
        event_context: &EventContext,
        runtime_context: &telers::Context,
    ) -> Self {
        Self {
            context: Arc::new(context.clone()),
            callback_data: callback_data.to_owned(),
            event: Arc::new(event.clone()),
            event_context: Arc::new(event_context.clone()),
            runtime_context: Arc::new(runtime_context.clone()),
        }
    }

    #[inline]
    #[must_use]
    pub fn callback_query(&self) -> Option<&CallbackQuery> {
        match self.event.as_ref() {
            ChatEvent::CallbackQuery(callback_query) => Some(callback_query),
            ChatEvent::ChatJoinRequest(_)
            | ChatEvent::ChatMember(_)
            | ChatEvent::DialogUpdateEvent(_)
            | ChatEvent::Message(_) => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn runtime_data<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.runtime_context.get(key)
    }

    #[inline]
    #[must_use]
    pub fn dialog_data(&self) -> &DataMap {
        &self.context.dialog_data
    }

    #[inline]
    #[must_use]
    pub fn widget_data(&self) -> &DataMap {
        &self.context.widget_data
    }
}

#[cfg(test)]
mod tests {
    use super::ClickContext;
    use crate::{
        entities::{ChatEvent, Context, EventContext},
        future::BoxFuture,
    };

    impl ClickContext {
        #[allow(dead_code)]
        pub(crate) async fn with_test<R>(
            ctx: &Context,
            callback_data: &str,
            handle: impl FnOnce(ClickContext) -> BoxFuture<'static, R>,
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
            let event_context =
                EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
            let runtime_context = telers::Context::default();
            let click =
                ClickContext::new(ctx, callback_data, &event, &event_context, &runtime_context);

            handle(click).await
        }
    }
}
