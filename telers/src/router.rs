//! Router combines all event observers.
//!
//! Each event observer is a special unit that handles a specific event type.
//! There are two types of event observers:
//!
//! * Simple observer:
//!   [`Simple observer`] is used to handle simple events like startup and shutdown.
//!   When you register a handler in this observer,
//!   you specify the arguments that pass to handler when the event is trigger.
//!   Return type of handler is [`Result<(), HandlerError>`].
//!   When observer is trigger, it calls all handlers in order of registration and stops if one of them returns an error.
//!
//! Registration of handlers looks like this:
//! ```rust
//! use telers::{Router, event::simple::{HandlerResult, Handler}}
//!
//! async fn on_startup(message: &str) -> HandlerResult {
//!     todo!()
//! }
//!
//! async fn on_shutdown(message: &str) -> HandlerResult {
//!     todo!()
//! }
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     let router = Router::new("example")
//!         .on_startup(|observer| observer.register(Handler::new(on_startup, ("Hello, world!",))))
//!         .on_shutdown(|observer| observer.register(Handler::new(on_shutdown, ("Goodbye, world!",))));
//! }
//! ```
//!
//! * Telegram observer:
//!   [`Telegram observer`] is used to handle telegram events like messages, callback queries, polls and all other event types.
//!   You can register a handler with any arguments that implement [`Extractor`] trait, see [`extractors module`] for more details.
//!   Return type of handler is [`Result<EventReturn, HandlerError>`],
//!   where [`EventReturn`] is a special enum that can be used to control the propagation of the event,
//!   see [`EventReturn`] for more details.
//!   When observer is trigger, it calls outer middlewares and checks all handlers in order of registration.
//!   It calls all filters for each handler and skips handler if one of them returns `false`.
//!   If handler is pass the filters, observer calls inner middlewares and handler itself (in the middleware).
//!   By default, the first handler that pass the filters stop the propagation of the event, so other handlers aren't calls,
//!   but you can change this behaviour by specify another variant of [`EventReturn`]).
//!
//! Registration of handlers looks like this:
//! ```rust
//! use telers::{Router, event::telegram::{HandlerResult, Handler}}
//!
//! async fn on_message(message: Message) -> HandlerResult {
//!    todo!()
//! }
//!
//! async fn on_callback_query(callback_query: CallbackQuery) -> HandlerResult {
//!   todo!()
//! }
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     let router = Router::new("example")
//!         .on_message(|observer| observer.register(Handler::new(on_message)))
//!         .on_callback_query(|observer| observer.register(Handler::new(on_callback_query)));
//! }
//! ```
//!
//! Routers can be nested, so you can create a tree of routers using [`Router::include_router`] method.
//! You can use [`Router::include_router`] method to include a router to the current router as sub router.
//! Inner middlewares of the parent router will be registered to the sub router and its sub routers in the order of registration.
//! Parent middlewares registers on the top of the stack, so parent middlewares calls before.
//!
//! [`OuterMiddlewaresConfig`] and [`InnerMiddlewaresConfig`] are used to configure outer and inner middlewares, respectively,
//! or just use [`OuterMiddlewaresConfigBuilder`] and [`InnerMiddlewaresConfigBuilder`] to create a config step by step.
//! You can use [`OuterMiddlewaresConfig::default`] and [`InnerMiddlewaresConfig::default`] to create a default config
//! with [`LoggingMiddleware`] to log all incoming updates and [`UserContextMiddleware`] to set up user context.
//! All config middlewares are registered in the order of registration and before other middlewares.
//!
//! You can propagate event with calls [`PropagateEvent::propagate_event`] or [`PropagateEvent::propagate_update_event`],
//! [`PropagateEvent::emit_startup`], [`PropagateEvent::emit_shutdown`] methods in [`Router`],
//! but it's better to use [`Dispatcher`] that does it for you.
//!
//! How does routing work? At the moment, there is such a sequence of actions:
//! > We have a sequence of routers that we call in the order they are registered.
//! > For each router, we first call the router's outer middleware,
//! > after which we check the handlers of the current router depending on the type of event (`Message`, `CallbackQuery`, etc.), and its filters.
//! > We call all filters of each handler until all filters of any handler return `true`.
//! > When a handler is selected, we call a sequence of the router's inner middlewares, with the handler at the end of the chain.
//! > At the moment when the handler is completed, we finish processing the event.
//! > If there are no handlers to execute (both due to their absence and due to a filter failure), we repeat the sequence of actions with the next router in the chain.
//! > In addition, we can influence the processing of events during code execution by [`EventReturn`].
//! > In outer middlewares, we can stop event propagation by returns [`EventReturn::Cancel`],
//! > save current [`Request`] changes made in the middleware by [`EventReturn::Finish`] or skip them by [`EventReturn::Skip`].
//! > In inner middlewares and handlers, we can stop event propagation for the current router and go to next router by returns [`EventReturn::Cancel`],
//! > finish event propagation by [`EventReturn::Finish`] or skip current handler and go to next handler (and its filters) by [`EventReturn::Skip`].
//! * The above also applies to the special update observer with some differences:
//! 1. Middlewares and handlers are called before other middlewares and handlers for the current event observer,
//!    so processing units in update observer have priority in processing.
//! 2. [`EventReturn::Cancel`] for update observer's innter middlrewares and handler don't stop event propagation for the current router,
//!    it doesn't affect the processing of the event in any way.
//!
//! [`Simple observer`]: SimpleObserver
//! [`Telegram observer`]: TelegramObserver
//! [`Dispatcher`]: telers::dispatcher::Dispatcher
//! [`Extractor`]: telers::Extractor
//! [`extractors module`]: telers::extractor
//! [`Router::include_router`]: Router#method.include_router

use crate::{
    client::Reqwest,
    enums::{
        telegram_observer_type::with_telegram_observer_variants, TelegramObserverType, UpdateType,
    },
    errors::EventErrorKind,
    event::{
        bases::{EventReturn, PropagateEventResult},
        service::Service as _,
        simple::{HandlerResult as SimpleHandlerResult, Observer as SimpleObserver},
        telegram::Observer as TelegramObserver,
    },
    middlewares::{
        inner::{
            boxed_middleware_factory as boxed_inner_middleware_factory,
            BoxedCloneMiddlewareService as BoxedCloneInnerMiddlewareService,
            Logging as LoggingMiddleware,
        },
        outer::{
            boxed_middleware_factory as boxed_outer_middleware_factory,
            BoxedCloneMiddlewareService as BoxedCloneOuterMiddlewareService,
            UserContext as UserContextMiddleware,
        },
        InnerMiddleware, OuterMiddleware,
    },
    Request,
};

use paste::paste;
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    future::Future,
    pin::Pin,
};
use tracing::{event, instrument, Level};

pub struct Response<Client> {
    pub request: Request<Client>,
    pub propagate_result: PropagateEventResult<Client>,
}

impl<Client> Response<Client> {
    #[inline]
    #[must_use]
    pub fn new(request: Request<Client>, propagate_result: PropagateEventResult<Client>) -> Self {
        Self {
            request,
            propagate_result,
        }
    }
}

impl<Client> fmt::Debug for Response<Client> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("request", &self.request)
            .field("propagate_result", &self.propagate_result)
            .finish()
    }
}

pub trait PropagateEvent<Client>: Clone + Send + Sync + 'static {
    /// Propagate event
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to handler
    fn propagate_event(
        &mut self,
        update_type: UpdateType,
        request: Request<Client>,
    ) -> impl Future<Output = Result<Response<Client>, EventErrorKind>> + Send
    where
        Client: Send + Sync + Clone;

    /// Propagate update event
    /// # Notes
    /// This calls the special event observer that used to handle all telegram events.
    /// It's called for router and its sub routers and before other telegram observers.
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to handler
    fn propagate_update_event(
        &mut self,
        request: Request<Client>,
    ) -> impl Future<Output = Result<Response<Client>, EventErrorKind>> + Send
    where
        Client: Send + Sync + Clone;

    /// Emit startup events
    /// # Errors
    /// If any startup observer returns error
    fn emit_startup(&mut self) -> impl Future<Output = SimpleHandlerResult> + Send;

    fn startup_handlers_len(&self) -> usize;

    /// Emit shutdown events
    /// # Errors
    /// If any shutdown observer returns error
    fn emit_shutdown(&mut self) -> impl Future<Output = SimpleHandlerResult> + Send;

    fn shutdown_handlers_len(&self) -> usize;
}

macro_rules! define_router_struct {
    ($(($variant:ident, $observer:ident)),+ $(,)?) => {
        /// Router combines all event observers.
        ///
        /// Each event observer is a special unit that handles a specific event type.
        /// There are two types of event observers:
        ///
        /// * Simple observer:
        ///   [`Simple observer`] is used to handle simple events like startup and shutdown.
        ///   When you register a handler in this observer,
        ///   you specify the arguments that pass to handler when the event is trigger.
        ///   Return type of handler is [`Result<(), HandlerError>`].
        ///   When observer is trigger, it calls all handlers in order of registration and stops if one of them returns an error.
        ///
        /// Registration of handlers looks like this:
        /// ```rust
        /// use telers::{Router, event::simple::{HandlerResult, Handler}}
        ///
        /// async fn on_startup(message: &str) -> HandlerResult {
        ///     todo!()
        /// }
        ///
        /// async fn on_shutdown(message: &str) -> HandlerResult {
        ///     todo!()
        /// }
        ///
        /// #[tokio::main(flavor = "current_thread")]
        /// async fn main() {
        ///     let router = Router::new("example")
        ///         .on_startup(|observer| observer.register(Handler::new(on_startup, ("Hello, world!",))))
        ///         .on_shutdown(|observer| observer.register(Handler::new(on_shutdown, ("Goodbye, world!",))));
        /// }
        /// ```
        ///
        /// * Telegram observer:
        ///   [`Telegram observer`] is used to handle telegram events like messages, callback queries, polls and all other event types.
        ///   You can register a handler with any arguments that implement `Extractor` trait, see [`extractors module`] for more details.
        ///   Return type of handler is [`Result<EventReturn, HandlerError>`],
        ///   where [`EventReturn`] is a special enum that can be used to control the propagation of the event,
        ///   see [`EventReturn`] for more details.
        ///   When observer is trigger, it calls outer middlewares and checks all handlers in order of registration.
        ///   It calls all filters for each handler and skips handler if one of them returns `false`.
        ///   If handler is pass the filters, observer calls inner middlewares and handler itself (in the middleware).
        ///   By default, the first handler that pass the filters stop the propagation of the event, so other handlers aren't calls,
        ///   but you can change this behaviour by specify another variant of [`EventReturn`]).
        ///
        /// Registration of handlers looks like this:
        /// ```rust
        /// use telers::{Router, event::telegram::{HandlerResult, Handler}}
        ///
        /// async fn on_message(message: Message) -> HandlerResult {
        ///    todo!()
        /// }
        ///
        /// async fn on_callback_query(callback_query: CallbackQuery) -> HandlerResult {
        ///   todo!()
        /// }
        ///
        /// #[tokio::main(flavor = "current_thread")]
        /// async fn main() {
        ///     let router = Router::new("example")
        ///         .on_message(|observer| observer.register(Handler::new(on_message)))
        ///         .on_callback_query(|observer| observer.register(Handler::new(on_callback_query)));
        /// }
        /// ```
        pub struct Router<Client = Reqwest> {
            name: &'static str,
            sub_routers: Vec<Router<Client>>,

            $(
                $observer: TelegramObserver<Client>,
            )+

            startup: SimpleObserver,
            shutdown: SimpleObserver,
        }
    };
}
with_telegram_observer_variants!(define_router_struct);

macro_rules! router_constructor {
    ($name:expr, $(($variant:ident, $observer:ident)),+ $(,)?) => {
        Self {
            name: $name,
            sub_routers: vec![],
            $(
                $observer: TelegramObserver::new(stringify!($observer)),
            )+
            startup: SimpleObserver::new("startup"),
            shutdown: SimpleObserver::new("shutdown"),
        }
    };
}

macro_rules! impl_router_on_methods {
    ($(($variant:ident, $observer:ident)),+ $(,)?) => {
        $(
            paste! {
                #[doc = concat!("Configure `", stringify!($observer), "` observer in builder style")]
                #[must_use]
                pub fn [<on_ $observer>]<F>(mut self, configure: F) -> Self
                where
                    F: FnOnce(TelegramObserver<Client>) -> TelegramObserver<Client>,
                {
                    self.$observer = configure(self.$observer);
                    self
                }
            }
        )+

        /// Apply the same observer configurator for every Telegram observer (including `update`).
        #[must_use]
        pub fn on_all<F>(mut self, mut configure: F) -> Self
        where
            F: FnMut(TelegramObserver<Client>) -> TelegramObserver<Client>,
        {
            $(
                self.$observer = configure(self.$observer);
            )+
            self
        }

        /// Configure startup observer in builder style.
        #[must_use]
        pub fn on_startup<F>(mut self, configure: F) -> Self
        where
            F: FnOnce(SimpleObserver) -> SimpleObserver,
        {
            self.startup = configure(self.startup);
            self
        }

        /// Configure shutdown observer in builder style.
        #[must_use]
        pub fn on_shutdown<F>(mut self, configure: F) -> Self
        where
            F: FnOnce(SimpleObserver) -> SimpleObserver,
        {
            self.shutdown = configure(self.shutdown);
            self
        }
    };
}

macro_rules! observer_refs_array {
    (ref, $ty:ident, $(($variant:ident, $observer:ident)),+ $(,)?) => {
        [
            $(
                &$ty.$observer,
            )+
        ]
    };
    (mut, $ty:ident, $(($variant:ident, $observer:ident)),+ $(,)?) => {
        [
            $(
                &mut $ty.$observer,
            )+
        ]
    };
}

impl<Client> Router<Client>
where
    Client: Send + Sync + 'static,
{
    with_telegram_observer_variants!(impl_router_on_methods);

    /// # Arguments
    /// * `name` - Name of the router. It can be used for logging and debugging and code clarity.
    #[must_use]
    #[rustfmt::skip]
    pub fn new(name: &'static str) -> Self {
        with_telegram_observer_variants!(router_constructor, name)
    }

    /// Include a router to the current router as sub router
    /// # Notes
    /// Inner middlewares of this router will be registered to the sub router and its sub routers
    /// in the order of registration. Parent middlewares registers on the top of the stack,
    /// so parent middlewares calls before.
    #[must_use]
    pub fn include_router(mut self, router: impl Into<Router<Client>>) -> Self {
        self.sub_routers.push(router.into());
        self
    }

    /// Include a router to the current router as sub router
    /// # Notes
    /// Inner middlewares of this router will be registered to the sub router and its sub routers
    /// in the order of registration. Parent middlewares registers on the top of the stack,
    /// so parent middlewares calls before.
    ///
    /// Alias to [`Router::include_router`] method
    #[inline]
    #[must_use]
    pub fn include(self, router: impl Into<Router<Client>>) -> Self {
        self.include_router(router)
    }
}

impl<Client> Router<Client> {
    #[must_use]
    const fn telegram_observers(&self) -> [&TelegramObserver<Client>; 26] {
        with_telegram_observer_variants!(observer_refs_array, ref, self)
    }

    #[inline]
    #[must_use]
    #[cfg(test)]
    const fn event_observers(&self) -> [&SimpleObserver; 2] {
        [&self.startup, &self.shutdown]
    }

    /// Resolve used update types from the current router and its sub routers with skip some update types.
    /// If observer has no handlers, then it will be skipped.
    /// If observer update type is in the skip list, then it will be skipped.
    /// This method is useful for getting updates only for registered update types.
    #[must_use]
    pub fn resolve_used_update_types_with_skip(
        &self,
        skip_update_types: impl IntoIterator<Item = &'static str>,
    ) -> HashSet<&'static str> {
        let skip_update_types = skip_update_types.into_iter().collect::<HashSet<_>>();
        let mut used_update_types = HashSet::new();

        for observer in self.telegram_observers() {
            if observer.handlers.is_empty() {
                continue;
            }
            if skip_update_types.contains(observer.event_name) {
                continue;
            }
            used_update_types.insert(observer.event_name);
        }
        for router in &self.sub_routers {
            used_update_types
                .extend(router.resolve_used_update_types_with_skip(skip_update_types.clone()));
        }

        used_update_types
    }

    /// Resolve used update types from the current router and its sub routers.
    /// If observer has no handlers, then it will be skipped.
    /// This method is useful for getting updates only for registered update types.
    #[inline]
    #[must_use]
    pub fn resolve_used_update_types(&self) -> HashSet<&'static str> {
        self.resolve_used_update_types_with_skip([])
    }
}

impl<Client> Router<Client> {
    /// Configures the current [`Router`] instance using the provided middlewares configuration
    ///
    /// This method performs the following steps:
    /// 1. **Register Inner Middlewares to Sub Routers:**
    ///    For each observer field (e.g., `message`, `edited_message`, etc.), it iterates through
    ///    all sub-routers contained in `self.sub_routers` and registers each inner middleware
    ///    from the router's field into the corresponding field of the sub-router. This is done by
    ///    cloning the inner middleware list and then, for each middleware, calling
    ///    `register_boxed_at_position` with the appropriate index.
    ///
    /// 2. **Register Middlewares from the Configuration:**
    ///    For each observer field, the method registers:
    ///      - **Outer Middlewares:** Iterates over the middlewares defined in `config.outer_middlewares`
    ///        for that observer, cloning each middleware and registering it in the router's
    ///        corresponding `outer_middlewares` field.
    ///      - **Inner Middlewares:** Similarly, it iterates over the middlewares defined in
    ///        `config.inner_middlewares` and registers them in the router's `inner_middlewares` field.
    ///
    /// 3. **Reset the Outer Middlewares in the Configuration:**
    ///    Since the outer middlewares from the configuration have been applied to the router,
    ///    the configuration’s `outer_middlewares` is reset to a new, empty instance.
    ///
    /// 4. **Recursively Configure Sub Routers and Build the Final Router:**
    ///    Each sub-router is recursively configured by calling `configure` on it with a cloned
    ///    copy of the configuration. Finally, a new [`Configured`] instance is created,
    ///    incorporating all the updated fields and middleware registrations.
    ///
    /// # Parameters
    /// - `config`: A configuration that contains default outer and inner middlewares.
    ///
    /// # Returns
    /// A fully configured router instance ([`Configured`]) with all middleware registrations applied.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn configure(mut self, mut config: Config<Client>) -> Configured<Client> {
        macro_rules! register_inner_middlewares_to_sub_routers {
            ($(($variant:ident, $observer:ident)),+ $(,)?) => {
                $(
                    for sub_router in self.sub_routers.iter_mut() {
                        for (index, middleware) in self.$observer.inner_middlewares.middlewares.clone().into_iter().enumerate() {
                            sub_router.$observer.inner_middlewares.register_boxed_at_position(index, middleware);
                        }
                    }
                )+
            };
        }
        with_telegram_observer_variants!(register_inner_middlewares_to_sub_routers);

        macro_rules! register_middlewares_from_config {
            ($(($variant:ident, $observer:ident)),+ $(,)?) => {
                $(
                    for (index, middleware) in config.outer_middlewares.$observer.iter().enumerate() {
                        self.$observer.outer_middlewares.register_boxed_at_position(index, middleware.clone());
                    }
                    for (index, middleware) in config.inner_middlewares.$observer.iter().enumerate() {
                        self.$observer.inner_middlewares.register_boxed_at_position(index, middleware.clone());
                    }
                )+
            };
        }
        with_telegram_observer_variants!(register_middlewares_from_config);

        // We don't need to register config outer middlewares to sub routers
        config.outer_middlewares = OuterMiddlewaresConfig::new();

        macro_rules! router_constructor {
            ($(($variant:ident, $observer:ident)),+ $(,)?) => {
                Configured {
                    name: self.name,
                    sub_routers: self
                        .sub_routers
                        .into_iter()
                        .map(|router| router.configure(config.clone()))
                        .collect(),
                    $(
                        $observer: self.$observer,
                    )+
                    startup: self.startup,
                    shutdown: self.shutdown,
                }
            };
        }
        with_telegram_observer_variants!(router_constructor)
    }

    /// Configures the current [`Router`] instance using default middlewares configuration
    /// # Docs
    /// More info about configuration process read in [`Self::configure`] method
    #[must_use]
    pub fn configure_default(self) -> Configured<Client>
    where
        Client: Send + Sync + 'static,
    {
        self.configure(Config::default())
    }
}

impl<Client> Debug for Router<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.name)
            .field("sub_routers", &self.sub_routers)
            .finish_non_exhaustive()
    }
}

impl<Client> Default for Router<Client>
where
    Client: Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new("default")
    }
}

impl<Client> Clone for Router<Client> {
    fn clone(&self) -> Self {
        macro_rules! router_construct {
            ($(($variant:ident, $observer:ident)),+ $(,)?) => {
                Self {
                    name: self.name,
                    sub_routers: self.sub_routers.clone(),
                    $(
                        $observer: self.$observer.clone(),
                    )+
                    startup: self.startup.clone(),
                    shutdown: self.shutdown.clone(),
                }
            };
        }
        with_telegram_observer_variants!(router_construct)
    }
}

macro_rules! define_configured_struct {
    ($(($variant:ident, $observer:ident)),+ $(,)?) => {
        pub struct Configured<Client = Reqwest> {
            name: &'static str,
            sub_routers: Vec<Configured<Client>>,

            $(
                $observer: TelegramObserver<Client>,
            )+

            pub startup: SimpleObserver,
            pub shutdown: SimpleObserver,
        }
    };
}
with_telegram_observer_variants!(define_configured_struct);

impl<Client> PropagateEvent<Client> for Configured<Client>
where
    Client: 'static,
{
    #[instrument(skip_all, fields(router = self.name))]
    fn propagate_event(
        &mut self,
        update_type: UpdateType,
        mut request: Request<Client>,
    ) -> impl Future<Output = Result<Response<Client>, EventErrorKind>> + Send
    where
        Client: Send + Sync + Clone,
    {
        Box::pin(async move {
            match self.propagate_update_event(request).await? {
                // If update event handled by router, then return a response
                Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                } => {
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Handled(response),
                    });
                }
                // If update event rejected by router, then return a response
                Response {
                    request,
                    propagate_result: PropagateEventResult::Rejected,
                } => {
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    });
                }
                // If update event unhandled by router, then continue propagation
                Response {
                    request: updated_request,
                    propagate_result: PropagateEventResult::Unhandled,
                } => {
                    request = updated_request;
                }
            }

            event!(Level::TRACE, "Propagate event to router");

            let observer = self.telegram_observer_by_update_type(update_type);

            for middleware in &mut observer.outer_middlewares.middlewares {
                let (updated_request, event_return) = middleware.call(request.clone()).await?;
                match event_return {
                    // If middleware returns finish then update request because the middleware could have changed it
                    EventReturn::Finish => {
                        event!(Level::TRACE, "Outer middleware returns finish");
                        request = updated_request;
                    }
                    // If middleware returns skip, then we should skip this middleware and its changes
                    EventReturn::Skip => {
                        event!(Level::TRACE, "Outer middleware returns skip");
                    }
                    // If middleware returns cancel, then we should reject propagation
                    EventReturn::Cancel => {
                        event!(Level::TRACE, "Outer middleware returns cancel");
                        return Ok(Response {
                            request,
                            propagate_result: PropagateEventResult::Rejected,
                        });
                    }
                }
            }

            let observer_response = observer.trigger(request).await?;
            let request = observer_response.request;

            match observer_response.propagate_result {
                // If observer unhandled, then propagate event to next observer
                PropagateEventResult::Unhandled => {
                    event!(Level::TRACE, "Event unhandled by router");
                }
                // If observer handled, then return a response
                PropagateEventResult::Handled(response) => {
                    event!(Level::TRACE, "Event handled by router");
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Handled(response),
                    });
                }
                // If observer rejected, then return a response.
                // Router don't know about rejected event by observer, so it returns unhandled response.
                PropagateEventResult::Rejected => {
                    event!(Level::TRACE, "Event rejected by router");
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Unhandled,
                    });
                }
            }

            // Propagate event to sub routers
            for router in &mut self.sub_routers {
                let router_response = router.propagate_event(update_type, request.clone()).await?;
                match router_response.propagate_result {
                    // If the event unhandled by the sub router's observer, then continue propagation
                    PropagateEventResult::Unhandled => {
                        event!(Level::TRACE, "Event unhandled by sub router");
                    }
                    // If the event handled by the sub router's observer, then return a response
                    PropagateEventResult::Handled(_) => {
                        event!(Level::TRACE, "Event handled by sub router");
                        return Ok(router_response);
                    }
                    // If the event rejected by the sub router's observer, then return a response
                    PropagateEventResult::Rejected => {
                        event!(Level::TRACE, "Event rejected by sub router");
                        return Ok(router_response);
                    }
                }
            }

            // If the event unhandled by all observers, then return an unhandled response
            Ok(Response {
                request,
                propagate_result: PropagateEventResult::Unhandled,
            })
        })
    }

    #[instrument(skip_all, fields(router = self.name))]
    async fn propagate_update_event(
        &mut self,
        mut request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + Clone,
    {
        event!(Level::TRACE, "Propagate update event to router");

        for middleware in &mut self.update.outer_middlewares.middlewares {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;
            match event_return {
                // If middleware returns finish, then update request because the middleware could have changed it
                EventReturn::Finish => {
                    event!(Level::TRACE, "Update outer middleware returns finish");
                    request = updated_request;
                }
                // If middleware returns skip, then we should skip this middleware and its changes
                EventReturn::Skip => {
                    event!(Level::TRACE, "Update outer middleware returns skip");
                }
                // If middleware returns cancel, then we should cancel propagation
                EventReturn::Cancel => {
                    event!(Level::TRACE, "Update outer middleware returns cancel");
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    });
                }
            }
        }

        let observer_response = self.update.trigger(request).await?;
        let request = observer_response.request;

        match observer_response.propagate_result {
            // If observer returns unhandled, then propagate event to next observer
            PropagateEventResult::Unhandled => {
                event!(Level::TRACE, "Update event unhandled by router");
                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                })
            }
            // If observer returns handled, then return a response
            PropagateEventResult::Handled(response) => {
                event!(Level::TRACE, "Update event handled by router");
                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                })
            }
            // If observer returns rejected, then return a response.
            // Router don't know about rejected event by observer, so it returns unhandled response.
            PropagateEventResult::Rejected => {
                event!(Level::TRACE, "Update event rejected by router");
                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                })
            }
        }
    }

    #[instrument(skip_all, fields(router = self.name))]
    async fn emit_startup(&mut self) -> SimpleHandlerResult {
        fn recurse<Client: 'static>(
            router: &mut Configured<Client>,
        ) -> Pin<Box<dyn Future<Output = SimpleHandlerResult> + Send + '_>> {
            Box::pin(async move {
                if let Err(err) = router.startup.trigger(()).await {
                    event!(Level::ERROR, error = %err, "Error while emit observers");
                    return Err(err);
                }

                for sub_router in &mut router.sub_routers {
                    recurse(sub_router).await?;
                }

                Ok(())
            })
        }

        event!(Level::DEBUG, "Start observers");
        recurse(self).await
    }

    #[instrument(skip_all, fields(router = self.name))]
    async fn emit_shutdown(&mut self) -> SimpleHandlerResult {
        fn recurse<Client: 'static>(
            router: &mut Configured<Client>,
        ) -> Pin<Box<dyn Future<Output = SimpleHandlerResult> + Send + '_>> {
            Box::pin(async move {
                if let Err(err) = router.shutdown.trigger(()).await {
                    event!(Level::ERROR, error = %err, "Error while emit observers");
                    return Err(err);
                }

                for sub_router in &mut router.sub_routers {
                    recurse(sub_router).await?;
                }

                Ok(())
            })
        }

        event!(Level::DEBUG, "Start observers");
        recurse(self).await
    }

    #[inline]
    fn startup_handlers_len(&self) -> usize {
        self.startup.handlers_len()
    }

    #[inline]
    fn shutdown_handlers_len(&self) -> usize {
        self.startup.handlers_len()
    }
}

impl<Client> Configured<Client> {
    #[must_use]
    #[cfg(test)]
    const fn telegram_observers(&self) -> [&TelegramObserver<Client>; 26] {
        with_telegram_observer_variants!(observer_refs_array, ref, self)
    }

    #[must_use]
    fn telegram_observer_by_update_type(
        &mut self,
        update_type: UpdateType,
    ) -> &mut TelegramObserver<Client> {
        macro_rules! by_observer_type_match_arms {
            ($(($variant:ident, $observer:ident)),+ $(,)?) => {
                match TelegramObserverType::from(update_type) {
                    $(
                        TelegramObserverType::$variant => &mut self.$observer,
                    )+
                }
            };
        }
        with_telegram_observer_variants!(by_observer_type_match_arms)
    }
}

impl<Client> Debug for Configured<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.name)
            .field("sub_routers", &self.sub_routers)
            .finish_non_exhaustive()
    }
}

impl<Client> Clone for Configured<Client> {
    fn clone(&self) -> Self {
        macro_rules! router_construct {
            ($(($variant:ident, $observer:ident)),+ $(,)?) => {
                Self {
                    name: self.name,
                    sub_routers: self.sub_routers.clone(),
                    $(
                        $observer: self.$observer.clone(),
                    )+
                    startup: self.startup.clone(),
                    shutdown: self.shutdown.clone(),
                }
            };
        }
        with_telegram_observer_variants!(router_construct)
    }
}

impl<Client> Default for Configured<Client>
where
    Client: Send + Sync + 'static,
{
    fn default() -> Self {
        Router::default().configure_default()
    }
}

/// Macro to generate a middleware configuration and builder.
///
/// # Parameters
/// - `$config`: The name of the generated config struct
/// - `$builder`: The name of the generated builder struct
/// - `$service`: The service type (e.g. `BoxedCloneOuterMiddlewareService<Client>`)
/// - `$middleware_trait`: The middleware trait (e.g. `OuterMiddleware`)
/// - `$factory`: The middleware factory function (e.g. `boxed_outer_middleware_factory`)
/// - `{ $($field),+ }`: A list of field names for which to generate methods
/// - `default_builder: $default_builder`: A closure that takes a builder and returns it after applying default middlewares
///
/// This macro generates:
/// - A configuration struct with a boxed slice for each middleware type
/// - A builder struct with a vector for each middleware type
/// - Methods on the builder for adding a middleware to each field
/// - An `all` method on the builder to add the same middleware to all fields
/// - A `Default` implementation for the config struct using the provided default builder
macro_rules! define_middleware_config {
    (
        $config:ident,
        $builder:ident,
        $service:ty,
        $middleware_trait:ident,
        $factory:ident,
        { $($field:ident),+ $(,)? }
        , default_builder: $default_builder:expr $(,)?
    ) => {
        pub struct $config<Client> {
            $(pub $field: Box<[$service]>,)+
        }

        impl<Client> $config<Client> {
            #[must_use]
            pub fn new() -> Self {
                Self::builder().build()
            }

            #[must_use]
            pub fn builder() -> $builder<Client> {
                $builder::default()
            }
        }

        impl<Client> Clone for $config<Client> {
            fn clone(&self) -> Self {
                Self {
                    $($field: self.$field.clone(),)+
                }
            }
        }

        impl<Client: Send + Sync + 'static> Default for $config<Client> {
            fn default() -> Self {
                $default_builder(Default::default()).build()
            }
        }

        pub struct $builder<Client> {
            $(pub $field: Vec<$service>,)+
        }

        impl<Client: Send + Sync + 'static> $builder<Client> {
            $(
                #[doc = concat!("Adds a middleware to the `", stringify!($field), "` observser")]
                #[must_use]
                pub fn $field(mut self, val: impl $middleware_trait<Client>) -> Self {
                    self.$field.push($factory(val));
                    self
                }
            )+

            #[doc = "Adds the same middleware to all Telegram observsers"]
            #[must_use]
            pub fn all(mut self, middleware: impl $middleware_trait<Client>) -> Self {
                $(
                    self = self.$field(middleware.clone());
                )+
                self
            }
        }

        impl<Client> $builder<Client> {
            #[must_use]
            pub fn build(self) -> $config<Client> {
                $config {
                    $($field: self.$field.into(),)+
                }
            }
        }

        impl<Client> Default for $builder<Client> {
            fn default() -> Self {
                Self {
                    $($field: vec![],)+
                }
            }
        }
    }
}

macro_rules! define_outer_middleware_config_for_observers {
    ($(($variant:ident, $observer:ident)),+ $(,)?) => {
        define_middleware_config!(
            OuterMiddlewaresConfig,
            OuterMiddlewaresConfigBuilder,
            BoxedCloneOuterMiddlewareService<Client>,
            OuterMiddleware,
            boxed_outer_middleware_factory,
            { $($observer,)+ },
            default_builder: |builder: OuterMiddlewaresConfigBuilder<Client>| builder.update(UserContextMiddleware),
        );
    };
}
with_telegram_observer_variants!(define_outer_middleware_config_for_observers);

macro_rules! define_inner_middleware_config_for_observers {
    ($(($variant:ident, $observer:ident)),+ $(,)?) => {
        define_middleware_config!(
            InnerMiddlewaresConfig,
            InnerMiddlewaresConfigBuilder,
            BoxedCloneInnerMiddlewareService<Client>,
            InnerMiddleware,
            boxed_inner_middleware_factory,
            { $($observer,)+ },
            default_builder: |builder: InnerMiddlewaresConfigBuilder<Client>| builder.all(LoggingMiddleware),
        );
    };
}
with_telegram_observer_variants!(define_inner_middleware_config_for_observers);

pub struct Config<Client> {
    outer_middlewares: OuterMiddlewaresConfig<Client>,
    inner_middlewares: InnerMiddlewaresConfig<Client>,
}

impl<Client> Config<Client> {
    #[must_use]
    pub const fn new(
        outer_middlewares: OuterMiddlewaresConfig<Client>,
        inner_middlewares: InnerMiddlewaresConfig<Client>,
    ) -> Self {
        Self {
            outer_middlewares,
            inner_middlewares,
        }
    }
}

impl<Client> Default for Config<Client>
where
    Client: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            outer_middlewares: OuterMiddlewaresConfig::default(),
            inner_middlewares: InnerMiddlewaresConfig::default(),
        }
    }
}

impl<Client> Clone for Config<Client> {
    fn clone(&self) -> Self {
        Self {
            outer_middlewares: self.outer_middlewares.clone(),
            inner_middlewares: self.inner_middlewares.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        event::{
            simple::Handler as SimpleHandler,
            telegram::{Handler as TelegramHandler, HandlerResult as TelegramHandlerResult},
            EventReturn,
        },
        middlewares::Next,
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
        Bot, Context, Extensions,
    };

    use std::{convert::Infallible, sync::Arc};
    use tokio;

    #[test]
    fn test_include_router() {
        let inner_middleware = |request, next: Next<_>| next(request);
        let outer_middleware = |request| async move { Ok((request, EventReturn::default())) };

        let router = Router::<Reqwest>::new("main")
            .on_message(|observer| {
                observer
                    .register_inner_middleware(inner_middleware)
                    .register_outer_middleware(outer_middleware)
            })
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

        let router_configured = router.configure(Config::new(
            OuterMiddlewaresConfig::new(),
            InnerMiddlewaresConfig::new(),
        ));

        assert_eq!(router_configured.sub_routers.len(), 3);
        assert_eq!(router_configured.name, "main");

        let message_observer_name = UpdateType::Message.as_ref();

        router_configured
            .sub_routers
            .iter()
            .for_each(|router_configured| {
                assert_eq!(router_configured.sub_routers.len(), 2);

                router_configured
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

                router_configured
                    .sub_routers
                    .iter()
                    .for_each(|router_configured| {
                        assert_eq!(router_configured.sub_routers.len(), 0);

                        router_configured
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
    fn test_observer_register() {
        async fn telegram_handler() -> TelegramHandlerResult {
            Ok(EventReturn::Finish)
        }

        async fn simple_handler() -> SimpleHandlerResult {
            Ok(())
        }

        let mut router = Router::<Reqwest>::new("main")
            .on_all(|observer| observer.register(TelegramHandler::new(telegram_handler)))
            .on_startup(|observer| observer.register(SimpleHandler::new(simple_handler, ())))
            .on_shutdown(|observer| observer.register(SimpleHandler::new(simple_handler, ())));

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

        let inner_middleware = |request, next: Next| next(request);
        let outer_middleware = |request| async move { Ok((request, EventReturn::Finish)) };

        router = router.on_message(|observer| {
            observer
                .register_inner_middleware(inner_middleware)
                .register_outer_middleware(outer_middleware)
        });

        let message_observer = router
            .telegram_observers()
            .into_iter()
            .find(|observer| observer.event_name == UpdateType::Message.as_ref())
            .unwrap();

        assert_eq!(message_observer.inner_middlewares.middlewares.len(), 1);
        assert_eq!(message_observer.outer_middlewares.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_propagate_event() {
        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };

        let router = Router::new("test_handler").on_message(|observer| {
            observer.register(TelegramHandler::new(|| async move {
                Ok::<_, Infallible>(EventReturn::Finish)
            }))
        });

        let mut router_configured = router.configure_default();
        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because it's registered for this event
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let response = router_configured
            .propagate_event(UpdateType::CallbackQuery, request.clone())
            .await
            .unwrap();

        // Handler shouldn't be called, because it's not registered for this event
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let router = Router::new("test_middleware_and_handler")
            .on_update(|observer| {
                observer.register_outer_middleware(|mut request: Request<Reqwest>| async move {
                    request.context.insert("test", "test");

                    Ok((request, EventReturn::Finish))
                })
            })
            .on_message(|observer| {
                observer.register(TelegramHandler::new(|context: Context| async move {
                    println!("{}", context.len());

                    // Check that middleware was called and context was modified
                    assert_eq!(context.get::<&str>("test").unwrap(), &"test");

                    Ok::<_, Infallible>(EventReturn::Finish)
                }))
            });

        let mut router_configured = router.configure_default();

        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because it's registered for this event
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let router = Router::new("test_skip_handler")
            .on_message(|observer| {
                observer.register(TelegramHandler::new(|| async move {
                    Ok::<_, Infallible>(EventReturn::Skip)
                }))
            })
            .on_message(|observer| {
                observer.register(TelegramHandler::new(|| async move {
                    Ok::<_, Infallible>(EventReturn::Finish)
                }))
            });

        let mut router_configured = router.configure_default();

        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because it's registered for this event.
        // First handler skipped, so second handler should be called.
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let router = Router::new("test_skip_handler_without_next").on_message(|observer| {
            observer.register(TelegramHandler::new(|| async move {
                Ok::<_, Infallible>(EventReturn::Skip)
            }))
        });

        let mut router_configured = router.configure_default();

        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because it's registered for this event.
        // First handler skipped, but there is no next handler, so event should be unhandled.
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    async fn test_propagate_event_with_filter() {
        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };

        let router = Router::new("test_handler_with_filter").on_message(|observer| {
            observer.register(
                TelegramHandler::new(|| async move { Ok::<_, Infallible>(EventReturn::Finish) })
                    .filter(|_req: &mut Request| async move { Ok::<_, Infallible>(true) }),
            )
        });

        let mut router_configured = router.configure_default();
        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because filter returns `true`
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let router = Router::new("test_handler_with_fail_filter").on_message(|observer| {
            observer.register(
                TelegramHandler::new(|| async move { Ok::<_, Infallible>(EventReturn::Finish) })
                    .filter(|_req: &mut Request| async move { Ok::<_, Infallible>(false) }),
            )
        });

        let mut router_configured = router.configure_default();
        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler shouldn't be called, because filter returns `false`
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let router = Router::new("test_handler_with_filters_and_one_fail").on_message(|observer| {
            observer.register(
                TelegramHandler::new(|| async move { Ok::<_, Infallible>(EventReturn::Finish) })
                    .filter(|_req: &mut Request| async move { Ok::<_, Infallible>(true) })
                    .filter(|_req: &mut Request| async move { Ok::<_, Infallible>(true) })
                    .filter(|_req: &mut Request| async move { Ok::<_, Infallible>(false) }),
            )
        });

        let mut router_configured = router.configure_default();
        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler shouldn't be called, because one filter returns `false`
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_resolve_used_update_types() {
        let router = Router::<Reqwest>::new("test")
            .on_message(|observer| {
                observer.register(TelegramHandler::new(|| async {
                    Ok::<_, Infallible>(EventReturn::Finish)
                }))
            })
            .on_edited_message(|observer| {
                observer.register(TelegramHandler::new(|| async {
                    Ok::<_, Infallible>(EventReturn::Finish)
                }))
            });

        let update_types = router.resolve_used_update_types();

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(UpdateType::Message.as_ref()));
        assert!(update_types.contains(UpdateType::EditedMessage.as_ref()));

        let router2 = Router::<Reqwest>::new("test2")
            .on_message(|observer| {
                observer.register(TelegramHandler::new(|| async {
                    Ok::<_, Infallible>(EventReturn::Finish)
                }))
            })
            .on_channel_post(|observer| {
                observer.register(TelegramHandler::new(|| async {
                    Ok::<_, Infallible>(EventReturn::Finish)
                }))
            });

        assert_eq!(router2.resolve_used_update_types().len(), 2);

        let router = router.include(router2);

        let update_types = router.resolve_used_update_types();

        println!("{update_types:?}");

        assert_eq!(update_types.len(), 3);
        assert!(update_types.contains(UpdateType::Message.as_ref()));
        assert!(update_types.contains(UpdateType::EditedMessage.as_ref()));
        assert!(update_types.contains(UpdateType::ChannelPost.as_ref()));

        let update_types =
            router.resolve_used_update_types_with_skip([UpdateType::Message.as_ref()]);

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(UpdateType::EditedMessage.as_ref()));
        assert!(update_types.contains(UpdateType::ChannelPost.as_ref()));
    }

    struct DummyClient;

    #[test]
    fn test_outer_middlewares_config_default() {
        let config = OuterMiddlewaresConfig::<DummyClient>::default();
        assert_eq!(config.update.len(), 1);
        assert_eq!(config.message.len(), 0);
        assert_eq!(config.edited_message.len(), 0);
    }

    #[test]
    fn test_inner_middlewares_config_default() {
        let config = InnerMiddlewaresConfig::<DummyClient>::default();
        assert_eq!(config.message.len(), 1);
        assert_eq!(config.edited_message.len(), 1);
        assert_eq!(config.callback_query.len(), 1);
    }

    #[test]
    fn test_middlewares_config_default() {
        let config = Config::<DummyClient>::default();
        assert_eq!(config.outer_middlewares.update.len(), 1);
        assert_eq!(config.outer_middlewares.message.len(), 0);
        assert_eq!(config.outer_middlewares.edited_message.len(), 0);
        assert_eq!(config.inner_middlewares.message.len(), 1);
        assert_eq!(config.inner_middlewares.edited_message.len(), 1);
        assert_eq!(config.inner_middlewares.callback_query.len(), 1);
    }
}
