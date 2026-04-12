use bon::bon;
use serde_json::Value;
use std::{borrow::Cow, sync::Arc};

use super::{
    super::{when::is_allowed, ButtonAction, ClickContext, Keyboard, WhenCondition},
    BaseScroll, OnPageChanged, Scroll,
};
use crate::entities::{Context, DataMap, RenderContext};
use telers::types::ReplyMarkup;

type DynPagesGetter = Arc<dyn for<'a> Fn(&RenderContext<'a>) -> usize + Send + Sync + 'static>;

/// Page-count source for [`StubScroll`].
#[derive(Clone)]
pub enum StubScrollPages {
    /// Fixed page count.
    Fixed(usize),
    /// Read page count from dialog data by field name.
    DataField(Cow<'static, str>),
    /// Compute page count dynamically from render context.
    Getter(DynPagesGetter),
}

impl StubScrollPages {
    /// Build a dynamic page-count getter.
    #[must_use]
    pub fn getter(
        getter: impl for<'a> Fn(&RenderContext<'a>) -> usize + Send + Sync + 'static,
    ) -> Self {
        Self::Getter(Arc::new(getter))
    }

    fn get(&self, render_ctx: &RenderContext<'_>) -> usize {
        match self {
            Self::Fixed(pages) => *pages,
            Self::DataField(field) => render_ctx
                .data
                .get(field.as_ref())
                .and_then(value_as_usize)
                .unwrap_or_default(),
            Self::Getter(getter) => getter(render_ctx),
        }
    }
}

impl From<usize> for StubScrollPages {
    fn from(pages: usize) -> Self {
        Self::Fixed(pages)
    }
}

impl From<&'static str> for StubScrollPages {
    fn from(field: &'static str) -> Self {
        Self::DataField(field.into())
    }
}

impl From<String> for StubScrollPages {
    fn from(field: String) -> Self {
        Self::DataField(field.into())
    }
}

impl From<Cow<'static, str>> for StubScrollPages {
    fn from(field: Cow<'static, str>) -> Self {
        Self::DataField(field)
    }
}

/// Non-rendering scroll widget for standalone pagers.
///
/// Use it when page state and page count are needed, but the paged content is
/// rendered outside a scrollable keyboard widget.
#[derive(Clone)]
pub struct StubScroll {
    base_scroll: BaseScroll,
    pages: StubScrollPages,
    when: Option<WhenCondition>,
}

#[bon]
impl StubScroll {
    /// Create a stub scroll bound to `widget_data[id]`.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] id: Cow<'static, str>,
        #[builder(with = |pages: impl Into<StubScrollPages>| pages.into())] pages: StubScrollPages,
        on_page_changed: Option<OnPageChanged>,
        when: Option<WhenCondition>,
    ) -> Self {
        Self {
            base_scroll: BaseScroll::new(id, on_page_changed),
            pages,
            when,
        }
    }
}

impl Scroll for StubScroll {
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    fn get_page_count(&self, render_ctx: &RenderContext<'_>) -> usize {
        self.pages.get(render_ctx)
    }
}

impl Keyboard for StubScroll {
    fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        if !self.is_visible(render_ctx.context, render_ctx.data) {
            return None;
        }
        None
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        if !self.is_visible(ctx, &ctx.dialog_data) {
            return None;
        }
        self.base_scroll.handle_callback(ctx, click.callback_data)
    }
}

fn value_as_usize(value: &Value) -> Option<usize> {
    value.as_u64().and_then(|value| usize::try_from(value).ok())
}
