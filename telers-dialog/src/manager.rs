use crate::{
    entities::{
        generate_id, ChatEvent, Context, Data, DataMap, EventContext, LaunchMode, OldMessage,
        ShowMode, Stack, StartMode, DEFAULT_STACK_ID, EVENT_CONTEXT_KEY,
    },
    errors::DialogError,
    message_manager::MessageManager,
    registry::DialogRegistry,
    widgets::ButtonAction,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, mem};
use telers::{
    client::Session,
    enums::ReplyMarkupType,
    fsm::Storage,
    methods::AnswerCallbackQuery,
    types::{CallbackQuery, MaybeInaccessibleMessage, ReplyMarkup},
    Bot,
};
use tracing::{debug, error, trace};

const STORAGE_KEY: &str = "td_storage";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct DialogStorage {
    stacks: BTreeMap<String, Stack>,
    contexts: BTreeMap<String, Context>,
    current_stack: String,
}

impl DialogStorage {
    fn current_stack_mut(&mut self) -> &mut Stack {
        let id = if self.current_stack.is_empty() {
            DEFAULT_STACK_ID
        } else {
            &self.current_stack
        };
        self.stacks.entry(id.to_string()).or_insert_with(|| Stack {
            id: id.to_string(),
            ..Stack::default()
        })
    }

    fn current_stack(&self) -> Option<&Stack> {
        let id = if self.current_stack.is_empty() {
            DEFAULT_STACK_ID
        } else {
            &self.current_stack
        };
        self.stacks.get(id)
    }
}

#[derive(Clone)]
pub struct DialogManager<S: Storage> {
    fsm: telers::fsm::Context<S>,
    registry: DialogRegistry,
    show_mode: ShowMode,
    context: telers::Context,
    event: ChatEvent,
}

impl<S: Storage> DialogManager<S> {
    /// Create a new dialog manager bound to FSM storage and runtime context.
    ///
    /// # Notes
    /// - `context` must contain `EventContext` under `EVENT_CONTEXT_KEY`.
    /// - `event` should match the same update used to build `EventContext`.
    #[inline]
    #[must_use]
    pub fn new(
        fsm: telers::fsm::Context<S>,
        registry: DialogRegistry,
        context: telers::Context,
        event: ChatEvent,
    ) -> Self {
        Self {
            fsm,
            registry,
            show_mode: ShowMode::Auto,
            context,
            event,
        }
    }

    /// Current event handled by this manager.
    ///
    /// # Notes
    /// Used for show mode calculation and extracting last message from callbacks.
    #[inline]
    #[must_use]
    pub fn event(&self) -> &ChatEvent {
        &self.event
    }

    /// Current show mode override.
    ///
    /// # Notes
    /// When not `Auto`, it forces show mode in `show()` and `done()`.
    #[inline]
    #[must_use]
    pub fn show_mode(&self) -> ShowMode {
        self.show_mode
    }

    /// Set show mode override.
    ///
    /// # Notes
    /// Affects subsequent `show()` calls, including implicit ones from `start()` and `done()`.
    #[inline]
    pub fn set_show_mode(&mut self, mode: ShowMode) {
        self.show_mode = mode;
    }

    /// Access registry used by this manager.
    ///
    /// # Notes
    /// Registry maps state strings to dialog instances.
    #[inline]
    #[must_use]
    pub fn registry(&self) -> &DialogRegistry {
        &self.registry
    }

    /// Get event context from runtime context.
    ///
    /// # Panics
    /// Panics if `EVENT_CONTEXT_KEY` is missing.
    #[inline]
    fn event_context(&self) -> &EventContext {
        self.context
            .get(EVENT_CONTEXT_KEY)
            .expect("Event context not found")
    }

    /// Build last shown message snapshot for the current stack.
    ///
    /// # Notes
    /// For callback queries tries to use message from callback payload.
    fn get_last_message(&self, stack: &Stack, event_ctx: &EventContext) -> Option<OldMessage> {
        if let ChatEvent::CallbackQuery(cb) = &self.event {
            if let Some(message) = cb.message.clone() {
                let (chat, message_id, text, reply_markup_value, link_preview_options_value) =
                    match *message {
                        MaybeInaccessibleMessage::InaccessibleMessage(m) => (
                            *m.chat,
                            m.message_id,
                            stack.last_text.clone(),
                            stack.last_reply_markup.clone(),
                            stack.last_link_preview_options.clone(),
                        ),
                        MaybeInaccessibleMessage::Message(m) => (
                            m.chat().clone(),
                            m.message_id(),
                            m.text().map(Into::into),
                            m.reply_markup()
                                .cloned()
                                .map(ReplyMarkup::InlineKeyboardMarkup)
                                .and_then(|markup| serde_json::to_value(markup).ok()),
                            None,
                        ),
                    };
                return Some(OldMessage::new(
                    chat,
                    message_id,
                    text,
                    stack.has_protected_content,
                    if stack.last_reply_keyboard {
                        Some(ReplyMarkupType::ReplyKeyboardMarkup)
                    } else {
                        None
                    },
                    reply_markup_value,
                    event_ctx.business_connection_id.clone(),
                    stack.message_type,
                    link_preview_options_value,
                ));
            }
        }
        let id = stack.last_message_id?;
        Some(OldMessage::new(
            event_ctx.chat.clone(),
            id,
            stack.last_text.clone(),
            stack.has_protected_content,
            if stack.last_reply_keyboard {
                Some(ReplyMarkupType::ReplyKeyboardMarkup)
            } else {
                None
            },
            stack.last_reply_markup.clone(),
            event_ctx.business_connection_id.clone(),
            stack.message_type,
            stack.last_link_preview_options.clone(),
        ))
    }

    /// Calculate show mode based on chat type, stack state and current event.
    fn calc_show_mode(&self, stack: &Stack, event_ctx: &EventContext) -> ShowMode {
        if self.show_mode != ShowMode::Auto {
            return self.show_mode;
        }
        let is_private = matches!(event_ctx.chat, telers::types::Chat::Private(_));
        if !is_private {
            return ShowMode::Edit;
        }
        if stack.last_reply_keyboard {
            return ShowMode::DeleteAndSend;
        }
        if stack.id != DEFAULT_STACK_ID {
            return ShowMode::Edit;
        }
        if let ChatEvent::Message(message) = &self.event {
            if message.media_group_id().is_none() {
                return ShowMode::Send;
            }
            let mg = message.media_group_id();
            if mg == stack.last_income_media_group_id.as_deref() {
                return ShowMode::Edit;
            }
            return ShowMode::Send;
        }
        ShowMode::Edit
    }

    /// Load dialog storage from FSM.
    async fn load_storage(&self) -> Result<DialogStorage, DialogError> {
        Ok(self
            .fsm
            .get_value::<_, DialogStorage>(STORAGE_KEY)
            .await
            .map_err(Into::into)?
            .unwrap_or_default())
    }

    /// Save dialog storage to FSM.
    async fn save_storage(&self, storage: DialogStorage) -> Result<(), DialogError> {
        self.fsm
            .set_value(STORAGE_KEY, storage)
            .await
            .map_err(Into::into)?;
        Ok(())
    }

    fn resolve_dialog<'a>(
        &self,
        state: &'a str,
    ) -> Result<std::sync::Arc<dyn crate::dialog::Dialog>, DialogError> {
        self.registry
            .find_by_state(state)
            .ok_or(DialogError::DialogNotFound)
    }

    fn clear_current_stack(storage: &mut DialogStorage) {
        let ids = {
            let stack = storage.current_stack_mut();
            mem::take(&mut stack.intents)
        };
        for id in ids {
            storage.contexts.remove(&id);
        }
    }

    /// Check whether current stack has context.
    ///
    /// # Errors
    /// Returns storage errors.
    pub async fn has_context(&self) -> Result<bool, DialogError> {
        let storage = self.load_storage().await?;
        Ok(storage
            .current_stack()
            .and_then(Stack::last_intent_id)
            .is_some_and(|id| storage.contexts.contains_key(id)))
    }

    /// Get current dialog context.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If storage error occurs.
    pub async fn current_context(&self) -> Result<Context, DialogError> {
        let storage = self.load_storage().await?;
        let stack = storage.current_stack().ok_or(DialogError::NoContext)?;
        let intent_id = stack.last_intent_id().ok_or(DialogError::NoContext)?;
        storage
            .contexts
            .get(intent_id)
            .cloned()
            .ok_or(DialogError::NoContext)
    }

    async fn apply_button_action<Client: Session>(
        &self,
        bot: &Bot<Client>,
        action: ButtonAction,
    ) -> Result<bool, DialogError> {
        debug!(action = ?action, "Apply button action");
        let mut needs_show = false;
        let mut handled = false;
        let mut pending = vec![action];
        while let Some(action) = pending.pop() {
            match action {
                ButtonAction::Noop => handled = true,
                ButtonAction::SwitchTo(state) => {
                    self.switch_to(state).await?;
                    needs_show = true;
                    handled = true;
                }
                ButtonAction::Next => {
                    self.next().await?;
                    needs_show = true;
                    handled = true;
                }
                ButtonAction::Back => {
                    self.back().await?;
                    needs_show = true;
                    handled = true;
                }
                ButtonAction::Start { state, data, mode } => {
                    let _ = self.start(bot, state, data, mode).await?;
                    handled = true;
                }
                ButtonAction::Done => {
                    let _ = self.done(bot, None).await?;
                    handled = true;
                }
                ButtonAction::SetDialogData(data) => {
                    self.set_dialog_data(data).await?;
                    needs_show = true;
                    handled = true;
                }
                ButtonAction::SetDialogValue { key, value } => {
                    self.set_dialog_value(key, value).await?;
                    needs_show = true;
                    handled = true;
                }
                ButtonAction::Chain(actions) => {
                    pending.extend(actions.into_vec().into_iter().rev());
                }
            }
        }
        if needs_show && self.has_context().await? {
            let _ = self.show(bot, None).await?;
        }
        Ok(handled)
    }

    /// Start dialog and show it.
    ///
    /// # Notes
    /// - `StartMode::ResetStack` clears current stack and its contexts.
    /// - `StartMode::NewStack` creates a new stack and makes it current.
    ///
    /// # Errors
    /// - If storage error occurs.
    /// - If showing fails.
    pub async fn start<Client: Session>(
        &self,
        bot: &Bot<Client>,
        state: impl Into<String>,
        data: Data,
        mode: StartMode,
    ) -> Result<Context, DialogError> {
        let state = state.into();
        debug!(state = %state, mode = ?mode, "Start dialog");
        let target_dialog = self.resolve_dialog(&state)?;
        let mut storage = self.load_storage().await?;
        if let Some(current_ctx) = storage
            .current_stack()
            .and_then(Stack::last_intent_id)
            .and_then(|id| storage.contexts.get(id))
            .cloned()
        {
            let current_dialog = self.resolve_dialog(&current_ctx.state)?;
            if current_dialog.launch_mode() == LaunchMode::Exclusive
                && !current_dialog.contains_state(&state)
            {
                error!(
                    current_state = %current_ctx.state,
                    requested_state = %state,
                    "Reject start because exclusive dialog is active"
                );
                return Err(DialogError::ExclusiveDialogActive);
            }
        }

        let effective_mode = match target_dialog.launch_mode() {
            LaunchMode::Root | LaunchMode::Exclusive => StartMode::ResetStack,
            _ => mode,
        };

        if target_dialog.launch_mode() == LaunchMode::SingleTop
            && effective_mode != StartMode::NewStack
        {
            let top_id = storage
                .current_stack()
                .and_then(Stack::last_intent_id)
                .map(ToOwned::to_owned);
            if let Some(top_id) = top_id {
                if let Some(ctx) = storage.contexts.get_mut(&top_id) {
                    if target_dialog.contains_state(&ctx.state) {
                        debug!(
                            context_id = %ctx.id,
                            state = %state,
                            "Reuse top dialog context"
                        );
                        ctx.state = state.clone();
                        ctx.start_data = data;
                        ctx.dialog_data.clear();
                        ctx.widget_data.clear();
                        ctx.access_settings = None;
                        let ctx = ctx.clone();
                        self.save_storage(storage).await?;
                        let _ = self.show(bot, None).await?;
                        return Ok(ctx);
                    }
                }
            }
        }

        match effective_mode {
            StartMode::Normal => {}
            StartMode::ResetStack => {
                debug!("Reset current dialog stack before start");
                Self::clear_current_stack(&mut storage)
            }
            StartMode::NewStack => {
                let mut stack = Stack::default();
                stack.id = generate_id();
                storage.current_stack = stack.id.clone();
                storage.stacks.insert(stack.id.clone(), stack);
                debug!(stack_id = %storage.current_stack, "Created new dialog stack");
            }
        }
        let ctx = { storage.current_stack_mut().push(state, data) };
        debug!(context_id = %ctx.id, state = %ctx.state, "Pushed new dialog context");
        storage.contexts.insert(ctx.id.clone(), ctx.clone());
        self.save_storage(storage).await?;
        let _ = self.show(bot, None).await?;
        Ok(ctx)
    }

    /// Switch current context state.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If storage error occurs.
    pub async fn switch_to(&self, state: impl Into<String>) -> Result<(), DialogError> {
        let state = state.into();
        debug!(state = %state, "Switch current dialog state");
        let mut storage = self.load_storage().await?;
        let stack = storage.current_stack_mut();
        let id = stack
            .last_intent_id()
            .ok_or(DialogError::NoContext)?
            .to_owned();
        let current_state = storage
            .contexts
            .get(&id)
            .map(|ctx| ctx.state.clone())
            .ok_or(DialogError::NoContext)?;
        let dialog = self.resolve_dialog(&current_state)?;
        if !dialog.contains_state(&state) {
            return Err(DialogError::InvalidState(state));
        }
        let ctx = storage
            .contexts
            .get_mut(&id)
            .ok_or(DialogError::NoContext)?;
        debug!(context_id = %ctx.id, from = %ctx.state, to = %state, "Dialog state switched");
        ctx.state = state;
        self.save_storage(storage).await?;
        Ok(())
    }

    /// Move to the next state of the current dialog.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If there is no next state.
    /// - If storage error occurs.
    pub async fn next(&self) -> Result<(), DialogError> {
        let ctx = self.current_context().await?;
        let dialog = self.resolve_dialog(&ctx.state)?;
        let next_state = dialog
            .next_state(&ctx.state)
            .ok_or_else(|| DialogError::TransitionNotFound(ctx.state.clone()))?;
        debug!(from = %ctx.state, to = %next_state, "Move to next dialog state");
        self.switch_to(next_state).await
    }

    /// Move to the previous state of the current dialog.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If there is no previous state.
    /// - If storage error occurs.
    pub async fn back(&self) -> Result<(), DialogError> {
        let ctx = self.current_context().await?;
        let dialog = self.resolve_dialog(&ctx.state)?;
        let prev_state = dialog
            .prev_state(&ctx.state)
            .ok_or_else(|| DialogError::TransitionNotFound(ctx.state.clone()))?;
        debug!(from = %ctx.state, to = %prev_state, "Move to previous dialog state");
        self.switch_to(prev_state).await
    }

    /// Try to handle a callback query with the current dialog keyboard.
    ///
    /// Returns `true` when callback data belongs to the current dialog and was handled.
    pub async fn handle_callback_query<Client: Session>(
        &self,
        bot: &Bot<Client>,
        callback_query: &CallbackQuery,
    ) -> Result<bool, DialogError> {
        let Some(callback_data) = callback_query.data.as_deref() else {
            trace!("Callback query has no data");
            return Ok(false);
        };
        debug!(callback_data = %callback_data, "Handle dialog callback query");
        let ctx = match self.current_context().await {
            Ok(ctx) => ctx,
            Err(DialogError::NoContext) => {
                trace!("Ignoring callback because no active dialog context exists");
                return Ok(false);
            }
            Err(err) => return Err(err),
        };
        let dialog = self.resolve_dialog(&ctx.state)?;
        let Some(action) = dialog.handle_callback(&ctx.state, &ctx, callback_data) else {
            trace!(state = %ctx.state, "Callback does not belong to current dialog");
            return Ok(false);
        };
        self.answer_callback(bot, callback_query).await?;
        self.apply_button_action(bot, action).await
    }

    /// Answer a callback query without notification text.
    pub async fn answer_callback<Client: Session>(
        &self,
        bot: &Bot<Client>,
        callback_query: &CallbackQuery,
    ) -> Result<(), DialogError> {
        trace!(callback_id = %callback_query.id, "Answer callback query");
        let _ = bot
            .send(AnswerCallbackQuery::new(callback_query.id.clone()))
            .await?;
        Ok(())
    }

    /// Close current dialog and show previous one if needed.
    ///
    /// # Notes
    /// Shows previous context only if the current context didn't change during close.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If storage error occurs.
    /// - If showing fails.
    pub async fn done<Client: Session>(
        &self,
        bot: &Bot<Client>,
        show_mode: Option<ShowMode>,
    ) -> Result<Option<Context>, DialogError> {
        debug!(show_mode = ?show_mode, "Close current dialog context");
        let mut storage = self.load_storage().await?;
        let event_ctx = self.event_context();
        let old_message = storage
            .current_stack()
            .and_then(|stack| self.get_last_message(stack, event_ctx));
        let close_show_mode = storage
            .current_stack()
            .map(|stack| {
                let show_mode = show_mode.unwrap_or(self.show_mode);
                if show_mode == ShowMode::Auto {
                    self.calc_show_mode(stack, event_ctx)
                } else {
                    show_mode
                }
            })
            .unwrap_or(ShowMode::Auto);
        let id = {
            let stack = storage.current_stack_mut();
            stack.pop().ok_or(DialogError::NoContext)?
        };
        let ctx = storage.contexts.remove(&id);
        trace!(context_id = %id, found = ctx.is_some(), "Removed current dialog context");
        let should_show = storage
            .current_stack()
            .and_then(|s| s.last_intent_id())
            .is_some_and(|id| storage.contexts.contains_key(id));
        if !should_show {
            storage.current_stack_mut().clear_last_message();
        }
        self.save_storage(storage).await?;
        if should_show {
            trace!("Show previous dialog context after done");
            let _ = self.show(bot, show_mode).await?;
        } else {
            MessageManager::close_message(bot, close_show_mode, old_message.as_ref()).await?;
        }
        Ok(ctx)
    }

    /// Get dialog data for current context.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If storage error occurs.
    pub async fn dialog_data(&self) -> Result<DataMap, DialogError> {
        Ok(self.current_context().await?.dialog_data)
    }

    /// Replace dialog data for current context.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If storage error occurs.
    pub async fn set_dialog_data(&self, data: DataMap) -> Result<(), DialogError> {
        debug!(keys = data.len(), "Replace dialog data");
        let mut storage = self.load_storage().await?;
        let stack = storage.current_stack_mut();
        let id = stack
            .last_intent_id()
            .ok_or(DialogError::NoContext)?
            .to_string();
        let ctx = storage
            .contexts
            .get_mut(&id)
            .ok_or(DialogError::NoContext)?;
        ctx.dialog_data = data;
        self.save_storage(storage).await?;
        Ok(())
    }

    /// Set single value in dialog data for current context.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If storage error occurs.
    pub async fn set_dialog_value(
        &self,
        key: impl Into<String>,
        value: Data,
    ) -> Result<(), DialogError> {
        let key = key.into();
        debug!(key = %key, value = %value, "Set dialog value");
        let mut storage = self.load_storage().await?;
        let stack = storage.current_stack_mut();
        let id = stack
            .last_intent_id()
            .ok_or(DialogError::NoContext)?
            .to_string();
        let ctx = storage
            .contexts
            .get_mut(&id)
            .ok_or(DialogError::NoContext)?;
        ctx.dialog_data.insert(key, value);
        self.save_storage(storage).await?;
        Ok(())
    }

    /// Render and show current dialog.
    ///
    /// # Notes
    /// If `show_mode` is provided, it overrides `self.show_mode` for this call.
    ///
    /// # Errors
    /// - If there is no current context.
    /// - If dialog is not found.
    /// - If sending/editing message fails.
    pub async fn show<Client: Session>(
        &self,
        bot: &Bot<Client>,
        show_mode: Option<ShowMode>,
    ) -> Result<Option<i64>, DialogError> {
        debug!(show_mode = ?show_mode, "Render and show current dialog");
        let event_ctx = self.event_context();
        let mut storage = self.load_storage().await?;
        let stack = storage.current_stack().ok_or(DialogError::NoContext)?;
        let intent_id = stack
            .last_intent_id()
            .ok_or(DialogError::NoContext)?
            .to_string();
        let ctx = storage
            .contexts
            .get(&intent_id)
            .cloned()
            .ok_or(DialogError::NoContext)?;
        let dialog = self
            .registry
            .find_by_state(&ctx.state)
            .ok_or(DialogError::DialogNotFound)?;
        let data = ctx.dialog_data.clone();
        let msg = dialog
            .render(&ctx.state, &ctx, &data, event_ctx)
            .ok_or(DialogError::DialogNotFound)?;
        let mut msg = msg;
        if let Some(sm) = show_mode {
            msg.show_mode = sm;
        }
        if msg.show_mode == ShowMode::Auto {
            msg.show_mode = self.calc_show_mode(stack, event_ctx);
        }
        let old_message = self.get_last_message(stack, event_ctx);
        let new_old = MessageManager::show_message(bot, msg, old_message).await?;
        let stack = storage.current_stack_mut();
        stack.last_message_id = Some(new_old.message_id);
        stack.last_text = new_old.text.clone();
        stack.last_reply_keyboard = matches!(
            new_old.reply_markup_type,
            Some(ReplyMarkupType::ReplyKeyboardMarkup)
        );
        stack.last_reply_markup = new_old.reply_markup_value.clone();
        stack.last_link_preview_options = new_old.link_preview_options_value.clone();
        stack.has_protected_content = new_old.has_protected_content;
        stack.message_type = new_old.message_type;
        if let ChatEvent::Message(message) = &self.event {
            stack.last_income_media_group_id = message.media_group_id().map(ToOwned::to_owned);
        }
        debug!(
            message_id = new_old.message_id,
            "Updated last dialog message snapshot"
        );
        self.save_storage(storage).await?;
        Ok(Some(new_old.message_id))
    }
}
