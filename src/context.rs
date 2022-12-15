use dashmap::DashMap;
use std::any::Any;

/// Context type, which can contain some data from handlers, filters and middlewares
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
