use crate::{
    dispatcher::event::{
        service::BoxFuture,
        telegram::handler::{BoxedHandlerService, Request, Response},
    },
    error::app,
};

use super::base::{Middleware, MiddlewaresIter};

use log::{self, Level, Log, Record};
use std::{sync::Arc, time::Instant};

const DEFAULT_TARGET: &str = "logging-middleware";

pub struct Logging {
    logger: Arc<Box<dyn Log>>,
    target: &'static str,
}

impl Logging {
    #[must_use]
    pub fn new(logger: Box<dyn Log>, target: Option<&'static str>) -> Self {
        Self {
            logger: Arc::new(logger),
            target: target.unwrap_or(DEFAULT_TARGET),
        }
    }
}

impl Middleware for Logging {
    #[allow(clippy::similar_names)]
    fn call(
        &self,
        handler: Arc<BoxedHandlerService>,
        req: Request,
        middlewares: MiddlewaresIter,
    ) -> BoxFuture<Result<Response, app::ErrorKind>> {
        let target = self.target;

        // Builder with logging level and
        let builder = move |level| {
            let mut builder = Record::builder();
            builder.level(level);
            builder.target(target);
            builder
        };

        let logger = Arc::clone(&self.logger);
        let handler = self.handler(handler, req, middlewares);

        Box::pin(async move {
            let now = Instant::now();
            let result = handler.await;
            let elapsed = now.elapsed();

            match result {
                Ok(res) => {
                    if res.response().is_skip() {
                        logger.log(
                            &builder(Level::Debug)
                                .args(format_args!(
                                    "Handler skipped with response: {res:?}. Execution time: {elapsed:.2?}"
                                ))
                                .build(),
                        );
                    } else {
                        logger.log(
                            &builder(Level::Debug)
                                .args(format_args!(
                                    "Handler returned response: {res:?}. Execution time: {elapsed:.2?}"
                                ))
                                .build(),
                        );
                    }
                    Ok(res)
                }
                Err(err) => {
                    logger.log(
                        &builder(Level::Error)
                            .args(format_args!(
                                "Handler returned error: {err:?}. Execution time: {elapsed:.2?}"
                            ))
                            .build(),
                    );
                    Err(err)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Bot,
        context::Context,
        dispatcher::event::{service::ServiceFactory as _, telegram::handler::handler_service},
        types::Update,
    };

    use log::{Log, Metadata, Record};
    use std::iter;
    use tokio;

    struct SimpleLogger;

    impl Log for SimpleLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            unreachable!();
        }

        fn log(&self, record: &Record) {
            println!("{} - {}", record.level(), record.args());
        }

        fn flush(&self) {
            unreachable!();
        }
    }

    #[tokio::test]
    async fn test_logging() {
        let middleware = Logging::new(Box::new(SimpleLogger), None);

        let handler_service_factory = handler_service(|| async {}).new_service(());
        let handler_service = Arc::new(handler_service_factory.unwrap());

        let req = Request::new(Bot::default(), Update::default(), Context::default());

        let res = middleware
            .call(handler_service, req, Box::new(iter::empty()))
            .await;
        assert!(res.is_ok());
    }
}
