use async_trait::async_trait;
use bon::bon;
use std::{fmt::Display, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{
    format_callback_data, macros::impl_button_row_helpers, parse_callback_data, render_button_row,
    when::is_allowed, Button, ButtonAction, ClickContext, Keyboard, WhenCondition,
};
use crate::entities::{Context, DataMap, RenderContext};

/// Inline-keyboard multiselect that stores the list of selected ids in `widget_data`.
///
/// The widget reads its current selection (an array of strings) from
/// `widget_data[id]`, renders one button per item, and on click toggles the
/// corresponding id while respecting the optional `min_selected` / `max_selected`
/// bounds.
///
/// When a toggle would violate the bounds, the click returns
/// [`ButtonAction::Noop`] so the click is consumed without changing state.
///
/// [`ButtonAction::Noop`]: crate::widgets::ButtonAction::Noop
pub struct Multiselect<
    WidgetId,
    ItemsGetter,
    ItemsIter,
    Item,
    CheckedRenderer,
    UncheckedRenderer,
    ItemStr,
    IdGetter,
    Id,
> {
    id: WidgetId,
    items_getter: ItemsGetter,
    checked_renderer: CheckedRenderer,
    unchecked_renderer: UncheckedRenderer,
    id_getter: IdGetter,
    min_selected: usize,
    max_selected: usize,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    when: Option<WhenCondition>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr, Id)>,
}

#[bon]
impl<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
    Multiselect<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
{
    /// Build a [`Multiselect`] widget. `min_selected = 0` disables the lower
    /// bound; `max_selected = 0` disables the upper bound.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        items_getter: ItemsGetter,
        checked_renderer: CheckedRenderer,
        unchecked_renderer: UncheckedRenderer,
        id_getter: IdGetter,
        #[builder(default = 0)] min_selected: usize,
        #[builder(default = 0)] max_selected: usize,
        when: Option<WhenCondition>,
    ) -> Self
    where
        WidgetId: Display,
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
        UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
        ItemStr: Into<Box<str>>,
        IdGetter: Fn(&Item) -> Id,
        Id: Display,
    {
        Self {
            id,
            items_getter,
            checked_renderer,
            unchecked_renderer,
            id_getter,
            min_selected,
            max_selected,
            header_rows,
            footer_rows,
            when,
            marker: PhantomData,
        }
    }
}

impl<
        S,
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
    MultiselectBuilder<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
        S,
    >
where
    S: multiselect_builder::State,
    WidgetId: Display,
    ItemsGetter: Fn(&DataMap) -> ItemsIter,
    ItemsIter: IntoIterator<Item = Item>,
    CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
    UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr,
    ItemStr: Into<Box<str>>,
    IdGetter: Fn(&Item) -> Id,
    Id: Display,
{
    impl_button_row_helpers!();
}

#[async_trait]
impl<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    > Keyboard
    for Multiselect<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        CheckedRenderer,
        UncheckedRenderer,
        ItemStr,
        IdGetter,
        Id,
    >
where
    WidgetId: Display + Send + Sync + 'static,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    CheckedRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    UncheckedRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
    IdGetter: Fn(&Item) -> Id + Send + Sync + 'static,
    Id: Display + Send + Sync + 'static,
{
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context.as_ref();
        let data = render_ctx.data.as_ref();
        if !self.is_visible(ctx, data).await {
            return None;
        }
        let widget_id = self.id.to_string();
        let checked = read_checked_list(ctx, &widget_id);

        let mut rows = Vec::new();
        for row in &self.header_rows {
            rows.push(render_button_row(row, render_ctx).await);
        }

        for item in (self.items_getter)(data) {
            let item_id = (self.id_getter)(&item).to_string();
            let is_checked = checked.iter().any(|id| id == &item_id);
            let text = if is_checked {
                (self.checked_renderer)(&item, data)
            } else {
                (self.unchecked_renderer)(&item, data)
            };
            rows.push(
                [
                    InlineKeyboardButton::new(text).callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some(&item_id),
                    )),
                ]
                .into(),
            );
        }

        for row in &self.footer_rows {
            rows.push(render_button_row(row, render_ctx).await);
        }

        if rows.is_empty() {
            None
        } else {
            Some(InlineKeyboardMarkup::new(rows).into())
        }
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        let callback_data = click.callback_data.as_str();
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data).await {
            return None;
        }
        for button in self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
        {
            if let Some(action) = button.resolve_callback(click).await {
                return Some(action);
            }
        }

        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.to_string() {
            return None;
        }
        let payload = parsed.payload?;
        let widget_id = self.id.to_string();
        let mut checked = read_checked_list(ctx, &widget_id);

        if let Some(pos) = checked.iter().position(|id| id == payload) {
            if self.min_selected > 0 && checked.len() <= self.min_selected {
                debug!(
                    context_id = %ctx.id,
                    widget_id = %self.id,
                    item_id = payload,
                    min = self.min_selected,
                    "Multiselect min_selected constraint prevents uncheck"
                );
                return Some(ButtonAction::noop());
            }
            checked.remove(pos);
        } else {
            if self.max_selected > 0 && checked.len() >= self.max_selected {
                debug!(
                    context_id = %ctx.id,
                    widget_id = %self.id,
                    item_id = payload,
                    max = self.max_selected,
                    "Multiselect max_selected constraint prevents check"
                );
                return Some(ButtonAction::noop());
            }
            checked.push(payload.to_owned());
        }

        debug!(
            context_id = %ctx.id,
            widget_id = %self.id,
            item_id = payload,
            count = checked.len(),
            "Resolved multiselect toggle callback"
        );
        Some(ButtonAction::set_widget_value(
            widget_id,
            serde_json::Value::Array(checked.into_iter().map(serde_json::Value::String).collect()),
        ))
    }
}

#[inline]
#[must_use]
fn read_checked_list(ctx: &Context, widget_id: &str) -> Vec<String> {
    ctx.widget_value_as::<Vec<String>>(widget_id)
        .unwrap_or_default()
}
