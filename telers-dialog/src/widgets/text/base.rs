#[cfg(test)]
use crate::entities::{ChatEvent, EventContext};
use crate::{entities::{Context, DataMap, RenderContext}, widgets::WhenCondition};
use async_trait::async_trait;

#[async_trait]
pub trait Text: Send + Sync + 'static {
    async fn render_text(&self, data: &DataMap) -> Box<str>;

    #[must_use]
    fn when(self, condition: WhenCondition) -> ConditionalText<Self>
    where
        Self: Sized,
    {
        ConditionalText {
            text: self,
            condition,
        }
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        self.render_text(render_ctx.data.as_ref()).await
    }

    #[cfg(test)]
    async fn render_text_in_context_for_test(
        &self,
        ctx: &crate::entities::Context,
        data: &DataMap,
    ) -> Box<str> {
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
        let event_context = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = RenderContext::new(ctx, data, &event, &event_context);
        self.render_text_in_context(&render_ctx).await
    }
}

/// Text which renders only when its visibility condition passes.
///
/// Contextless rendering supplies an empty dialog context to the condition.
pub struct ConditionalText<T> {
    text: T,
    condition: WhenCondition,
}

#[async_trait]
impl<T: Text> Text for ConditionalText<T> {
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        let context = Context::new("", "", telers::serialization::Value::default());
        if self.condition.check(&context, data).await {
            self.text.render_text(data).await
        } else {
            Box::default()
        }
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        if self.condition.check(&render_ctx.context, &render_ctx.data).await {
            self.text.render_text_in_context(render_ctx).await
        } else {
            Box::default()
        }
    }
}

#[async_trait]
impl<T> Text for T
where
    T: ToString + Send + Sync + 'static,
{
    async fn render_text(&self, _data: &DataMap) -> Box<str> {
        self.to_string().into_boxed_str()
    }
}

/// Text widget that delegates rendering to a closure.
///
/// Use this to render small values from render data as a [`Text`] (for
/// example, the URL passed to [`Button::url_dynamic`]).
///
/// [`Button::url_dynamic`]: crate::widgets::Button::url_dynamic
pub struct FnText<Renderer> {
    renderer: Renderer,
}

impl<Renderer> FnText<Renderer> {
    /// Build a closure-backed text widget.
    #[inline]
    #[must_use]
    pub const fn new(renderer: Renderer) -> Self {
        Self {
            renderer,
        }
    }
}

#[async_trait]
impl<Renderer, Item> Text for FnText<Renderer>
where
    Renderer: Fn(&DataMap) -> Item + Send + Sync + 'static,
    Item: Into<Box<str>>,
{
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        (self.renderer)(data).into()
    }
}
