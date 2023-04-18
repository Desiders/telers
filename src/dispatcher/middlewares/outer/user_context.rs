use super::{Middleware, MiddlewareResponse};

use crate::{
    dispatcher::{event::EventReturn, RouterRequest},
    error::AppErrorKind,
};

use async_trait::async_trait;

/// Middleware for adding [`crate::types::User`] and [`crate::types::Chat`] to context,
/// if they are present in [`crate::types::Update`] struct
#[derive(Debug, Default, Clone)]
pub struct UserContext;

impl UserContext {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl<Client> Middleware<Client> for UserContext
where
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: RouterRequest<Client>,
    ) -> Result<MiddlewareResponse<Client>, AppErrorKind> {
        let context = &request.context;
        let update = &request.update;

        if let Some(user) = update.user() {
            context.insert("event_user", Box::new(user.clone()));
        }

        if let Some(chat) = update.chat() {
            context.insert("event_chat", Box::new(chat.clone()));
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
        dispatcher::{event::ToServiceProvider as _, Router},
        enums::UpdateType,
        types::{Chat, Message, Update, User},
    };

    #[tokio::test]
    async fn test_user_context() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update {
            message: Some(Message {
                chat: Box::new(Chat::default()),
                from: Some(User::default()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut router = Router::new("main");
        router
            .update
            .outer_middlewares
            .register(UserContext::default());
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

            Ok(EventReturn::default())
        });

        let router_service = router.to_service_provider_default().unwrap();

        let request = RouterRequest::new(bot, update, context);
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
        router
            .update
            .outer_middlewares
            .register(UserContext::default());
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

            Ok(EventReturn::default())
        });

        let router_service = router.to_service_provider_default().unwrap();

        let request = RouterRequest::new(bot, update, context);
        router_service
            .propagate_event(UpdateType::Message, request)
            .await
            .unwrap();
    }
}
