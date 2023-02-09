use crate::{
    dispatcher::event::{
        telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
        EventReturn,
    },
    error::AppErrorKind,
};

use super::base::{call_handler, Middleware, MiddlewaresIter};

use async_trait::async_trait;
use log::{self, Level, Log, Record};
use std::{fmt, sync::Arc, time::Instant};

const DEFAULT_TARGET: &str = "logging-middleware";

pub struct Logging {
    logger: &'static dyn Log,
    target: &'static str,
}

impl Logging {
    #[must_use]
    pub fn new(logger: Option<&'static dyn Log>, target: Option<&'static str>) -> Self {
        Self {
            logger: logger.unwrap_or_else(log::logger),
            target: target.unwrap_or(DEFAULT_TARGET),
        }
    }

    fn record<'a>(&self, level: Level, args: fmt::Arguments<'a>) -> Record<'a> {
        Record::builder()
            .level(level)
            .target(self.target)
            .args(args)
            .build()
    }
}

impl Default for Logging {
    #[must_use]
    fn default() -> Self {
        Self::new(None, None)
    }
}

#[async_trait]
impl Middleware for Logging {
    async fn call(
        &self,
        handler: Arc<BoxedHandlerService>,
        request: HandlerRequest,
        middlewares: MiddlewaresIter,
    ) -> Result<HandlerResponse, AppErrorKind> {
        let now = Instant::now();
        let result = call_handler(handler, request, middlewares).await;
        let elapsed = now.elapsed();

        match result {
            Ok(ref response) => match response.handler_result {
                Ok(ref event_return) => match event_return {
                    EventReturn::Finish => {
                        self.logger.log(&self.record(
                            Level::Debug,
                            format_args!("Handler proccessed. Execution time: {elapsed:.2?}"),
                        ));
                    }
                    EventReturn::Skip => {
                        self.logger.log(&self.record(
                            Level::Debug,
                            format_args!("Handler skipped. Execution time: {elapsed:.2?}"),
                        ));
                    }
                    EventReturn::Cancel => {
                        self.logger.log(&self.record(
                            Level::Debug,
                            format_args!("Handler canceled. Execution time: {elapsed:.2?}"),
                        ));
                    }
                },
                Err(ref err) => {
                    self.logger.log(&self.record(
                        Level::Error,
                        format_args!(
                            "Handler returned error: {err}. Execution time: {elapsed:.2?}"
                        ),
                    ));
                }
            },
            Err(ref err_kind) => match err_kind {
                AppErrorKind::Extraction(err) => {
                    self.logger.log(&self.record(
                        Level::Error,
                        format_args!(
                            "Extraction returned error: {err}. Execution time: {elapsed:.2?}"
                        ),
                    ));
                }
                AppErrorKind::User(err) => {
                    self.logger.log(&self.record(
                        Level::Error,
                        format_args!(
                            "Middleware returned error: {err}. Execution time: {elapsed:.2?}"
                        ),
                    ));
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
        client::Bot,
        context::Context,
        dispatcher::event::{service::ServiceFactory as _, telegram::handler_service},
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
        let middleware = Logging::new(Some(&SimpleLogger), None);

        let handler_service_factory =
            handler_service(|| async { Ok(EventReturn::Finish) }).new_service(());
        let handler_service = Arc::new(handler_service_factory.unwrap());

        let request = HandlerRequest::new(Bot::default(), Update::default(), Context::default());
        let res = middleware
            .call(handler_service, request, Box::new(iter::empty()))
            .await;

        assert!(res.is_ok());
    }
}
