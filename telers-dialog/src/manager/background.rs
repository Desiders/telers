use telers::{client::Session, fsm::Storage, Bot};

use super::DialogManager;
use crate::{
    entities::{ChatEvent, Context, Data, DataMap, DialogAction, DialogUpdateEvent, ShowMode, StartMode},
    DialogError,
};

/// A background controller bound to an explicit stack and, optionally, an intent.
///
/// Create it before spawning work. A pinned controller rejects updates after
/// its dialog closes or another dialog covers it, so delayed work cannot modify
/// a newer conversation. Updates share the event isolation used by dialog middleware.
#[derive(Clone)]
pub struct BackgroundManager<S: Storage, Client: Session> {
    manager: DialogManager<S>,
    bot: Bot<Client>,
    intent_id: Option<String>,
}

impl<S, Client> BackgroundManager<S, Client>
where
    S: Storage + Send + Sync,
    Client: Session + Clone,
{
    fn new(manager: DialogManager<S>, bot: Bot<Client>, intent_id: Option<String>) -> Self {
        Self {
            manager,
            bot,
            intent_id,
        }
    }

    fn operation(
        &self,
        action: DialogAction,
        data: Data,
        show_mode: Option<ShowMode>,
    ) -> DialogManager<S> {
        let mut manager = self.manager.clone();
        if let ChatEvent::DialogUpdateEvent(event) = &mut manager.event {
            event.action = action;
            event.data = data;
            event.show_mode = show_mode;
        }
        manager
    }

    async fn check_target(&self) -> Result<(), DialogError> {
        let storage = self.manager.load_storage().await?;
        let stack = storage.current_stack();
        let context = stack
            .and_then(|stack| stack.last_intent_id())
            .and_then(|id| storage.contexts.get(id));
        if self
            .intent_id
            .as_ref()
            .is_some_and(|id| context.is_none_or(|context| &context.id != id))
        {
            return Err(DialogError::NoContext);
        }
        if self.intent_id.is_some()
            && stack.is_some_and(|stack| {
                stack.last_message_chat.as_ref().is_some_and(|chat| {
                    chat.id() != self.manager.event_context().chat.id()
                        || stack.last_message_thread_id != self.manager.event_context().thread_id
                        || stack.last_business_connection_id.as_deref()
                            != self.manager.event_context().business_connection_id.as_deref()
                })
            })
        {
            return Err(DialogError::NoContext);
        }
        if let Some(stack) = stack {
            self.manager
                .check_access(stack, context, self.manager.event_context())?;
        }
        Ok(())
    }

    pub async fn current_context(&self) -> Result<Context, DialogError> {
        self.manager
            .registry
            .run_isolated(self.manager.event_context(), async {
                self.check_target().await?;
                self.manager.current_context().await
            })
            .await
    }

    /// Merge data and refresh the bound dialog.
    pub async fn update(
        &self,
        data: DataMap,
        show_mode: Option<ShowMode>,
    ) -> Result<Option<i64>, DialogError> {
        self.manager
            .registry
            .run_isolated(self.manager.event_context(), async {
                self.check_target().await?;
                let manager = self.operation(
                    DialogAction::Update,
                    telers::serialization::to_value(&data).unwrap_or_default(),
                    show_mode,
                );
                manager.extend_dialog_data(data).await?;
                manager.show(&self.bot, show_mode).await
            })
            .await
    }

    pub async fn show(&self, show_mode: Option<ShowMode>) -> Result<Option<i64>, DialogError> {
        self.manager
            .registry
            .run_isolated(self.manager.event_context(), async {
                self.check_target().await?;
                self.operation(DialogAction::Update, Data::default(), show_mode)
                    .show(&self.bot, show_mode)
                    .await
            })
            .await
    }

    pub async fn switch_to(
        &self,
        state: impl Into<String>,
        show_mode: Option<ShowMode>,
    ) -> Result<Option<i64>, DialogError> {
        let state = state.into();
        self.manager
            .registry
            .run_isolated(self.manager.event_context(), async {
                self.check_target().await?;
                let manager = self.operation(
                    DialogAction::Switch,
                    telers::serialization::json!({"state": state.clone()}),
                    show_mode,
                );
                manager.switch_to(state).await?;
                manager.show(&self.bot, show_mode).await
            })
            .await
    }

    pub async fn done(&self, result: Option<Data>) -> Result<Option<Context>, DialogError> {
        self.manager
            .registry
            .run_isolated(self.manager.event_context(), async {
                self.check_target().await?;
                let manager = self.operation(
                    DialogAction::Done,
                    result.clone().unwrap_or_default(),
                    None,
                );
                match result {
                    Some(result) => manager.done_with_result(&self.bot, None, result).await,
                    None => manager.done(&self.bot, None).await,
                }
            })
            .await
    }

    /// Start on the bound stack and pin subsequent operations to the new intent.
    pub async fn start(
        &mut self,
        state: impl Into<String>,
        data: Data,
        mode: StartMode,
    ) -> Result<Context, DialogError> {
        let state = state.into();
        let registry = self.manager.registry.clone();
        let event = self.manager.event_context().clone();
        registry
            .run_isolated(&event, async {
                self.check_target().await?;
                let manager = self.operation(DialogAction::Start, data.clone(), None);
                let context = manager.start(&self.bot, state, data, mode).await?;
                self.intent_id = Some(context.id.clone());
                self.manager = manager.background_event(
                    context.stack_id.clone(),
                    Some(context.id.clone()),
                );
                Ok(context)
            })
            .await
    }
}

impl<S> DialogManager<S>
where
    S: Storage + Send + Sync,
{
    fn background_event(&self, stack_id: String, intent_id: Option<String>) -> DialogManager<S> {
        let event_context = self.event_context();
        let mut manager = self.clone().for_stack(stack_id.clone());
        manager.event = ChatEvent::DialogUpdateEvent(DialogUpdateEvent {
            user: event_context.user.clone(),
            chat: event_context.chat.clone(),
            action: DialogAction::Update,
            data: Data::default(),
            intent_id,
            stack_id: Some(stack_id),
            thread_id: event_context.thread_id,
            business_connection_id: event_context.business_connection_id.clone(),
            show_mode: None,
        });
        manager
    }

    pub async fn background<Client: Session + Clone>(
        &self,
        bot: &Bot<Client>,
    ) -> Result<BackgroundManager<S, Client>, DialogError> {
        let context = self.current_context().await?;
        Ok(BackgroundManager::new(
            self.background_event(context.stack_id, Some(context.id.clone())),
            bot.clone(),
            Some(context.id),
        ))
    }

    /// Create an unpinned controller for a named stack in the current FSM scope.
    #[must_use]
    pub fn background_for_stack<Client: Session + Clone>(
        &self,
        bot: &Bot<Client>,
        stack_id: impl Into<String>,
    ) -> BackgroundManager<S, Client> {
        BackgroundManager::new(self.background_event(stack_id.into(), None), bot.clone(), None)
    }
}
