use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::{
        bases::EventReturn,
        handler::{Handler, HandlerObject},
    },
    extract::FromEventAndContext,
    filters::BoxFilter,
    types::Update,
};

use std::{cell::RefCell, rc::Rc};

/// Event observer for Telegram events.
/// Here you can register a handler with filters for the event or filters for all handlers in the observer.
/// This observer will stop event propagation when first handler is pass.
#[allow(clippy::module_name_repetitions)]
pub struct TelegramEventObserver<H, Args>
where
    H: Handler<Args>,
    H::Output: EventReturn,
    Args: FromEventAndContext,
{
    /// Event observer name
    event_name: String,
    /// Filters for all handlers in the observer
    filters: Vec<BoxFilter>,
    /// Handlers of the observer
    handlers: Vec<HandlerObject<H, Args>>,
}

impl<H, Args> TelegramEventObserver<H, Args>
where
    H: Handler<Args> + 'static,
    H::Output: EventReturn,
    Args: FromEventAndContext + 'static,
{
    /// Creates a new event observer
    #[must_use]
    pub fn new(event_name: String) -> Self {
        Self {
            event_name,
            filters: vec![],
            handlers: vec![],
        }
    }

    /// Get event observer name
    #[must_use]
    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    /// Get filters of the observer
    #[must_use]
    pub fn filters(&self) -> &[BoxFilter] {
        &self.filters
    }

    /// Get handlers of the observer.
    #[must_use]
    pub fn handlers(&self) -> &[HandlerObject<H, Args>] {
        &self.handlers
    }

    /// Add a filter to the observer.
    pub fn filter(&mut self, filter: BoxFilter) {
        self.filters.push(filter);
    }

    /// Add a handler with handler's filters to the observer.
    pub fn register(&mut self, handler: H, filters: Vec<BoxFilter>) {
        self.handlers.push(HandlerObject::new(handler, filters));
    }

    /// Propagate event to handlers and stops propagation on first match.
    /// Handler will be called when all its filters is pass.
    /// # Arguments
    /// * `bot` - [Bot] instance
    /// * `update` - [Update] instance
    /// * `context` - [Context] instance
    /// # Returns
    /// `true` if pass at least one handler, otherwise `false`.
    pub async fn trigger(&self, bot: Rc<Bot>, update: Rc<Update>, context: Rc<RefCell<Context>>) {
        for handler in self.handlers() {
            // Check if the handler pass the filters
            if handler
                .check(bot.clone(), update.clone(), context.clone())
                .await
            {
                // Call the handler
                let result = handler
                    .call(bot.clone(), update.clone(), context.clone())
                    .await;
                if result.is_skip() {
                    continue;
                }
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bot, Context, Rc, RefCell, TelegramEventObserver, Update};

    use crate::{
        dispatcher::event::bases::{Action, EventReturn},
        filters::{self, CommandObject, CommandPatternType},
        types::Message,
    };

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_event_observer_trigger() {
        /// # Arguments
        /// * `message` - [`Message`] instance
        /// * `command` - [`CommandObject`] instance here from [`Command`] and [`extract.filters`]
        async fn handler(message: Message, command: CommandObject) {
            assert_eq!(message.text.unwrap(), "/test");

            assert_eq!(command.command, "test");
            assert_eq!(command.prefix, "/");
            assert_eq!(command.mention, None);
            assert_eq!(command.args, Vec::<String>::new());
        }

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

        let message = Message {
            text: Some("/test".to_string()),
            ..Default::default()
        };
        let bot = Rc::new(Bot::new());
        let context = Rc::new(RefCell::new(Context::new()));
        let update = Rc::new(Update {
            message: Some(message.clone()),
            ..Update::default()
        });

        r#await!(observer.trigger(bot, update, context));
    }

    #[test]
    fn test_event_observer_event_return() {
        async fn handler_first() -> impl EventReturn {
            Action::Skip
        }

        async fn handler_second() -> impl EventReturn {
            Action::Cancel
        }

        let mut observer = TelegramEventObserver::new("test".to_string());

        observer.register(handler_first, vec![]);
        // observer.register(handler_second, vec![]); TODO: fix with bug zero-sized value of the function's item type

        let bot = Rc::new(Bot::new());
        let context = Rc::new(RefCell::new(Context::new()));
        let update = Rc::new(Update {
            ..Update::default()
        });

        r#await!(observer.trigger(bot, update, context));
    }
}
