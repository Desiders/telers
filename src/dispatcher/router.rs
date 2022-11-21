use super::event::{
    bases::PropagateEventResult,
    service::{Service, ServiceFactory},
    simple, telegram,
};

use crate::{client::Bot, context::Context, error::app, types::Update};

use async_recursion::async_recursion;
use futures::future::join_all;
use futures_core::future::LocalBoxFuture;
use log;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Formatter},
    iter::once,
    rc::Rc,
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

const STARTUP_OBSERVER_NAME: &str = "startup";
const SHUTDOWN_OBSERVER_NAME: &str = "shutdown";

/// Data for router service
#[derive(Clone)]
pub struct Request {
    bot: Rc<Bot>,
    update: Rc<Update>,
    context: Rc<RefCell<Context>>,
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.bot, &other.bot)
            && Rc::ptr_eq(&self.update, &other.update)
            && Rc::ptr_eq(&self.context, &other.context)
    }
}

impl Request {
    #[must_use]
    pub fn new(bot: Rc<Bot>, update: Rc<Update>, context: Rc<RefCell<Context>>) -> Self {
        Self {
            bot,
            update,
            context,
        }
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
/// - By observer method - [`router.register_<event_type>(handler, <filters, ...>)`
/// - By observer method - [`router.on_<event_type>(handler, <filters, ...>)`
pub struct Router {
    /// Router name
    router_name: &'static str,
    /// Sub routers
    sub_routers: Vec<Router>,

    /// Telegram event observers
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

    /// Event observers
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
            message: telegram::Observer::new(MESSAGE_OBSERVER_NAME),
            edited_message: telegram::Observer::new(EDITED_MESSAGE_OBSERVER_NAME),
            channel_post: telegram::Observer::new(CHANNEL_POST_OBSERVER_NAME),
            edited_channel_post: telegram::Observer::new(EDITED_CHANNEL_POST_OBSERVER_NAME),
            inline_query: telegram::Observer::new(INLINE_QUERY_OBSERVER_NAME),
            chosen_inline_result: telegram::Observer::new(CHOSEN_INLINE_RESULT_OBSERVER_NAME),
            callback_query: telegram::Observer::new(CALLBACK_QUERY_OBSERVER_NAME),
            shipping_query: telegram::Observer::new(SHIPPING_QUERY_OBSERVER_NAME),
            pre_checkout_query: telegram::Observer::new(PRE_CHECKOUT_QUERY_OBSERVER_NAME),
            poll: telegram::Observer::new(POLL_OBSERVER_NAME),
            poll_answer: telegram::Observer::new(POLL_ANSWER_OBSERVER_NAME),
            my_chat_member: telegram::Observer::new(MY_CHAT_MEMBER_OBSERVER_NAME),
            chat_member: telegram::Observer::new(CHAT_MEMBER_OBSERVER_NAME),
            chat_join_request: telegram::Observer::new(CHAT_JOIN_REQUEST_OBSERVER_NAME),
            startup: simple::Observer::new(STARTUP_OBSERVER_NAME),
            shutdown: simple::Observer::new(SHUTDOWN_OBSERVER_NAME),
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
    pub fn telegram_observers(&self) -> HashMap<&str, &telegram::Observer> {
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
    #[must_use]
    pub fn event_observers(&self) -> Vec<&simple::Observer> {
        vec![&self.startup, &self.shutdown]
    }

    /// Include a sub router
    pub fn include_router(&mut self, mut router: Router) {
        /// Register middlewares from sub router's observers to parent router's observers
        macro_rules! register_middlewares {
            ($observer:ident) => {
                self.$observer
                    .middlewares
                    .middlewares()
                    .iter()
                    .for_each(|middleware| {
                        router.$observer
                            .middlewares
                            .register_wrapper(Rc::clone(middleware));
                    });
            };
            ($observer:ident, $($observers:ident),+) => {
                register_middlewares!($observer);
                register_middlewares!($($observers),+);
            };
        }

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

        self.sub_routers.push(router);
    }

    /// Alias to [`Router::include_router`] method
    pub fn include(&mut self, router: Router) {
        self.include_router(router);
    }

    /// Resolve registered event names
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

        self.telegram_observers()
            .iter()
            .filter(|(key, observer)| !skip_events.contains(key) && !observer.handlers().is_empty())
            .for_each(|(key, _)| {
                used_update_types.insert(key);
            });

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
    /// Router name
    router_name: &'static str,
    /// Sub router services
    sub_routers: Vec<RouterService>,

    /// Telegram event observer services
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

    /// Event observer services
    startup: simple::ObserverService,
    shutdown: simple::ObserverService,
}

impl RouterService {
    /// Get telegram event observer by update type
    #[must_use]
    pub fn telegram_observer_by_key(
        &self,
        update_type: &str,
    ) -> Option<&telegram::ObserverService> {
        match update_type {
            "message" => Some(&self.message),
            "edited_message" => Some(&self.edited_message),
            "channel_post" => Some(&self.channel_post),
            "edited_channel_post" => Some(&self.edited_channel_post),
            "inline_query" => Some(&self.inline_query),
            "chosen_inline_result" => Some(&self.chosen_inline_result),
            "callback_query" => Some(&self.callback_query),
            "shipping_query" => Some(&self.shipping_query),
            "pre_checkout_query" => Some(&self.pre_checkout_query),
            "poll" => Some(&self.poll),
            "poll_answer" => Some(&self.poll_answer),
            "my_chat_member" => Some(&self.my_chat_member),
            "chat_member" => Some(&self.chat_member),
            "chat_join_request" => Some(&self.chat_join_request),
            _ => None,
        }
    }

    /// Call startup callbacks
    /// # Errors
    /// If any startup observer returns error
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
    /// If any shutdown observer returns error
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
    /// If any outer middleware returns error
    /// If any inner middleware returns error
    /// If any handler returns error. Probably it's error to extract args to the handler.
    #[async_recursion(?Send)]
    #[allow(clippy::similar_names)]
    #[must_use]
    pub async fn propagate_event(
        &self,
        update_type: &str,
        req: Request,
    ) -> Result<Response, app::Error> {
        let observer = self
            .telegram_observer_by_key(update_type)
            .expect("RouterService::propagate_event: Telegram event observer not found");

        let outer_middlewares = observer.outer_middlewares();
        // If outer middlewares is empty, we can call `RouterService::propagate_event_by_observer` directly
        if outer_middlewares.is_empty() {
            return self
                .propagate_event_by_observer(observer, update_type, req)
                .await;
        }

        for middleware in outer_middlewares {
            let res = middleware.call(&req).await?;
            if res.is_skip() {
                continue;
            }
            if res.is_cancel() {
                return Ok(Response {
                    request: req,
                    response: PropagateEventResult::Rejected,
                });
            }
        }

        self.propagate_event_by_observer(observer, update_type, req)
            .await
    }

    /// Propagate event to routers by observer
    /// # Errors
    /// If any outer middleware returns error
    /// If any inner middleware returns error
    /// If any handler returns error. Probably it's error to extract args to the handler.
    #[allow(clippy::similar_names)]
    async fn propagate_event_by_observer(
        &self,
        observer: &telegram::ObserverService,
        update_type: &str,
        req: Request,
    ) -> Result<Response, app::Error> {
        let observer_req = req.clone().into();
        let observer_res = observer.trigger(observer_req).await?;
        match observer_res.response() {
            // Return a response if the event rejected
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
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

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

    use crate::dispatcher::middlewares::inner::Logging;

    use log::{Log, Metadata, Record};

    struct SimpleLogger;

    impl Log for SimpleLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            unimplemented!();
        }

        fn log(&self, record: &Record) {
            println!("{} - {}", record.level(), record.args());
        }

        fn flush(&self) {
            unimplemented!();
        }
    }

    #[test]
    fn test_router_include() {
        let mut router = Router::new("main");

        router
            .message
            .middlewares
            .register(Box::new(Logging::new(Box::new(SimpleLogger), None)));

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

        router.routers().into_iter().for_each(|router| {
            assert_eq!(router.routers().len(), 2);

            router
                .telegram_observers()
                .into_iter()
                .for_each(|(name, observer)| {
                    if name == MESSAGE_OBSERVER_NAME {
                        assert_eq!(observer.middlewares.middlewares().len(), 1);
                    } else {
                        assert_eq!(observer.middlewares.middlewares().len(), 0);
                    }
                });

            router.routers().into_iter().for_each(|router| {
                assert_eq!(router.routers().len(), 0);
            });
        });
    }

    #[test]
    fn test_router_observers_register() {
        async fn handler() {
            unimplemented!();
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
            .for_each(|(_, observer)| {
                assert_eq!(observer.handlers().len(), 1);

                observer.handlers().iter().for_each(|handler| {
                    assert!(handler.filters().is_empty());
                });
            });

        // Check event observers
        router.event_observers().into_iter().for_each(|observer| {
            assert_eq!(observer.handlers().len(), 1);
        });

        router
            .message
            .middlewares
            .register(Box::new(Logging::new(Box::new(SimpleLogger), None)));
        assert_eq!(router.message.middlewares.middlewares().len(), 1);
    }
}
