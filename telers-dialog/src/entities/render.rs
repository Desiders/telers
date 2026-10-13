use std::sync::Arc;

use super::{ChatEvent, Context, DataMap, EventContext};

/// Runtime data available while rendering a dialog window.
///
/// Unlike [`Context`], this structure is not persisted. It groups the stored
/// dialog context, merged render data, and the current update-derived event.
#[derive(Clone, Debug)]
pub struct RenderContext {
    /// Stored dialog context for the active intent.
    pub context: Arc<Context>,
    /// Merged render data available to widgets.
    pub data: Arc<DataMap>,
    /// Current event that triggered this render.
    pub event: Arc<ChatEvent>,
    /// Normalized event metadata used for target chat/thread routing.
    pub event_context: Arc<EventContext>,
    /// Request data supplied by middleware and filters.
    pub runtime_context: Arc<telers::Context>,
}

impl RenderContext {
    #[inline]
    #[must_use]
    pub fn new(
        context: &Context,
        data: &DataMap,
        event: &ChatEvent,
        event_context: &EventContext,
    ) -> Self {
        Self {
            context: Arc::new(context.clone()),
            data: Arc::new(data.clone()),
            event: Arc::new(event.clone()),
            event_context: Arc::new(event_context.clone()),
            runtime_context: Arc::new(telers::Context::default()),
        }
    }

    #[must_use]
    pub fn with_runtime_context(mut self, runtime_context: &telers::Context) -> Self {
        self.runtime_context = Arc::new(runtime_context.clone());
        self
    }

    #[must_use]
    pub fn with_data(mut self, data: DataMap) -> Self {
        self.data = Arc::new(data);
        self
    }

    #[must_use]
    pub fn runtime_data<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.runtime_context.get(key)
    }
}

/// Build the shared data available to dialog and window getters.
#[must_use]
pub fn base_data(context: &Context) -> DataMap {
    let mut data = context.dialog_data.clone();
    data.insert("start_data".into(), context.start_data.clone());
    data.insert("dialog_data".into(), telers::serialization::to_value(&context.dialog_data).unwrap_or_default());
    data.insert("widget_data".into(), telers::serialization::to_value(&context.widget_data).unwrap_or_default());
    data
}

#[cfg(test)]
mod tests {
    use super::{ChatEvent, Context, DataMap, EventContext, RenderContext};
    use crate::future::BoxFuture;

    impl RenderContext {
        #[allow(dead_code)]
        pub(crate) async fn with_test<R>(
            ctx: &Context,
            data: &DataMap,
            render: impl FnOnce(RenderContext) -> BoxFuture<'static, R>,
        ) -> R {
            use telers::{
                client::Reqwest,
                types::{ChatPrivate, MessageText, User},
                Bot,
            };

            let event = ChatEvent::Message(
                MessageText::new(1, 1, ChatPrivate::new(10), "/test")
                    .from(User::new(10, false, "tester"))
                    .into(),
            );
            let event_context =
                EventContext::new(Bot::<Reqwest>::default(), event.clone());
            let render_ctx = RenderContext::new(ctx, data, &event, &event_context);

            render(render_ctx).await
        }
    }
}
