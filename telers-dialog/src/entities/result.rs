//! Result hook context for processing sub-dialog completion.

use std::sync::Arc;

use super::{ChatEvent, Context, Data, DataMap, EventContext};

/// Context available when processing a sub-dialog result.
///
/// This context is passed to `on_process_result` hooks on dialogs and windows
/// when a child dialog completes with `done_with_result()`.
///
/// # Example
///
/// ```ignore
/// use telers_dialog::{dialog, window, widgets::{text, ButtonAction}, ResultContext};
///
/// async fn handle_child_result(ctx: ResultContext) -> Option<ButtonAction> {
///     let username = ctx.result().get("username")?;
///     Some(ButtonAction::set_dialog_value("selected_user", username.clone()))
/// }
///
/// let d = dialog([window("parent", [text("Parent")])])
///     .on_process_result(handle_child_result);
/// ```
#[derive(Clone, Debug)]
pub struct ResultContext {
    /// Parent context that receives the result.
    pub context: Arc<Context>,
    /// Start data the child dialog was launched with.
    pub start_data: Arc<Data>,
    /// Result data returned by the child dialog.
    pub result: Arc<Data>,
    /// Current event that triggered the child completion.
    pub event: Arc<ChatEvent>,
    /// Normalized event metadata.
    pub event_context: Arc<EventContext>,
    /// Request-scoped context populated by telers middlewares and filters.
    pub runtime_context: Arc<telers::Context>,
}

impl ResultContext {
    /// Create a new result context.
    #[inline]
    #[must_use]
    pub fn new(
        context: &Context,
        start_data: &Data,
        result: &Data,
        event: &ChatEvent,
        event_context: &EventContext,
        runtime_context: &telers::Context,
    ) -> Self {
        Self {
            context: Arc::new(context.clone()),
            start_data: Arc::new(start_data.clone()),
            result: Arc::new(result.clone()),
            event: Arc::new(event.clone()),
            event_context: Arc::new(event_context.clone()),
            runtime_context: Arc::new(runtime_context.clone()),
        }
    }

    /// Get the parent dialog context.
    #[inline]
    #[must_use]
    pub fn parent_context(&self) -> &Context {
        &self.context
    }

    /// Get the start data the child dialog was launched with.
    #[inline]
    #[must_use]
    pub fn start_data(&self) -> &Data {
        &self.start_data
    }

    /// Get the result data returned by the child dialog.
    #[inline]
    #[must_use]
    pub fn result(&self) -> &Data {
        &self.result
    }

    /// Get runtime data from telers context.
    #[inline]
    #[must_use]
    pub fn runtime_data<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.runtime_context.get(key)
    }

    /// Get the parent's dialog data.
    #[inline]
    #[must_use]
    pub fn dialog_data(&self) -> &DataMap {
        &self.context.dialog_data
    }

    /// Get the parent's widget data.
    #[inline]
    #[must_use]
    pub fn widget_data(&self) -> &DataMap {
        &self.context.widget_data
    }

    /// Get a value from the result data.
    #[inline]
    #[must_use]
    pub fn result_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.result.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::ResultContext;
    use crate::entities::{ChatEvent, Context, EventContext};
    use serde_json::json;

    #[test]
    fn result_context_provides_access_to_all_fields() {
        use telers::{
            client::Reqwest,
            types::{ChatPrivate, MessageText, User},
            Bot,
        };

        let ctx = Context::new("test_ctx", "parent_state", json!({"parent": true}));
        let start_data = json!({"launch_mode": "normal"});
        let result = json!({"username": "alice", "selected": true});
        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatPrivate::new(10), "/done")
                .from(User::new(10, false, "tester"))
                .into(),
        );
        let event_context = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        let runtime_context = telers::Context::default();

        let result_ctx = ResultContext::new(
            &ctx,
            &start_data,
            &result,
            &event,
            &event_context,
            &runtime_context,
        );

        assert_eq!(result_ctx.parent_context().state, "parent_state");
        assert_eq!(result_ctx.parent_context().stack_id, "test_ctx");
        assert_eq!(
            result_ctx.start_data().get("launch_mode"),
            Some(&json!("normal"))
        );
        assert_eq!(result_ctx.result_value("username"), Some(&json!("alice")));
        assert_eq!(result_ctx.result_value("selected"), Some(&json!(true)));
    }
}
