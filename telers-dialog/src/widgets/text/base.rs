#[cfg(test)]
use crate::entities::{ChatEvent, EventContext};
use crate::{
    entities::{DataMap, RenderContext},
    future::BoxFuture,
};

pub trait Text: Send + Sync + 'static {
    #[must_use]
    fn render_text<'a>(&'a self, data: &'a DataMap) -> BoxFuture<'a, Box<str>>;

    #[must_use]
    fn render_text_in_context<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, Box<str>> {
        self.render_text(render_ctx.data.as_ref())
    }

    #[cfg(test)]
    fn render_text_in_context_for_test<'a>(
        &'a self,
        ctx: &'a crate::entities::Context,
        data: &'a DataMap,
    ) -> BoxFuture<'a, Box<str>> {
        Box::pin(async move {
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
                EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
            let render_ctx = RenderContext::new(ctx, data, &event, &event_context);
            self.render_text_in_context(&render_ctx).await
        })
    }
}

impl<T> Text for T
where
    T: ToString + Send + Sync + 'static,
{
    fn render_text<'a>(&'a self, _data: &'a DataMap) -> BoxFuture<'a, Box<str>> {
        Box::pin(async move { self.to_string().into_boxed_str() })
    }
}

pub(crate) struct FnText<Renderer> {
    renderer: Renderer,
}

impl<Renderer> FnText<Renderer> {
    #[inline]
    #[must_use]
    pub(crate) const fn new(renderer: Renderer) -> Self {
        Self {
            renderer,
        }
    }
}

impl<Renderer, Item> Text for FnText<Renderer>
where
    Renderer: Fn(&DataMap) -> Item + Send + Sync + 'static,
    Item: Into<Box<str>>,
{
    fn render_text<'a>(&'a self, data: &'a DataMap) -> BoxFuture<'a, Box<str>> {
        Box::pin(async move { (self.renderer)(data).into() })
    }
}
