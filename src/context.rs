use dashmap::DashMap;
use std::any::Any;

/// Creates for each update and can be passed to all handlers, filters and middlewares. \
/// Can be used to store data that is needed to transfer between handlers, filters and middlewares.
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
                args: Vec::new(),
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
