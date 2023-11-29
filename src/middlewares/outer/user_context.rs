use super::{Middleware, MiddlewareResponse};

use crate::{errors::EventErrorKind, event::EventReturn, router::Request};

use async_trait::async_trait;
use tracing::instrument;

/// Middleware for adding [`crate::types::User`] and [`crate::types::Chat`] to context,
/// if they are present in [`crate::types::Update`] struct
#[derive(Debug, Default, Clone)]
pub struct UserContext;

impl UserContext {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl<Client> Middleware<Client> for UserContext
where
    Client: Send + Sync + 'static,
{
    #[instrument(skip(self, request))]
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let context = &request.context;
        let update = &request.update;

        if let Some(from) = update.from() {
            context.insert("event_user", Box::new(from.clone()));
        }

        if let Some(chat) = update.chat() {
            context.insert("event_chat", Box::new(chat.clone()));
        }

        if let Some(message_thread_id) = update.message_thread_id() {
            context.insert("event_message_thread_id", Box::new(message_thread_id));
        }

        Ok((request, EventReturn::default()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        enums::UpdateType,
        event::ToServiceProvider as _,
        router::{PropagateEvent as _, Router},
        types::{Chat, Message, MessageText, Update, UpdateKind, User},
    };

    #[tokio::test]
    async fn test_user_context() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update {
            kind: UpdateKind::Message(Message::Text(MessageText {
                from: Some(User::default()),
                thread_id: Some(1),
                ..Default::default()
            })),
            ..Default::default()
        };

        let mut router = Router::new("main");
        router.update.outer_middlewares.register(UserContext);
        router.message.register(|context: Arc<Context>| async move {
            context
                .get("event_user")
                .unwrap()
                .downcast_ref::<User>()
                .unwrap();
            context
                .get("event_chat")
                .unwrap()
                .downcast_ref::<Chat>()
                .unwrap();
            context
                .get("event_message_thread_id")
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap();

            Ok(EventReturn::default())
        });

        let router_service = router.to_service_provider_default().unwrap();

        let request = Request::new(Arc::new(bot), Arc::new(update), Arc::new(context));
        router_service
            .propagate_event(UpdateType::Message, request)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_user_context_panic() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        let mut router = Router::new("main");
        router.update.outer_middlewares.register(UserContext);
        router.message.register(|context: Arc<Context>| async move {
            // This should panic, because update doesn't contain user
            context
                .get("event_user")
                .unwrap()
                .downcast_ref::<User>()
                .unwrap();
            // This should panic, because update doesn't contain chat
            context
                .get("event_chat")
                .unwrap()
                .downcast_ref::<Chat>()
                .unwrap();
            // This should panic, because update doesn't contain message thread id
            context
                .get("event_message_thread_id")
                .unwrap()
                .downcast_ref::<i64>()
                .unwrap();

            Ok(EventReturn::default())
        });

        let router_service = router.to_service_provider_default().unwrap();

        let request = Request::new(Arc::new(bot), Arc::new(update), Arc::new(context));
        router_service
            .propagate_event(UpdateType::Message, request)
            .await
            .unwrap();
    }
}
