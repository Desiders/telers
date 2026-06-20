use async_fn_traits::AsyncFn1;
use async_trait::async_trait;
use bon::bon;
use serde_json::Value;
use std::{borrow::Cow, sync::Arc};

use super::{
    super::{when::is_allowed, ButtonAction, ClickContext, Keyboard, WhenCondition},
    BaseScroll, OnPageChanged, Scroll,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};
use telers::types::ReplyMarkup;

type DynPagesGetter =
    Arc<dyn Fn(RenderContext) -> BoxFuture<'static, usize> + Send + Sync + 'static>;

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
    pub fn getter<F>(getter: F) -> Self
    where
        F: AsyncFn(RenderContext) -> usize
            + AsyncFn1<RenderContext, Output = usize>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<RenderContext>>::OutputFuture: Send + 'static,
    {
        let getter = Arc::new(getter);
        Self::Getter(Arc::new(move |render_ctx| {
            let getter = getter.clone();
            Box::pin(async move { getter(render_ctx).await })
        }))
    }

    fn get<'a>(&'a self, render_ctx: &'a RenderContext) -> BoxFuture<'a, usize> {
        Box::pin(async move {
            match self {
                Self::Fixed(pages) => *pages,
                Self::DataField(field) => render_ctx
                    .data
                    .get(field.as_ref())
                    .and_then(value_as_usize)
                    .unwrap_or_default(),
                Self::Getter(getter) => getter(render_ctx.clone()).await,
            }
        })
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

#[async_trait]
impl Scroll for StubScroll {
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    async fn get_page_count(&self, render_ctx: RenderContext) -> usize {
        self.pages.get(&render_ctx).await
    }
}

#[async_trait]
impl Keyboard for StubScroll {
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        if !self
            .is_visible(render_ctx.context.as_ref(), render_ctx.data.as_ref())
            .await
        {
            return None;
        }
        None
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        if !self.is_visible(ctx, &ctx.dialog_data).await {
            return None;
        }
        self.base_scroll
            .handle_callback(ctx, click.callback_data.as_str())
            .await
    }
}

fn value_as_usize(value: &Value) -> Option<usize> {
    value.as_u64().and_then(|value| usize::try_from(value).ok())
}
