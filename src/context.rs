use std::{any::Any, collections::HashMap};

pub type Context = HashMap<String, Box<dyn Any>>;

#[cfg(test)]
mod tests {
    use super::Context;
    use crate::filters::CommandObject;

    #[test]
    fn test_context() {
        let mut context = Context::new();
        context.insert("test".to_string(), Box::new(1));

        assert_eq!(
            context.get("test").unwrap().downcast_ref::<i32>().unwrap(),
            &1
        );
        assert_eq!(
            context
                .get("test")
                .unwrap()
                .downcast_ref::<i32>()
                .unwrap()
                .clone(),
            1
        );

        context.insert(
            "command_object".to_string(),
            Box::new(CommandObject {
                command: "test".to_string(),
                prefix: "/".to_string(),
                mention: None,
                args: Vec::new(),
            }),
        );

        assert_eq!(
            context
                .get("command_object")
                .unwrap()
                .downcast_ref::<CommandObject>()
                .unwrap()
                .clone(),
            CommandObject {
                command: "test".to_string(),
                prefix: "/".to_string(),
                mention: None,
                args: Vec::new(),
            }
        );
    }
}
