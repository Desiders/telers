use super::event::{
    bases::PropagateEventResult,
    service::{BoxFuture, Service, ServiceFactory},
    simple::observer::{Observer as SimpleObserver, ObserverService as SimpleObserverService},
    telegram::observer::{
        Observer as TelegramObserver, ObserverService as TelegramObserverService,
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
    error::app,
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
#[derive(Clone)]
pub struct Request {
    bot: Arc<Bot>,
    update: Arc<Update>,
    context: Arc<Context>,
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

impl Request {
    #[must_use]
    pub fn new<B, U, C>(bot: B, update: U, context: C) -> Self
    where
        B: Into<Arc<Bot>>,
        U: Into<Arc<Update>>,
        C: Into<Arc<Context>>,
    {
        Self {
            bot: bot.into(),
            update: update.into(),
            context: context.into(),
        }
    }

    #[must_use]
    pub fn bot(&self) -> Arc<Bot> {
        Arc::clone(&self.bot)
    }

    #[must_use]
    pub fn update(&self) -> Arc<Update> {
        Arc::clone(&self.update)
    }

    #[must_use]
    pub fn context(&self) -> Arc<Context> {
        Arc::clone(&self.context)
    }
}

impl From<Request> for TelegramObserverRequest {
    fn from(req: Request) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

/// Response from a router service
pub struct Response {
    request: Request,
    response: PropagateEventResult,
}

impl Response {
    #[must_use]
    pub fn new(request: Request, response: PropagateEventResult) -> Self {
        Self { request, response }
    }

    #[must_use]
    pub fn request(&self) -> &Request {
        &self.request
    }

    #[must_use]
    pub fn response(&self) -> &PropagateEventResult {
        &self.response
    }
}

/// Router can route update, and it nested update types like messages, callback query, polls and all other event types.
/// Event handlers can be registered in observer by following methods:
/// - By observer method - [`router.<event_type>.register(handler, <filters, ...>)`
/// - By observer method - [`router.<event_type>.on(handler, <filters, ...>)`
/// - By observer method (if no filters) - [`router.<event_type>.register_no_filters(handler)`
/// - By observer method (if no filters) - [`router.<event_type>.on_no_filters(handler)`
pub struct Router {
    router_name: &'static str,
    sub_routers: Vec<Router>,

    pub message: TelegramObserver,
    pub edited_message: TelegramObserver,
    pub channel_post: TelegramObserver,
    pub edited_channel_post: TelegramObserver,
    pub inline_query: TelegramObserver,
    pub chosen_inline_result: TelegramObserver,
    pub callback_query: TelegramObserver,
    pub shipping_query: TelegramObserver,
    pub pre_checkout_query: TelegramObserver,
    pub poll: TelegramObserver,
    pub poll_answer: TelegramObserver,
    pub my_chat_member: TelegramObserver,
    pub chat_member: TelegramObserver,
    pub chat_join_request: TelegramObserver,

    pub startup: SimpleObserver,
    pub shutdown: SimpleObserver,
}

impl Router {
    /// Create a new router
    /// # Arguments
    /// * `router_name` - Router name, can be used for logging
    #[must_use]
    #[rustfmt::skip]
    pub fn new(router_name: &'static str) -> Self {
        Self {
            router_name,
            sub_routers: vec![],
            message: TelegramObserver::new(TelegramObserverName::Message.into()),
            edited_message: TelegramObserver::new(TelegramObserverName::EditedMessage.into()),
            channel_post: TelegramObserver::new(TelegramObserverName::ChannelPost.into()),
            edited_channel_post: TelegramObserver::new(TelegramObserverName::EditedChannelPost.into()),
            inline_query: TelegramObserver::new(TelegramObserverName::InlineQuery.into()),
            chosen_inline_result: TelegramObserver::new(TelegramObserverName::ChosenInlineResult.into()),
            callback_query: TelegramObserver::new(TelegramObserverName::CallbackQuery.into()),
            shipping_query: TelegramObserver::new(TelegramObserverName::ShippingQuery.into()),
            pre_checkout_query: TelegramObserver::new(TelegramObserverName::PreCheckoutQuery.into()),
            poll: TelegramObserver::new(TelegramObserverName::Poll.into()),
            poll_answer: TelegramObserver::new(TelegramObserverName::PollAnswer.into()),
            my_chat_member: TelegramObserver::new(TelegramObserverName::MyChatMember.into()),
            chat_member: TelegramObserver::new(TelegramObserverName::ChatMember.into()),
            chat_join_request: TelegramObserver::new(TelegramObserverName::ChatJoinRequest.into()),
            startup: SimpleObserver::new(SimpleObserverName::Startup.into()),
            shutdown: SimpleObserver::new(SimpleObserverName::Shutdown.into()),
        }
    }

    #[must_use]
    pub fn router_name(&self) -> &str {
        self.router_name
    }

    /// Alias to [`Router::router_name`] method
    #[must_use]
    pub fn name(&self) -> &str {
        self.router_name()
    }

    #[must_use]
    pub fn sub_routers(&self) -> Vec<&Router> {
        self.sub_routers.iter().collect()
    }

    /// Alias to [`Router::sub_routers`] method
    #[must_use]
    pub fn routers(&self) -> Vec<&Router> {
        self.sub_routers()
    }

    #[must_use]
    pub fn telegram_observers(&self) -> Vec<&TelegramObserver> {
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
    fn register_inner_middlewares_in_sub_router(&self, sub_router: &mut Router) {
        // Register middlewares of current router observers to sub router observers at first positions
        macro_rules! register_middlewares {
            ($observer:ident) => {
                let mut index = 0;
                for middleware in self.$observer.middlewares.middlewares() {
                    sub_router.$observer.middlewares.register_wrapper_at_position(index, Arc::clone(middleware));
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
    pub fn include_router(&mut self, mut router: Router) {
        self.register_inner_middlewares_in_sub_router(&mut router);

        self.sub_routers.push(router);
    }

    /// Alias to [`Router::include_router`] method
    pub fn include(&mut self, router: Router) {
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
            let event_name = observer.event_name();

            if !observer.handlers().is_empty() && !skip_events.contains(&event_name) {
                used_update_types.insert(event_name);
            }
        }

        used_update_types.into_iter().collect()
    }
}

impl Debug for Router {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .finish()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new("default")
    }
}

impl AsRef<Router> for Router {
    #[must_use]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ServiceFactory<Request> for Router {
    type Response = Response;
    type Error = ();
    type Config = ();
    type Service = RouterService;
    type InitError = ();

    fn new_service(&self, _: Self::Config) -> Result<Self::Service, Self::InitError> {
        let router_name = self.router_name;
        let sub_routers = self
            .sub_routers
            .iter()
            .map(|router| router.new_service(()))
            .collect::<Result<Vec<_>, _>>()?;
        let message = self.message.new_service(())?;
        let edited_message = self.edited_message.new_service(())?;
        let channel_post = self.channel_post.new_service(())?;
        let edited_channel_post = self.edited_channel_post.new_service(())?;
        let inline_query = self.inline_query.new_service(())?;
        let chosen_inline_result = self.chosen_inline_result.new_service(())?;
        let callback_query = self.callback_query.new_service(())?;
        let shipping_query = self.shipping_query.new_service(())?;
        let pre_checkout_query = self.pre_checkout_query.new_service(())?;
        let poll = self.poll.new_service(())?;
        let poll_answer = self.poll_answer.new_service(())?;
        let my_chat_member = self.my_chat_member.new_service(())?;
        let chat_member = self.chat_member.new_service(())?;
        let chat_join_request = self.chat_join_request.new_service(())?;
        let startup = self.startup.new_service(())?;
        let shutdown = self.shutdown.new_service(())?;

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
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct RouterService {
    router_name: &'static str,
    sub_routers: Vec<RouterService>,

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

    startup: SimpleObserverService,
    shutdown: SimpleObserverService,
}

impl RouterService {
    #[must_use]
    pub fn telegram_observer_by_update_type(
        &self,
        update_type: &UpdateType,
    ) -> &TelegramObserverService {
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
    /// - If any startup observer returns error
    pub async fn emit_startup(&self) -> Result<(), app::ErrorKind> {
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
    /// - If any shutdown observer returns error
    pub async fn emit_shutdown(&self) -> Result<(), app::ErrorKind> {
        log::debug!("{self:?}: Emit shutdown");

        for shutdown in
            once(&self.shutdown).chain(self.sub_routers.iter().map(|router| &router.shutdown))
        {
            shutdown.trigger(()).await?;
        }
        Ok(())
    }

    /// Propagate event to routers
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    #[async_recursion]
    #[allow(clippy::similar_names)]
    #[must_use]
    pub async fn propagate_event(
        &self,
        update_type: &UpdateType,
        req: Request,
    ) -> Result<Response, app::ErrorKind> {
        let observer = self.telegram_observer_by_update_type(update_type);

        let outer_middlewares = self
            .telegram_observer_by_update_type(update_type)
            .outer_middlewares();

        let mut req = req;
        for middleware in outer_middlewares {
            let (updated_req, res) = middleware.call(req.clone()).await?;

            // If middleware returns skip, then we should skip this middleware
            if res.is_skip() {
                continue;
            }
            // If middleware returns cancel, then we should cancel propagation
            if res.is_cancel() {
                return Ok(Response {
                    request: req,
                    response: PropagateEventResult::Rejected,
                });
            }
            // Update request because the middleware could have changed it
            req = updated_req;
        }

        self.propagate_event_by_observer(observer, update_type, req)
            .await
    }

    /// Propagate event to routers by observer
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    #[allow(clippy::similar_names)]
    async fn propagate_event_by_observer(
        &self,
        observer: &TelegramObserverService,
        update_type: &UpdateType,
        req: Request,
    ) -> Result<Response, app::ErrorKind> {
        let observer_req = req.clone().into();
        let observer_res = observer.trigger(observer_req).await?;

        match observer_res.response() {
            // Return a response if the event rejected
            // Router don't know about rejected event by observer
            PropagateEventResult::Rejected => {
                return Ok(Response {
                    request: req,
                    response: PropagateEventResult::Unhandled,
                })
            }
            // Propagate event to sub routers
            PropagateEventResult::Unhandled => {}
            // Return a response if the event handled
            PropagateEventResult::Handled(res) => {
                return Ok(Response {
                    request: req,
                    response: PropagateEventResult::Handled(res.clone()),
                })
            }
        };

        // Propagate event to sub routers' observer
        for router in &self.sub_routers {
            let res = router.propagate_event(update_type, req.clone()).await?;
            match res.response() {
                // Propagate event to next sub router's observer if the event unhandled by the sub router's observer
                PropagateEventResult::Unhandled => continue,
                // Return a response if the event isn't unhandled
                _ => res,
            };
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request: req,
            response: PropagateEventResult::Unhandled,
        })
    }
}

impl Debug for RouterService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .finish()
    }
}

impl Service<Request> for RouterService {
    type Response = Response;
    type Error = ();
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn call(&self, _: Request) -> Self::Future {
        log::error!("{self:?}: Should not be called");

        unimplemented!(
            "RouterService is not intended to be called directly. \
            Use RouterService::emit_startup, RouterService::emit_shutdown \
            or RouterSevice::propagate_event instead"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dispatcher::{
            event::{
                bases::{Action, EventReturn},
                telegram::handler::BoxedHandlerService,
            },
            router::Request as RouterRequest,
        },
        filters::command,
    };

    use tokio;

    #[test]
    fn test_router_include() {
        let mut router = Router::new("main");

        let middleware =
            |handler: Arc<BoxedHandlerService>, req: _, _| async move { handler.call(req).await };
        let outer_middleware = |req: _| async move { Ok((req, EventReturn::default())) };

        router.message.middlewares.register(middleware);
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

        assert_eq!(router.routers().len(), 3);
        assert_eq!(router.name(), "main");

        let message_observer_name = UpdateType::Message.as_str();

        router.routers().into_iter().for_each(|router| {
            assert_eq!(router.routers().len(), 2);

            router
                .telegram_observers()
                .into_iter()
                .for_each(|observer| {
                    if observer.event_name() == message_observer_name {
                        assert_eq!(observer.middlewares.middlewares().len(), 1);
                    } else {
                        assert_eq!(observer.middlewares.middlewares().len(), 0);
                    }
                    // Router outer middlewares don't clone to children routers
                    assert_eq!(observer.outer_middlewares.middlewares().len(), 0);
                });

            router.routers().into_iter().for_each(|router| {
                assert_eq!(router.routers().len(), 0);

                router
                    .telegram_observers()
                    .into_iter()
                    .for_each(|observer| {
                        if observer.event_name() == message_observer_name {
                            assert_eq!(observer.middlewares.middlewares().len(), 1);
                        } else {
                            assert_eq!(observer.middlewares.middlewares().len(), 0);
                        }
                        // Router outer middlewares don't clone to children routers
                        assert_eq!(observer.outer_middlewares.middlewares().len(), 0);
                    });
            });
        });
    }

    #[test]
    fn test_router_observers_register() {
        async fn handler() {
            unreachable!();
        }

        let mut router = Router::new("main");
        // Telegram event observers
        router.message.register_no_filters(handler);
        router.edited_message.register_no_filters(handler);
        router.channel_post.register_no_filters(handler);
        router.edited_channel_post.register_no_filters(handler);
        router.inline_query.register_no_filters(handler);
        router.chosen_inline_result.register_no_filters(handler);
        router.callback_query.register_no_filters(handler);
        router.shipping_query.register_no_filters(handler);
        router.pre_checkout_query.register_no_filters(handler);
        router.poll.register_no_filters(handler);
        router.poll_answer.register_no_filters(handler);
        router.my_chat_member.register_no_filters(handler);
        router.chat_member.register_no_filters(handler);
        router.chat_join_request.register_no_filters(handler);
        // Event observers
        router.startup.register(handler, ());
        router.shutdown.register(handler, ());

        // Check telegram event observers
        router
            .telegram_observers()
            .into_iter()
            .for_each(|observer| {
                assert_eq!(observer.handlers().len(), 1);

                observer.handlers().iter().for_each(|handler| {
                    assert!(handler.filters().is_empty());
                });
            });

        // Check event observers
        router.event_observers().into_iter().for_each(|observer| {
            assert_eq!(observer.handlers().len(), 1);
        });

        let middleware =
            |handler: Arc<BoxedHandlerService>, req: _, _| async move { handler.call(req).await };
        let outer_middleware = |req: _| async move { Ok((req, EventReturn::default())) };

        router.message.middlewares.register(middleware);
        router.message.outer_middlewares.register(outer_middleware);

        assert_eq!(router.message.middlewares.middlewares().len(), 1);
        assert_eq!(router.message.outer_middlewares.middlewares().len(), 1);
    }

    #[tokio::test]
    async fn test_router_propagate_event() {
        let bot = Bot::default();
        let context = Context::new();
        let update = Update::default();

        let mut router = Router::new("main");
        router.message.register_no_filters(|| async {});

        let router_service = router.new_service(()).unwrap();

        let req = RouterRequest::new(bot, update, context);

        let res = router_service
            .propagate_event(&UpdateType::Message, req.clone())
            .await
            .unwrap();

        // Event should be handled, because there is a message handler registered
        match res.response() {
            PropagateEventResult::Handled(handler_res) => {
                assert_eq!(*handler_res.response(), EventReturn::default());
            }
            _ => panic!("Unexpected result"),
        }

        let res = router_service
            .propagate_event(&UpdateType::CallbackQuery, req.clone())
            .await
            .unwrap();

        // Event shouldn't be handled, because there is no callback query handler registered
        match res.response() {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let filter = command::Command {
            commands: vec![command::PatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        };

        let mut router = Router::new("main");
        router.message.filter(filter.clone());
        router.message.register_no_filters(|| async {});

        let router_service = router.new_service(()).unwrap();

        let res = router_service
            .propagate_event(&UpdateType::Message, req.clone())
            .await
            .unwrap();

        // Message event observer filter not pass, so router should be unhandled
        match res.response() {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        router
            .callback_query
            .register_no_filters(|| async { Action::Cancel });
        router
            .callback_query
            .register_no_filters(|| async { unreachable!() });

        let res = router_service
            .propagate_event(&UpdateType::CallbackQuery, req)
            .await
            .unwrap();

        // Handler returns `Action::Cancel`,
        // so response from callback query event observer should be `PropagateEventResult::Rejected`
        // and router unhandled
        match res.response() {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }
    }
}
