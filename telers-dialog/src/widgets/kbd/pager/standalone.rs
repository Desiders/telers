use bon::bon;
use std::{borrow::Cow, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use super::{
    super::{
        format_callback_data, when::is_allowed, ButtonAction, ClickContext, Keyboard, WhenCondition,
    },
    handle_pager_callback, read_page, render_direction_button, OnPageChanged, PageDirection,
    Scroll,
};
use crate::entities::{Context, DataMap, RenderContext};

type DynPageCountGetter = Box<dyn for<'a> Fn(&RenderContext<'a>) -> usize + Send + Sync + 'static>;

/// Builder input for standalone pager widgets.
///
/// Use a plain widget id for manual configuration:
/// `NumberedPager::builder("pager").page_count_getter(...)`
///
/// Or pass a scroll widget directly:
/// `NumberedPager::builder(scrolling_text)`
pub struct PagerBinding {
    id: Cow<'static, str>,
    page_count_getter: Option<DynPageCountGetter>,
    on_page_changed: Option<OnPageChanged>,
}

impl PagerBinding {
    #[must_use]
    pub fn new(id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: id.into(),
            page_count_getter: None,
            on_page_changed: None,
        }
    }
}

impl From<&'static str> for PagerBinding {
    #[inline]
    fn from(id: &'static str) -> Self {
        Self::new(id)
    }
}

impl From<String> for PagerBinding {
    #[inline]
    fn from(id: String) -> Self {
        Self::new(id)
    }
}

impl From<Cow<'static, str>> for PagerBinding {
    #[inline]
    fn from(id: Cow<'static, str>) -> Self {
        Self::new(id)
    }
}

impl<S> From<S> for PagerBinding
where
    S: Scroll + Send + Sync + 'static,
{
    fn from(scroll: S) -> Self {
        let id = scroll.widget_id().to_owned().into();
        let on_page_changed = scroll.on_page_changed().cloned();
        let page_count_getter = Box::new(move |render_ctx: &RenderContext<'_>| -> usize {
            scroll.get_page_count(render_ctx)
        });
        Self {
            id,
            page_count_getter: Some(page_count_getter),
            on_page_changed,
        }
    }
}

#[must_use]
fn resolve_pager_binding(
    binding: PagerBinding,
    page_count_getter: Option<DynPageCountGetter>,
    on_page_changed: Option<OnPageChanged>,
) -> (Cow<'static, str>, DynPageCountGetter, Option<OnPageChanged>) {
    let PagerBinding {
        id,
        page_count_getter: binding_page_count_getter,
        on_page_changed: binding_on_page_changed,
    } = binding;
    let page_count_getter = page_count_getter.or(binding_page_count_getter).expect(
        "Standalone pagers require `page_count_getter(...)` unless built from a `Scroll` widget",
    );
    (
        id,
        page_count_getter,
        on_page_changed.or(binding_on_page_changed),
    )
}

pub struct SwitchPage<LabelRenderer, Label> {
    id: Cow<'static, str>,
    direction: PageDirection,
    page_count_getter: DynPageCountGetter,
    label_renderer: LabelRenderer,
    on_page_changed: Option<OnPageChanged>,
    when: Option<WhenCondition>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> Label>,
}

#[bon]
impl<LabelRenderer, Label> SwitchPage<LabelRenderer, Label> {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] binding: PagerBinding,
        direction: PageDirection,
        #[builder(with = |page_count_getter: impl for<'a> Fn(&RenderContext<'a>) -> usize + Send + Sync + 'static| {
            Box::new(page_count_getter)
        })]
        page_count_getter: Option<DynPageCountGetter>,
        label_renderer: LabelRenderer,
        on_page_changed: Option<OnPageChanged>,
        when: Option<WhenCondition>,
    ) -> Self
    where
        LabelRenderer: Fn(usize, usize, &DataMap) -> Label,
        Label: Into<Box<str>>,
    {
        let (id, page_count_getter, on_page_changed) =
            resolve_pager_binding(binding, page_count_getter, on_page_changed);
        Self {
            id,
            direction,
            page_count_getter,
            label_renderer,
            on_page_changed,
            when,
            marker: PhantomData,
        }
    }
}

impl<LabelRenderer, Label> Keyboard for SwitchPage<LabelRenderer, Label>
where
    LabelRenderer: Fn(usize, usize, &DataMap) -> Label + Send + Sync + 'static,
    Label: Into<Box<str>> + 'static,
{
    fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context;
        let data = render_ctx.data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        render_direction_button(
            render_ctx,
            &self.id,
            &self.page_count_getter,
            self.direction,
            &self.label_renderer,
        )
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        handle_pager_callback(
            ctx,
            self.id.as_ref(),
            click.callback_data,
            self.on_page_changed.as_ref(),
        )
    }
}

macro_rules! fixed_pager_type {
    ($name:ident, $direction:expr, $default_label:literal) => {
        pub struct $name {
            id: Cow<'static, str>,
            page_count_getter: DynPageCountGetter,
            label: Cow<'static, str>,
            on_page_changed: Option<OnPageChanged>,
            when: Option<WhenCondition>,
        }

        #[bon]
        impl $name {
            #[builder]
            #[must_use]
            pub fn new(
                #[builder(start_fn, into)] binding: PagerBinding,
                #[builder(with = |page_count_getter: impl for<'a> Fn(&RenderContext<'a>) -> usize + Send + Sync + 'static| {
                    Box::new(page_count_getter)
                })]
                page_count_getter: Option<DynPageCountGetter>,
                #[builder(default = $default_label.into())] label: Cow<'static, str>,
                on_page_changed: Option<OnPageChanged>,
                when: Option<WhenCondition>,
            ) -> Self {
                let (id, page_count_getter, on_page_changed) =
                    resolve_pager_binding(binding, page_count_getter, on_page_changed);
                Self {
                    id,
                    page_count_getter,
                    label,
                    on_page_changed,
                    when,
                }
            }
        }

        impl Keyboard for $name {
            fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
                is_allowed(self.when.as_ref(), ctx, data)
            }

            fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
                let ctx = render_ctx.context;
                let data = render_ctx.data;
                if !self.is_visible(ctx, data) {
                    return None;
                }
                render_direction_button(
                    render_ctx,
                    &self.id,
                    &self.page_count_getter,
                    $direction,
                    &|_target, _current, _data| self.label.clone(),
                )
            }

            fn handle_callback(
                &self,
                click: &ClickContext<'_>,
            ) -> Option<ButtonAction> {
                let ctx = click.context;
                let data = &ctx.dialog_data;
                if !self.is_visible(ctx, data) {
                    return None;
                }
                handle_pager_callback(
                    ctx,
                    self.id.as_ref(),
                    click.callback_data,
                    self.on_page_changed.as_ref(),
                )
            }
        }
    };
}

fixed_pager_type!(FirstPage, PageDirection::First, "<<");
fixed_pager_type!(PrevPage, PageDirection::Prev, "<");
fixed_pager_type!(NextPage, PageDirection::Next, ">");
fixed_pager_type!(LastPage, PageDirection::Last, ">>");

pub struct CurrentPage {
    id: Cow<'static, str>,
    page_count_getter: DynPageCountGetter,
    on_page_changed: Option<OnPageChanged>,
    when: Option<WhenCondition>,
}

#[bon]
impl CurrentPage {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] binding: PagerBinding,
        #[builder(with = |page_count_getter: impl for<'a> Fn(&RenderContext<'a>) -> usize + Send + Sync + 'static| {
            Box::new(page_count_getter)
        })]
        page_count_getter: Option<DynPageCountGetter>,
        on_page_changed: Option<OnPageChanged>,
        when: Option<WhenCondition>,
    ) -> Self {
        let (id, page_count_getter, on_page_changed) =
            resolve_pager_binding(binding, page_count_getter, on_page_changed);
        Self {
            id,
            page_count_getter,
            on_page_changed,
            when,
        }
    }
}

impl Keyboard for CurrentPage {
    fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context;
        let data = render_ctx.data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        let pages_count = (self.page_count_getter)(render_ctx);
        if pages_count == 0 {
            return None;
        }

        let current_page = read_page(ctx, self.id.as_ref()).min(pages_count.saturating_sub(1));
        let button = InlineKeyboardButton::new(format!("{}", current_page + 1)).callback_data(
            format_callback_data(ctx, self.id.as_ref(), Some(&format!("{}", current_page))),
        );

        Some(InlineKeyboardMarkup::new(vec![vec![button].into_boxed_slice()]).into())
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        handle_pager_callback(
            ctx,
            self.id.as_ref(),
            click.callback_data,
            self.on_page_changed.as_ref(),
        )
    }
}

pub struct NumberedPager<PageRenderer, CurrentPageRenderer, PageLabel, CurrentPageLabel> {
    id: Cow<'static, str>,
    page_count_getter: DynPageCountGetter,
    page_renderer: PageRenderer,
    current_page_renderer: CurrentPageRenderer,
    length: Option<usize>,
    on_page_changed: Option<OnPageChanged>,
    when: Option<WhenCondition>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (PageLabel, CurrentPageLabel)>,
}

#[bon]
impl<PageRenderer, CurrentPageRenderer, PageLabel, CurrentPageLabel>
    NumberedPager<PageRenderer, CurrentPageRenderer, PageLabel, CurrentPageLabel>
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] binding: PagerBinding,
        #[builder(with = |page_count_getter: impl for<'a> Fn(&RenderContext<'a>) -> usize + Send + Sync + 'static| {
            Box::new(page_count_getter)
        })]
        page_count_getter: Option<DynPageCountGetter>,
        page_renderer: PageRenderer,
        current_page_renderer: CurrentPageRenderer,
        length: Option<usize>,
        on_page_changed: Option<OnPageChanged>,
        when: Option<WhenCondition>,
    ) -> Self
    where
        PageRenderer: Fn(usize, &DataMap) -> PageLabel,
        CurrentPageRenderer: Fn(usize, &DataMap) -> CurrentPageLabel,
        PageLabel: Into<Box<str>>,
        CurrentPageLabel: Into<Box<str>>,
    {
        let (id, page_count_getter, on_page_changed) =
            resolve_pager_binding(binding, page_count_getter, on_page_changed);
        Self {
            id,
            page_count_getter,
            page_renderer,
            current_page_renderer,
            length,
            on_page_changed,
            when,
            marker: PhantomData,
        }
    }
}

impl<PageRenderer, CurrentPageRenderer, PageLabel, CurrentPageLabel> Keyboard
    for NumberedPager<PageRenderer, CurrentPageRenderer, PageLabel, CurrentPageLabel>
where
    PageRenderer: Fn(usize, &DataMap) -> PageLabel + Send + Sync + 'static,
    CurrentPageRenderer: Fn(usize, &DataMap) -> CurrentPageLabel + Send + Sync + 'static,
    PageLabel: Into<Box<str>> + 'static,
    CurrentPageLabel: Into<Box<str>> + 'static,
{
    fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context;
        let data = render_ctx.data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        let pages_count = (self.page_count_getter)(render_ctx);
        if pages_count == 0 {
            return None;
        }

        let current_page = read_page(ctx, self.id.as_ref()).min(pages_count.saturating_sub(1));
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        let row_len = self.length.unwrap_or(pages_count).max(1);

        for page in 0..pages_count {
            let label = if page == current_page {
                (self.current_page_renderer)(page, data).into()
            } else {
                (self.page_renderer)(page, data).into()
            };
            current_row.push(
                InlineKeyboardButton::new(label).callback_data(format_callback_data(
                    ctx,
                    self.id.as_ref(),
                    Some(&format!("{}", page)),
                )),
            );

            if current_row.len() == row_len {
                rows.push(std::mem::take(&mut current_row).into_boxed_slice());
            }
        }

        if !current_row.is_empty() {
            rows.push(current_row.into_boxed_slice());
        }

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        handle_pager_callback(
            ctx,
            self.id.as_ref(),
            click.callback_data,
            self.on_page_changed.as_ref(),
        )
    }
}
