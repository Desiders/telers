use crate::{
    client::Bot,
    context::Context,
    enums::{
        observer_name::{Simple as SimpleObserverName, Telegram as TelegramObserverName},
        update_type::UpdateType,
    },
    error::EventErrorKind,
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
        inner::{
            Logging as LoggingMiddleware, Middleware as InnerMiddleware,
            Middlewares as InnerMiddlewares,
        },
        outer::{
            Middleware as OuterMiddleware, Middlewares as OuterMiddlewares,
            UserContext as UserContextMiddleware,
        },
    },
    types::Update,
};

use async_trait::async_trait;
use log;
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    iter::once,
    sync::Arc,
};

#[derive(Debug)]
pub struct Request<Client> {
    pub bot: Arc<Bot<Client>>,
    pub update: Arc<Update>,
    pub context: Arc<Context>,
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

impl<Client> Clone for Request<Client> {
    fn clone(&self) -> Self {
        Self {
            bot: Arc::clone(&self.bot),
            update: Arc::clone(&self.update),
            context: Arc::clone(&self.context),
        }
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
/// You can register a handler with any arguments that implement [`crate::extract::FromEventAndContext`] trait,
/// see [`crate::extract`] for more details. \
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
    ///
    /// The order of calls looks simplistically like this: \
    /// `Dispatcher -> Router -> Update observer -> Sub router -> Update observer
    ///            -> Router -> Telegram observer -> Sub router -> Telegram observer`
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
    /// # Warning
    /// You shouldn't count on the fact that the middlewares of this router will be registered to the sub routers
    /// immediately after calling this method. This implementation detail that can be changed in the future.
    /// Right now, middlewares registers to the sub routers when the router is converts to the [`RouterService`]
    /// by calls `Router::to_service_provider` method.
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
    /// # Warning
    /// You shouldn't count on the fact that the middlewares of this router will be registered to the sub routers
    /// immediately after calling this method. This implementation detail that can be changed in the future.
    /// Right now, middlewares registers to the sub routers when the router is converts to the [`RouterService`]
    /// by calls `Router::to_service_provider` method.
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

    /// Resolve registered update types from the current router and its sub routers.
    /// It is useful for getting updates only for registered update types.
    /// # Warning
    /// This method doesn't preserve order registration of update types
    /// # Panics
    /// If can't convert observer event name to [`UpdateType`]
    #[must_use]
    pub fn resolve_used_update_types(&self) -> Vec<UpdateType> {
        let mut used_update_types = HashSet::new();

        self.sub_routers.iter().for_each(|router| {
            used_update_types.extend(router.resolve_used_update_types());
        });

        used_update_types.extend(
            self.telegram_observers()
                .iter()
                .filter(|observer| !observer.handlers.is_empty())
                .map(|observer| {
                    <&str as TryInto<UpdateType>>::try_into(observer.event_name).expect(
                        "Can't convert event name to UpdateType. This is a bug. Please, report it.",
                    )
                }),
        );

        used_update_types.into_iter().collect()
    }

    /// Resolve registered update types from the current router and its sub routers with skip updates types.
    /// It is useful for getting updates only for registered update types.
    /// # Arguments
    /// * `skip_updates` - Skip update types
    /// # Warning
    /// This method doesn't preserve order registration of update types
    #[must_use]
    pub fn resolve_used_update_types_with_skip(
        &self,
        skip_updates: impl IntoIterator<Item = UpdateType>,
    ) -> Vec<UpdateType> {
        let skip_updates = skip_updates.into_iter().collect::<HashSet<_>>();

        self.resolve_used_update_types()
            .into_iter()
            .filter(|update_type| !skip_updates.contains(update_type))
            .collect()
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
                    for middleware in &config.outer_middlewares.$observer {
                        self.$observer.outer_middlewares.register_at_position(index, Arc::clone(middleware));
                        index += 1;
                    }
                )+

                $(
                    let mut index = 0;
                    for middleware in &config.inner_middlewares.$observer {
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

        // Clear outer middlewares from the config, because they don't need for sub routers
        config.outer_middlewares.clear();

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
    sub_routers: Vec<RouterService<Client>>,

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
    async fn propagate_event(
        &self,
        update_type: UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
    {
        self.propagate_update_event(request.clone()).await?;

        let observer = self.telegram_observer_by_update_type(update_type);

        let mut request = request;
        for middleware in &observer.outer_middlewares {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;

            match event_return {
                // Update request because the middleware could have changed it
                EventReturn::Finish => request = updated_request,
                // If middleware returns skip, then we should skip this middleware and its changes
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

    async fn propagate_update_event(
        &self,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
    {
        let mut request = request;
        for middleware in &self.update.outer_middlewares {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;

            match event_return {
                // Update request because the middleware could have changed it
                EventReturn::Finish => request = updated_request,
                // If middleware returns skip, then we should skip this middleware and its changes
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

        let observer_request = request.clone().into();
        let observer_response = self.update.trigger(observer_request).await?;

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
            let router_response = router.propagate_update_event(request.clone()).await?;
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

    async fn emit_startup(&self) -> SimpleHandlerResult {
        log::debug!("{self:?}: Emit startup");

        for startup in
            once(&self.startup).chain(self.sub_routers.iter().map(|router| &router.startup))
        {
            startup.trigger(()).await?;
        }
        Ok(())
    }

    async fn emit_shutdown(&self) -> SimpleHandlerResult {
        log::debug!("{self:?}: Emit shutdown");

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

impl<Client> Clone for Config<Client> {
    fn clone(&self) -> Self {
        Self {
            outer_middlewares: self.outer_middlewares.clone(),
            inner_middlewares: self.inner_middlewares.clone(),
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

impl<Client> Default for OuterMiddlewaresConfig<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self::builder().update(UserContextMiddleware::new()).build()
    }
}

impl<Client> Default for InnerMiddlewaresConfig<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self::builder()
            .message(LoggingMiddleware::default())
            .edited_message(LoggingMiddleware::default())
            .channel_post(LoggingMiddleware::default())
            .edited_channel_post(LoggingMiddleware::default())
            .inline_query(LoggingMiddleware::default())
            .chosen_inline_result(LoggingMiddleware::default())
            .callback_query(LoggingMiddleware::default())
            .shipping_query(LoggingMiddleware::default())
            .pre_checkout_query(LoggingMiddleware::default())
            .poll(LoggingMiddleware::default())
            .poll_answer(LoggingMiddleware::default())
            .my_chat_member(LoggingMiddleware::default())
            .chat_member(LoggingMiddleware::default())
            .chat_join_request(LoggingMiddleware::default())
            .update(LoggingMiddleware::default())
            .build()
    }
}

macro_rules! create_middleware_config_struct {
    ($name:ident, $type_middlewares:ident) => {
        pub struct $name<Client> {
            message: $type_middlewares<Client>,
            edited_message: $type_middlewares<Client>,
            channel_post: $type_middlewares<Client>,
            edited_channel_post: $type_middlewares<Client>,
            inline_query: $type_middlewares<Client>,
            chosen_inline_result: $type_middlewares<Client>,
            callback_query: $type_middlewares<Client>,
            shipping_query: $type_middlewares<Client>,
            pre_checkout_query: $type_middlewares<Client>,
            poll: $type_middlewares<Client>,
            poll_answer: $type_middlewares<Client>,
            my_chat_member: $type_middlewares<Client>,
            chat_member: $type_middlewares<Client>,
            chat_join_request: $type_middlewares<Client>,
            update: $type_middlewares<Client>,
        }
    };
}

create_middleware_config_struct!(InnerMiddlewaresConfig, InnerMiddlewares);
create_middleware_config_struct!(InnerMiddlewaresConfigBuilder, InnerMiddlewares);

create_middleware_config_struct!(OuterMiddlewaresConfig, OuterMiddlewares);
create_middleware_config_struct!(OuterMiddlewaresConfigBuilder, OuterMiddlewares);

macro_rules! impl_middleware_config_base_methods {
    ($name:ident, $name_builder:ident, $type_middlewares:ident) => {
        impl<Client> $name<Client>
        where
            Client: Send + Sync + 'static,
        {
            #[must_use]
            pub fn new() -> Self {
                Self::builder().build()
            }
        }

        impl<Client> $name<Client> {
            /// Get all observers middlewares as references
            #[must_use]
            pub const fn observers_middlewares(&self) -> [&$type_middlewares<Client>; 15] {
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

            /// Get all observers middlewares as mutable references
            /// # Notes
            /// This method is useful for registering middlewares to the many observers without code duplication and macros
            #[must_use]
            pub fn observers_middlewares_mut(&mut self) -> Vec<&mut $type_middlewares<Client>> {
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

            pub fn clear(&mut self) {
                self.observers_middlewares_mut()
                    .iter_mut()
                    .for_each(|middlewares| middlewares.clear());
            }

            #[must_use]
            pub fn builder() -> $name_builder<Client> {
                $name_builder::default()
            }
        }

        impl<Client> Clone for $name<Client> {
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
    };
}

impl_middleware_config_base_methods!(
    InnerMiddlewaresConfig,
    InnerMiddlewaresConfigBuilder,
    InnerMiddlewares
);
impl_middleware_config_base_methods!(
    OuterMiddlewaresConfig,
    OuterMiddlewaresConfigBuilder,
    OuterMiddlewares
);

macro_rules! impl_builder_methods {
    ($type_middleware:ident, $($method:ident => $update_type:ident),*) => {
        $(
            #[must_use]
            pub fn $method<T>(self, val: T) -> Self
            where
                T: $type_middleware<Client> + 'static,
            {
                Self {
                    $update_type: self.$update_type.into_iter().chain(Some(Arc::new(val) as _)).collect(),
                    ..self
                }
            }
        )*
    };
}

macro_rules! impl_middleware_config_builder_base_methods {
    ($name:ident, $name_builder:ident, $type_middleware:ident) => {
        impl<Client> $name_builder<Client>
        where
            Client: Send + Sync + 'static,
        {
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            impl_builder_methods! {
                $type_middleware,
                message => message,
                edited_message => edited_message,
                channel_post => channel_post,
                edited_channel_post => edited_channel_post,
                inline_query => inline_query,
                chosen_inline_result => chosen_inline_result,
                callback_query => callback_query,
                shipping_query => shipping_query,
                pre_checkout_query => pre_checkout_query,
                poll => poll,
                poll_answer => poll_answer,
                my_chat_member => my_chat_member,
                chat_member => chat_member,
                chat_join_request => chat_join_request,
                update => update
            }

            #[must_use]
            pub fn build(self) -> $name<Client> {
                $name {
                    message: self.message,
                    edited_message: self.edited_message,
                    channel_post: self.channel_post,
                    edited_channel_post: self.edited_channel_post,
                    inline_query: self.inline_query,
                    chosen_inline_result: self.chosen_inline_result,
                    callback_query: self.callback_query,
                    shipping_query: self.shipping_query,
                    pre_checkout_query: self.pre_checkout_query,
                    poll: self.poll,
                    poll_answer: self.poll_answer,
                    my_chat_member: self.my_chat_member,
                    chat_member: self.chat_member,
                    chat_join_request: self.chat_join_request,
                    update: self.update,
                }
            }
        }

        impl<Client> Default for $name_builder<Client> {
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
    };
}

impl_middleware_config_builder_base_methods!(
    InnerMiddlewaresConfig,
    InnerMiddlewaresConfigBuilder,
    InnerMiddleware
);
impl_middleware_config_builder_base_methods!(
    OuterMiddlewaresConfig,
    OuterMiddlewaresConfigBuilder,
    OuterMiddleware
);

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
                            assert_eq!(observer.inner_middlewares.len(), 1);
                        } else {
                            assert_eq!(observer.inner_middlewares.len(), 0);
                        }
                        // Router outer middlewares don't clone to children routers
                        assert_eq!(observer.outer_middlewares.len(), 0);
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
                                    assert_eq!(observer.inner_middlewares.len(), 1);
                                } else {
                                    assert_eq!(observer.inner_middlewares.len(), 0);
                                }
                                // Router outer middlewares don't clone to children routers
                                assert_eq!(observer.outer_middlewares.len(), 0);
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
                assert_eq!(observer.handlers.len(), 1);

                observer.handlers.iter().for_each(|handler| {
                    assert!(handler.filters.is_empty());
                });
            });

        // Check event observers
        router.event_observers().into_iter().for_each(|observer| {
            assert_eq!(observer.handlers.len(), 1);
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
