use super::event::{
    bases::PropagateEventResult,
    service::{BoxFuture, Service, ServiceFactory},
    simple, telegram,
};

use crate::{
    client::Bot,
    context::Context,
    enums::{observer_name, update_type::UpdateType},
    error::app,
    types::Update,
};

use async_recursion::async_recursion;
use futures::future::join_all;
use log;
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    iter::once,
    sync::{Arc, RwLock},
};

/// Data for router service
#[derive(Clone)]
pub struct Request {
    bot: Arc<Bot>,
    update: Arc<Update>,
    context: Arc<RwLock<Context>>,
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
    pub fn new<B: Into<Arc<Bot>>, U: Into<Arc<Update>>, C: Into<Arc<RwLock<Context>>>>(
        bot: B,
        update: U,
        context: C,
    ) -> Self {
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
    pub fn context(&self) -> Arc<RwLock<Context>> {
        Arc::clone(&self.context)
    }
}

impl From<Request> for telegram::ObserverRequest {
    fn from(req: Request) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

/// Response from router service
pub struct Response {
    request: Request,
    response: PropagateEventResult,
}

impl Response {
    #[must_use]
    pub fn new(request: Request, response: PropagateEventResult) -> Self {
        Self { request, response }
    }

    /// Get request
    #[must_use]
    pub fn request(&self) -> &Request {
        &self.request
    }

    /// Get response
    #[must_use]
    pub fn response(&self) -> &PropagateEventResult {
        &self.response
    }
}

/// Router can route update, and it nested update types like messages, callback query, polls and all other event types.
/// Event handlers can be registered in observer by two ways:
/// - By observer method - [`router.<event_type>.register(handler, <filters, ...>)`
/// - By observer method - [`router.<event_type>.on(handler, <filters, ...>)`
pub struct Router {
    router_name: &'static str,
    sub_routers: Vec<Router>,

    pub message: telegram::Observer,
    pub edited_message: telegram::Observer,
    pub channel_post: telegram::Observer,
    pub edited_channel_post: telegram::Observer,
    pub inline_query: telegram::Observer,
    pub chosen_inline_result: telegram::Observer,
    pub callback_query: telegram::Observer,
    pub shipping_query: telegram::Observer,
    pub pre_checkout_query: telegram::Observer,
    pub poll: telegram::Observer,
    pub poll_answer: telegram::Observer,
    pub my_chat_member: telegram::Observer,
    pub chat_member: telegram::Observer,
    pub chat_join_request: telegram::Observer,

    pub startup: simple::Observer,
    pub shutdown: simple::Observer,
}

impl Router {
    /// Create a new router
    /// # Arguments
    /// * `router_name` - Router name, can be used for logging
    #[must_use]
    pub fn new(router_name: &'static str) -> Self {
        Self {
            router_name,
            sub_routers: vec![],
            message: telegram::Observer::new(observer_name::Telegram::Message.into()),
            edited_message: telegram::Observer::new(observer_name::Telegram::EditedMessage.into()),
            channel_post: telegram::Observer::new(observer_name::Telegram::ChannelPost.into()),
            edited_channel_post: telegram::Observer::new(
                observer_name::Telegram::EditedChannelPost.into(),
            ),
            inline_query: telegram::Observer::new(observer_name::Telegram::InlineQuery.into()),
            chosen_inline_result: telegram::Observer::new(
                observer_name::Telegram::ChosenInlineResult.into(),
            ),
            callback_query: telegram::Observer::new(observer_name::Telegram::CallbackQuery.into()),
            shipping_query: telegram::Observer::new(observer_name::Telegram::ShippingQuery.into()),
            pre_checkout_query: telegram::Observer::new(
                observer_name::Telegram::PreCheckoutQuery.into(),
            ),
            poll: telegram::Observer::new(observer_name::Telegram::Poll.into()),
            poll_answer: telegram::Observer::new(observer_name::Telegram::PollAnswer.into()),
            my_chat_member: telegram::Observer::new(observer_name::Telegram::MyChatMember.into()),
            chat_member: telegram::Observer::new(observer_name::Telegram::ChatMember.into()),
            chat_join_request: telegram::Observer::new(
                observer_name::Telegram::ChatJoinRequest.into(),
            ),
            startup: simple::Observer::new(observer_name::Simple::Startup.into()),
            shutdown: simple::Observer::new(observer_name::Simple::Shutdown.into()),
        }
    }

    /// Get a router name
    #[must_use]
    pub fn router_name(&self) -> &str {
        self.router_name
    }

    /// Alias to [`Router::router_name`] method
    #[must_use]
    pub fn name(&self) -> &str {
        self.router_name()
    }

    /// Get sub routers
    #[must_use]
    pub fn sub_routers(&self) -> Vec<&Router> {
        self.sub_routers.iter().collect()
    }

    /// Alias to [`Router::sub_routers`] method
    #[must_use]
    pub fn routers(&self) -> Vec<&Router> {
        self.sub_routers()
    }

    /// Get telegram event observers
    #[must_use]
    #[rustfmt::skip]
    pub fn telegram_observers(&self) -> Vec<&telegram::Observer> {
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

    /// Get event observers
    #[must_use]
    pub fn event_observers(&self) -> Vec<&simple::Observer> {
        vec![&self.startup, &self.shutdown]
    }

    /// Register inner middlewares to sub router (and sub routers of sub router)
    fn register_inner_middlewares_in_sub_router(&self, sub_router: &mut Router) {
        // Register middlewares of current router observers to sub router observers
        macro_rules! register_middlewares {
            ($observer:ident) => {
                let mut index = 0;
                for middleware in self.$observer.middlewares.middlewares() {
                    sub_router.$observer.middlewares.register_wrapper_in_position(index, Arc::clone(middleware));
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

impl ServiceFactory<Request> for Router {
    type Response = Response;
    type Error = ();
    type Config = ();
    type Service = RouterService;
    type InitError = ();
    type Future = BoxFuture<Result<Self::Service, Self::InitError>>;

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
                sub_routers.push(router?);
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
#[allow(clippy::module_name_repetitions)]
pub struct RouterService {
    router_name: &'static str,
    sub_routers: Vec<RouterService>,

    message: telegram::ObserverService,
    edited_message: telegram::ObserverService,
    channel_post: telegram::ObserverService,
    edited_channel_post: telegram::ObserverService,
    inline_query: telegram::ObserverService,
    chosen_inline_result: telegram::ObserverService,
    callback_query: telegram::ObserverService,
    shipping_query: telegram::ObserverService,
    pre_checkout_query: telegram::ObserverService,
    poll: telegram::ObserverService,
    poll_answer: telegram::ObserverService,
    my_chat_member: telegram::ObserverService,
    chat_member: telegram::ObserverService,
    chat_join_request: telegram::ObserverService,

    startup: simple::ObserverService,
    shutdown: simple::ObserverService,
}

impl RouterService {
    /// Get event observer by update type
    #[must_use]
    pub fn telegram_observer_by_update_type(
        &self,
        update_type: &UpdateType,
    ) -> &telegram::ObserverService {
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

    /// Call startup callbacks
    /// # Errors
    /// - If any startup observer returns error
    pub async fn emit_startup(&self) -> Result<(), app::Error> {
        log::debug!("{:?}: Emit startup", self);

        for startup in
            once(&self.startup).chain(self.sub_routers.iter().map(|router| &router.startup))
        {
            startup.call(()).await?;
        }
        Ok(())
    }

    /// Call shutdown callbacks
    /// # Errors
    /// - If any shutdown observer returns error
    pub async fn emit_shutdown(&self) -> Result<(), app::Error> {
        log::debug!("{:?}: Emit shutdown", self);

        for shutdown in
            once(&self.shutdown).chain(self.sub_routers.iter().map(|router| &router.shutdown))
        {
            shutdown.call(()).await?;
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
    ) -> Result<Response, app::Error> {
        let observer = self.telegram_observer_by_update_type(update_type);

        let outer_middlewares = observer.outer_middlewares();
        // If outer middlewares is empty, we can call `RouterService::propagate_event_by_observer` directly
        if outer_middlewares.is_empty() {
            return self
                .propagate_event_by_observer(observer, update_type, req)
                .await;
        }

        let mut req = req;
        for middleware in outer_middlewares {
            let (updated_req, res) = middleware.call(req.clone()).await?;
            if res.is_skip() {
                continue;
            }
            if res.is_cancel() {
                return Ok(Response {
                    request: req,
                    response: PropagateEventResult::Rejected,
                });
            }
            // Update current request, because middleware can change it
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
        observer: &telegram::ObserverService,
        update_type: &UpdateType,
        req: Request,
    ) -> Result<Response, app::Error> {
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
        log::error!("{:?}: Should not be called", self);

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
                telegram::BoxedHandlerService,
            },
            RouterRequest,
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

        router.message.middlewares.register(Box::new(middleware));
        router
            .message
            .outer_middlewares
            .register(Box::new(outer_middleware));

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

        let message_observer_name: &str = UpdateType::Message.into();

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
        router.message.register(handler, vec![]);
        router.edited_message.register(handler, vec![]);
        router.channel_post.register(handler, vec![]);
        router.edited_channel_post.register(handler, vec![]);
        router.inline_query.register(handler, vec![]);
        router.chosen_inline_result.register(handler, vec![]);
        router.callback_query.register(handler, vec![]);
        router.shipping_query.register(handler, vec![]);
        router.pre_checkout_query.register(handler, vec![]);
        router.poll.register(handler, vec![]);
        router.poll_answer.register(handler, vec![]);
        router.my_chat_member.register(handler, vec![]);
        router.chat_member.register(handler, vec![]);
        router.chat_join_request.register(handler, vec![]);
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

        router.message.middlewares.register(Box::new(middleware));
        router
            .message
            .outer_middlewares
            .register(Box::new(outer_middleware));

        assert_eq!(router.message.middlewares.middlewares().len(), 1);
        assert_eq!(router.message.outer_middlewares.middlewares().len(), 1);
    }

    #[tokio::test]
    async fn test_router_propagate_event() {
        let bot = Bot::default();
        let context = RwLock::new(Context::new());
        let update = Update::default();

        let mut router = Router::new("main");
        router.message.register(|| async {}, vec![]);

        let router_service = router.new_service(()).await.unwrap();

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

        let filter = Box::new(command::Command {
            commands: vec![command::PatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        });

        let mut router = Router::new("main");
        router.message.filter(filter.clone());
        router.message.register(|| async {}, vec![]);

        let router_service = router.new_service(()).await.unwrap();

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
            .register(|| async { Action::Cancel }, vec![]);
        router
            .callback_query
            .register(|| async { unreachable!() }, vec![]);

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
