//! Context that is used to transmit data between processing-units when propagating an event (middleware, filters, handlers, etc.).
//!
//! Usually you don't need to use this type directly in handlers,
//! implement [`crate::extract::FromEventAndContext`] for your own types if you need to use it in handlers more clean way.
//! [`crate::extract::FromEventAndContext`] is implemented for many types by default (include base filters and middlewares),
//! so you can use them in handlers.
//! Check [`crate::extract`] documentation for more information.
//!
//! [`Context`] is implemented as [`DashMap`], so it is thread-safe and can be used in async handlers.
//! Every value that is stored in [`Context`] is wrapped in [`Box`] and can be accessed by [`str`] key.
//! Values can be of any type that implements [`Any`], [`Send`] and [`Sync`] traits.
//!
//! Start of the event propagation is `Dispatcher::feed_update` method,
//! where [`Context`] it's creates and pass to every processing-unit.
//! You the same can use `Dispatcher::feed_update_with_context` method
//! for passing your own [`Context`] for propagation event.

use dashmap::DashMap;
use std::any::Any;

pub type Context = DashMap<&'static str, Box<dyn Any + Send + Sync>>;

#[cfg(test)]
mod tests {
    use super::Context;
    use crate::filters::command::CommandObject;

    #[test]
    fn test_context() {
        let context = Context::new();

        context.insert("test", Box::new(1));
        context.insert(
            "command_object",
            Box::new(CommandObject {
                command: "test".to_string(),
                prefix: "/".to_string(),
                mention: None,
                args: vec![],
            }),
        );
        assert_eq!(
            *context.get("test").unwrap().downcast_ref::<i32>().unwrap(),
            1
        );
        assert_eq!(
            *context
                .get("command_object")
                .unwrap()
                .downcast_ref::<CommandObject>()
                .unwrap(),
            CommandObject {
                command: "test".to_string(),
                prefix: "/".to_string(),
                mention: None,
                args: vec![],
            }
        );
    }
}
