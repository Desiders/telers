use dashmap::DashMap;
use std::any::Any;

/// Creates for each update. \
/// Can be used to store data between handlers, filters, middlewares and etc.
///
/// Usually you don't need to use this type directly in handlers,
/// implement [`crate::extract::FromEventAndContext`] for your own types if you need to use it in handlers more clean way.
/// This trait is implemented for many types by default (include filters and middlewares).
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
