use super::{Middleware, MiddlewareResponse};
use crate::{
    context::Context as RequestContext,
    errors::{EventErrorKind, MiddlewareError},
    event::EventReturn,
    fsm::{
        storage::base::{StorageKey, DEFAULT_DESTINY},
        strategy::Strategy,
        Context, Storage,
    },
    types::{Chat, User},
    Request,
};

use async_trait::async_trait;
use std::fmt::{self, Debug, Formatter};
use tracing::instrument;

/// Middleware for creating FSM [`Context`]
#[derive(Clone)]
pub struct FSMContext<S> {
    storage: S,
    strategy: Strategy,
    destiny: &'static str,
}

impl<S> FSMContext<S> {
    #[must_use]
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            strategy: Strategy::default(),
            destiny: DEFAULT_DESTINY,
        }
    }

    #[must_use]
    pub fn strategy(self, val: Strategy) -> Self {
        Self {
            strategy: val,
            ..self
        }
    }

    #[must_use]
    pub fn destiny(self, val: &'static str) -> Self {
        Self {
            destiny: val,
            ..self
        }
    }
}

impl<S> Default for FSMContext<S>
where
    S: Default,
{
    #[must_use]
    fn default() -> Self {
        Self {
            storage: S::default(),
            strategy: Strategy::default(),
            destiny: DEFAULT_DESTINY,
        }
    }
}

impl<S> Debug for FSMContext<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FSMContext")
            .field("strategy", &self.strategy)
            .field("destiny", &self.destiny)
            .finish_non_exhaustive()
    }
}

impl<S> FSMContext<S>
where
    S: Clone,
{
    #[must_use]
    fn resolve_event_context(&self, bot_id: i64, context: &RequestContext) -> Option<Context<S>> {
        let user_id = context.get::<User>("event_user").map(|user| user.id);
        let chat_id = context.get::<Chat>("event_chat").map(Chat::id);
        let message_thread_id = context.get("event_message_thread_id").copied();
        let business_connection_id = context.get("event_business_connection_id").cloned();

        self.resolve_context(
            bot_id,
            chat_id,
            user_id,
            message_thread_id,
            business_connection_id,
        )
    }

    #[must_use]
    fn resolve_context(
        &self,
        bot_id: i64,
        chat_id: Option<i64>,
        user_id: Option<i64>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<String>,
    ) -> Option<Context<S>> {
        user_id.map(|user_id| {
            let id_pair = self.strategy.apply(
                chat_id.unwrap_or(user_id),
                user_id,
                message_thread_id,
                business_connection_id,
            );

            self.get_context(
                bot_id,
                id_pair.chat_id,
                id_pair.user_id,
                id_pair.message_thread_id,
                id_pair.business_connection_id,
            )
        })
    }

    #[must_use]
    fn get_context(
        &self,
        bot_id: i64,
        chat_id: i64,
        user_id: i64,
        message_thread_id: Option<i64>,
        business_connection_id: Option<String>,
    ) -> Context<S> {
        Context::new(
            self.storage.clone(),
            StorageKey {
                bot_id,
                chat_id,
                user_id,
                message_thread_id,
                business_connection_id,
                destiny: self.destiny,
            },
        )
    }
}

#[async_trait]
impl<Client, S> Middleware<Client> for FSMContext<S>
where
    Client: Send + Sync + 'static,
    S: Storage + Send + Sync + 'static,
{
    #[instrument(skip_all)]
    async fn call(
        &mut self,
        mut request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        let context = &mut request.context;

        if let Some(fsm_context) = self.resolve_event_context(request.bot.id, context) {
            if let Some(state) = fsm_context
                .get_state()
                .await
                .map_err(|err| MiddlewareError::new(err.into()))?
            {
                context.insert("fsm_state", state);
            }

            context.insert("fsm_context", fsm_context);
        }

        context.insert("fsm_storage", self.storage.clone());

        Ok((request, EventReturn::default()))
    }
}
