use super::{ChatEvent, Context, DataMap, EventContext};

/// Runtime data available while rendering a dialog window.
///
/// Unlike [`Context`], this structure is not persisted. It groups the stored
/// dialog context, merged render data, and the current update-derived event.
#[derive(Clone, Copy, Debug)]
pub struct RenderContext<'a> {
    /// Stored dialog context for the active intent.
    pub context: &'a Context,
    /// Merged render data available to widgets.
    pub data: &'a DataMap,
    /// Current event that triggered this render.
    pub event: &'a ChatEvent,
    /// Normalized event metadata used for target chat/thread routing.
    pub event_context: &'a EventContext,
}

impl<'a> RenderContext<'a> {
    #[inline]
    #[must_use]
    pub const fn new(
        context: &'a Context,
        data: &'a DataMap,
        event: &'a ChatEvent,
        event_context: &'a EventContext,
    ) -> Self {
        Self {
            context,
            data,
            event,
            event_context,
        }
    }
}

#[cfg(test)]
impl RenderContext<'_> {
    pub(crate) fn with_test<R>(
        ctx: &Context,
        data: &DataMap,
        render: impl FnOnce(&RenderContext<'_>) -> R,
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
        let event_context = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = RenderContext {
            context: ctx,
            data,
            event: &event,
            event_context: &event_context,
        };

        render(&render_ctx)
    }
}
