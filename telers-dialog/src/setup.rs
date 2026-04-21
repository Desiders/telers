use crate::{
    entities::{
        chat_event_from_update, ChatEvent, EventContext, CHAT_EVENT_KEY, EVENT_CONTEXT_KEY,
    },
    manager::DialogManager,
    registry::DialogRegistry,
};
use std::{future::Future, marker::PhantomData};
use telers::{
    client::Session,
    errors::{EventErrorKind, ExtractionError, HandlerError},
    event::{
        telegram::{Handler as TelegramHandler, HandlerResult, Observer as TelegramObserver},
        EventReturn,
    },
    extractor::Extractor,
    fsm::{self, Storage},
    middlewares::outer::{Middleware, MiddlewareResponse},
    types::{CallbackQuery, Message},
    Bot, Request,
};

pub const DIALOG_MANAGER_KEY: &str = "td_dialog_manager";

/// Outer middleware that derives dialog-specific event data from `telers::Request`.
///
/// The middleware is intentionally small:
/// - supported update types produce `ChatEvent`
/// - `EventContext` and `ChatEvent` are inserted into request context
/// - unsupported updates are ignored without failing the request
#[derive(Clone, Copy, Debug, Default)]
pub struct DialogContextMiddleware;

impl DialogContextMiddleware {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Outer middleware that prepares a typed `DialogManager<S>` in request context.
#[derive(Debug)]
pub struct DialogManagerMiddleware<S> {
    marker: PhantomData<fn() -> S>,
}

impl<S> DialogManagerMiddleware<S> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<S> Clone for DialogManagerMiddleware<S> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<S> Default for DialogManagerMiddleware<S> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<Client> Middleware<Client> for DialogContextMiddleware
where
    Client: Clone + Send + Sync + 'static,
{
    fn call(
        &mut self,
        mut request: Request<Client>,
    ) -> impl Future<Output = Result<MiddlewareResponse<Client>, EventErrorKind>> + Send {
        let chat_event = chat_event_from_update(request.update.as_ref());
        if let Some(chat_event) = chat_event {
            let event_context = EventContext::new(request.bot.clone(), chat_event.clone());
            request.context.insert(EVENT_CONTEXT_KEY, event_context);
            request.context.insert(CHAT_EVENT_KEY, chat_event);
        }
        async move { Ok((request, EventReturn::default())) }
    }
}

impl<Client, S> Middleware<Client> for DialogManagerMiddleware<S>
where
    Client: Clone + Send + Sync + 'static,
    S: Storage + Send + Sync + 'static,
{
    fn call(
        &mut self,
        mut request: Request<Client>,
    ) -> impl Future<Output = Result<MiddlewareResponse<Client>, EventErrorKind>> + Send {
        let fsm = request.context.get::<fsm::Context<S>>("fsm_context");
        let registry = request.extensions.get::<DialogRegistry>();
        let event = request.context.get::<ChatEvent>(CHAT_EVENT_KEY);

        if let (Some(fsm), Some(registry), Some(event)) = (fsm, registry, event) {
            let manager = DialogManager::new(
                fsm.clone(),
                registry.clone(),
                request.context.clone(),
                event.clone(),
            );
            request.context.insert(DIALOG_MANAGER_KEY, manager);
        }
        async move { Ok((request, EventReturn::default())) }
    }
}

impl<Client, S> Extractor<Client> for DialogManager<S>
where
    Client: Clone + Send + Sync + 'static,
    S: Storage + Send + Sync + 'static,
{
    type Error = ExtractionError;

    async fn extract(request: &Request<Client>) -> Result<Self, Self::Error> {
        let Some(manager) = request.context.get::<DialogManager<S>>(DIALOG_MANAGER_KEY) else {
            return Err(ExtractionError::new(
                "`DialogManager` is missing in request context. Make sure to register \
                 `DialogManagerMiddleware` after `DialogContextMiddleware` in your middleware \
                 stack. Also ensure that `DialogRegistry` is properly registered in \
                 `telers::Extensions`.",
            ));
        };
        Ok(manager.clone())
    }
}

async fn dispatch_dialog_event<Client, S>(
    bot: Bot<Client>,
    message: Option<Message>,
    callback_query: Option<CallbackQuery>,
    manager: Option<DialogManager<S>>,
) -> HandlerResult<EventReturn>
where
    Client: Session + Clone + Send + Sync + 'static,
    S: Storage + Send + Sync + 'static,
{
    let Some(manager) = manager else {
        return Ok(EventReturn::Skip);
    };

    if let Some(callback_query) = callback_query {
        return manager
            .handle_callback_query(&bot, callback_query)
            .await
            .map(|handled| {
                if handled {
                    EventReturn::Finish
                } else {
                    EventReturn::Skip
                }
            })
            .map_err(HandlerError::new);
    }

    if let Some(message) = message {
        return manager
            .handle_message(&bot, message)
            .await
            .map(|handled| {
                if handled {
                    EventReturn::Finish
                } else {
                    EventReturn::Skip
                }
            })
            .map_err(HandlerError::new);
    }

    Ok(EventReturn::Skip)
}

pub trait DialogObserverExt<Client>: Sized {
    #[must_use]
    fn setup_dialogs<S>(self) -> Self
    where
        S: Storage + Send + Sync + 'static;
}

impl<Client> DialogObserverExt<Client> for TelegramObserver<Client>
where
    Client: Session + Clone + Send + Sync + 'static,
{
    #[inline]
    fn setup_dialogs<S>(self) -> Self
    where
        S: Storage + Send + Sync + 'static,
    {
        self.register_outer_middleware(DialogContextMiddleware::new())
            .register_outer_middleware(DialogManagerMiddleware::<S>::new())
            .register(TelegramHandler::new(dispatch_dialog_event::<Client, S>))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DialogContextMiddleware, DialogManagerMiddleware, DialogObserverExt, DIALOG_MANAGER_KEY,
    };
    use crate::{
        dialog,
        entities::{CHAT_EVENT_KEY, EVENT_CONTEXT_KEY},
        widgets::text,
        window, DialogManager, DialogRegistry,
    };
    use std::{convert::Infallible, sync::Arc};
    use telers::{
        client::Reqwest,
        context::Context,
        enums::UpdateType,
        event::{bases::PropagateEventResult, telegram::Handler as TelegramHandler, EventReturn},
        extractor::Extractor,
        fsm::{Context as FSMContext, MemoryStorage, StorageKey},
        middlewares::outer::Middleware,
        router::PropagateEvent,
        types::{
            CallbackQuery, ChatPrivate, Message, MessageText, Update, UpdateCallbackQuery,
            UpdateMessage, User,
        },
        Bot, Extensions, Request, Router,
    };

    #[tokio::test]
    async fn middleware_inserts_dialog_event_context_for_message() {
        let message = MessageText::new(1, 1, ChatPrivate::new(10), "hello")
            .from(User::new(10, false, "user"));
        let request = Request::<Reqwest> {
            bot: Bot::default(),
            update: Arc::new(Update::Message(UpdateMessage::new(1, message))),
            context: Context::default(),
            extensions: Extensions::default(),
        };
        let mut middleware = DialogContextMiddleware::new();

        let (request, _) = middleware.call(request).await.expect("middleware");

        assert!(request.context.contains_key(EVENT_CONTEXT_KEY));
        assert!(request.context.contains_key(CHAT_EVENT_KEY));
    }

    #[tokio::test]
    async fn dialog_manager_middleware_requires_dialog_context() {
        let bot = Bot::default();
        let user = User::new(5, false, "tester");
        let message: Message = MessageText::new(1, 1, ChatPrivate::new(5), "hello")
            .from(user.clone())
            .into();
        let update = Update::CallbackQuery(UpdateCallbackQuery::new(
            1,
            CallbackQuery::new("callback", user, "chat").message(message),
        ));
        let mut request = Request::<Reqwest> {
            bot: bot.clone(),
            update: Arc::new(update),
            context: Context::default(),
            extensions: Extensions::default(),
        };
        let key = StorageKey::new(bot.id, 5, 5, None, None);
        request
            .context
            .insert("fsm_context", FSMContext::new(MemoryStorage::new(), key));
        request.extensions.insert(
            DialogRegistry::new()
                .register(dialog([window("state", [text("hello")])]))
                .expect("dialog registration"),
        );

        let mut middleware = DialogManagerMiddleware::<MemoryStorage>::new();
        let (request, _) = middleware.call(request).await.expect("middleware");

        assert!(!request.context.contains_key(EVENT_CONTEXT_KEY));
        assert!(!request.context.contains_key(CHAT_EVENT_KEY));
        assert!(!request.context.contains_key(DIALOG_MANAGER_KEY));
    }

    #[tokio::test]
    async fn dialog_manager_is_extractable_after_setup() {
        let bot = Bot::default();
        let user = User::new(5, false, "tester");
        let message: Message = MessageText::new(1, 1, ChatPrivate::new(5), "hello")
            .from(user.clone())
            .into();
        let update = Update::CallbackQuery(UpdateCallbackQuery::new(
            1,
            CallbackQuery::new("callback", user, "chat").message(message),
        ));
        let mut request = Request::<Reqwest> {
            bot: bot.clone(),
            update: Arc::new(update),
            context: Context::default(),
            extensions: Extensions::default(),
        };
        let key = StorageKey::new(bot.id, 5, 5, None, None);
        request
            .context
            .insert("fsm_context", FSMContext::new(MemoryStorage::new(), key));
        request.extensions.insert(
            DialogRegistry::new()
                .register(dialog([window("state", [text("hello")])]))
                .expect("dialog registration"),
        );

        let mut context_middleware = DialogContextMiddleware::new();
        let (request, _) = context_middleware.call(request).await.expect("middleware");
        let mut manager_middleware = DialogManagerMiddleware::<MemoryStorage>::new();
        let (request, _) = manager_middleware.call(request).await.expect("middleware");
        let extracted = DialogManager::<MemoryStorage>::extract(&request).await;

        assert!(
            extracted.is_ok(),
            "{:?}",
            extracted.err().map(|err| err.to_string())
        );
    }

    #[tokio::test]
    async fn dialog_observer_ext_registers_working_middlewares() {
        let bot = Bot::default();
        let user = User::new(5, false, "tester");
        let message: Message = MessageText::new(1, 1, ChatPrivate::new(5), "hello")
            .from(user.clone())
            .into();
        let update = Arc::new(Update::CallbackQuery(UpdateCallbackQuery::new(
            1,
            CallbackQuery::new("callback", user, "chat").message(message),
        )));
        let key = StorageKey::new(bot.id, 5, 5, None, None);

        let router = Router::new("dialogs").on_callback_query(|observer| {
            observer
                .setup_dialogs::<MemoryStorage>()
                .register(TelegramHandler::new(
                    |_: DialogManager<MemoryStorage>| async move {
                        Ok::<_, Infallible>(EventReturn::Finish)
                    },
                ))
        });
        let mut router = router.configure_default();
        let mut context = Context::default();
        context.insert("fsm_context", FSMContext::new(MemoryStorage::new(), key));
        let mut extensions = Extensions::default();
        extensions.insert(
            DialogRegistry::new()
                .register(dialog([window("state", [text("hello")])]))
                .expect("dialog registration"),
        );
        let request = Request::<Reqwest> {
            bot: bot.clone(),
            update,
            context,
            extensions,
        };

        let response = router
            .propagate_event(UpdateType::CallbackQuery, request)
            .await
            .expect("router");

        match response.propagate_result {
            PropagateEventResult::Handled(response) => {
                assert!(matches!(response.result, Ok(EventReturn::Finish)));
            }
            other => panic!("unexpected propagation result: {other:?}"),
        }
    }

    #[tokio::test]
    async fn dialog_observer_ext_marks_message_and_callback_observers_as_used() {
        let router = Router::<Reqwest>::new("dialogs")
            .on_message(|observer| observer.setup_dialogs::<MemoryStorage>())
            .on_callback_query(|observer| observer.setup_dialogs::<MemoryStorage>());

        let update_types = router.resolve_used_update_types();

        assert!(update_types.contains(UpdateType::Message.as_ref()));
        assert!(update_types.contains(UpdateType::CallbackQuery.as_ref()));
    }
}
