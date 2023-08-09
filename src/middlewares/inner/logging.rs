use super::base::{Middleware, Next};

use crate::{
    errors::EventErrorKind,
    event::{
        telegram::{HandlerRequest, HandlerResponse},
        EventReturn,
    },
};

use async_trait::async_trait;
use std::{
    fmt::{self, Display, Formatter},
    time::Instant,
};
use tracing::{event, instrument, Level};

#[derive(Debug)]
pub struct Logging {
    target: &'static str,
}

impl Logging {
    #[must_use]
    pub fn new(target: &'static str) -> Self {
        Self { target }
    }
}

impl Default for Logging {
    #[must_use]
    fn default() -> Self {
        Self {
            target: module_path!(),
        }
    }
}

impl Display for Logging {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Logging({})", self.target)
    }
}

#[async_trait]
impl<Client> Middleware<Client> for Logging
where
    Client: Send + Sync + 'static,
{
    #[instrument(skip(self, request, next), fields(target = self.target))]
    async fn call(
        &self,
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        let now = Instant::now();
        let result = next(request).await;
        let elapsed = now.elapsed();

        match result {
            // `unwrap` is safe because handler error is wrapped to event error by next function
            Ok(ref response) => match response.handler_result.as_ref().unwrap() {
                EventReturn::Finish => {
                    event!(
                        Level::DEBUG,
                        "Handler finished. Execution time: {elapsed:.2?}",
                    );
                }
                EventReturn::Skip => {
                    event!(
                        Level::DEBUG,
                        "Handler skipped. Execution time: {elapsed:.2?}",
                    );
                }
                EventReturn::Cancel => {
                    event!(
                        Level::DEBUG,
                        "Handler canceled. Execution time: {elapsed:.2?}",
                    );
                }
            },
            Err(ref err_kind) => match err_kind {
                EventErrorKind::Extraction(err) => {
                    event!(
                        Level::ERROR,
                        error = %err,
                        "Extraction returns error. Execution time: {elapsed:.2?}",

                    );
                }
                EventErrorKind::Handler(err) => {
                    event!(
                        Level::ERROR,
                        error = %err,
                        "Handler returns error. Execution time: {elapsed:.2?}",

                    );
                }
                EventErrorKind::Middleware(err) => {
                    event!(
                        Level::ERROR,
                        error = %err,
                        "Middleware returns error. Execution time: {elapsed:.2?}",
                    );
                }
            },
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        event::{service::ServiceFactory as _, telegram::handler_service},
        middlewares::inner::wrap_handler_and_middlewares_to_next,
        types::Update,
    };

    use std::sync::Arc;

    #[tokio::test]
    async fn test_logging() {
        let middleware = Logging::default();

        let handler_service_factory =
            handler_service(|| async { Ok(EventReturn::Finish) }).new_service(());
        let handler_service = Arc::new(handler_service_factory.unwrap());

        let request = HandlerRequest::new(
            Bot::<Reqwest>::default(),
            Update::default(),
            Context::default(),
        );
        let response = middleware
            .call(
                request,
                wrap_handler_and_middlewares_to_next(handler_service, []),
            )
            .await;

        assert!(response.is_ok());
    }
}
