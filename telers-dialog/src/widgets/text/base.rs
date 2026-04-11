use crate::entities::{Context, DataMap};

pub trait Text: Send + Sync + 'static {
    #[must_use]
    fn render_text(&self, data: &DataMap) -> Box<str>;

    #[must_use]
    fn render_text_in_context(&self, _ctx: &Context, data: &DataMap) -> Box<str> {
        self.render_text(data)
    }
}

impl<T> Text for T
where
    T: ToString + Send + Sync + 'static,
{
    fn render_text(&self, _data: &DataMap) -> Box<str> {
        self.to_string().into_boxed_str()
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
    fn render_text(&self, data: &DataMap) -> Box<str> {
        (self.renderer)(data).into()
    }
}
