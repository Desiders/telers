use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::{
        bases::EventReturn,
        service::{factory, fn_service, BoxService, BoxServiceFactory, Service, ServiceFactory},
    },
    error::app,
    extract::FromEventAndContext,
    filters::Filter,
    types::Update,
};

use futures_core::future::LocalBoxFuture;
use std::{cell::RefCell, future::Future, rc::Rc};

pub type BoxedHandlerService = BoxService<Request, Response, app::Error>;
pub type BoxedHandlerServiceFactory = BoxServiceFactory<(), Request, Response, app::Error, ()>;

#[derive(Debug, Clone)]
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

impl Eq for Request {}

impl Request {
    #[must_use]
    pub fn new(bot: Rc<Bot>, update: Rc<Update>, context: Rc<RefCell<Context>>) -> Self {
        Self {
            bot,
            update,
            context,
        }
    }

    #[must_use]
    fn bot(&self) -> &Rc<Bot> {
        &self.bot
    }

    #[must_use]
    fn update(&self) -> &Rc<Update> {
        &self.update
    }

    #[must_use]
    fn context(&self) -> &Rc<RefCell<Context>> {
        &self.context
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    request: Request,
    response: EventReturn,
}

impl Response {
    #[must_use]
    pub fn new(request: Request, response: EventReturn) -> Self {
        Self { request, response }
    }

    #[must_use]
    pub fn request(&self) -> &Request {
        &self.request
    }

    #[must_use]
    pub fn response(&self) -> &EventReturn {
        &self.response
    }
}

pub trait Handler<Args>: Clone + 'static {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

/// [`Handler`] wrapped into a [`BoxedHandlerServiceFactory`] with filters
#[allow(clippy::module_name_repetitions)]
pub struct HandlerObject {
    service: BoxedHandlerServiceFactory,
    pub(crate) filters: Rc<Vec<Box<dyn Filter>>>,
}

impl HandlerObject {
    /// Creates a new [`HandlerObject`]
    /// # Arguments
    /// * `handler` - Handler function
    /// * `filters` - Filters to the handler
    pub fn new<H, Args>(handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: Handler<Args> + 'static,
        Args: FromEventAndContext + 'static,
        H::Output: Into<EventReturn>,
    {
        Self {
            service: handler_service(handler),
            filters: Rc::new(filters),
        }
    }

    /// Get the handler filters
    #[must_use]
    pub fn filters(&self) -> &[Box<dyn Filter>] {
        &self.filters
    }

    /// Add a filter to the handler filters.
    /// # Arguments
    /// * `filter` - [`Filter`] instance
    /// # Panics
    /// If there are other [`Rc`] or [`Weak`] pointers to the same allocation
    pub fn filter(&mut self, filter: Box<dyn Filter>) {
        Rc::get_mut(&mut self.filters).unwrap().push(filter);
    }
}

impl ServiceFactory<Request> for HandlerObject {
    type Response = Response;
    type Error = app::Error;
    type Config = ();
    type Service = HandlerObjectService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    /// Create a new [`HandlerObjectService`]
    fn new_service(&self, _: ()) -> Self::Future {
        let fut = self.service.new_service(());
        let filters = Rc::clone(&self.filters);

        Box::pin(async move {
            let service = fut.await?;

            Ok(HandlerObjectService { service, filters })
        })
    }
}

/// [`Handler`] wrapped into a [`BoxedHandlerService`] with filters
#[allow(clippy::module_name_repetitions)]
pub struct HandlerObjectService {
    service: BoxedHandlerService,
    filters: Rc<Vec<Box<dyn Filter>>>,
}

impl HandlerObjectService {
    /// Check if the handler pass the filters.
    /// If the handler pass all them, it will be called.
    #[must_use]
    pub fn check(&self, req: &Request) -> bool {
        self.filters
            .iter()
            .all(|filter| filter.check(&req.bot, &req.update, &req.context))
    }
}

impl Service<Request> for HandlerObjectService {
    type Response = Response;
    type Error = app::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    /// Call [`BoxedHandlerService`], which is wrapped [`Handler`]
    fn call(&self, req: Request) -> Self::Future {
        self.service.call(req)
    }
}

/// Wrap a [`Handler`] into a [`BoxedHandlerServiceFactory`]
#[allow(clippy::module_name_repetitions)]
pub fn handler_service<H, Args>(handler: H) -> BoxedHandlerServiceFactory
where
    H: Handler<Args> + 'static,
    Args: FromEventAndContext + 'static,
    H::Output: Into<EventReturn>,
{
    factory(fn_service(move |req: Request| {
        let handler = handler.clone();

        async move {
            match Args::extract(req.bot.as_ref(), req.update.as_ref(), req.context.clone()).await {
                // Call handler with extracted arguments
                Ok(args) => Ok(Response {
                    request: req,
                    response: handler.call(args).await.into(),
                }),
                // Return error which implement `Into<app::Error>`
                Err(err) => Err(err.into()),
            }
        }
    }))
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Fut + Clone + 'static,
        Fut: Future,
    {
        type Output = Fut::Output;
        type Future = Fut;

        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
            (self)($($param,)*)
        }
    }
});

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
factory_tuple! { A B C D E F G H I }
factory_tuple! { A B C D E F G H I J }
factory_tuple! { A B C D E F G H I J K }
factory_tuple! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{Command, CommandPatternType};

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<T: FromEventAndContext>(_: impl Handler<T>) {}

        assert_impl_handler(|| async { unimplemented!() });
        assert_impl_handler(
            |_01: (),
             _02: (),
             _03: (),
             _04: (),
             _05: (),
             _06: (),
             _07: (),
             _08: (),
             _09: (),
             _10: (),
             _11: (),
             _12: ()| async { unimplemented!() },
        );
    }

    #[test]
    fn test_handler_object_filter() {
        let filter = Box::new(Command {
            commands: vec![CommandPatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        });

        let mut handler_object = HandlerObject::new(
            || async { unimplemented!("It's shouldn't call in the test") },
            vec![],
        );
        assert_eq!(handler_object.filters().is_empty(), true);

        handler_object.filter(filter.clone());
        assert_eq!(handler_object.filters().len(), 1);

        let handler_object = HandlerObject::new(
            || async { unimplemented!("It's shouldn't call in the test") },
            vec![filter.clone()],
        );
        assert_eq!(handler_object.filters().len(), 1);
    }

    #[test]
    fn test_handler_object_service() {
        let handler_object = HandlerObject::new(|| async {}, vec![]);
        let handler_object_service = r#await!(handler_object.new_service(())).unwrap();

        let req = Request {
            bot: Rc::new(Bot::default()),
            update: Rc::new(Update::default()),
            context: Rc::new(RefCell::new(Context::new())),
        };
        assert_eq!(handler_object_service.check(&req), true);

        let res = r#await!(handler_object_service.call(req)).unwrap();
        assert_eq!(res.response.is_cancel(), false);
        assert_eq!(res.response.is_skip(), false);
    }
}
