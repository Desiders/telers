use serde_json::Value;
use std::{borrow::Cow, fmt::Display, sync::Arc};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{format_callback_data, parse_callback_data, ButtonAction};
use crate::entities::{Context, DataMap};

type PageChangedHandler = dyn Fn(PageChange) -> ButtonAction + Send + Sync + 'static;

/// Details about a pager state transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageChange {
    /// Pager widget id that handled the callback.
    pub widget_id: Box<str>,
    /// Page stored before the callback was applied.
    pub old_page: usize,
    /// Page requested by the callback.
    pub new_page: usize,
}

/// Side-effect hook executed after pager widgets resolve a page change.
#[derive(Clone)]
pub struct OnPageChanged(pub(super) Arc<PageChangedHandler>);

impl OnPageChanged {
    /// Create a page-change hook with access to the widget id and old/new page.
    #[must_use]
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(PageChange) -> ButtonAction + Send + Sync + 'static,
    {
        Self(Arc::new(handler))
    }

    #[must_use]
    pub(super) fn call(&self, change: PageChange) -> ButtonAction {
        (self.0)(change)
    }
}

/// Build a hook that copies the new page into another scroll widget id.
#[must_use]
pub fn sync_scroll(scroll_id: impl Into<Cow<'static, str>>) -> OnPageChanged {
    let scroll_id = scroll_id.into();
    OnPageChanged::new(move |change| {
        ButtonAction::set_widget_value(scroll_id.clone(), change.new_page)
    })
}

/// Build a hook that copies the new page into multiple scroll widget ids.
#[must_use]
pub fn sync_scrolls<T>(scroll_ids: impl IntoIterator<Item = T>) -> OnPageChanged
where
    T: Into<Cow<'static, str>>,
{
    let scroll_ids = scroll_ids
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Cow<'static, str>>>();
    OnPageChanged::new(move |change| {
        ButtonAction::chain(
            scroll_ids
                .iter()
                .cloned()
                .map(|id| ButtonAction::set_widget_value(id, change.new_page)),
        )
    })
}

/// Logical direction used by standalone pager buttons.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PageDirection {
    First,
    Prev,
    Current,
    Next,
    Last,
}

#[inline]
#[must_use]
pub(super) fn resolve_page_target(
    direction: PageDirection,
    current_page: usize,
    pages_count: usize,
) -> usize {
    let last_page = pages_count.saturating_sub(1);
    match direction {
        PageDirection::First => 0,
        PageDirection::Prev => current_page.saturating_sub(1),
        PageDirection::Current => current_page,
        PageDirection::Next => (current_page + 1).min(last_page),
        PageDirection::Last => last_page,
    }
}

pub(super) fn render_fixed_direction_button<WidgetId, PageCountGetter>(
    ctx: &Context,
    data: &DataMap,
    id: &WidgetId,
    page_count_getter: &PageCountGetter,
    direction: PageDirection,
    label: &str,
) -> Option<ReplyMarkup>
where
    WidgetId: Display,
    PageCountGetter: Fn(&DataMap) -> usize,
{
    let pages_count = page_count_getter(data);
    if pages_count == 0 {
        return None;
    }

    let widget_id = id.to_string();
    let current_page = read_page(ctx, &widget_id).min(pages_count.saturating_sub(1));
    let target_page = resolve_page_target(direction, current_page, pages_count);
    let button = InlineKeyboardButton::new(label).callback_data(format_callback_data(
        ctx,
        id,
        Some(&target_page.to_string()),
    ));

    Some(InlineKeyboardMarkup::new(vec![vec![button].into_boxed_slice()]).into())
}

#[inline]
#[must_use]
pub(super) fn build_pager_row(
    ctx: &Context,
    widget_id: &str,
    current_page: usize,
    pages_count: usize,
) -> Box<[InlineKeyboardButton]> {
    let last_page = pages_count - 1;
    let prev_page = current_page.saturating_sub(1);
    let next_page = (current_page + 1).min(last_page);
    [
        InlineKeyboardButton::new("1").callback_data(format_callback_data(
            ctx,
            widget_id,
            Some("0"),
        )),
        InlineKeyboardButton::new("<").callback_data(format_callback_data(
            ctx,
            widget_id,
            Some(&prev_page.to_string()),
        )),
        InlineKeyboardButton::new((current_page + 1).to_string()).callback_data(
            format_callback_data(ctx, widget_id, Some(&current_page.to_string())),
        ),
        InlineKeyboardButton::new(">").callback_data(format_callback_data(
            ctx,
            widget_id,
            Some(&next_page.to_string()),
        )),
        InlineKeyboardButton::new(pages_count.to_string()).callback_data(format_callback_data(
            ctx,
            widget_id,
            Some(&last_page.to_string()),
        )),
    ]
    .into()
}

pub(super) fn build_filler_button(
    ctx: &Context,
    widget_id: &str,
    current_page: usize,
    filler_text: &str,
) -> InlineKeyboardButton {
    // Telegram inline keyboards do not support disabled buttons, so filler
    // slots point to the current page and behave as inert placeholders.
    InlineKeyboardButton::new(filler_text).callback_data(format_callback_data(
        ctx,
        widget_id,
        Some(&current_page.to_string()),
    ))
}

pub(super) fn render_fixed_width_page(
    ctx: &Context,
    widget_id: &str,
    rows: &[Box<[InlineKeyboardButton]>],
    width: usize,
    height: usize,
    filler_text: &str,
) -> Option<(Vec<Box<[InlineKeyboardButton]>>, usize)> {
    let width = width.max(1);
    let page_capacity = width * height;
    if page_capacity == 0 {
        return None;
    }

    let buttons = rows
        .iter()
        .flat_map(|row| row.iter().cloned())
        .collect::<Vec<_>>();
    if buttons.is_empty() {
        return None;
    }

    let pages_count = page_count_from_rows(buttons.len(), page_capacity);
    let current_page = read_page(ctx, widget_id).min(pages_count.saturating_sub(1));
    let start = current_page * page_capacity;
    let end = (start + page_capacity).min(buttons.len());
    let mut page_buttons = buttons[start..end].to_vec();

    while page_buttons.len() < page_capacity {
        page_buttons.push(build_filler_button(
            ctx,
            widget_id,
            current_page,
            filler_text,
        ));
    }

    let rows = page_buttons
        .chunks(width)
        .map(|chunk| chunk.to_vec().into_boxed_slice())
        .collect::<Vec<_>>();
    Some((rows, pages_count))
}

pub(super) fn handle_pager_callback(
    ctx: &Context,
    widget_id: &str,
    callback_data: &str,
    on_page_changed: Option<&OnPageChanged>,
) -> Option<ButtonAction> {
    let parsed = parse_callback_data(ctx, callback_data)?;
    if parsed.target_id != widget_id {
        return None;
    }

    let old_page = read_page(ctx, widget_id);
    let page: usize = parsed.payload?.parse().ok()?;
    debug!(
        context_id = %ctx.id,
        widget_id,
        old_page,
        page,
        "Resolved pager navigation callback"
    );
    let current_action =
        ButtonAction::set_widget_value(widget_id.to_owned(), Value::Number(page.into()));
    Some(match on_page_changed {
        Some(on_page_changed) => ButtonAction::chain([
            current_action,
            on_page_changed.call(PageChange {
                widget_id: widget_id.into(),
                old_page,
                new_page: page,
            }),
        ]),
        None => current_action,
    })
}

#[inline]
#[must_use]
pub(super) fn read_page(ctx: &Context, widget_id: &str) -> usize {
    ctx.widget_value_as::<usize>(widget_id).unwrap_or(0)
}

#[inline]
#[must_use]
pub(super) fn page_count_from_rows(total_rows: usize, height: usize) -> usize {
    total_rows / height + usize::from(total_rows % height != 0)
}
