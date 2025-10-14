use super::{Middleware, MiddlewareResponse};
use crate::{errors::EventErrorKind, event::EventReturn, Request};

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

impl<Client> Middleware<Client> for UserContext
where
    Client: Send + Sync + 'static,
{
    #[instrument(skip_all)]
    async fn call(
        &mut self,
        mut request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        if let Some(from) = request.update.from() {
            request.context.insert("event_user", from.clone());
        }
        if let Some(chat) = request.update.chat() {
            request.context.insert("event_chat", chat.clone());
        }
        if let Some(message_thread_id) = request.update.message_thread_id() {
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
        router::{PropagateEvent as _, Router},
        types::{Chat, Message, MessageText, Update, UpdateKind, User},
    };

    use std::{convert::Infallible, sync::Arc};

    #[tokio::test]
    async fn test_user_context() {
        let mut router = Router::new("main");
        router.update.outer_middlewares.register(UserContext);
        router.message.register(|context: Context| async move {
            context.get::<User>("event_user").unwrap();
            context.get::<Chat>("event_chat").unwrap();
            context.get::<i64>("event_message_thread_id").unwrap();

            Ok::<_, Infallible>(EventReturn::default())
        });

        let mut router_configured = router.configure_default();

        let request = Request::<Reqwest> {
            update: Arc::new(Update {
                kind: UpdateKind::Message(Message::Text(Box::new(MessageText {
                    from: Some(Default::default()),
                    thread_id: Some(1),
                    ..Default::default()
                }))),
                ..Default::default()
            }),
            ..Default::default()
        };

        router_configured
            .propagate_event(UpdateType::Message, request)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_user_context_panic() {
        let mut router = Router::new("main");
        router.update.outer_middlewares.register(UserContext);
        router.message.register(|context: Context| async move {
            // This should panic, because update doesn't contain user
            context.get::<User>("event_user").unwrap();
            // This should panic, because update doesn't contain chat
            context.get::<Chat>("event_chat").unwrap();
            // This should panic, because update doesn't contain message thread id
            context.get::<i64>("event_message_thread_id").unwrap();

            Ok::<_, Infallible>(EventReturn::default())
        });

        let mut router_configured = router.configure_default();

        let request = Request::<Reqwest>::default();
        router_configured
            .propagate_event(UpdateType::Message, request)
            .await
            .unwrap();
    }
}
