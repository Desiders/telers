//! [`Context`] is a type that is used to transmit data between processing-units when propagating an event.
//! Context creates at the start of the event propagation by [`Dispatcher`] and pass to every processing-unit.
//! Processing-units can add their own data to context and use data from context that was added by others.
//!
//! In [`OuterMiddleware`] context is passed as [`RouterRequest`] field `context`.
//! Modify context in outer middlewares if you need to pass some data to next outer/inner middlewares or to filters.
//! Usually data for handlers is passed by inner middlewares, but you can use outer middlewares for this too.
//! Check [`outer middleware module`] documentation for more information (**recommended**).
//!
//! In [`InnerMiddleware`] context is passed as [`HandlerRequest`] field `context`.
//! Modify context in inner middlewares if you need to pass some data to next inner middlewares or to handler.
//! Check [`inner middleware module`] documentation for more information (**recommended**).
//!
//! In [`Filter`] context is passed as parameter `context` in [`Filter::check`] method.
//! Usually you don't need to change the context in filters, and it's better to use middleware for that, but you can do it.
//! Check [`filter module`] documentation for more information.
//!
//! In [`Handler`] context is can be passed as parameter of handler function.
//! You can use context in handlers to get data that was added by middlewares and filters.
//! For convenience, you can implement [`FromEventAndContext`] for your own types and use them as handler arguments,
//! so you don't need to pass context as parameter of handler and extract data from context manually.
//! Check [`extract module`] documentation for more information (**recommended**).
//!
//! [`Dispatcher`]: crate::Dispatcher
//! [`OuterMiddleware`]: crate::middlewares::OuterMiddleware
//! [`InnerMiddleware`]: crate::middlewares::InnerMiddleware
//! [`RouterRequest`]: crate::router::Request
//! [`HandlerRequest`]: crate::handler::Request
//! [`Filter`]: crate::filters::Filter
//! [`Filter::check`]: crate::filters::Filter#method.check
//! [`Handler`]: crate::event::telegram::Handler
//! [`FromEventAndContext`]: crate::extract::FromEventAndContext
//! [`outer middleware module`]: crate::middlewares::outer
//! [`inner middleware module`]: crate::middlewares::inner
//! [`filter module`]: crate::filters
//! [`extract module`]: crate::extract

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
                prefix: '/',
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
                prefix: '/',
                mention: None,
                args: vec![],
            }
        );
    }
}
