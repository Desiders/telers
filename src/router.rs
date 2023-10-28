//! Router combines all event observers.
//!
//! Each event observer is a special unit that handles a specific event type.
//! There are two types of event observers:
//!
//! * Simple observer:
//! [`Simple observer`] is used to handle simple events like startup and shutdown.
//! When you register a handler in this observer,
//! you specify the arguments that pass to the handler when the event is trigger.
//! Return type of the handler is [`Result<(), HandlerError>`].
//! When observer is trigger, it calls all handlers in order of registration and stops if one of them returns an error.
//!
//! Registration of the handlers looks like this:
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
//! [`Telegram observer`] is used to handle telegram events like messages, callback queries, polls and all other event types.
//! You can register a handler with any arguments that implement [`FromEventAndContext`] trait, see [`extractors module`] for more details.
//! Return type of the handler is [`Result<EventReturn, HandlerError>`],
//! where [`EventReturn`] is a special enum that can be used to control the propagation of the event,
//! see [`EventReturn`] for more details.
//! When observer is trigger, it calls outer middlewares and checks all handlers in order of registration.
//! It calls all filters for each handler and skips the handler if one of them returns `false`.
//! If the handler is pass the filters, observer calls inner middlewares and the handler itself (in the middleware).
//! By default, the first handler that pass the filters stop the propagation of the event, so other handlers aren't calls.
//! (You can change this behaviour by specify another variant of [`EventReturn`]).
//!
//! Registration of the handlers looks like this:
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
//! Order of event propagation when propagate event is called for router:
//! 1) Call outer middlewares of update observer in order of registration;
//! 1.1) If middleware returns [`EventReturn::Finish`], then update [`Request`] because the middleware could have changed it and go to the 1 step;
//! 1.2) If middleware returns [`EventReturn::Skip`], then skip this middleware and go to the 1 step;
//! 1.3) If middleware returns [`EventReturn::Cancel`], then cancel propagation of outer middlewares of update observer and go to the 2 step;
//! 1.4) If all middlewares by step 1.1 are passed or skipped on 1.2 or propagation cancelled by step 1.3, then go to the 2 step;
//! 2) Call filters of handlers of update observer in order of registration;
//! 2.1) If one of the handler filters returns `false`, then skip the handler and go back to the 2 step;
//! 2.2) If any handler filter returns `true`, then go to the 3 step;
//! 2.3) If all handlers are skipped on 2.1, then go to the 4 step;
//! 3) Call inner middlewares of update observer and the handler itself (handler is called in the middleware);
//! 3.1) If the handler or middleware returns [`EventReturn::Skip`], then we should skip it and go to the 3 step;
//! 3.2) If the handler or middleware returns [`EventReturn::Cancel`], then we should stop propagation of innder middlewares of update observer
//! and go to the 4 step;
//! 3.3) If the handler or middleware returns [`EventReturn::Finish`], then we should stop propagation and return a response and go to the 4 step;
//! 3.4) If the handler of middleware returns error, then we should stop propagation and return a response
//! because the error is the correct result from the point of view of observer and go to the 4 step;
//! 3.5) If all handlers or middlewares are skipped on 3.1, then go to the 4 step;
//! 4) Stop propagation of update observer;
//! 5) Check which observer respond to the current [`UpdateType`];
//! 6) Call outer middlewares of the observer in order of registration;
//! 6.1) If middleware returns [`EventReturn::Finish`], then update [`Request`] because the middleware could have changed it and go to the 6 step;
//! 6.2) If middleware returns [`EventReturn::Skip`], then skip this middleware and go to the 6 step;
//! 6.3) If middleware returns [`EventReturn::Cancel`], then cancel event propagation and go to the 10 step;
//! 6.4) If all middlewares by step 1.1 are passed or skipped on 1.2, then go to the 7 step;
//! 7) Call filters of handlers of the observer in order of registration;
//! 7.1) If one of the handler filters returns `false`, then skip the handler and go back to the 7 step;
//! 7.2) If any handler filter returns `true`, then go to the 8 step;
//! 7.3) If all handlers are skipped on 7.1, then cancel propagation of the observer and go to the 9 step;
//! 8) Call inner middlewares of the observer and the handler itself (handler is called in the middleware);
//! 8.1) If the handler or middleware returns [`EventReturn::Skip`], then we should skip it and go to the 8 step;
//! 8.2) If the handler or middleware returns [`EventReturn::Cancel`], then we should stop propagation of the observer
//! and go to the 10 step;
//! 8.3) If the handler or middleware returns [`EventReturn::Finish`], then we should stop propagation and return a response
//! and go to the 10 step;
//! 8.4) If the handler of middleware returns error, then we should stop propagation and return a response
//! because the error is the correct result from the point of view of observer and go to the 10 step;
//! 8.5) If all handlers or middlewares are skipped on 8.1, then go to the 9 step;
//! 9) Check to which router from current router's sub routers propagate event next;
//! 9.1) If there is no sub routers, then go to the 10 step;
//! 9.2) If there is sub routers, then go to the 1 step for the first sub router;
//! 9.2.1) If the propagate event [`PropagateEventResult::Unhandled`] by the sub router's observer, then go to the 9 step;
//! 9.2.2) If the propagate event [`PropagateEventResult::Handled`] by the sub router's observer, then go to the 10 step;
//! 9.2.3) If the propagate event [`PropagateEventResult::Rejected`], then return an unhandled response and go to the 10 step;
//! 10) Finish event propagation.
//!
//! [`Simple observer`]: SimpleObserver
//! [`Telegram observer`]: TelegramObserver
//! [`Dispatcher`]: crate::dispatcher::Dispatcher
//! [`FromEventAndContext`]: crate::extractors::FromEventAndContext
//! [`extractors module`]: crate::extractors
//! [`Router::include_router`]: Router#method.include_router

use crate::{
    client::Bot,
    context::Context,
    enums::{
        observer_name::{Simple as SimpleObserverName, Telegram as TelegramObserverName},
        update_type::UpdateType,
    },
    errors::EventErrorKind,
    event::{
        bases::{EventReturn, PropagateEventResult},
        service::{ServiceProvider, ToServiceProvider},
        simple::{
            handler::Result as SimpleHandlerResult,
            observer::{Observer as SimpleObserver, ObserverService as SimpleObserverService},
        },
        telegram::observer::{
            Observer as TelegramObserver, ObserverService as TelegramObserverService,
            Request as TelegramObserverRequest,
        },
    },
    middlewares::{
        inner::{Logging as LoggingMiddleware, Middleware as InnerMiddleware},
        outer::{Middleware as OuterMiddleware, UserContext as UserContextMiddleware},
    },
    types::Update,
};

use async_trait::async_trait;
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    iter::once,
    sync::Arc,
};
use tracing::{event, instrument, Level};

pub struct Request<Client> {
    pub bot: Arc<Bot<Client>>,
    pub update: Arc<Update>,
    pub context: Arc<Context>,
}

impl<Client> Request<Client> {
    #[must_use]
    pub fn new(
        bot: impl Into<Arc<Bot<Client>>>,
        update: impl Into<Arc<Update>>,
        context: impl Into<Arc<Context>>,
    ) -> Self {
        Self {
            bot: bot.into(),
            update: update.into(),
            context: context.into(),
        }
    }
}

impl<Client> Clone for Request<Client> {
    fn clone(&self) -> Self {
        Self {
            bot: Arc::clone(&self.bot),
            update: Arc::clone(&self.update),
            context: Arc::clone(&self.context),
        }
    }
}

impl<Client> Debug for Request<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("bot", &self.bot)
            .field("update", &self.update)
            .field("context", &self.context)
            .finish()
    }
}

impl<Client> PartialEq for Request<Client> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

impl<Client> From<Request<Client>> for TelegramObserverRequest<Client> {
    fn from(req: Request<Client>) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

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

impl<Client> Debug for Response<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("request", &self.request)
            .field("propagate_result", &self.propagate_result)
            .finish()
    }
}

#[async_trait]
pub trait PropagateEvent<Client>: Send + Sync {
    /// Propagate event
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    async fn propagate_event(
        &self,
        update_type: UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static;

    /// Propagate update event
    /// # Notes
    /// This calls the special event observer that used to handle all telegram events.
    /// It's called for router and its sub routers and before other telegram observers.
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    async fn propagate_update_event(
        &self,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static;

    /// Emit startup events
    /// # Errors
    /// If any startup observer returns error
    async fn emit_startup(&self) -> SimpleHandlerResult;

    /// Emit shutdown events
    /// # Errors
    /// If any shutdown observer returns error
    async fn emit_shutdown(&self) -> SimpleHandlerResult;
}

#[async_trait]
impl<Client, P: ?Sized> PropagateEvent<Client> for Arc<P>
where
    P: PropagateEvent<Client> + Send + Sync,
{
    async fn propagate_event(
        &self,
        update_type: UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
    {
        P::propagate_event(self, update_type, request).await
    }

    async fn propagate_update_event(
        &self,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
    {
        P::propagate_update_event(self, request).await
    }

    async fn emit_startup(&self) -> SimpleHandlerResult {
        P::emit_startup(self).await
    }

    async fn emit_shutdown(&self) -> SimpleHandlerResult {
        P::emit_shutdown(self).await
    }
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
/// you specify the arguments that pass to the handler when the event is trigger. \
/// Return type of the handler is `Result<(), HandlerError>`. \
/// When observer is trigger, it calls all handlers in order of registration and stops if one of them returns an error.
///
/// Registration of the handlers looks like this:
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
/// You can register a handler with any arguments that implement [`crate::extractors::FromEventAndContext`] trait,
/// see [`crate::extractors`] for more details. \
/// Return type of the handler is `Result<EventReturn, HandlerError>`,
/// where [`EventReturn`] is a special enum that can be used to control the propagation of the event,
/// see [`EventReturn`] for more details. \
/// When observer is trigger, it calls outer middlewares and checks all handlers in order of registration.
/// It calls all filters for each handler and skips the handler if one of them returns `false`.
/// If the handler is pass the filters, observer calls inner middlewares and the handler itself (in the middleware).
/// By default, the first handler that pass the filters stop the propagation of the event, so other handlers aren't calls.
/// (You can change this behaviour by specify another variant of [`EventReturn`]).
///
/// Registration of the handlers looks like this:
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
pub struct Router<Client> {
    router_name: &'static str,
    sub_routers: Vec<Router<Client>>,

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
            update: TelegramObserver::new(TelegramObserverName::Update.as_str()),
            startup: SimpleObserver::new(SimpleObserverName::Startup.as_str()),
            shutdown: SimpleObserver::new(SimpleObserverName::Shutdown.as_str()),
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
    pub const fn telegram_observers(&self) -> [&TelegramObserver<Client>; 15] {
        [
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
            &self.update,
        ]
    }

    /// Get all telegram event observers as mutable references
    /// # Notes
    /// This method is useful for registering middlewares to the many observers without code duplication and macros
    #[must_use]
    pub fn telegram_observers_mut(&mut self) -> Vec<&mut TelegramObserver<Client>> {
        let mut observers = Vec::with_capacity(15);

        observers.extend([
            &mut self.message,
            &mut self.edited_message,
            &mut self.channel_post,
            &mut self.edited_channel_post,
            &mut self.inline_query,
            &mut self.chosen_inline_result,
            &mut self.callback_query,
            &mut self.shipping_query,
            &mut self.pre_checkout_query,
            &mut self.poll,
            &mut self.poll_answer,
            &mut self.my_chat_member,
            &mut self.chat_member,
            &mut self.chat_join_request,
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

            #[allow(clippy::expect_fun_call)]
            let update_type = <&str as TryInto<UpdateType>>::try_into(observer.event_name).expect(
                format!(
                    "Can't convert event name to UpdateType. This is a bug. Please, report it. Event name: {}",
                    observer.event_name
                )
                .as_str(),
            );

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

impl<Client> Debug for Router<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .field("sub_routers", &self.sub_routers)
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

impl<Client> ToServiceProvider for Router<Client>
where
    Client: Send + Sync + 'static,
{
    type Config = Config<Client>;
    type ServiceProvider = RouterService<Client>;
    type InitError = ();

    fn to_service_provider(
        mut self,
        mut config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        macro_rules! register_inner_middlewares_to_sub_routers {
            ($($observers:ident),+) => {
                $(
                    self.sub_routers.iter_mut().for_each(|sub_router| {
                        let mut index = 0;
                        for middleware in &self.$observers.inner_middlewares.middlewares {
                            sub_router.$observers.inner_middlewares.register_at_position(index, Arc::clone(middleware));
                            index += 1;
                        }
                    });
                )+
            };
        }

        register_inner_middlewares_to_sub_routers!(
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
            update
        );

        macro_rules! register_middlewares_from_config {
            ($($observer:ident),+) => {
                $(
                    let mut index = 0;
                    for middleware in config.outer_middlewares.$observer.iter() {
                        self.$observer.outer_middlewares.register_at_position(index, Arc::clone(middleware));
                        index += 1;
                    }
                )+

                $(
                    let mut index = 0;
                    for middleware in config.inner_middlewares.$observer.iter() {
                        self.$observer.inner_middlewares.register_at_position(index, Arc::clone(middleware));
                        index += 1;
                    }
                )+
            };
        }

        register_middlewares_from_config!(
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
            update
        );

        // We don't need to register config outer middlewares to sub routers
        config.outer_middlewares = OuterMiddlewaresConfig::new();

        Ok(RouterService {
            router_name: self.router_name,
            sub_routers: self
                .sub_routers
                .into_iter()
                .map(|router| router.to_service_provider(config.clone()))
                .collect::<Result<_, _>>()?,
            message: self.message.to_service_provider_default()?,
            edited_message: self.edited_message.to_service_provider_default()?,
            channel_post: self.channel_post.to_service_provider_default()?,
            edited_channel_post: self.edited_channel_post.to_service_provider_default()?,
            inline_query: self.inline_query.to_service_provider_default()?,
            chosen_inline_result: self.chosen_inline_result.to_service_provider_default()?,
            callback_query: self.callback_query.to_service_provider_default()?,
            shipping_query: self.shipping_query.to_service_provider_default()?,
            pre_checkout_query: self.pre_checkout_query.to_service_provider_default()?,
            poll: self.poll.to_service_provider_default()?,
            poll_answer: self.poll_answer.to_service_provider_default()?,
            my_chat_member: self.my_chat_member.to_service_provider_default()?,
            chat_member: self.chat_member.to_service_provider_default()?,
            chat_join_request: self.chat_join_request.to_service_provider_default()?,
            update: self.update.to_service_provider_default()?,
            startup: self.startup.to_service_provider_default()?,
            shutdown: self.shutdown.to_service_provider_default()?,
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct RouterService<Client> {
    router_name: &'static str,
    sub_routers: Box<[RouterService<Client>]>,

    message: TelegramObserverService<Client>,
    edited_message: TelegramObserverService<Client>,
    channel_post: TelegramObserverService<Client>,
    edited_channel_post: TelegramObserverService<Client>,
    inline_query: TelegramObserverService<Client>,
    chosen_inline_result: TelegramObserverService<Client>,
    callback_query: TelegramObserverService<Client>,
    shipping_query: TelegramObserverService<Client>,
    pre_checkout_query: TelegramObserverService<Client>,
    poll: TelegramObserverService<Client>,
    poll_answer: TelegramObserverService<Client>,
    my_chat_member: TelegramObserverService<Client>,
    chat_member: TelegramObserverService<Client>,
    chat_join_request: TelegramObserverService<Client>,
    update: TelegramObserverService<Client>,

    startup: SimpleObserverService,
    shutdown: SimpleObserverService,
}

impl<Client> ServiceProvider for RouterService<Client> {}

#[async_trait]
impl<Client> PropagateEvent<Client> for RouterService<Client> {
    #[instrument(skip(self, update_type, request), fields(router_name = self.router_name))]
    async fn propagate_event(
        &self,
        update_type: UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
    {
        self.propagate_update_event(request.clone()).await?;

        event!(Level::TRACE, "Propagate event to router");

        let observer = self.telegram_observer_by_update_type(update_type);

        let mut request = request;
        for middleware in observer.outer_middlewares().iter() {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;

            match event_return {
                // If middleware returns finish, then update request because the middleware could have changed it
                EventReturn::Finish => {
                    event!(Level::TRACE, "Middleware returns finish");

                    request = updated_request;
                }
                // If middleware returns skip, then we should skip this middleware and its changes
                EventReturn::Skip => {
                    event!(Level::TRACE, "Middleware returns skip");

                    continue;
                }
                // If middleware returns cancel, then we should cancel propagation
                EventReturn::Cancel => {
                    event!(Level::TRACE, "Middleware returns cancel");

                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    });
                }
            }
        }

        let observer_request = request.clone().into();
        let observer_response = observer.trigger(observer_request).await?;

        match observer_response.propagate_result {
            // If observer returns unhandled, then propagate event to sub routers' observers
            PropagateEventResult::Unhandled => {
                event!(Level::TRACE, "Event unhandled by router");
            }
            // If observer returns handled, then return a response
            PropagateEventResult::Handled(response) => {
                event!(Level::TRACE, "Event handled by router");

                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                });
            }
            // If observer returns rejected, then return a response.
            // Router don't know about rejected event by observer, so it returns unhandled response.
            PropagateEventResult::Rejected => {
                event!(Level::TRACE, "Event rejected by router");

                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                });
            }
        };

        // Propagate event to sub routers' observers
        for router in self.sub_routers.iter() {
            let router_response = router.propagate_event(update_type, request.clone()).await?;
            match router_response.propagate_result {
                // If the event unhandled by the sub router's observer, then propagate event to next sub router's observer
                PropagateEventResult::Unhandled => {
                    event!(Level::TRACE, "Event unhandled by syb router");

                    continue;
                }
                // If the event handled by the sub router's observer, then return a response
                PropagateEventResult::Handled(_) => {
                    event!(Level::TRACE, "Event handled by sub router");

                    return Ok(router_response);
                }
                // If the event rejected by the sub router's observer, then return a response.
                PropagateEventResult::Rejected => {
                    event!(Level::TRACE, "Event rejected by sub router");

                    return Ok(router_response);
                }
            };
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request,
            propagate_result: PropagateEventResult::Unhandled,
        })
    }

    #[instrument(skip(self, request), fields(router_name = self.router_name))]
    async fn propagate_update_event(
        &self,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
    {
        event!(Level::TRACE, "Propagate update event to router");

        let mut request = request;
        for middleware in self.update.outer_middlewares().iter() {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;

            match event_return {
                // If middleware returns finish, then update request because the middleware could have changed it
                EventReturn::Finish => {
                    event!(Level::TRACE, "Middleware returns finish");

                    request = updated_request;
                }
                // If middleware returns skip, then we should skip this middleware and its changes
                EventReturn::Skip => {
                    event!(Level::TRACE, "Middleware returns skip");

                    continue;
                }
                // If middleware returns cancel, then we should cancel propagation
                EventReturn::Cancel => {
                    event!(Level::TRACE, "Middleware returns cancel");

                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    });
                }
            }
        }

        let observer_request = request.clone().into();
        let observer_response = self.update.trigger(observer_request).await?;

        match observer_response.propagate_result {
            // If observer returns unhandled, then return an unhandled response
            PropagateEventResult::Unhandled => {
                event!(Level::TRACE, "Update event unhandled by router");

                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                })
            }
            // If observer returns handled, then return a handled response
            PropagateEventResult::Handled(response) => {
                event!(Level::TRACE, "Update event handled by router");

                Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                })
            }
            // If observer returns rejected, then return an unhandled response.
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

    #[instrument(skip(self), fields(router_name = self.router_name))]
    async fn emit_startup(&self) -> SimpleHandlerResult {
        for startup in
            once(&self.startup).chain(self.sub_routers.iter().map(|router| &router.startup))
        {
            startup.trigger(()).await?;
        }
        Ok(())
    }

    #[instrument(skip(self), fields(router_name = self.router_name))]
    async fn emit_shutdown(&self) -> SimpleHandlerResult {
        for shutdown in
            once(&self.shutdown).chain(self.sub_routers.iter().map(|router| &router.shutdown))
        {
            shutdown.trigger(()).await?;
        }
        Ok(())
    }
}

impl<Client> RouterService<Client> {
    #[must_use]
    pub const fn telegram_observers(&self) -> [&TelegramObserverService<Client>; 15] {
        [
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
            &self.update,
        ]
    }

    #[must_use]
    pub const fn event_observers(&self) -> [&SimpleObserverService; 2] {
        [&self.startup, &self.shutdown]
    }

    #[must_use]
    pub const fn telegram_observer_by_update_type(
        &self,
        update_type: UpdateType,
    ) -> &TelegramObserverService<Client> {
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
}

impl<Client> Debug for RouterService<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .field("sub_routers", &self.sub_routers)
            .finish()
    }
}

pub struct Config<Client> {
    outer_middlewares: OuterMiddlewaresConfig<Client>,
    inner_middlewares: InnerMiddlewaresConfig<Client>,
}

impl<Client> Config<Client> {
    #[must_use]
    pub fn new(
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

pub struct OuterMiddlewaresConfig<Client> {
    pub message: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub edited_message: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub channel_post: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub edited_channel_post: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub inline_query: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub chosen_inline_result: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub callback_query: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub shipping_query: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub pre_checkout_query: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub poll: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub poll_answer: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub my_chat_member: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub chat_member: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub chat_join_request: Box<[Arc<dyn OuterMiddleware<Client>>]>,
    pub update: Box<[Arc<dyn OuterMiddleware<Client>>]>,
}

impl<Client> OuterMiddlewaresConfig<Client> {
    #[must_use]
    pub fn new() -> Self {
        Self::builder().build()
    }

    #[must_use]
    pub fn builder() -> OuterMiddlewaresConfigBuilder<Client> {
        OuterMiddlewaresConfigBuilder::default()
    }
}

impl<Client> Default for OuterMiddlewaresConfig<Client>
where
    Client: Send + Sync + 'static,
{
    fn default() -> Self {
        Self::builder().update(UserContextMiddleware).build()
    }
}

impl<Client> Clone for OuterMiddlewaresConfig<Client> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            edited_message: self.edited_message.clone(),
            channel_post: self.channel_post.clone(),
            edited_channel_post: self.edited_channel_post.clone(),
            inline_query: self.inline_query.clone(),
            chosen_inline_result: self.chosen_inline_result.clone(),
            callback_query: self.callback_query.clone(),
            shipping_query: self.shipping_query.clone(),
            pre_checkout_query: self.pre_checkout_query.clone(),
            poll: self.poll.clone(),
            poll_answer: self.poll_answer.clone(),
            my_chat_member: self.my_chat_member.clone(),
            chat_member: self.chat_member.clone(),
            chat_join_request: self.chat_join_request.clone(),
            update: self.update.clone(),
        }
    }
}

pub struct OuterMiddlewaresConfigBuilder<Client> {
    pub message: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub edited_message: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub channel_post: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub edited_channel_post: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub inline_query: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub chosen_inline_result: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub callback_query: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub shipping_query: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub pre_checkout_query: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub poll: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub poll_answer: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub my_chat_member: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub chat_member: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub chat_join_request: Vec<Arc<dyn OuterMiddleware<Client>>>,
    pub update: Vec<Arc<dyn OuterMiddleware<Client>>>,
}

impl<Client> OuterMiddlewaresConfigBuilder<Client> {
    #[must_use]
    pub fn message(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.message.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn edited_message(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.edited_message.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn channel_post(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.channel_post.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn edited_channel_post(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.edited_channel_post.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn inline_query(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.inline_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn chosen_inline_result(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.chosen_inline_result.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn callback_query(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.callback_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn shipping_query(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.shipping_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn pre_checkout_query(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.pre_checkout_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn poll(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.poll.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn poll_answer(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.poll_answer.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn my_chat_member(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.my_chat_member.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn chat_member(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.chat_member.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn chat_join_request(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.chat_join_request.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn update(mut self, val: impl OuterMiddleware<Client> + 'static) -> Self {
        self.update.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn build(self) -> OuterMiddlewaresConfig<Client> {
        OuterMiddlewaresConfig {
            message: self.message.into(),
            edited_message: self.edited_message.into(),
            channel_post: self.channel_post.into(),
            edited_channel_post: self.edited_channel_post.into(),
            inline_query: self.inline_query.into(),
            chosen_inline_result: self.chosen_inline_result.into(),
            callback_query: self.callback_query.into(),
            shipping_query: self.shipping_query.into(),
            pre_checkout_query: self.pre_checkout_query.into(),
            poll: self.poll.into(),
            poll_answer: self.poll_answer.into(),
            my_chat_member: self.my_chat_member.into(),
            chat_member: self.chat_member.into(),
            chat_join_request: self.chat_join_request.into(),
            update: self.update.into(),
        }
    }
}

impl<Client> Default for OuterMiddlewaresConfigBuilder<Client> {
    #[must_use]
    fn default() -> Self {
        Self {
            message: vec![],
            edited_message: vec![],
            channel_post: vec![],
            edited_channel_post: vec![],
            inline_query: vec![],
            chosen_inline_result: vec![],
            callback_query: vec![],
            shipping_query: vec![],
            pre_checkout_query: vec![],
            poll: vec![],
            poll_answer: vec![],
            my_chat_member: vec![],
            chat_member: vec![],
            chat_join_request: vec![],
            update: vec![],
        }
    }
}

pub struct InnerMiddlewaresConfig<Client> {
    pub message: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub edited_message: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub channel_post: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub edited_channel_post: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub inline_query: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub chosen_inline_result: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub callback_query: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub shipping_query: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub pre_checkout_query: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub poll: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub poll_answer: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub my_chat_member: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub chat_member: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub chat_join_request: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    pub update: Box<[Arc<dyn InnerMiddleware<Client>>]>,
}

impl<Client> InnerMiddlewaresConfig<Client> {
    #[must_use]
    pub fn new() -> Self {
        Self::builder().build()
    }

    #[must_use]
    pub fn builder() -> InnerMiddlewaresConfigBuilder<Client> {
        InnerMiddlewaresConfigBuilder::default()
    }
}

impl<Client> Default for InnerMiddlewaresConfig<Client>
where
    Client: Send + Sync + 'static,
{
    fn default() -> Self {
        let logging_middleware = Arc::new(LoggingMiddleware);

        Self::builder()
            .message(logging_middleware.clone())
            .edited_message(logging_middleware.clone())
            .channel_post(logging_middleware.clone())
            .edited_channel_post(logging_middleware.clone())
            .inline_query(logging_middleware.clone())
            .chosen_inline_result(logging_middleware.clone())
            .callback_query(logging_middleware.clone())
            .shipping_query(logging_middleware.clone())
            .pre_checkout_query(logging_middleware.clone())
            .poll(logging_middleware.clone())
            .poll_answer(logging_middleware.clone())
            .my_chat_member(logging_middleware.clone())
            .chat_member(logging_middleware.clone())
            .chat_join_request(logging_middleware.clone())
            .update(logging_middleware)
            .build()
    }
}

impl<Client> Clone for InnerMiddlewaresConfig<Client> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            edited_message: self.edited_message.clone(),
            channel_post: self.channel_post.clone(),
            edited_channel_post: self.edited_channel_post.clone(),
            inline_query: self.inline_query.clone(),
            chosen_inline_result: self.chosen_inline_result.clone(),
            callback_query: self.callback_query.clone(),
            shipping_query: self.shipping_query.clone(),
            pre_checkout_query: self.pre_checkout_query.clone(),
            poll: self.poll.clone(),
            poll_answer: self.poll_answer.clone(),
            my_chat_member: self.my_chat_member.clone(),
            chat_member: self.chat_member.clone(),
            chat_join_request: self.chat_join_request.clone(),
            update: self.update.clone(),
        }
    }
}

pub struct InnerMiddlewaresConfigBuilder<Client> {
    pub message: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub edited_message: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub channel_post: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub edited_channel_post: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub inline_query: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub chosen_inline_result: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub callback_query: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub shipping_query: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub pre_checkout_query: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub poll: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub poll_answer: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub my_chat_member: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub chat_member: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub chat_join_request: Vec<Arc<dyn InnerMiddleware<Client>>>,
    pub update: Vec<Arc<dyn InnerMiddleware<Client>>>,
}

impl<Client> InnerMiddlewaresConfigBuilder<Client> {
    #[must_use]
    pub fn message(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.message.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn edited_message(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.edited_message.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn channel_post(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.channel_post.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn edited_channel_post(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.edited_channel_post.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn inline_query(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.inline_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn chosen_inline_result(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.chosen_inline_result.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn callback_query(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.callback_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn shipping_query(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.shipping_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn pre_checkout_query(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.pre_checkout_query.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn poll(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.poll.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn poll_answer(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.poll_answer.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn my_chat_member(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.my_chat_member.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn chat_member(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.chat_member.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn chat_join_request(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.chat_join_request.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn update(mut self, val: impl InnerMiddleware<Client> + 'static) -> Self {
        self.update.push(Arc::new(val));
        self
    }

    #[must_use]
    pub fn build(self) -> InnerMiddlewaresConfig<Client> {
        InnerMiddlewaresConfig {
            message: self.message.into(),
            edited_message: self.edited_message.into(),
            channel_post: self.channel_post.into(),
            edited_channel_post: self.edited_channel_post.into(),
            inline_query: self.inline_query.into(),
            chosen_inline_result: self.chosen_inline_result.into(),
            callback_query: self.callback_query.into(),
            shipping_query: self.shipping_query.into(),
            pre_checkout_query: self.pre_checkout_query.into(),
            poll: self.poll.into(),
            poll_answer: self.poll_answer.into(),
            my_chat_member: self.my_chat_member.into(),
            chat_member: self.chat_member.into(),
            chat_join_request: self.chat_join_request.into(),
            update: self.update.into(),
        }
    }
}

impl<Client> Default for InnerMiddlewaresConfigBuilder<Client> {
    #[must_use]
    fn default() -> Self {
        Self {
            message: vec![],
            edited_message: vec![],
            channel_post: vec![],
            edited_channel_post: vec![],
            inline_query: vec![],
            chosen_inline_result: vec![],
            callback_query: vec![],
            shipping_query: vec![],
            pre_checkout_query: vec![],
            poll: vec![],
            poll_answer: vec![],
            my_chat_member: vec![],
            chat_member: vec![],
            chat_join_request: vec![],
            update: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        event::{telegram::HandlerResult as TelegramHandlerResult, EventReturn},
        middlewares::inner::Next,
    };

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

        let router_service = router
            .to_service_provider(Config::new(
                OuterMiddlewaresConfig::new(),
                InnerMiddlewaresConfig::new(),
            ))
            .unwrap();

        assert_eq!(router_service.sub_routers.len(), 3);
        assert_eq!(router_service.router_name, "main");

        let message_observer_name = UpdateType::Message.as_str();

        router_service
            .sub_routers
            .into_iter()
            .for_each(|router_service| {
                assert_eq!(router_service.sub_routers.len(), 2);

                router_service
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

                router_service
                    .sub_routers
                    .into_iter()
                    .for_each(|router_service| {
                        assert_eq!(router_service.sub_routers.len(), 0);

                        router_service
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
        router.inline_query.register(telegram_handler);
        router.chosen_inline_result.register(telegram_handler);
        router.callback_query.register(telegram_handler);
        router.shipping_query.register(telegram_handler);
        router.pre_checkout_query.register(telegram_handler);
        router.poll.register(telegram_handler);
        router.poll_answer.register(telegram_handler);
        router.my_chat_member.register(telegram_handler);
        router.chat_member.register(telegram_handler);
        router.chat_join_request.register(telegram_handler);
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

        let inner_middleware = |request, next: Next<_>| next(request);
        let outer_middleware = |request| async move { Ok((request, EventReturn::Finish)) };

        router.message.inner_middlewares.register(inner_middleware);
        router.message.outer_middlewares.register(outer_middleware);

        assert_eq!(router.message.inner_middlewares.middlewares.len(), 1);
        assert_eq!(router.message.outer_middlewares.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_propagate_event() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        let request = Request::new(bot, update, context);

        let mut router = Router::new("test_handler");
        router
            .message
            .register(|| async move { Ok(EventReturn::Finish) });

        let router_service = router.to_service_provider_default().unwrap();
        let response = router_service
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

        let response = router_service
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
            .register(|request: Request<Reqwest>| async move {
                request.context.insert("test", Box::new("test"));

                Ok((request, EventReturn::Finish))
            });
        router.message.register(|context: Arc<Context>| async move {
            // Check that middleware was called and context was modified
            assert_eq!(
                context.get("test").unwrap().downcast_ref::<&str>().unwrap(),
                &"test"
            );

            Ok(EventReturn::Finish)
        });

        let router_service = router.to_service_provider_default().unwrap();

        let response = router_service
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
            .register(|| async move { Ok(EventReturn::Skip) });
        router
            .message
            .register(|| async move { Ok(EventReturn::Finish) });

        let router_service = router.to_service_provider_default().unwrap();

        let response = router_service
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
            .register(|| async move { Ok(EventReturn::Skip) });

        let router_service = router.to_service_provider_default().unwrap();

        let response = router_service
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
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        let request = Request::new(bot, update, context);

        let mut router = Router::new("test_handler_with_filter");
        router
            .message
            .register(|| async move { Ok(EventReturn::Finish) })
            .filter(|_: &Bot<_>, _: &Update, _: &Context| async move { true });

        let router_service = router.to_service_provider_default().unwrap();
        let response = router_service
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
            .register(|| async move { Ok(EventReturn::Finish) })
            .filter(|_: &Bot<_>, _: &Update, _: &Context| async move { false });

        let router_service = router.to_service_provider_default().unwrap();
        let response = router_service
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
            .register(|| async move { Ok(EventReturn::Finish) })
            .filter(|_: &Bot<_>, _: &Update, _: &Context| async move { true })
            .filter(|_: &Bot<_>, _: &Update, _: &Context| async move { true })
            .filter(|_: &Bot<_>, _: &Update, _: &Context| async move { false });

        let router_service = router.to_service_provider_default().unwrap();
        let response = router_service
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
            .register(|| async { Ok(EventReturn::Finish) });
        router
            .edited_message
            .register(|| async { Ok(EventReturn::Finish) });

        let update_types = router.resolve_used_update_types();

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(&UpdateType::Message));
        assert!(update_types.contains(&UpdateType::EditedMessage));

        let mut router2 = Router::<Reqwest>::new("test2");

        router2
            .message
            .register(|| async { Ok(EventReturn::Finish) });
        router2
            .channel_post
            .register(|| async { Ok(EventReturn::Finish) });

        assert_eq!(router2.resolve_used_update_types().len(), 2);

        router.include(router2);

        let update_types = router.resolve_used_update_types();

        println!("{:?}", update_types);

        assert_eq!(update_types.len(), 3);
        assert!(update_types.contains(&UpdateType::Message));
        assert!(update_types.contains(&UpdateType::EditedMessage));
        assert!(update_types.contains(&UpdateType::ChannelPost));

        let update_types = router.resolve_used_update_types_with_skip([UpdateType::Message]);

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(&UpdateType::EditedMessage));
        assert!(update_types.contains(&UpdateType::ChannelPost));
    }
}
