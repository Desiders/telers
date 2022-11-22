use crate::{
    dispatcher::event::{
        service::BoxFuture,
        telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    },
    error::app,
};

use super::base::Middleware;

use log::{self, Level, Log, Record};
use std::{rc::Rc, time::Instant};

const DEFAULT_TARGET: &str = "logging-middleware";

pub struct Logging {
    logger: Rc<Box<dyn Log>>,
    target: &'static str,
}

impl Logging {
    #[must_use]
    pub fn new(logger: Box<dyn Log>, target: Option<&'static str>) -> Self {
        Self {
            logger: Rc::new(logger),
            target: target.unwrap_or(DEFAULT_TARGET),
        }
    }
}

impl Middleware for Logging {
    #[allow(clippy::similar_names)]
    fn call(
        &self,
        handler: Rc<BoxedHandlerService>,
        req: HandlerRequest,
        middlewares: Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>,
    ) -> BoxFuture<Result<HandlerResponse, app::Error>> {
        let target = self.target;
        let logger = Rc::clone(&self.logger);
        let fut = self.handler(handler, req, middlewares);

        // Builder with logging level and
        let builder = move |level| {
            let mut builder = Record::builder();
            builder.level(level);
            builder.target(target);
            builder
        };

        Box::pin(async move {
            let now = Instant::now();
            let result = fut.await;
            let elapsed = now.elapsed();

            match result {
                Ok(res) => {
                    if res.response().is_skip() {
                        logger.log(
                            &builder(Level::Debug)
                                .args(format_args!(
                                    "Handler skipped with response: {:?}. Execution time: {:.2?}",
                                    res, elapsed,
                                ))
                                .build(),
                        );
                    } else {
                        logger.log(
                            &builder(Level::Debug)
                                .args(format_args!(
                                    "Handler returned response: {:?}. Execution time: {:.2?}",
                                    res, elapsed,
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
                                "Handler returned error: {:?}. Execution time: {:.2?}",
                                err, elapsed,
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
        dispatcher::event::{service::ServiceFactory as _, telegram::handler_service},
        types::Update,
    };

    use log::{Log, Metadata, Record};
    use std::{cell::RefCell, iter};

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    struct SimpleLogger;

    impl Log for SimpleLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            unimplemented!();
        }

        fn log(&self, record: &Record) {
            println!("{} - {}", record.level(), record.args());
        }

        fn flush(&self) {
            unimplemented!();
        }
    }

    #[test]
    fn test_logging() {
        let middleware = Logging::new(Box::new(SimpleLogger), None);

        let handler_service_factory = handler_service(|| async {}).new_service(());
        let handler_service = Rc::new(r#await!(handler_service_factory).unwrap());

        let bot = Rc::new(Bot::default());
        let update = Rc::new(Update::default());
        let context = Rc::new(RefCell::new(Context::default()));
        let req = HandlerRequest::new(bot, update, context);

        let res = r#await!(middleware.call(handler_service, req, Box::new(iter::empty())));
        assert!(res.is_ok());
    }
}
