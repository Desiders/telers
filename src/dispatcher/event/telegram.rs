use crate::{
    dispatcher::event::handler::{Handler, HandlerObject},
    extract::FromEventAndContext,
    filters::BoxFilter,
};

#[allow(clippy::module_name_repetitions)]
pub struct TelegramEventObserver<H, Args>
where
    H: Handler<Args>,
    Args: FromEventAndContext,
{
    /// Event observer handlers name
    event_name: String,
    /// Common filters for all handlers in the observer.
    filters: Vec<BoxFilter>,
    /// Handlers of the observer.
    handlers: Vec<HandlerObject<H, Args>>,
}

impl<H, Args> TelegramEventObserver<H, Args>
where
    H: Handler<Args> + 'static,
    Args: FromEventAndContext + 'static,
{
    /// Creates a new event observer.
    pub fn new(event_name: String) -> Self {
        Self {
            event_name,
            filters: Vec::new(),
            handlers: Vec::new(),
        }
    }

    /// Get event observer name
    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    /// Get filters of the observer
    pub fn filters(&self) -> &[BoxFilter] {
        &self.filters
    }

    /// Get handlers of the observer.
    pub fn handlers(&self) -> &[HandlerObject<H, Args>] {
        &self.handlers
    }

    /// Add a filter to the observer.
    pub fn filter(&mut self, filter: BoxFilter) {
        self.filters.push(filter);
    }

    /// Add a handler with handler' filters to the observer.
    pub fn register(&mut self, handler: H, filters: Vec<BoxFilter>) {
        self.handlers.push(HandlerObject::new(handler, filters));
    }
}

#[cfg(test)]
mod tests {
    use super::TelegramEventObserver;

    use crate::{
        filters::{self, CommandPatternType},
        types::Message,
    };

    #[test]
    fn test_event_observer() {
        async fn handler(_: Message) {}

        let mut observer = TelegramEventObserver::new("test".to_string());

        // observer.filter(); TODO: add filters
        observer.register(
            handler,
            vec![Box::new(filters::Command {
                commands: vec![CommandPatternType::Text("test".to_string())],
                prefix: "/".to_string(),
                ignore_case: false,
                ignore_mention: false,
            })],
        );

        assert_eq!(observer.event_name(), "test");
        assert_eq!(observer.handlers().len(), 1);
    }
}
