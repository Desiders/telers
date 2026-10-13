use super::{Middleware, Next};
use crate::client::session::base::ClientResponse;

use futures_util::future::BoxFuture;
use std::{sync::Arc, time::Instant};
use tracing::{event, Level};

/// Reports how long every outgoing request takes, and reports the failed ones as errors.
///
/// A method passed to [`Logging::ignore_methods`] is left alone entirely.
#[derive(Debug, Default, Clone)]
pub struct Logging {
    ignored: Box<[Box<str>]>,
}

impl Logging {
    /// Create a logger that reports every request.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Do not report the given methods, named as the Telegram API names them.
    ///
    /// `getUpdates` is worth ignoring while polling, since it reports every update
    /// the bot receives and drowns out the rest.
    #[must_use]
    pub fn ignore_methods<M>(mut self, methods: impl IntoIterator<Item = M>) -> Self
    where
        M: Into<Box<str>>,
    {
        self.ignored = methods.into_iter().map(Into::into).collect();
        self
    }

    fn reports(&self, method_name: &str) -> bool {
        !self.ignored.iter().any(|name| name.as_ref() == method_name)
    }
}

impl Middleware for Logging {
    fn call<'a>(
        self: Arc<Self>,
        next: Next<'a>,
        bot_id: i64,
        method_name: &'static str,
    ) -> BoxFuture<'a, Result<ClientResponse, anyhow::Error>> {
        Box::pin(async move {
            if !self.reports(method_name) {
                return next().await;
            }

            let started = Instant::now();

            let response = next().await;

            match &response {
                Ok(_) => event!(
                    Level::DEBUG,
                    method_name,
                    bot_id,
                    "Request execution time: {elapsed:.2?}",
                    elapsed = started.elapsed()
                ),
                Err(err) => event!(
                    Level::ERROR,
                    method_name,
                    bot_id,
                    error = %err,
                    "Request failed after {elapsed:.2?}",
                    elapsed = started.elapsed()
                ),
            }

            response
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Logging, Middleware, Next};
    use crate::client::session::base::ClientResponse;

    use std::{
        fmt,
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc, Mutex,
        },
    };
    use tracing::{
        field::{Field, Visit},
        span::{Attributes, Id, Record},
        Event, Metadata, Subscriber,
    };

    /// Collects the events this middleware reports, so a test can tell reporting from passing.
    #[derive(Default)]
    struct Reported {
        events: Mutex<Vec<(String, String)>>,
    }

    /// Starts recording on this thread. The returned guard stops it when dropped, so the other
    /// tests keep whatever subscriber they had.
    fn recording() -> (Arc<Reported>, tracing::subscriber::DefaultGuard) {
        let reported = Arc::new(Reported::default());
        let guard =
            tracing::subscriber::set_default(Collector(Arc::clone(&reported), AtomicU64::new(0)));
        (reported, guard)
    }

    /// The levels reported for `method_name`, in order.
    fn levels_of(reported: &Reported, method_name: &str) -> Vec<String> {
        reported
            .events
            .lock()
            .expect("the lock is not poisoned")
            .iter()
            .filter(|(_, name)| name == method_name)
            .map(|(level, _)| level.clone())
            .collect()
    }

    struct MethodName(Option<String>);

    impl Visit for MethodName {
        fn record_str(&mut self, field: &Field, value: &str) {
            if field.name() == "method_name" {
                self.0 = Some(value.to_owned());
            }
        }

        fn record_debug(&mut self, _: &Field, _: &dyn fmt::Debug) {}
    }

    struct Collector(Arc<Reported>, AtomicU64);

    impl Subscriber for Collector {
        fn enabled(&self, _: &Metadata<'_>) -> bool {
            true
        }

        fn event(&self, event: &Event<'_>) {
            let mut visitor = MethodName(None);
            event.record(&mut visitor);

            if let MethodName(Some(name)) = visitor {
                let level = event.metadata().level().to_string();
                self.0
                    .events
                    .lock()
                    .expect("the lock is not poisoned")
                    .push((level, name));
            }
        }

        fn new_span(&self, _: &Attributes<'_>) -> Id {
            // `tracing` rejects the id `0`, so hand out a number that is never used twice.
            Id::from_u64(self.1.fetch_add(1, Ordering::Relaxed) + 1)
        }

        fn record(&self, _: &Id, _: &Record<'_>) {}

        fn record_follows_from(&self, _: &Id, _: &Id) {}

        fn enter(&self, _: &Id) {}

        fn exit(&self, _: &Id) {}
    }

    fn session_returning(response: Result<ClientResponse, anyhow::Error>) -> Next<'static> {
        Box::new(move || Box::pin(std::future::ready(response)))
    }

    fn ok() -> Result<ClientResponse, anyhow::Error> {
        Ok(ClientResponse::new(200, r#"{"ok":true}"#))
    }

    #[tokio::test]
    async fn a_successful_request_is_reported_as_debug() {
        let (reported, _guard) = recording();

        let next: Next<'_> = Box::new(session_returning(ok()));
        let response = Arc::new(Logging::new())
            .call(next, 1, "logging_success")
            .await
            .expect("logging should not change the result");

        assert!(
            response.status_code.is_success(),
            "the response should reach the caller"
        );
        assert_eq!(levels_of(&reported, "logging_success"), ["DEBUG"]);
    }

    #[tokio::test]
    async fn a_failed_request_is_reported_as_error() {
        let (reported, _guard) = recording();

        let next: Next<'_> = Box::new(session_returning(Err(anyhow::anyhow!("no route"))));
        let result = Arc::new(Logging::new())
            .call(next, 1, "logging_failure")
            .await;

        assert_eq!(
            result
                .expect_err("logging should not change the error")
                .to_string(),
            "no route"
        );
        assert_eq!(levels_of(&reported, "logging_failure"), ["ERROR"]);
    }

    #[test]
    fn every_method_is_reported_by_default() {
        let logging = Logging::new();

        assert!(logging.reports("getUpdates"));
        assert!(logging.reports("sendMessage"));
    }

    #[test]
    fn ignored_methods_are_not_reported() {
        let logging = Logging::new().ignore_methods(["getUpdates", "getMe"]);

        assert!(!logging.reports("getUpdates"));
        assert!(!logging.reports("getMe"));
        assert!(logging.reports("sendMessage"));
    }

    #[tokio::test]
    async fn an_ignored_method_reaches_the_session_without_a_report() {
        let (reported, _guard) = recording();

        let next: Next<'_> = Box::new(session_returning(ok()));
        let response = Arc::new(Logging::new().ignore_methods(["logging_ignored"]))
            .call(next, 1, "logging_ignored")
            .await
            .expect("skipping the log should not change the result");

        assert!(
            response.status_code.is_success(),
            "the response should reach the caller"
        );
        assert!(levels_of(&reported, "logging_ignored").is_empty());
    }
}
