use super::event::{
    service::{Service, ServiceFactory},
    EventHandler, EventObserver, EventObserverService, EventReturn, TelegramHandler,
    TelegramObserver, TelegramObserverService,
};

use crate::{error::app, extract::FromEventAndContext, filters::Filter};

use futures::future::join_all;
use futures_core::future::LocalBoxFuture;
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

const MESSAGE_OBSERVER_NAME: &str = "message";
const EDITED_MESSAGE_OBSERVER_NAME: &str = "edited_message";
const CHANNEL_POST_OBSERVER_NAME: &str = "channel_post";
const EDITED_CHANNEL_POST_OBSERVER_NAME: &str = "edited_channel_post";
const INLINE_QUERY_OBSERVER_NAME: &str = "inline_query";
const CHOSEN_INLINE_RESULT_OBSERVER_NAME: &str = "chosen_inline_result";
const CALLBACK_QUERY_OBSERVER_NAME: &str = "callback_query";
const SHIPPING_QUERY_OBSERVER_NAME: &str = "shipping_query";
const PRE_CHECKOUT_QUERY_OBSERVER_NAME: &str = "pre_checkout_query";
const POLL_OBSERVER_NAME: &str = "poll";
const POLL_ANSWER_OBSERVER_NAME: &str = "poll_answer";
const MY_CHAT_MEMBER_OBSERVER_NAME: &str = "my_chat_member";
const CHAT_MEMBER_OBSERVER_NAME: &str = "chat_member";
const CHAT_JOIN_REQUEST_OBSERVER_NAME: &str = "chat_join_request";

/// Router can route update, and it nested update types like messages, callback query, polls and all other event types.
/// Event handlers can be registered in observer by two ways:
/// - By observer method - [`router.register_<event_type>(handler, <filters, ...>)`
/// - By observer method - [`router.on_<event_type>(handler, <filters, ...>)`
#[derive(Debug)]
pub struct Router {
    /// Router name
    router_name: &'static str,
    /// Sub routers
    sub_routers: Vec<Box<Router>>,

    /// Telegram event observers
    message: TelegramObserver,
    edited_message: TelegramObserver,
    channel_post: TelegramObserver,
    edited_channel_post: TelegramObserver,
    inline_query: TelegramObserver,
    chosen_inline_result: TelegramObserver,
    callback_query: TelegramObserver,
    shipping_query: TelegramObserver,
    pre_checkout_query: TelegramObserver,
    poll: TelegramObserver,
    poll_answer: TelegramObserver,
    my_chat_member: TelegramObserver,
    chat_member: TelegramObserver,
    chat_join_request: TelegramObserver,

    /// Event observers
    startup: EventObserver,
    shutdown: EventObserver,
}

impl Router {
    /// Create a new router
    /// # Arguments
    /// * `router_name` - Router name, can be used for logging
    pub fn new(router_name: &'static str) -> Self {
        Self {
            router_name,
            sub_routers: vec![],
            message: TelegramObserver::new(MESSAGE_OBSERVER_NAME),
            edited_message: TelegramObserver::new(EDITED_MESSAGE_OBSERVER_NAME),
            channel_post: TelegramObserver::new(CHANNEL_POST_OBSERVER_NAME),
            edited_channel_post: TelegramObserver::new(EDITED_CHANNEL_POST_OBSERVER_NAME),
            inline_query: TelegramObserver::new(INLINE_QUERY_OBSERVER_NAME),
            chosen_inline_result: TelegramObserver::new(CHOSEN_INLINE_RESULT_OBSERVER_NAME),
            callback_query: TelegramObserver::new(CALLBACK_QUERY_OBSERVER_NAME),
            shipping_query: TelegramObserver::new(SHIPPING_QUERY_OBSERVER_NAME),
            pre_checkout_query: TelegramObserver::new(PRE_CHECKOUT_QUERY_OBSERVER_NAME),
            poll: TelegramObserver::new(POLL_OBSERVER_NAME),
            poll_answer: TelegramObserver::new(POLL_ANSWER_OBSERVER_NAME),
            my_chat_member: TelegramObserver::new(MY_CHAT_MEMBER_OBSERVER_NAME),
            chat_member: TelegramObserver::new(CHAT_MEMBER_OBSERVER_NAME),
            chat_join_request: TelegramObserver::new(CHAT_JOIN_REQUEST_OBSERVER_NAME),
            startup: EventObserver::new(),
            shutdown: EventObserver::new(),
        }
    }

    /// Get a router name
    pub fn router_name(&self) -> &str {
        self.router_name
    }

    /// Alias to [`Router::router_name`] method
    pub fn name(&self) -> &str {
        self.router_name()
    }

    /// Get sub routers
    pub fn sub_routers(&self) -> Vec<&Router> {
        self.sub_routers.iter().map(|r| r.as_ref()).collect()
    }

    /// Alias to [`Router::sub_routers`] method
    pub fn routers(&self) -> Vec<&Router> {
        self.sub_routers()
    }

    /// Get telegram event observers
    #[rustfmt::skip]
    pub fn telegram_observers(&self) -> HashMap<&str, &TelegramObserver> {
        HashMap::from([
            (MESSAGE_OBSERVER_NAME, &self.message),
            (EDITED_MESSAGE_OBSERVER_NAME, &self.edited_message),
            (CHANNEL_POST_OBSERVER_NAME, &self.channel_post),
            (EDITED_CHANNEL_POST_OBSERVER_NAME, &self.edited_channel_post),
            (INLINE_QUERY_OBSERVER_NAME, &self.inline_query),
            (CHOSEN_INLINE_RESULT_OBSERVER_NAME, &self.chosen_inline_result),
            (CALLBACK_QUERY_OBSERVER_NAME, &self.callback_query),
            (SHIPPING_QUERY_OBSERVER_NAME, &self.shipping_query),
            (PRE_CHECKOUT_QUERY_OBSERVER_NAME, &self.pre_checkout_query),
            (POLL_OBSERVER_NAME, &self.poll),
            (POLL_ANSWER_OBSERVER_NAME, &self.poll_answer),
            (MY_CHAT_MEMBER_OBSERVER_NAME, &self.my_chat_member),
            (CHAT_MEMBER_OBSERVER_NAME, &self.chat_member),
            (CHAT_JOIN_REQUEST_OBSERVER_NAME, &self.chat_join_request),
        ])
    }

    /// Get event observers
    pub fn event_observers(&self) -> Vec<&EventObserver> {
        vec![&self.startup, &self.shutdown]
    }

    /// Include a sub router
    pub fn include_router(mut self, router: Router) -> Self {
        self.sub_routers.push(Box::new(router));
        self
    }

    /// Alias to [`Router::include_router`] method
    pub fn include(mut self, router: Router) -> Self {
        self.include_router(router)
    }

    /// Register event handler to the message observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_message<H, Args>(mut self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.message = self.message.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_message`] method
    pub fn on_message<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_message(handler, filters)
    }

    /// Register event handler to the edited message observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_edited_message<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.edited_message = self.edited_message.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_edited_message`] method
    pub fn on_edited_message<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_edited_message(handler, filters)
    }

    /// Register event handler to the channel post observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_channel_post<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.channel_post = self.channel_post.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_channel_post`] method
    pub fn on_channel_post<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_channel_post(handler, filters)
    }

    /// Register event handler to the edited channel post observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_edited_channel_post<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.edited_channel_post = self.edited_channel_post.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_edited_channel_post`] method
    pub fn on_edited_channel_post<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_edited_channel_post(handler, filters)
    }

    /// Register event handler to the inline query observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_inline_query<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.inline_query = self.inline_query.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_inline_query`] method
    pub fn on_inline_query<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_inline_query(handler, filters)
    }

    /// Register event handler to the chosen inline result observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_chosen_inline_result<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.chosen_inline_result = self.chosen_inline_result.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_chosen_inline_result`] method
    pub fn on_chosen_inline_result<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_chosen_inline_result(handler, filters)
    }

    /// Register event handler to the callback query observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_callback_query<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.callback_query = self.callback_query.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_callback_query`] method
    pub fn on_callback_query<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_callback_query(handler, filters)
    }

    /// Register event handler to the shipping query observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_shipping_query<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.shipping_query = self.shipping_query.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_shipping_query`] method
    pub fn on_shipping_query<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_shipping_query(handler, filters)
    }

    /// Register event handler to the pre checkout query observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_pre_checkout_query<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.pre_checkout_query = self.pre_checkout_query.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_pre_checkout_query`] method
    pub fn on_pre_checkout_query<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_pre_checkout_query(handler, filters)
    }

    /// Register event handler to the poll observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_poll<H, Args>(mut self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.poll = self.poll.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_poll`] method
    pub fn on_poll<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_poll(handler, filters)
    }

    /// Register event handler to the poll answer observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_poll_answer<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.poll_answer = self.poll_answer.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_poll_answer`] method
    pub fn on_poll_answer<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_poll_answer(handler, filters)
    }

    /// Register event handler to the my chat member observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_my_chat_member<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.my_chat_member = self.my_chat_member.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_my_chat_member`] method
    pub fn on_my_chat_member<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_my_chat_member(handler, filters)
    }

    /// Register event handler to the chat member observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_chat_member<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.chat_member = self.chat_member.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_chat_member`] method
    pub fn on_chat_member<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_chat_member(handler, filters)
    }

    /// Register event handler to the chat join request observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register_chat_join_request<H, Args>(
        mut self,
        handler: H,
        filters: Vec<Box<dyn Filter>>,
    ) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.chat_join_request = self.chat_join_request.register(handler, filters);
        self
    }

    /// Alias to [`Router::register_chat_join_request`] method
    pub fn on_chat_join_request<H, Args>(self, handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register_chat_join_request(handler, filters)
    }

    /// Add a handler to the startup observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    pub fn register_startup<H, Args>(mut self, handler: H, args: Args) -> Self
    where
        H: EventHandler<Args> + 'static,
        Args: Clone + 'static,
    {
        self.startup = self.startup.register(handler, args);
        self
    }

    /// Alias to [`Router::register_startup`] method
    pub fn on_startup<H, Args>(self, handler: H, args: Args) -> Self
    where
        H: EventHandler<Args> + 'static,
        Args: Clone + 'static,
    {
        self.register_startup(handler, args)
    }

    /// Add a handler to the shutdown observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    pub fn register_shutdown<H, Args>(mut self, handler: H, args: Args) -> Self
    where
        H: EventHandler<Args> + 'static,
        Args: Clone + 'static,
    {
        self.shutdown = self.shutdown.register(handler, args);
        self
    }

    /// Alias to [`Router::register_shutdown`] method
    pub fn on_shutdown<H, Args>(self, handler: H, args: Args) -> Self
    where
        H: EventHandler<Args> + 'static,
        Args: Clone + 'static,
    {
        self.register_shutdown(handler, args)
    }

    /// Resolve registered event names
    /// Is useful for getting updates only for registered event types.
    /// # Arguments
    /// * `skip_events` - Skip specified event names
    /// # Returns
    /// Registered event names
    pub fn resolve_used_update_types(&self, skip_events: &[&str]) -> Vec<&str> {
        let mut used_update_types = HashSet::new();

        self.sub_routers.iter().for_each(|router| {
            used_update_types.extend(router.resolve_used_update_types(skip_events));
        });

        self.telegram_observers()
            .iter()
            .filter(|(key, observer)| !skip_events.contains(key) && !observer.handlers().is_empty())
            .for_each(|(key, _)| {
                used_update_types.insert(key);
            });

        used_update_types.into_iter().collect()
    }
}

impl ServiceFactory<()> for Router {
    type Response = ();
    type Error = ();
    type Config = ();
    type Service = RouterService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    /// Create [`RouterService`] from [`Router`]
    fn new_service(&self, _: Self::Config) -> Self::Future {
        let router_name = self.router_name;
        let routers = self
            .sub_routers
            .iter()
            .map(|router| router.new_service(()))
            .collect::<Vec<_>>();
        let message = self.message.new_service(());
        let edited_message = self.edited_message.new_service(());
        let channel_post = self.channel_post.new_service(());
        let edited_channel_post = self.edited_channel_post.new_service(());
        let inline_query = self.inline_query.new_service(());
        let chosen_inline_result = self.chosen_inline_result.new_service(());
        let callback_query = self.callback_query.new_service(());
        let shipping_query = self.shipping_query.new_service(());
        let pre_checkout_query = self.pre_checkout_query.new_service(());
        let poll = self.poll.new_service(());
        let poll_answer = self.poll_answer.new_service(());
        let my_chat_member = self.my_chat_member.new_service(());
        let chat_member = self.chat_member.new_service(());
        let chat_join_request = self.chat_join_request.new_service(());
        let startup = self.startup.new_service(());
        let shutdown = self.shutdown.new_service(());

        Box::pin(async move {
            let mut sub_routers = vec![];
            for router in join_all(routers).await {
                sub_routers.push(Box::new(router?));
            }

            let message = message.await?;
            let edited_message = edited_message.await?;
            let channel_post = channel_post.await?;
            let edited_channel_post = edited_channel_post.await?;
            let inline_query = inline_query.await?;
            let chosen_inline_result = chosen_inline_result.await?;
            let callback_query = callback_query.await?;
            let shipping_query = shipping_query.await?;
            let pre_checkout_query = pre_checkout_query.await?;
            let poll = poll.await?;
            let poll_answer = poll_answer.await?;
            let my_chat_member = my_chat_member.await?;
            let chat_member = chat_member.await?;
            let chat_join_request = chat_join_request.await?;
            let startup = startup.await?;
            let shutdown = shutdown.await?;

            Ok(RouterService {
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
        })
    }
}

/// Service for [`Router`]
pub struct RouterService {
    /// Router name
    router_name: &'static str,
    /// Sub router services
    sub_routers: Vec<Box<RouterService>>,

    /// Telegram event observer services
    message: TelegramObserverService,
    edited_message: TelegramObserverService,
    channel_post: TelegramObserverService,
    edited_channel_post: TelegramObserverService,
    inline_query: TelegramObserverService,
    chosen_inline_result: TelegramObserverService,
    callback_query: TelegramObserverService,
    shipping_query: TelegramObserverService,
    pre_checkout_query: TelegramObserverService,
    poll: TelegramObserverService,
    poll_answer: TelegramObserverService,
    my_chat_member: TelegramObserverService,
    chat_member: TelegramObserverService,
    chat_join_request: TelegramObserverService,

    /// Event observer services
    startup: EventObserverService,
    shutdown: EventObserverService,
}

impl RouterService {
    /// Get telegram event observers
    #[rustfmt::skip]
    fn telegram_observers(&self) -> HashMap<&str, &TelegramObserverService> {
        HashMap::from([
            (MESSAGE_OBSERVER_NAME, &self.message),
            (EDITED_MESSAGE_OBSERVER_NAME, &self.edited_message),
            (CHANNEL_POST_OBSERVER_NAME, &self.channel_post),
            (EDITED_CHANNEL_POST_OBSERVER_NAME, &self.edited_channel_post),
            (INLINE_QUERY_OBSERVER_NAME, &self.inline_query),
            (CHOSEN_INLINE_RESULT_OBSERVER_NAME, &self.chosen_inline_result),
            (CALLBACK_QUERY_OBSERVER_NAME, &self.callback_query),
            (SHIPPING_QUERY_OBSERVER_NAME, &self.shipping_query),
            (PRE_CHECKOUT_QUERY_OBSERVER_NAME, &self.pre_checkout_query),
            (POLL_OBSERVER_NAME, &self.poll),
            (POLL_ANSWER_OBSERVER_NAME, &self.poll_answer),
            (MY_CHAT_MEMBER_OBSERVER_NAME, &self.my_chat_member),
            (CHAT_MEMBER_OBSERVER_NAME, &self.chat_member),
            (CHAT_JOIN_REQUEST_OBSERVER_NAME, &self.chat_join_request),
        ])
    }

    /// Call startup callbacks
    pub async fn emit_startup(&self) -> Result<(), app::Error> {
        Self::emit_startup_without_self(
            once(self.startup.clone())
                .chain(self.sub_routers.iter().map(|r| r.startup.clone()))
                .collect(),
        )
        .await
    }

    /// We need this method to possible call without [`EventObserverService`] lifetime
    async fn emit_startup_without_self(
        routers_startup: Vec<EventObserverService>,
    ) -> Result<(), app::Error> {
        for startup in routers_startup {
            startup.call(()).await?;
        }
        Ok(())
    }

    /// Call shutdown callbacks
    pub async fn emit_shutdown(&self) -> Result<(), app::Error> {
        Self::emit_shutdown_without_self(
            once(self.shutdown.clone())
                .chain(self.sub_routers.iter().map(|r| r.shutdown.clone()))
                .collect(),
        )
        .await
    }

    /// We need this method to possible call without [`EventObserverService`] lifetime
    async fn emit_shutdown_without_self(
        routers_shutdown: Vec<EventObserverService>,
    ) -> Result<(), app::Error> {
        for shutdown in routers_shutdown {
            shutdown.call(()).await?;
        }
        Ok(())
    }
}

#[doc(hidden)]
impl Service<()> for RouterService {
    type Response = ();
    type Error = ();
    type Future = LocalBoxFuture<'static, Result<(), Self::Error>>;

    fn call(&self, _: ()) -> Self::Future {
        unimplemented!(
            "RouterService is not intended to be called directly. \
            Use RouterService::emit_startup or RouterService::emit_shutdown instead"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_include() {
        let router = Router::new("main")
            .include(
                Router::new("sub1")
                    .include(Router::new("sub1.1"))
                    .include(Router::new("sub1.2")),
            )
            .include(
                Router::new("sub2")
                    .include(Router::new("sub2.1"))
                    .include(Router::new("sub2.2")),
            )
            .include(
                Router::new("sub3")
                    .include(Router::new("sub3.1"))
                    .include(Router::new("sub3.2")),
            );
        assert_eq!(router.routers().len(), 3);
        assert_eq!(router.name(), "main");

        router.routers().iter().for_each(|r| {
            assert_eq!(r.routers().len(), 2);

            r.routers().iter().for_each(|r| {
                assert_eq!(r.routers().len(), 0);
            });
        });
    }

    #[test]
    fn test_router_observer() {
        let router = Router::new("main")
            .register_message(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_edited_message(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_channel_post(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_edited_channel_post(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_inline_query(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_chosen_inline_result(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_callback_query(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_shipping_query(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_pre_checkout_query(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_poll(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_poll_answer(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_my_chat_member(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_chat_member(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_chat_join_request(
                || async {
                    unimplemented!();
                },
                vec![],
            )
            .register_startup(
                || async {
                    unimplemented!();
                },
                (),
            )
            .register_shutdown(
                || async {
                    unimplemented!();
                },
                (),
            );

        router
            .telegram_observers()
            .iter()
            .for_each(|(_, observer)| {
                assert_eq!(observer.handlers().len(), 1);

                observer.handlers().iter().for_each(|handler| {
                    assert!(handler.filters().is_empty());
                });
            });

        router.event_observers().iter().for_each(|observer| {
            assert_eq!(observer.handlers().len(), 1);
        });
    }
}
