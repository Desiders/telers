use crate::{
    client::Reqwest,
    errors::EventErrorKind,
    event::{
        service::{service_fn, BoxCloneService},
        EventReturn,
    },
    Request,
};

use std::future::Future;

pub(crate) type BoxedCloneMiddlewareService<Client> =
    BoxCloneService<Request<Client>, MiddlewareResponse<Client>, EventErrorKind>;

/// Response from middleware.
/// First element is/isn't updated [`Request`] and second is [`EventReturn`] for the manipulate processing event,
/// see [`EventReturn`] for more info.
pub type MiddlewareResponse<Client = Reqwest> = (Request<Client>, EventReturn);

/// Outer middlewares called before filters, inner middlewares and handlers
///
/// Prefer to use outer middlewares over inner middlewares in some cases:
/// - If you need to call middlewares before filters, inner middlewares and handlers
/// - If you need to manipulate with [`Request`] and [`crate::context::Context`] in it
///
/// Usually outer middlewares are used to manipulate with [`Request`].
///
/// Implement this trait for your own middlewares
pub trait Middleware<Client = Reqwest>: Clone + Send + Sync + 'static {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for observers, filters, handler and middlewares
    /// # Errors
    /// If outer middleware returns error
    fn call(
        &mut self,
        request: Request<Client>,
    ) -> impl Future<Output = Result<MiddlewareResponse<Client>, EventErrorKind>> + Send;
}

/// To possible use function-like as middlewares
impl<Client, F, Fut> Middleware<Client> for F
where
    Client: Send + Sync + 'static,
    F: FnMut(Request<Client>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<MiddlewareResponse<Client>, EventErrorKind>> + Send,
{
    fn call(
        &mut self,
        request: Request<Client>,
    ) -> impl Future<Output = Result<MiddlewareResponse<Client>, EventErrorKind>> + Send {
        self(request)
    }
}

pub(crate) fn boxed_middleware_factory<Client, M>(
    middleware: M,
) -> BoxedCloneMiddlewareService<Client>
where
    Client: Send + Sync + 'static,
    M: Middleware<Client>,
{
    BoxCloneService::new(service_fn(move |request| {
        let mut middleware = middleware.clone();

        async move { middleware.call(request).await }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        context::Context,
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
        Bot, Extensions,
    };

    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_call() {
        let mut middleware =
            |request: Request<Reqwest>| async move { Ok((request, EventReturn::default())) };

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0, "", ""), ""),
            ))),
            bot: Bot::default(),
            context: Context::default(),
            extensions: Extensions::default(),
        };
        let (_, event_return) = Middleware::call(&mut middleware, request.clone())
            .await
            .unwrap();
        assert_eq!(event_return, EventReturn::default());
    }
}
