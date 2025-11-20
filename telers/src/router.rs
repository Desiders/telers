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
//! ```ignore
//! async fn on_startup(message: &str) -> HandlerResult {
//!     ...
//! }
//!
//! async fn on_shutdown(message: &str) -> HandlerResult {
//!     ...
//! }
//!
//! let mut router = Router::new("example");
//! router.startup.register(on_startup, ("Hello, world!",));
//! router.shutdown.register(on_shutdown, ("Goodbye, world!",));
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
//! ```ignore
//! async fn on_message(message: Message) -> HandlerResult {
//!    ...
//! }
//!
//! async fn on_callback_query(callback_query: CallbackQuery) -> HandlerResult {
//!   ...
//! }
//!
//! let mut router = Router::new("example");
//! router.message.register(on_message);
//! router.callback_query.register(on_callback_query);
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
    enums::{SimpleObserverName, TelegramObserverName, UpdateType},
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

use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    future::Future,
    iter::once,
};
use tracing::{event, instrument, Level};

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

/// Router combines all event observers.
///
/// Each event observer is a special unit that handles a specific event type.
/// There are two types of event observers:
///
/// - Simple observer - [`SimpleObserver`]
///
/// Simple observer is used to handle simple events like startup and shutdown. \
/// When you register a handler in this observer,
/// you specify the arguments that pass to handler when the event is trigger. \
/// Return type of handler is `Result<(), HandlerError>`. \
/// When observer is trigger, it calls all handlers in order of registration and stops if one of them returns an error.
///
/// Registration of handlers looks like this:
/// ```ignore
/// async fn on_startup(message: &str) -> HandlerResult {
///     ...
/// }
///
/// async fn on_shutdown(message: &str) -> HandlerResult {
///     ...
/// }
///
/// let mut router = Router::new("example");
/// router.startup.register(on_startup, ("Hello, world!",));
/// router.shutdown.register(on_shutdown, ("Goodbye, world!",));
/// ```
///
/// - Telegram observer - [`TelegramObserver`]
///
/// Telegram observer is used to handle telegram events like messages, callback queries, polls and all other event types. \
/// You can register a handler with any arguments that implement [`crate::Extractor`] trait,
/// see [`crate::extractor`] for more details. \
/// Return type of handler is `Result<EventReturn, HandlerError>`,
/// where [`EventReturn`] is a special enum that can be used to control the propagation of the event,
/// see [`EventReturn`] for more details. \
/// When observer is trigger, it calls outer middlewares and checks all handlers in order of registration.
/// It calls all filters for each handler and skips handler if one of them returns `false`.
/// If handler is pass the filters, observer calls inner middlewares and handler itself (in the middleware).
/// By default, the first handler that pass the filters stop the propagation of the event, so other handlers aren't calls.
/// (You can change this behaviour by specify another variant of [`EventReturn`]).
///
/// Registration of handlers looks like this:
/// ```ignore
/// async fn on_message(message: Message) -> HandlerResult {
///    ...
/// }
///
/// async fn on_callback_query(callback_query: CallbackQuery) -> HandlerResult {
///   ...
/// }
///
/// let mut router = Router::new("example");
/// router.message.register(on_message);
/// router.callback_query.register(on_callback_query);
/// ```
pub struct Router<Client = Reqwest> {
    name: &'static str,
    sub_routers: Vec<Router<Client>>,

    pub message: TelegramObserver<Client>,
    pub edited_message: TelegramObserver<Client>,
    pub channel_post: TelegramObserver<Client>,
    pub edited_channel_post: TelegramObserver<Client>,
    pub business_connection: TelegramObserver<Client>,
    pub business_message: TelegramObserver<Client>,
    pub edited_business_message: TelegramObserver<Client>,
    pub deleted_business_messages: TelegramObserver<Client>,
    pub message_reaction: TelegramObserver<Client>,
    pub message_reaction_count: TelegramObserver<Client>,
    pub inline_query: TelegramObserver<Client>,
    pub chosen_inline_result: TelegramObserver<Client>,
    pub callback_query: TelegramObserver<Client>,
    pub shipping_query: TelegramObserver<Client>,
    pub pre_checkout_query: TelegramObserver<Client>,
    pub purchased_paid_media: TelegramObserver<Client>,
    pub poll: TelegramObserver<Client>,
    pub poll_answer: TelegramObserver<Client>,
    pub my_chat_member: TelegramObserver<Client>,
    pub chat_member: TelegramObserver<Client>,
    pub chat_join_request: TelegramObserver<Client>,
    pub chat_boost: TelegramObserver<Client>,
    pub removed_chat_boost: TelegramObserver<Client>,
    /// This special event observer is used to handle all telegram events.
    /// It's called for router and its sub routers and before other telegram observers.
    /// This observer is useful for register important middlewares (often libraries) like `FSMContext` and `UserContext`,
    /// that set up context for other.
    pub update: TelegramObserver<Client>,

    pub startup: SimpleObserver,
    pub shutdown: SimpleObserver,
}

impl<Client> Router<Client>
where
    Client: Send + Sync + 'static,
{
    /// # Arguments
    /// * `router_name` - Name of the router. It can be used for logging and debugging and code clarity.
    #[must_use]
    #[rustfmt::skip]
    pub fn new(router_name: &'static str) -> Self {
        Self {
            name: router_name,
            sub_routers: vec![],
            message: TelegramObserver::new(TelegramObserverName::Message),
            edited_message: TelegramObserver::new(TelegramObserverName::EditedMessage),
            channel_post: TelegramObserver::new(TelegramObserverName::ChannelPost),
            edited_channel_post: TelegramObserver::new(TelegramObserverName::EditedChannelPost),
            business_connection: TelegramObserver::new(TelegramObserverName::BusinessConnection),
            business_message: TelegramObserver::new(TelegramObserverName::BusinessMessage),
            edited_business_message: TelegramObserver::new(TelegramObserverName::EditedBusinessMessage),
            deleted_business_messages: TelegramObserver::new(TelegramObserverName::DeletedBusinessMessages),
            message_reaction: TelegramObserver::new(TelegramObserverName::MessageReaction),
            message_reaction_count: TelegramObserver::new(TelegramObserverName::MessageReactionCount),
            inline_query: TelegramObserver::new(TelegramObserverName::InlineQuery),
            chosen_inline_result: TelegramObserver::new(TelegramObserverName::ChosenInlineResult),
            callback_query: TelegramObserver::new(TelegramObserverName::CallbackQuery),
            shipping_query: TelegramObserver::new(TelegramObserverName::ShippingQuery),
            pre_checkout_query: TelegramObserver::new(TelegramObserverName::PreCheckoutQuery),
            purchased_paid_media: TelegramObserver::new(TelegramObserverName::PurchasedPaidMedia),
            poll: TelegramObserver::new(TelegramObserverName::Poll),
            poll_answer: TelegramObserver::new(TelegramObserverName::PollAnswer),
            my_chat_member: TelegramObserver::new(TelegramObserverName::MyChatMember),
            chat_member: TelegramObserver::new(TelegramObserverName::ChatMember),
            chat_join_request: TelegramObserver::new(TelegramObserverName::ChatJoinRequest),
            chat_boost: TelegramObserver::new(TelegramObserverName::ChatBoost),
            removed_chat_boost: TelegramObserver::new(TelegramObserverName::RemovedChatBoost),
            update: TelegramObserver::new(TelegramObserverName::Update),
            startup: SimpleObserver::new(SimpleObserverName::Startup),
            shutdown: SimpleObserver::new(SimpleObserverName::Shutdown),
        }
    }

    /// Include a router to the current router as sub router
    /// # Notes
    /// Inner middlewares of this router will be registered to the sub router and its sub routers
    /// in the order of registration. Parent middlewares registers on the top of the stack,
    /// so parent middlewares calls before.
    pub fn include_router(&mut self, router: impl Into<Router<Client>>) -> &mut Self {
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
    pub fn include(&mut self, router: impl Into<Router<Client>>) -> &mut Self {
        self.include_router(router)
    }
}

impl<Client> Router<Client> {
    /// Get all telegram event observers
    #[must_use]
    pub const fn telegram_observers(&self) -> [&TelegramObserver<Client>; 24] {
        [
            &self.message,
            &self.edited_message,
            &self.channel_post,
            &self.edited_channel_post,
            &self.business_connection,
            &self.business_message,
            &self.edited_business_message,
            &self.deleted_business_messages,
            &self.message_reaction,
            &self.message_reaction_count,
            &self.inline_query,
            &self.chosen_inline_result,
            &self.callback_query,
            &self.shipping_query,
            &self.pre_checkout_query,
            &self.purchased_paid_media,
            &self.poll,
            &self.poll_answer,
            &self.my_chat_member,
            &self.chat_member,
            &self.chat_join_request,
            &self.chat_boost,
            &self.removed_chat_boost,
            &self.update,
        ]
    }

    /// Get all telegram event observers as mutable references
    /// # Notes
    /// This method is useful for registering middlewares to the many observers without code duplication and macros
    #[must_use]
    pub fn telegram_observers_mut(&mut self) -> Vec<&mut TelegramObserver<Client>> {
        let mut observers = Vec::with_capacity(24);

        observers.extend([
            &mut self.message,
            &mut self.edited_message,
            &mut self.channel_post,
            &mut self.edited_channel_post,
            &mut self.business_connection,
            &mut self.business_message,
            &mut self.edited_business_message,
            &mut self.deleted_business_messages,
            &mut self.message_reaction,
            &mut self.message_reaction_count,
            &mut self.inline_query,
            &mut self.chosen_inline_result,
            &mut self.callback_query,
            &mut self.shipping_query,
            &mut self.pre_checkout_query,
            &mut self.purchased_paid_media,
            &mut self.poll,
            &mut self.poll_answer,
            &mut self.my_chat_member,
            &mut self.chat_member,
            &mut self.chat_join_request,
            &mut self.chat_boost,
            &mut self.removed_chat_boost,
            &mut self.update,
        ]);

        observers
    }

    /// Get all simple event observers
    #[must_use]
    pub const fn event_observers(&self) -> [&SimpleObserver; 2] {
        [&self.startup, &self.shutdown]
    }

    /// Resolve used update types from the current router and its sub routers with skip some update types.
    /// If observer has no handlers, then it will be skipped.
    /// If observer update type is in the skip list, then it will be skipped.
    /// This method is useful for getting updates only for registered update types.
    /// # Panics
    /// If can't convert observer event name to [`UpdateType`]
    #[must_use]
    pub fn resolve_used_update_types_with_skip(
        &self,
        skip_update_types: impl IntoIterator<Item = UpdateType>,
    ) -> HashSet<UpdateType> {
        let skip_update_types = skip_update_types.into_iter().collect::<HashSet<_>>();
        let mut used_update_types = HashSet::new();

        for observer in self.telegram_observers() {
            if observer.handlers().is_empty() {
                continue;
            }

            let Some(update_type) = observer.event_name.into() else {
                // If can't convert observer event name to `UpdateType`, then skip it, because it's `TelegramObserverName::Update`
                continue;
            };

            if skip_update_types.contains(&update_type) {
                continue;
            }

            used_update_types.insert(update_type);
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
    /// # Panics
    /// If can't convert observer event name to [`UpdateType`]
    #[must_use]
    pub fn resolve_used_update_types(&self) -> HashSet<UpdateType> {
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
    ///    copy of the configuration. Finally, a new [`RouterConfigured`] instance is created,
    ///    incorporating all the updated fields and middleware registrations.
    ///
    /// # Parameters
    /// - `config`: A configuration that contains default outer and inner middlewares.
    ///
    /// # Returns
    /// A fully configured router instance ([`RouterConfigured`]) with all middleware registrations applied.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn configure(mut self, mut config: Config<Client>) -> Configured<Client> {
        macro_rules! register_inner_middlewares_to_sub_routers {
            ($($observer:ident),+ $(,)?) => {
                $(
                    for sub_router in self.sub_routers.iter_mut() {
                        for (index, middleware) in self.$observer.inner_middlewares.middlewares.clone().into_iter().enumerate() {
                            sub_router.$observer.inner_middlewares.register_boxed_at_position(index, middleware);
                        }
                    }
                )+
            };
        }

        register_inner_middlewares_to_sub_routers!(
            message,
            edited_message,
            channel_post,
            edited_channel_post,
            business_connection,
            business_message,
            edited_business_message,
            deleted_business_messages,
            message_reaction,
            message_reaction_count,
            inline_query,
            chosen_inline_result,
            callback_query,
            shipping_query,
            pre_checkout_query,
            purchased_paid_media,
            poll,
            poll_answer,
            my_chat_member,
            chat_member,
            chat_join_request,
            chat_boost,
            removed_chat_boost,
            update,
        );

        macro_rules! register_middlewares_from_config {
            ($($observer:ident),+ $(,)?) => {
                $(
                    for (index, middleware) in config.outer_middlewares.$observer.iter().enumerate() {
                        self.$observer.outer_middlewares.register_boxed_at_position(index, middleware.clone());
                    }
                    // Регистрация inner middlewares
                    for (index, middleware) in config.inner_middlewares.$observer.iter().enumerate() {
                        self.$observer.inner_middlewares.register_boxed_at_position(index, middleware.clone());
                    }
                )+
            };
        }

        register_middlewares_from_config!(
            message,
            edited_message,
            channel_post,
            edited_channel_post,
            business_connection,
            business_message,
            edited_business_message,
            deleted_business_messages,
            message_reaction,
            message_reaction_count,
            inline_query,
            chosen_inline_result,
            callback_query,
            shipping_query,
            pre_checkout_query,
            purchased_paid_media,
            poll,
            poll_answer,
            my_chat_member,
            chat_member,
            chat_join_request,
            chat_boost,
            removed_chat_boost,
            update,
        );

        // We don't need to register config outer middlewares to sub routers
        config.outer_middlewares = OuterMiddlewaresConfig::new();

        Configured {
            name: self.name,
            sub_routers: self
                .sub_routers
                .into_iter()
                .map(|router| router.configure(config.clone()))
                .collect(),
            message: self.message,
            edited_message: self.edited_message,
            channel_post: self.channel_post,
            edited_channel_post: self.edited_channel_post,
            business_connection: self.business_connection,
            business_message: self.business_message,
            edited_business_message: self.edited_business_message,
            deleted_business_messages: self.deleted_business_messages,
            message_reaction: self.message_reaction,
            message_reaction_count: self.message_reaction_count,
            inline_query: self.inline_query,
            chosen_inline_result: self.chosen_inline_result,
            callback_query: self.callback_query,
            shipping_query: self.shipping_query,
            pre_checkout_query: self.pre_checkout_query,
            purchased_paid_media: self.purchased_paid_media,
            poll: self.poll,
            poll_answer: self.poll_answer,
            my_chat_member: self.my_chat_member,
            chat_member: self.chat_member,
            chat_join_request: self.chat_join_request,
            chat_boost: self.chat_boost,
            removed_chat_boost: self.removed_chat_boost,
            update: self.update,
            startup: self.startup,
            shutdown: self.shutdown,
        }
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
        Self {
            name: self.name,
            sub_routers: self.sub_routers.clone(),
            message: self.message.clone(),
            edited_message: self.edited_message.clone(),
            channel_post: self.channel_post.clone(),
            edited_channel_post: self.edited_channel_post.clone(),
            business_connection: self.business_connection.clone(),
            business_message: self.business_message.clone(),
            edited_business_message: self.edited_business_message.clone(),
            deleted_business_messages: self.deleted_business_messages.clone(),
            message_reaction: self.message_reaction.clone(),
            message_reaction_count: self.message_reaction_count.clone(),
            inline_query: self.inline_query.clone(),
            chosen_inline_result: self.chosen_inline_result.clone(),
            callback_query: self.callback_query.clone(),
            shipping_query: self.shipping_query.clone(),
            pre_checkout_query: self.pre_checkout_query.clone(),
            purchased_paid_media: self.purchased_paid_media.clone(),
            poll: self.poll.clone(),
            poll_answer: self.poll_answer.clone(),
            my_chat_member: self.my_chat_member.clone(),
            chat_member: self.chat_member.clone(),
            chat_join_request: self.chat_join_request.clone(),
            chat_boost: self.chat_boost.clone(),
            removed_chat_boost: self.removed_chat_boost.clone(),
            update: self.update.clone(),
            startup: self.startup.clone(),
            shutdown: self.shutdown.clone(),
        }
    }
}

pub struct Configured<Client = Reqwest> {
    name: &'static str,
    sub_routers: Vec<Configured<Client>>,

    message: TelegramObserver<Client>,
    edited_message: TelegramObserver<Client>,
    channel_post: TelegramObserver<Client>,
    edited_channel_post: TelegramObserver<Client>,
    business_connection: TelegramObserver<Client>,
    business_message: TelegramObserver<Client>,
    edited_business_message: TelegramObserver<Client>,
    deleted_business_messages: TelegramObserver<Client>,
    message_reaction: TelegramObserver<Client>,
    message_reaction_count: TelegramObserver<Client>,
    inline_query: TelegramObserver<Client>,
    chosen_inline_result: TelegramObserver<Client>,
    callback_query: TelegramObserver<Client>,
    shipping_query: TelegramObserver<Client>,
    pre_checkout_query: TelegramObserver<Client>,
    purchased_paid_media: TelegramObserver<Client>,
    poll: TelegramObserver<Client>,
    poll_answer: TelegramObserver<Client>,
    my_chat_member: TelegramObserver<Client>,
    chat_member: TelegramObserver<Client>,
    chat_join_request: TelegramObserver<Client>,
    chat_boost: TelegramObserver<Client>,
    removed_chat_boost: TelegramObserver<Client>,

    update: TelegramObserver<Client>,

    startup: SimpleObserver,
    shutdown: SimpleObserver,
}

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

            for middleware in observer.outer_middlewares_mut() {
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

        for middleware in self.update.outer_middlewares_mut() {
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
        if self.startup_handlers_len() == 0 {
            event!(Level::TRACE, "Observers empty");
            return Ok(());
        }

        event!(Level::DEBUG, "Start observers");
        for startup in once(&mut self.startup).chain(
            self.sub_routers
                .iter_mut()
                .map(|router| &mut router.startup),
        ) {
            if let Err(err) = startup.trigger(()).await {
                event!(Level::ERROR, error = %err, "Error while emit observers");
                return Err(err);
            }
        }
        Ok(())
    }

    fn startup_handlers_len(&self) -> usize {
        self.startup.handlers().len()
    }

    #[instrument(skip_all, fields(router = self.name))]
    async fn emit_shutdown(&mut self) -> SimpleHandlerResult {
        if self.shutdown_handlers_len() == 0 {
            event!(Level::TRACE, "Observers empty");
            return Ok(());
        }

        event!(Level::DEBUG, "Start observers");
        for shutdown in once(&mut self.shutdown).chain(
            self.sub_routers
                .iter_mut()
                .map(|router| &mut router.shutdown),
        ) {
            if let Err(err) = shutdown.trigger(()).await {
                event!(Level::ERROR, error = %err, "Error while emit observers");
                return Err(err);
            }
        }
        Ok(())
    }

    fn shutdown_handlers_len(&self) -> usize {
        self.shutdown.handlers().len()
    }
}

impl<Client> Configured<Client> {
    #[must_use]
    pub const fn telegram_observers(&self) -> [&TelegramObserver<Client>; 24] {
        [
            &self.message,
            &self.edited_message,
            &self.channel_post,
            &self.edited_channel_post,
            &self.business_connection,
            &self.business_message,
            &self.edited_business_message,
            &self.deleted_business_messages,
            &self.message_reaction,
            &self.message_reaction_count,
            &self.inline_query,
            &self.chosen_inline_result,
            &self.callback_query,
            &self.shipping_query,
            &self.pre_checkout_query,
            &self.purchased_paid_media,
            &self.poll,
            &self.poll_answer,
            &self.my_chat_member,
            &self.chat_member,
            &self.chat_join_request,
            &self.chat_boost,
            &self.removed_chat_boost,
            &self.update,
        ]
    }

    #[must_use]
    pub const fn event_observers(&self) -> [&SimpleObserver; 2] {
        [&self.startup, &self.shutdown]
    }

    #[must_use]
    pub fn telegram_observer_by_update_type(
        &mut self,
        update_type: UpdateType,
    ) -> &mut TelegramObserver<Client> {
        match update_type {
            UpdateType::Message => &mut self.message,
            UpdateType::EditedMessage => &mut self.edited_message,
            UpdateType::ChannelPost => &mut self.channel_post,
            UpdateType::EditedChannelPost => &mut self.edited_channel_post,
            UpdateType::BusinessConnection => &mut self.business_connection,
            UpdateType::BusinessMessage => &mut self.business_message,
            UpdateType::EditedBusinessMessage => &mut self.edited_business_message,
            UpdateType::DeletedBusinessMessages => &mut self.deleted_business_messages,
            UpdateType::MessageReaction => &mut self.message_reaction,
            UpdateType::MessageReactionCount => &mut self.message_reaction_count,
            UpdateType::InlineQuery => &mut self.inline_query,
            UpdateType::ChosenInlineResult => &mut self.chosen_inline_result,
            UpdateType::CallbackQuery => &mut self.callback_query,
            UpdateType::ShippingQuery => &mut self.shipping_query,
            UpdateType::PreCheckoutQuery => &mut self.pre_checkout_query,
            UpdateType::PurchasedPaidMedia => &mut self.purchased_paid_media,
            UpdateType::Poll => &mut self.poll,
            UpdateType::PollAnswer => &mut self.poll_answer,
            UpdateType::MyChatMember => &mut self.my_chat_member,
            UpdateType::ChatMember => &mut self.chat_member,
            UpdateType::ChatJoinRequest => &mut self.chat_join_request,
            UpdateType::ChatBoost => &mut self.chat_boost,
            UpdateType::RemovedChatBoost => &mut self.removed_chat_boost,
        }
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
        Self {
            name: self.name,
            sub_routers: self.sub_routers.clone(),
            message: self.message.clone(),
            edited_message: self.edited_message.clone(),
            channel_post: self.channel_post.clone(),
            edited_channel_post: self.edited_channel_post.clone(),
            business_connection: self.business_connection.clone(),
            business_message: self.business_message.clone(),
            edited_business_message: self.edited_business_message.clone(),
            deleted_business_messages: self.deleted_business_messages.clone(),
            message_reaction: self.message_reaction.clone(),
            message_reaction_count: self.message_reaction_count.clone(),
            inline_query: self.inline_query.clone(),
            chosen_inline_result: self.chosen_inline_result.clone(),
            callback_query: self.callback_query.clone(),
            shipping_query: self.shipping_query.clone(),
            pre_checkout_query: self.pre_checkout_query.clone(),
            purchased_paid_media: self.purchased_paid_media.clone(),
            poll: self.poll.clone(),
            poll_answer: self.poll_answer.clone(),
            my_chat_member: self.my_chat_member.clone(),
            chat_member: self.chat_member.clone(),
            chat_join_request: self.chat_join_request.clone(),
            chat_boost: self.chat_boost.clone(),
            removed_chat_boost: self.removed_chat_boost.clone(),
            update: self.update.clone(),
            startup: self.startup.clone(),
            shutdown: self.shutdown.clone(),
        }
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

define_middleware_config!(
    OuterMiddlewaresConfig,
    OuterMiddlewaresConfigBuilder,
    BoxedCloneOuterMiddlewareService<Client>,
    OuterMiddleware,
    boxed_outer_middleware_factory,
    {
        message,
        edited_message,
        channel_post,
        edited_channel_post,
        business_connection,
        business_message,
        edited_business_message,
        deleted_business_messages,
        message_reaction,
        message_reaction_count,
        inline_query,
        chosen_inline_result,
        callback_query,
        shipping_query,
        pre_checkout_query,
        purchased_paid_media,
        poll,
        poll_answer,
        my_chat_member,
        chat_member,
        chat_join_request,
        chat_boost,
        removed_chat_boost,
        update,
    },
    default_builder: |builder: OuterMiddlewaresConfigBuilder<Client>| builder.update(UserContextMiddleware),
);

define_middleware_config!(
    InnerMiddlewaresConfig,
    InnerMiddlewaresConfigBuilder,
    BoxedCloneInnerMiddlewareService<Client>,
    InnerMiddleware,
    boxed_inner_middleware_factory,
    {
        message,
        edited_message,
        channel_post,
        edited_channel_post,
        business_connection,
        business_message,
        edited_business_message,
        deleted_business_messages,
        message_reaction,
        message_reaction_count,
        inline_query,
        chosen_inline_result,
        callback_query,
        shipping_query,
        pre_checkout_query,
        purchased_paid_media,
        poll,
        poll_answer,
        my_chat_member,
        chat_member,
        chat_join_request,
        chat_boost,
        removed_chat_boost,
        update,
    },
    default_builder: |builder: InnerMiddlewaresConfigBuilder<Client>| builder.all(LoggingMiddleware),
);

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
        event::{telegram::HandlerResult as TelegramHandlerResult, EventReturn},
        middlewares::Next,
        Context,
    };

    use std::convert::Infallible;
    use tokio;

    #[test]
    fn test_include_router() {
        let mut router = Router::<Reqwest>::new("main");

        let inner_middleware = |request, next: Next<_>| next(request);
        let outer_middleware = |request| async move { Ok((request, EventReturn::default())) };

        router.message.inner_middlewares.register(inner_middleware);
        router.message.outer_middlewares.register(outer_middleware);

        router
            .include({
                let mut router = Router::new("sub1");
                router
                    .include(Router::new("sub1.1"))
                    .include(Router::new("sub1.2"));
                router
            })
            .include({
                let mut router = Router::new("sub2");
                router
                    .include(Router::new("sub2.1"))
                    .include(Router::new("sub2.2"));
                router
            })
            .include({
                let mut router = Router::new("sub3");
                router
                    .include(Router::new("sub3.1"))
                    .include(Router::new("sub3.2"));
                router
            });

        let router_configured = router.configure(Config::new(
            OuterMiddlewaresConfig::new(),
            InnerMiddlewaresConfig::new(),
        ));

        assert_eq!(router_configured.sub_routers.len(), 3);
        assert_eq!(router_configured.name, "main");

        let message_observer_name = UpdateType::Message;

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
                            assert_eq!(observer.inner_middlewares().len(), 1);
                        } else {
                            assert_eq!(observer.inner_middlewares().len(), 0);
                        }
                        // Router outer middlewares don't clone to children routers
                        assert_eq!(observer.outer_middlewares().len(), 0);
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
                                    assert_eq!(observer.inner_middlewares().len(), 1);
                                } else {
                                    assert_eq!(observer.inner_middlewares().len(), 0);
                                }
                                // Router outer middlewares don't clone to children routers
                                assert_eq!(observer.outer_middlewares().len(), 0);
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

        let mut router = Router::<Reqwest>::new("main");
        // Telegram event observers
        router.message.register(telegram_handler);
        router.edited_message.register(telegram_handler);
        router.channel_post.register(telegram_handler);
        router.edited_channel_post.register(telegram_handler);
        router.business_connection.register(telegram_handler);
        router.business_message.register(telegram_handler);
        router.edited_business_message.register(telegram_handler);
        router.deleted_business_messages.register(telegram_handler);
        router.message_reaction.register(telegram_handler);
        router.message_reaction_count.register(telegram_handler);
        router.inline_query.register(telegram_handler);
        router.chosen_inline_result.register(telegram_handler);
        router.callback_query.register(telegram_handler);
        router.shipping_query.register(telegram_handler);
        router.pre_checkout_query.register(telegram_handler);
        router.purchased_paid_media.register(telegram_handler);
        router.poll.register(telegram_handler);
        router.poll_answer.register(telegram_handler);
        router.my_chat_member.register(telegram_handler);
        router.chat_member.register(telegram_handler);
        router.chat_join_request.register(telegram_handler);
        router.chat_boost.register(telegram_handler);
        router.removed_chat_boost.register(telegram_handler);
        router.update.register(telegram_handler);
        // Event observers
        router.startup.register(simple_handler, ());
        router.shutdown.register(simple_handler, ());

        // Check telegram event observers
        router
            .telegram_observers()
            .into_iter()
            .for_each(|observer| {
                assert_eq!(observer.handlers().len(), 1);

                observer.handlers().iter().for_each(|handler| {
                    assert!(handler.filters.is_empty());
                });
            });

        // Check event observers
        router.event_observers().into_iter().for_each(|observer| {
            assert_eq!(observer.handlers().len(), 1);
        });

        let inner_middleware = |request, next: Next| next(request);
        let outer_middleware = |request| async move { Ok((request, EventReturn::Finish)) };

        router.message.inner_middlewares.register(inner_middleware);
        router.message.outer_middlewares.register(outer_middleware);

        assert_eq!(router.message.inner_middlewares.middlewares.len(), 1);
        assert_eq!(router.message.outer_middlewares.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_propagate_event() {
        let request = Request::<Reqwest>::default();

        let mut router = Router::new("test_handler");
        router
            .message
            .register(|| async move { Ok::<_, Infallible>(EventReturn::Finish) });

        let mut router_configured = router.configure_default();
        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because it's registered for this event
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
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

        let mut router = Router::new("test_middleware_and_handler");
        router
            .update
            .outer_middlewares
            .register(|mut request: Request<Reqwest>| async move {
                request.context.insert("test", "test");

                Ok((request, EventReturn::Finish))
            });
        router.message.register(|context: Context| async move {
            println!("{}", context.len());

            // Check that middleware was called and context was modified
            assert_eq!(context.get::<&str>("test").unwrap(), &"test");

            Ok::<_, Infallible>(EventReturn::Finish)
        });

        let mut router_configured = router.configure_default();

        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because it's registered for this event
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("test_skip_handler");
        router
            .message
            .register(|| async move { Ok::<_, Infallible>(EventReturn::Skip) });
        router
            .message
            .register(|| async move { Ok::<_, Infallible>(EventReturn::Finish) });

        let mut router_configured = router.configure_default();

        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because it's registered for this event.
        // First handler skipped, so second handler should be called.
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("test_skip_handler_without_next");
        router
            .message
            .register(|| async move { Ok::<_, Infallible>(EventReturn::Skip) });

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
        let request = Request::<Reqwest>::default();

        let mut router = Router::new("test_handler_with_filter");
        router
            .message
            .register(|| async move { Ok::<_, Infallible>(EventReturn::Finish) })
            .filter(|_req: &mut Request| async move { true });

        let mut router_configured = router.configure_default();
        let response = router_configured
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Handler should be called, because filter returns `true`
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("test_handler_with_fail_filter");
        router
            .message
            .register(|| async move { Ok::<_, Infallible>(EventReturn::Finish) })
            .filter(|_req: &mut Request| async move { false });

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

        let mut router = Router::new("test_handler_with_filters_and_one_fail");
        router
            .message
            .register(|| async move { Ok::<_, Infallible>(EventReturn::Finish) })
            .filter(|_req: &mut Request| async move { true })
            .filter(|_req: &mut Request| async move { true })
            .filter(|_req: &mut Request| async move { false });

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
        let mut router = Router::<Reqwest>::new("test");

        router
            .message
            .register(|| async { Ok::<_, Infallible>(EventReturn::Finish) });
        router
            .edited_message
            .register(|| async { Ok::<_, Infallible>(EventReturn::Finish) });

        let update_types = router.resolve_used_update_types();

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(&UpdateType::Message));
        assert!(update_types.contains(&UpdateType::EditedMessage));

        let mut router2 = Router::<Reqwest>::new("test2");

        router2
            .message
            .register(|| async { Ok::<_, Infallible>(EventReturn::Finish) });
        router2
            .channel_post
            .register(|| async { Ok::<_, Infallible>(EventReturn::Finish) });

        assert_eq!(router2.resolve_used_update_types().len(), 2);

        router.include(router2);

        let update_types = router.resolve_used_update_types();

        println!("{update_types:?}");

        assert_eq!(update_types.len(), 3);
        assert!(update_types.contains(&UpdateType::Message));
        assert!(update_types.contains(&UpdateType::EditedMessage));
        assert!(update_types.contains(&UpdateType::ChannelPost));

        let update_types = router.resolve_used_update_types_with_skip([UpdateType::Message]);

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(&UpdateType::EditedMessage));
        assert!(update_types.contains(&UpdateType::ChannelPost));
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
