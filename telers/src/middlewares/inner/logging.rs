use super::base::{Middleware, Next};
use crate::{
    errors::EventErrorKind,
    event::{telegram::HandlerResponse, EventReturn},
    Request,
};

use std::{
    fmt::{self, Display, Formatter},
    time::Instant,
};
use tracing::{event, instrument, Level};

#[derive(Debug, Default, Clone, Copy)]
pub struct Logging;

impl Logging {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Display for Logging {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Logging")
    }
}

impl<Client> Middleware<Client> for Logging
where
    Client: Send + Sync + 'static,
{
    #[instrument(skip(self, request, next))]
    async fn call(
        &mut self,
        request: Request<Client>,
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
        client::Reqwest,
        event::telegram::handler::boxed_handler_factory,
        middlewares::inner::wrap_to_next,
        types::{Message, Update, UpdateKind},
    };

    use std::{convert::Infallible, sync::Arc};

    #[tokio::test]
    async fn test_logging() {
        let handler_service =
            boxed_handler_factory(|| async { Ok::<_, Infallible>(EventReturn::Finish) });

        let mut request = Request::<Reqwest>::default();
        request.update = Arc::new(Update {
            id: 0,
            kind: UpdateKind::Message(Message::default()),
        });

        let response = Logging
            .call(request, wrap_to_next(handler_service, [].into()))
            .await;

        assert!(response.is_ok());
    }
}
