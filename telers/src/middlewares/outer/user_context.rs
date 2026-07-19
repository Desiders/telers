use super::{Middleware, MiddlewareResponse};
use crate::{
    errors::EventErrorKind,
    event::EventReturn,
    types::{Chat, MaybeInaccessibleMessage, Update, User},
    Request,
};

use tracing::instrument;

/// Middleware for adding [`crate::types::User`] and [`crate::types::Chat`] to context,
/// if they are present in [`crate::types::Update`] struct
#[derive(Debug, Default, Clone)]
pub struct UserContext;

impl UserContext {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

fn resolve_event_chat(update: &Update) -> Option<&Chat> {
    if let Some(chat) = update.chat() {
        return Some(chat);
    }
    if let Update::CallbackQuery(update) = update {
        return update
            .callback_query
            .message
            .as_deref()
            .map(MaybeInaccessibleMessage::chat);
    }
    None
}

fn resolve_event_user(update: &Update) -> Option<&User> {
    update.from().or_else(|| update.user())
}

fn resolve_event_message_thread_id(update: &Update) -> Option<i64> {
    if let Some(message_thread_id) = update.message_thread_id() {
        return Some(message_thread_id);
    }
    if let Update::CallbackQuery(update) = update {
        return update
            .callback_query
            .message
            .as_deref()
            .and_then(MaybeInaccessibleMessage::message_thread_id);
    }
    None
}

impl<Client> Middleware<Client> for UserContext
where
    Client: Send + Sync + 'static,
{
    #[instrument(skip_all)]
    async fn call(
        &mut self,
        mut request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        if let Some(from) = resolve_event_user(&request.update) {
            request.context.insert("event_user", from.clone());
        }
        if let Some(chat) = resolve_event_chat(&request.update) {
            request.context.insert("event_chat", chat.clone());
        }
        if let Some(message_thread_id) = resolve_event_message_thread_id(&request.update) {
            request
                .context
                .insert("event_message_thread_id", message_thread_id);
        }
        if let Some(business_connection_id) = request.update.business_connection_id() {
            request.context.insert(
                "event_business_connection_id",
                business_connection_id.to_owned(),
            );
        }

        Ok((request, EventReturn::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        context::Context,
        enums::UpdateType,
        event::telegram::Handler,
        router::{PropagateEvent as _, Router},
        types::{Chat, ChatPrivate, Message, MessageText, Update, UpdateMessage, User},
        Bot, Extensions,
    };

    use std::{convert::Infallible, sync::Arc};

    #[tokio::test]
    async fn test_user_context() {
        let router = Router::new("main")
            .on_update(|observer| observer.register_outer_middleware(UserContext))
            .on_message(|observer| {
                observer.register(Handler::new(|context: Context| async move {
                    context.get::<User>("event_user").unwrap();
                    context.get::<Chat>("event_chat").unwrap();
                    context.get::<i64>("event_message_thread_id").unwrap();

                    Ok::<_, Infallible>(EventReturn::default())
                }))
            });

        let mut router_configured = router.configure_default();

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                Message::Text(
                    MessageText::new(0, 0, ChatPrivate::new(0), "")
                        .from(User::new(0, true, ""))
                        .message_thread_id(0),
                ),
            ))),
            bot: Bot::default(),
            context: Context::default(),
            extensions: Extensions::default(),
        };

        router_configured
            .propagate_event(UpdateType::Message, request)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_user_context_panic() {
        let router = Router::new("main")
            .on_message(|observer| {
                observer.register(Handler::new(|context: Context| async move {
                    // This should panic, because update doesn't contain user
                    context.get::<User>("event_user").unwrap();
                    // This should panic, because update doesn't contain chat
                    context.get::<Chat>("event_chat").unwrap();
                    // This should panic, because update doesn't contain message thread id
                    context.get::<i64>("event_message_thread_id").unwrap();

                    Ok::<_, Infallible>(EventReturn::default())
                }))
            })
            .on_update(|observer| observer.register_outer_middleware(UserContext));

        let mut router_configured = router.configure_default();

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };
        router_configured
            .propagate_event(UpdateType::Message, request)
            .await
            .unwrap();
    }
}
