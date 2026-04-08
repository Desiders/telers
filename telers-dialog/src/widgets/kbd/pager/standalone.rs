use bon::bon;
use std::{fmt::Display, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

use super::{
    super::{format_callback_data, ButtonAction, Keyboard},
    handle_pager_callback, read_page, render_fixed_direction_button, resolve_page_target,
    OnPageChanged, PageDirection,
};
use crate::entities::{Context, DataMap};

pub struct SwitchPage<WidgetId, PageCountGetter, LabelRenderer, Label> {
    id: WidgetId,
    direction: PageDirection,
    page_count_getter: PageCountGetter,
    label_renderer: LabelRenderer,
    on_page_changed: Option<OnPageChanged>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> Label>,
}

#[bon]
impl<WidgetId, PageCountGetter, LabelRenderer, Label>
    SwitchPage<WidgetId, PageCountGetter, LabelRenderer, Label>
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        direction: PageDirection,
        page_count_getter: PageCountGetter,
        label_renderer: LabelRenderer,
        on_page_changed: Option<OnPageChanged>,
    ) -> Self
    where
        WidgetId: Display,
        PageCountGetter: Fn(&DataMap) -> usize,
        LabelRenderer: Fn(usize, usize, &DataMap) -> Label,
        Label: Into<Box<str>>,
    {
        Self {
            id,
            direction,
            page_count_getter,
            label_renderer,
            on_page_changed,
            marker: PhantomData,
        }
    }
}

impl<WidgetId, PageCountGetter, LabelRenderer, Label> Keyboard
    for SwitchPage<WidgetId, PageCountGetter, LabelRenderer, Label>
where
    WidgetId: Display + Send + Sync + 'static,
    PageCountGetter: Fn(&DataMap) -> usize + Send + Sync + 'static,
    LabelRenderer: Fn(usize, usize, &DataMap) -> Label + Send + Sync + 'static,
    Label: Into<Box<str>> + 'static,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let pages_count = (self.page_count_getter)(data);
        if pages_count == 0 {
            return None;
        }

        let widget_id = self.id.to_string();
        let current_page = read_page(ctx, &widget_id).min(pages_count.saturating_sub(1));
        let target_page = resolve_page_target(self.direction, current_page, pages_count);
        let label = (self.label_renderer)(target_page, current_page, data);

        let button = InlineKeyboardButton::new(label).callback_data(format_callback_data(
            ctx,
            &self.id,
            Some(&target_page.to_string()),
        ));

        Some(InlineKeyboardMarkup::new(vec![vec![button].into_boxed_slice()]).into())
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        handle_pager_callback(
            ctx,
            &self.id.to_string(),
            callback_data,
            self.on_page_changed.as_ref(),
        )
    }
}

macro_rules! fixed_pager_type {
    ($name:ident, $direction:expr, $label:literal) => {
        pub struct $name<WidgetId, PageCountGetter> {
            id: WidgetId,
            page_count_getter: PageCountGetter,
            on_page_changed: Option<OnPageChanged>,
        }

        #[bon]
        impl<WidgetId, PageCountGetter> $name<WidgetId, PageCountGetter> {
            #[builder]
            #[must_use]
            pub fn new(
                #[builder(start_fn)] id: WidgetId,
                page_count_getter: PageCountGetter,
                on_page_changed: Option<OnPageChanged>,
            ) -> Self
            where
                WidgetId: Display,
                PageCountGetter: Fn(&DataMap) -> usize,
            {
                Self {
                    id,
                    page_count_getter,
                    on_page_changed,
                }
            }
        }

        impl<WidgetId, PageCountGetter> Keyboard for $name<WidgetId, PageCountGetter>
        where
            WidgetId: Display + Send + Sync + 'static,
            PageCountGetter: Fn(&DataMap) -> usize + Send + Sync + 'static,
        {
            fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
                render_fixed_direction_button(
                    ctx,
                    data,
                    &self.id,
                    &self.page_count_getter,
                    $direction,
                    $label,
                )
            }

            fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
                handle_pager_callback(
                    ctx,
                    &self.id.to_string(),
                    callback_data,
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

pub struct CurrentPage<WidgetId, PageCountGetter> {
    id: WidgetId,
    page_count_getter: PageCountGetter,
    on_page_changed: Option<OnPageChanged>,
}

#[bon]
impl<WidgetId, PageCountGetter> CurrentPage<WidgetId, PageCountGetter> {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        page_count_getter: PageCountGetter,
        on_page_changed: Option<OnPageChanged>,
    ) -> Self
    where
        WidgetId: Display,
        PageCountGetter: Fn(&DataMap) -> usize,
    {
        Self {
            id,
            page_count_getter,
            on_page_changed,
        }
    }
}

impl<WidgetId, PageCountGetter> Keyboard for CurrentPage<WidgetId, PageCountGetter>
where
    WidgetId: Display + Send + Sync + 'static,
    PageCountGetter: Fn(&DataMap) -> usize + Send + Sync + 'static,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let pages_count = (self.page_count_getter)(data);
        if pages_count == 0 {
            return None;
        }

        let widget_id = self.id.to_string();
        let current_page = read_page(ctx, &widget_id).min(pages_count.saturating_sub(1));
        let button = InlineKeyboardButton::new((current_page + 1).to_string()).callback_data(
            format_callback_data(ctx, &self.id, Some(&current_page.to_string())),
        );

        Some(InlineKeyboardMarkup::new(vec![vec![button].into_boxed_slice()]).into())
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        handle_pager_callback(
            ctx,
            &self.id.to_string(),
            callback_data,
            self.on_page_changed.as_ref(),
        )
    }
}

pub struct NumberedPager<
    WidgetId,
    PageCountGetter,
    PageRenderer,
    CurrentPageRenderer,
    PageLabel,
    CurrentPageLabel,
> {
    id: WidgetId,
    page_count_getter: PageCountGetter,
    page_renderer: PageRenderer,
    current_page_renderer: CurrentPageRenderer,
    length: Option<usize>,
    on_page_changed: Option<OnPageChanged>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (PageLabel, CurrentPageLabel)>,
}

#[bon]
impl<WidgetId, PageCountGetter, PageRenderer, CurrentPageRenderer, PageLabel, CurrentPageLabel>
    NumberedPager<
        WidgetId,
        PageCountGetter,
        PageRenderer,
        CurrentPageRenderer,
        PageLabel,
        CurrentPageLabel,
    >
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        page_count_getter: PageCountGetter,
        page_renderer: PageRenderer,
        current_page_renderer: CurrentPageRenderer,
        length: Option<usize>,
        on_page_changed: Option<OnPageChanged>,
    ) -> Self
    where
        WidgetId: Display,
        PageCountGetter: Fn(&DataMap) -> usize,
        PageRenderer: Fn(usize, &DataMap) -> PageLabel,
        CurrentPageRenderer: Fn(usize, &DataMap) -> CurrentPageLabel,
        PageLabel: Into<Box<str>>,
        CurrentPageLabel: Into<Box<str>>,
    {
        Self {
            id,
            page_count_getter,
            page_renderer,
            current_page_renderer,
            length,
            on_page_changed,
            marker: PhantomData,
        }
    }
}

impl<WidgetId, PageCountGetter, PageRenderer, CurrentPageRenderer, PageLabel, CurrentPageLabel>
    Keyboard
    for NumberedPager<
        WidgetId,
        PageCountGetter,
        PageRenderer,
        CurrentPageRenderer,
        PageLabel,
        CurrentPageLabel,
    >
where
    WidgetId: Display + Send + Sync + 'static,
    PageCountGetter: Fn(&DataMap) -> usize + Send + Sync + 'static,
    PageRenderer: Fn(usize, &DataMap) -> PageLabel + Send + Sync + 'static,
    CurrentPageRenderer: Fn(usize, &DataMap) -> CurrentPageLabel + Send + Sync + 'static,
    PageLabel: Into<Box<str>> + 'static,
    CurrentPageLabel: Into<Box<str>> + 'static,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let pages_count = (self.page_count_getter)(data);
        if pages_count == 0 {
            return None;
        }

        let widget_id = self.id.to_string();
        let current_page = read_page(ctx, &widget_id).min(pages_count.saturating_sub(1));
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
                    &self.id,
                    Some(&page.to_string()),
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

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        handle_pager_callback(
            ctx,
            &self.id.to_string(),
            callback_data,
            self.on_page_changed.as_ref(),
        )
    }
}
