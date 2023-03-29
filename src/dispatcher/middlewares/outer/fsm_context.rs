use super::{Middleware, MiddlewareResponse};

use crate::{
    context::Context as RequestContext,
    dispatcher::{event::EventReturn, RouterRequest},
    error::AppErrorKind,
    fsm::{
        storage::base::{StorageKey, DEFAULT_DESTINY},
        strategy::Strategy,
        Context, Storage,
    },
    types::User,
};

use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::Arc;

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

impl<S> FSMContext<S>
where
    S: Clone,
{
    #[must_use]
    fn resolve_event_context(
        &self,
        bot_id: i64,
        context: &Arc<RequestContext>,
    ) -> Option<Context<S>> {
        let user = context.get("event_user").expect(
            "Event context should contain user. \
             This is a bug, please report it.",
        );
        let chat = context.get("event_chat").expect(
            "Event context should contain chat. \
             This is a bug, please report it.",
        );

        let user_id = user.downcast_ref().map(|user: &User| user.id);
        let chat_id = chat.downcast_ref().map(|chat: &User| chat.id);

        self.resolve_context(bot_id, chat_id, user_id)
    }

    #[must_use]
    fn resolve_context(
        &self,
        bot_id: i64,
        chat_id: Option<i64>,
        user_id: Option<i64>,
    ) -> Option<Context<S>> {
        user_id.map(|user_id| {
            let id_pair = self.strategy.apply(chat_id.unwrap_or(user_id), user_id);

            self.get_context(bot_id, id_pair.chat_id, id_pair.user_id)
        })
    }

    #[must_use]
    fn get_context(&self, bot_id: i64, chat_id: i64, user_id: i64) -> Context<S> {
        Context::new(
            self.storage.clone(),
            StorageKey {
                bot_id,
                chat_id,
                user_id,
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
    async fn call(
        &self,
        request: RouterRequest<Client>,
    ) -> Result<MiddlewareResponse<Client>, AppErrorKind> {
        if let Some(fsm_context) = self.resolve_event_context(request.bot.id(), &request.context) {
            let state = fsm_context
                .get_state()
                .await
                .map_err(|err| anyhow!("Failed to get FSM state: {err}"))?;

            request.context.insert("state", Box::new(state));
            request.context.insert("fsm_context", Box::new(fsm_context));
        }

        request
            .context
            .insert("fsm_storage", Box::new(self.storage.clone()));

        Ok((request, EventReturn::default()))
    }
}
