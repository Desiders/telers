//! [`Context`] is a type that is used to transmit data between processing-units when propagating an event.
//! Context creates at the start of the event propagation by [`Dispatcher`] and pass to every processing-unit.
//! Processing-units can add their own data to context and use data from context that was added by others.
//!
//! Modify context in outer middlewares if you need to pass some data to next outer/inner middlewares or to filters.
//! Usually data for handlers is passed by inner middlewares, but you can use outer middlewares for this too.
//! Check [`outer middleware module`] documentation for more information (**recommended**).
//!
//! Modify context in inner middlewares if you need to pass some data to next inner middlewares or to handler.
//! Check [`inner middleware module`] documentation for more information (**recommended**).
//!
//! Usually you don't need to change the context in filters, and it's better to use middleware for that, but you can do it.
//! Check [`filter module`] documentation for more information.
//!
//! In [`Handler`] context is can be passed as parameter of handler function.
//! You can use context in handlers to get data that was added by middlewares and filters.
//! For convenience, you can implement [`Extractor`] for your own types and use them as handler arguments,
//! so you don't need to pass context as parameter of handler and extract data from context manually.
//! Check [`extractors module`] documentation for more information (**recommended**).
//!
//! [`Dispatcher`]: telers::Dispatcher
//! [`OuterMiddleware`]: telers::middlewares::OuterMiddleware
//! [`InnerMiddleware`]: telers::middlewares::InnerMiddleware
//! [`Filter::check`]: telers::filters::Filter#method.check
//! [`Handler`]: telers::event::telegram::Handler
//! [`Extractor`]: telers::Extractor
//! [`outer middleware module`]: telers::middlewares::outer
//! [`inner middleware module`]: telers::middlewares::inner
//! [`filter module`]: telers::filters
//! [`extractors module`]: telers::extractor

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
        assert_eq!(
            *context.get("test").unwrap().downcast_ref::<i32>().unwrap(),
            1
        );

        context.insert("test_box", Box::new(Box::new("test")));
        assert_eq!(
            *context
                .get("test_box")
                .unwrap()
                .downcast_ref::<Box<&str>>()
                .unwrap(),
            Box::new("test"),
        );

        context.insert("test_str", Box::new("test"));
        assert_eq!(
            *context
                .get("test_str")
                .unwrap()
                .downcast_ref::<&str>()
                .unwrap(),
            "test"
        );

        context.insert("test_str_box", Box::new(Box::new("test")));
        assert_eq!(
            *context
                .get("test_str_box")
                .unwrap()
                .downcast_ref::<Box<&str>>()
                .unwrap(),
            Box::new("test")
        );

        context.insert("test_string", Box::new("test".to_string()));
        assert_eq!(
            *context
                .get("test_string")
                .unwrap()
                .downcast_ref::<String>()
                .unwrap(),
            "test".to_string()
        );

        context.insert(
            "command_object",
            Box::new(CommandObject {
                command: "test".into(),
                prefix: '/',
                mention: None,
                args: [].into(),
            }),
        );
        assert_eq!(
            *context
                .get("command_object")
                .unwrap()
                .downcast_ref::<CommandObject>()
                .unwrap(),
            CommandObject {
                command: "test".into(),
                prefix: '/',
                mention: None,
                args: [].into(),
            }
        );
    }
}
