use super::event::{
    bases::{EventReturn, PropagateEventResult},
    service::{ServiceProvider, ToServiceProvider},
    simple::{
        handler::Result as SimpleHandlerResult,
        observer::{Observer as SimpleObserver, ObserverInner as SimpleObserverInner},
    },
    telegram::observer::{
        Observer as TelegramObserver, ObserverInner as TelegramObserverInner,
        Request as TelegramObserverRequest,
    },
};

use crate::{
    client::Bot,
    context::Context,
    enums::{
        observer_name::{Simple as SimpleObserverName, Telegram as TelegramObserverName},
        update_type::UpdateType,
    },
    error::AppErrorKind,
    types::Update,
};

use async_recursion::async_recursion;
use log;
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    iter::once,
    sync::Arc,
};

/// Request for a router service
#[derive(Debug, Clone)]
pub struct Request<Client> {
    pub bot: Arc<Bot<Client>>,
    pub update: Arc<Update>,
    pub context: Arc<Context>,
}

impl<Client> PartialEq for Request<Client> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

impl<Client> Request<Client> {
    #[must_use]
    pub fn new<B, U, C>(bot: B, update: U, context: C) -> Self
    where
        B: Into<Arc<Bot<Client>>>,
        U: Into<Arc<Update>>,
        C: Into<Arc<Context>>,
    {
        Self {
            bot: bot.into(),
            update: update.into(),
            context: context.into(),
        }
    }
}

impl<Client> From<Request<Client>> for TelegramObserverRequest<Client> {
    fn from(req: Request<Client>) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

/// Response from a router service
#[derive(Debug)]
pub struct Response<Client> {
    pub request: Request<Client>,
    pub propagate_result: PropagateEventResult<Client>,
}

impl<Client> Response<Client> {
    #[must_use]
    pub fn new(request: Request<Client>, propagate_result: PropagateEventResult<Client>) -> Self {
        Self {
            request,
            propagate_result,
        }
    }
}

/// Router can route update, and it nested update types like messages, callback query, polls and all other event types.
/// Event handlers can be registered in observer by following methods:
/// - By observer method - [`router.<event_type>.register(handler, <filters, ...>)`
/// - By observer method - [`router.<event_type>.on(handler, <filters, ...>)`
/// - By observer method (if no filters) - [`router.<event_type>.register_no_filters(handler)`
/// - By observer method (if no filters) - [`router.<event_type>.on_no_filters(handler)`
pub struct Router<Client> {
    pub router_name: &'static str,
    pub sub_routers: Vec<Router<Client>>,

    pub message: TelegramObserver<Client>,
    pub edited_message: TelegramObserver<Client>,
    pub channel_post: TelegramObserver<Client>,
    pub edited_channel_post: TelegramObserver<Client>,
    pub inline_query: TelegramObserver<Client>,
    pub chosen_inline_result: TelegramObserver<Client>,
    pub callback_query: TelegramObserver<Client>,
    pub shipping_query: TelegramObserver<Client>,
    pub pre_checkout_query: TelegramObserver<Client>,
    pub poll: TelegramObserver<Client>,
    pub poll_answer: TelegramObserver<Client>,
    pub my_chat_member: TelegramObserver<Client>,
    pub chat_member: TelegramObserver<Client>,
    pub chat_join_request: TelegramObserver<Client>,

    pub startup: SimpleObserver,
    pub shutdown: SimpleObserver,
}

impl<Client> Router<Client>
where
    Client: Send + Sync + 'static,
{
    /// Create a new router
    /// # Arguments
    /// * `router_name` - Router name, can be used for logging
    #[must_use]
    #[rustfmt::skip]
    pub fn new(router_name: &'static str) -> Self {
        Self {
            router_name,
            sub_routers: vec![],
            message: TelegramObserver::new(TelegramObserverName::Message.as_str()),
            edited_message: TelegramObserver::new(TelegramObserverName::EditedMessage.as_str()),
            channel_post: TelegramObserver::new(TelegramObserverName::ChannelPost.as_str()),
            edited_channel_post: TelegramObserver::new(TelegramObserverName::EditedChannelPost.as_str()),
            inline_query: TelegramObserver::new(TelegramObserverName::InlineQuery.as_str()),
            chosen_inline_result: TelegramObserver::new(TelegramObserverName::ChosenInlineResult.as_str()),
            callback_query: TelegramObserver::new(TelegramObserverName::CallbackQuery.as_str()),
            shipping_query: TelegramObserver::new(TelegramObserverName::ShippingQuery.as_str()),
            pre_checkout_query: TelegramObserver::new(TelegramObserverName::PreCheckoutQuery.as_str()),
            poll: TelegramObserver::new(TelegramObserverName::Poll.as_str()),
            poll_answer: TelegramObserver::new(TelegramObserverName::PollAnswer.as_str()),
            my_chat_member: TelegramObserver::new(TelegramObserverName::MyChatMember.as_str()),
            chat_member: TelegramObserver::new(TelegramObserverName::ChatMember.as_str()),
            chat_join_request: TelegramObserver::new(TelegramObserverName::ChatJoinRequest.as_str()),
            startup: SimpleObserver::new(SimpleObserverName::Startup.as_str()),
            shutdown: SimpleObserver::new(SimpleObserverName::Shutdown.as_str()),
        }
    }
}

impl<Client> Router<Client> {
    #[must_use]
    pub fn telegram_observers(&self) -> Vec<&TelegramObserver<Client>> {
        vec![
            &self.message,
            &self.edited_message,
            &self.channel_post,
            &self.edited_channel_post,
            &self.inline_query,
            &self.chosen_inline_result,
            &self.callback_query,
            &self.shipping_query,
            &self.pre_checkout_query,
            &self.poll,
            &self.poll_answer,
            &self.my_chat_member,
            &self.chat_member,
            &self.chat_join_request,
        ]
    }

    #[must_use]
    pub fn event_observers(&self) -> Vec<&SimpleObserver> {
        vec![&self.startup, &self.shutdown]
    }

    /// Register inner middlewares to sub router (and sub routers of sub router)
    fn register_inner_middlewares_in_sub_router(&self, sub_router: &mut Router<Client>) {
        // Register middlewares of current router observers to sub router observers at first positions
        macro_rules! register_middlewares {
            ($observer:ident) => {
                let mut index = 0;
                for middleware in &self.$observer.inner_middlewares.middlewares {
                    sub_router.$observer.inner_middlewares.register_wrapper_at_position(index, Arc::clone(middleware));
                    index += 1;
                }
            };
            ($observer:ident, $($observers:ident),+) => {
                register_middlewares!($observer);
                register_middlewares!($($observers),+);
            };
        }

        // Call register middlewares macro for all telegram event observers
        register_middlewares!(
            message,
            edited_message,
            channel_post,
            edited_channel_post,
            inline_query,
            chosen_inline_result,
            callback_query,
            shipping_query,
            pre_checkout_query,
            poll,
            poll_answer,
            my_chat_member,
            chat_member,
            chat_join_request
        );

        sub_router.sub_routers.iter_mut().for_each(|sub_router| {
            self.register_inner_middlewares_in_sub_router(sub_router);
        });
    }

    /// Include a sub router.
    /// This method will register all middlewares of current router (and parent routers),
    /// which registered before call this method, to sub router (and sub routers of sub router).
    pub fn include_router(&mut self, mut router: Router<Client>) {
        self.register_inner_middlewares_in_sub_router(&mut router);

        self.sub_routers.push(router);
    }

    /// Alias to [`Router::include_router`] method
    pub fn include(&mut self, router: Router<Client>) {
        self.include_router(router);
    }

    /// Resolve registered event names.
    /// Is useful for getting updates only for registered event types.
    /// # Arguments
    /// * `skip_events` - Skip specified event names
    /// # Returns
    /// Registered event names
    #[must_use]
    pub fn resolve_used_update_types(&self, skip_events: &[&str]) -> Vec<&str> {
        let mut used_update_types = HashSet::new();

        self.sub_routers.iter().for_each(|router| {
            used_update_types.extend(router.resolve_used_update_types(skip_events));
        });

        for observer in self.telegram_observers() {
            let event_name = observer.event_name;

            if !observer.handlers.is_empty() && !skip_events.contains(&event_name) {
                used_update_types.insert(event_name);
            }
        }

        used_update_types.into_iter().collect()
    }
}

impl<Client> Debug for Router<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .finish()
    }
}

impl<Client> Default for Router<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self::new("default")
    }
}

impl<Client> AsRef<Router<Client>> for Router<Client> {
    #[must_use]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Client> ToServiceProvider for Router<Client> {
    type Config = ();
    type ServiceProvider = RouterInner<Client>;
    type InitError = ();

    fn to_service_provider(
        self,
        config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        let router_name = self.router_name;
        let sub_routers = self
            .sub_routers
            .into_iter()
            .map(|router| router.to_service_provider(config))
            .collect::<Result<Vec<_>, _>>()?;
        let message = self.message.to_service_provider(config)?;
        let edited_message = self.edited_message.to_service_provider(config)?;
        let channel_post = self.channel_post.to_service_provider(config)?;
        let edited_channel_post = self.edited_channel_post.to_service_provider(config)?;
        let inline_query = self.inline_query.to_service_provider(config)?;
        let chosen_inline_result = self.chosen_inline_result.to_service_provider(config)?;
        let callback_query = self.callback_query.to_service_provider(config)?;
        let shipping_query = self.shipping_query.to_service_provider(config)?;
        let pre_checkout_query = self.pre_checkout_query.to_service_provider(config)?;
        let poll = self.poll.to_service_provider(config)?;
        let poll_answer = self.poll_answer.to_service_provider(config)?;
        let my_chat_member = self.my_chat_member.to_service_provider(config)?;
        let chat_member = self.chat_member.to_service_provider(config)?;
        let chat_join_request = self.chat_join_request.to_service_provider(config)?;
        let startup = self.startup.to_service_provider(config)?;
        let shutdown = self.shutdown.to_service_provider(config)?;

        Ok(RouterInner {
            router_name,
            sub_routers,
            message,
            edited_message,
            channel_post,
            edited_channel_post,
            inline_query,
            chosen_inline_result,
            callback_query,
            shipping_query,
            pre_checkout_query,
            poll,
            poll_answer,
            my_chat_member,
            chat_member,
            chat_join_request,
            startup,
            shutdown,
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct RouterInner<Client> {
    router_name: &'static str,
    sub_routers: Vec<RouterInner<Client>>,

    message: TelegramObserverInner<Client>,
    edited_message: TelegramObserverInner<Client>,
    channel_post: TelegramObserverInner<Client>,
    edited_channel_post: TelegramObserverInner<Client>,
    inline_query: TelegramObserverInner<Client>,
    chosen_inline_result: TelegramObserverInner<Client>,
    callback_query: TelegramObserverInner<Client>,
    shipping_query: TelegramObserverInner<Client>,
    pre_checkout_query: TelegramObserverInner<Client>,
    poll: TelegramObserverInner<Client>,
    poll_answer: TelegramObserverInner<Client>,
    my_chat_member: TelegramObserverInner<Client>,
    chat_member: TelegramObserverInner<Client>,
    chat_join_request: TelegramObserverInner<Client>,

    startup: SimpleObserverInner,
    shutdown: SimpleObserverInner,
}

impl<Client> ServiceProvider for RouterInner<Client> {}

impl<Client> RouterInner<Client>
where
    Client: Send + Sync + Clone + 'static,
{
    /// Propagate event to routers
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    /// # Warning
    /// This function doesn't compare the update type with the request update type.
    /// Assumed that [`UpdateType`] is correct because it is derived from [`Update`].
    /// This behaviour allows you not to get recursively [`UpdateType`] and can be used in tests.
    #[async_recursion]
    #[must_use]
    pub async fn propagate_event(
        &self,
        update_type: &UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, AppErrorKind> {
        let observer = self.telegram_observer_by_update_type(update_type);

        let mut request = request;
        for middleware in &observer.outer_middlewares {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;

            match event_return {
                // Update request because the middleware could have changed it
                EventReturn::Finish => request = updated_request,
                // If middleware returns skip, then we should skip this middleware
                EventReturn::Skip => continue,
                // If middleware returns cancel, then we should cancel propagation
                EventReturn::Cancel => {
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    })
                }
            }
        }

        self.propagate_event_by_observer(observer, update_type, request)
            .await
    }

    /// Propagate event to routers by observer
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    async fn propagate_event_by_observer(
        &self,
        observer: &TelegramObserverInner<Client>,
        update_type: &UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, AppErrorKind> {
        let observer_request = request.clone().into();
        let observer_response = observer.trigger(observer_request).await?;

        match observer_response.propagate_result {
            // Propagate event to sub routers
            PropagateEventResult::Unhandled => {}
            // Return a response if the event handled
            PropagateEventResult::Handled(response) => {
                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                });
            }
            // Return a response if the event rejected
            // Router don't know about rejected event by observer
            PropagateEventResult::Rejected => {
                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                });
            }
        };

        // Propagate event to sub routers' observer
        for router in &self.sub_routers {
            let router_response = router.propagate_event(update_type, request.clone()).await?;
            match router_response.propagate_result {
                // Propagate event to next sub router's observer if the event unhandled by the sub router's observer
                PropagateEventResult::Unhandled => continue,
                PropagateEventResult::Handled(_) | PropagateEventResult::Rejected => {
                    return Ok(router_response)
                }
            };
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request,
            propagate_result: PropagateEventResult::Unhandled,
        })
    }
}

impl<Client> RouterInner<Client> {
    #[must_use]
    pub const fn telegram_observer_by_update_type(
        &self,
        update_type: &UpdateType,
    ) -> &TelegramObserverInner<Client> {
        match update_type {
            UpdateType::Message => &self.message,
            UpdateType::EditedMessage => &self.edited_message,
            UpdateType::ChannelPost => &self.channel_post,
            UpdateType::EditedChannelPost => &self.edited_channel_post,
            UpdateType::InlineQuery => &self.inline_query,
            UpdateType::ChosenInlineResult => &self.chosen_inline_result,
            UpdateType::CallbackQuery => &self.callback_query,
            UpdateType::ShippingQuery => &self.shipping_query,
            UpdateType::PreCheckoutQuery => &self.pre_checkout_query,
            UpdateType::Poll => &self.poll,
            UpdateType::PollAnswer => &self.poll_answer,
            UpdateType::MyChatMember => &self.my_chat_member,
            UpdateType::ChatMember => &self.chat_member,
            UpdateType::ChatJoinRequest => &self.chat_join_request,
        }
    }

    /// Call startup events
    /// # Errors
    /// If any startup observer returns error
    pub async fn emit_startup(&self) -> SimpleHandlerResult {
        log::debug!("{self:?}: Emit startup");

        for startup in
            once(&self.startup).chain(self.sub_routers.iter().map(|router| &router.startup))
        {
            startup.trigger(()).await?;
        }
        Ok(())
    }

    /// Call shutdown events
    /// # Errors
    /// If any shutdown observer returns error
    pub async fn emit_shutdown(&self) -> SimpleHandlerResult {
        log::debug!("{self:?}: Emit shutdown");

        for shutdown in
            once(&self.shutdown).chain(self.sub_routers.iter().map(|router| &router.shutdown))
        {
            shutdown.trigger(()).await?;
        }
        Ok(())
    }
}

impl<Client> Debug for RouterInner<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        dispatcher::{
            event::{telegram::HandlerResult as TelegramHandlerResult, EventReturn},
            middlewares::inner::call_handler,
        },
        filters::Command,
    };

    use tokio;

    #[test]
    fn test_router_include() {
        let mut router = Router::<Reqwest>::new("main");

        let inner_middleware =
            |handler, request, middlewares| call_handler(handler, request, middlewares);
        let outer_middleware = |request| async move { Ok((request, EventReturn::default())) };

        router.message.inner_middlewares.register(inner_middleware);
        router.message.outer_middlewares.register(outer_middleware);

        router.include({
            let mut router = Router::new("sub1");
            router.include(Router::new("sub1.1"));
            router.include(Router::new("sub1.2"));
            router
        });
        router.include({
            let mut router = Router::new("sub2");
            router.include(Router::new("sub2.1"));
            router.include(Router::new("sub2.2"));
            router
        });
        router.include({
            let mut router = Router::new("sub3");
            router.include(Router::new("sub3.1"));
            router.include(Router::new("sub3.2"));
            router
        });

        assert_eq!(router.sub_routers.len(), 3);
        assert_eq!(router.router_name, "main");

        let message_observer_name = UpdateType::Message.as_str();

        router.sub_routers.into_iter().for_each(|router| {
            assert_eq!(router.sub_routers.len(), 2);

            router
                .telegram_observers()
                .into_iter()
                .for_each(|observer| {
                    if observer.event_name == message_observer_name {
                        assert_eq!(observer.inner_middlewares.middlewares.len(), 1);
                    } else {
                        assert_eq!(observer.inner_middlewares.middlewares.len(), 0);
                    }
                    // Router outer middlewares don't clone to children routers
                    assert_eq!(observer.outer_middlewares.middlewares.len(), 0);
                });

            router.sub_routers.into_iter().for_each(|router| {
                assert_eq!(router.sub_routers.len(), 0);

                router
                    .telegram_observers()
                    .into_iter()
                    .for_each(|observer| {
                        if observer.event_name == message_observer_name {
                            assert_eq!(observer.inner_middlewares.middlewares.len(), 1);
                        } else {
                            assert_eq!(observer.inner_middlewares.middlewares.len(), 0);
                        }
                        // Router outer middlewares don't clone to children routers
                        assert_eq!(observer.outer_middlewares.middlewares.len(), 0);
                    });
            });
        });
    }

    #[rustfmt::skip]
    #[test]
    fn test_router_observers_register() {
        async fn telegram_handler() -> TelegramHandlerResult {
            Ok(EventReturn::Finish)
        }

        async fn simple_handler() -> SimpleHandlerResult {
            Ok(())
        }

        let mut router = Router::<Reqwest>::new("main");
        // Telegram event observers
        router.message.register_no_filters(telegram_handler);
        router.edited_message.register_no_filters(telegram_handler);
        router.channel_post.register_no_filters(telegram_handler);
        router.edited_channel_post.register_no_filters(telegram_handler);
        router.inline_query.register_no_filters(telegram_handler);
        router.chosen_inline_result.register_no_filters(telegram_handler);
        router.callback_query.register_no_filters(telegram_handler);
        router.shipping_query.register_no_filters(telegram_handler);
        router.pre_checkout_query.register_no_filters(telegram_handler);
        router.poll.register_no_filters(telegram_handler);
        router.poll_answer.register_no_filters(telegram_handler);
        router.my_chat_member.register_no_filters(telegram_handler);
        router.chat_member.register_no_filters(telegram_handler);
        router.chat_join_request.register_no_filters(telegram_handler);
        // Event observers
        router.startup.register(simple_handler, ());
        router.shutdown.register(simple_handler, ());

        // Check telegram event observers
        router
            .telegram_observers()
            .into_iter()
            .for_each(|observer| {
                assert_eq!(observer.handlers.len(), 1);

                observer.handlers.iter().for_each(|handler| {
                    assert!(handler.filters.is_empty());
                });
            });

        // Check event observers
        router.event_observers().into_iter().for_each(|observer| {
            assert_eq!(observer.handlers.len(), 1);
        });

        let inner_middleware = |handler, request, middlewares| call_handler(handler, request, middlewares);
        let outer_middleware = |request| async move { Ok((request, EventReturn::Finish)) };

        router.message.inner_middlewares.register(inner_middleware);
        router.message.outer_middlewares.register(outer_middleware);

        assert_eq!(router.message.inner_middlewares.middlewares.len(), 1);
        assert_eq!(router.message.outer_middlewares.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_router_propagate_event() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        let mut router = Router::new("test1");
        router
            .message
            .register_no_filters(|| async { Ok(EventReturn::Finish) });

        let router_service = router.to_service_provider(()).unwrap();

        let request = Request::new(bot, update, context);
        let response = router_service
            .propagate_event(&UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Event should be handled, because there is a message handler registered
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let response = router_service
            .propagate_event(&UpdateType::CallbackQuery, request.clone())
            .await
            .unwrap();

        // Event shouldn't be handled, because there is no callback query handler registered
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("test2");
        router.message.filter(Command::default());
        router
            .message
            .register_no_filters(|| async { Ok(EventReturn::Finish) });

        let router_service = router.to_service_provider(()).unwrap();

        let response = router_service
            .propagate_event(&UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Event shouldn't be handled, because there is a message handler registered, but it has filter, which doesn't pass
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("test3");
        router
            .callback_query
            .register_no_filters(|| async { Ok(EventReturn::Skip) });
        router
            .callback_query
            .register_no_filters(|| async { Ok(EventReturn::Finish) });

        let router_service = router.to_service_provider(()).unwrap();

        let response = router_service
            .propagate_event(&UpdateType::CallbackQuery, request)
            .await
            .unwrap();

        // Callback query event should be handled, because there is a callback query handler registered
        // and it returns `EventReturn::Skip`, so next handler should be called
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }
    }
}
