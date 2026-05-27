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

/// Single-button cyclic toggle backed by `widget_data`.
///
/// The widget displays only the currently selected item; clicking it stores
/// the next item id from `items_getter`. When no value is stored yet, the
/// first item is shown.
pub struct Toggle<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id> {
    id: WidgetId,
    items_getter: ItemsGetter,
    item_renderer: ItemRenderer,
    id_getter: IdGetter,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    when: Option<WhenCondition>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr, Id)>,
}

#[bon]
impl<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
    Toggle<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
{
    /// Build a [`Toggle`] widget.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        items_getter: ItemsGetter,
        item_renderer: ItemRenderer,
        id_getter: IdGetter,
        when: Option<WhenCondition>,
    ) -> Self
    where
        WidgetId: Display,
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        ItemRenderer: Fn(&Item, &DataMap) -> ItemStr,
        ItemStr: Into<Box<str>>,
        IdGetter: Fn(&Item) -> Id,
        Id: Display,
    {
        Self {
            id,
            items_getter,
            item_renderer,
            id_getter,
            header_rows,
            footer_rows,
            when,
            marker: PhantomData,
        }
    }
}

impl<S, WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
    ToggleBuilder<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id, S>
where
    S: toggle_builder::State,
    WidgetId: Display,
    ItemsGetter: Fn(&DataMap) -> ItemsIter,
    ItemsIter: IntoIterator<Item = Item>,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr,
    ItemStr: Into<Box<str>>,
    IdGetter: Fn(&Item) -> Id,
    Id: Display,
{
    impl_button_row_helpers!();
}

#[async_trait]
impl<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id> Keyboard
    for Toggle<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
where
    WidgetId: Display + Send + Sync + 'static,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: Send + 'static,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
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
        let selected: Option<String> = ctx.widget_value_as(&widget_id);
        let items: Vec<_> = (self.items_getter)(data).into_iter().collect();
        if items.is_empty() && self.header_rows.is_empty() && self.footer_rows.is_empty() {
            return None;
        }

        let mut rows = Vec::new();
        for row in &self.header_rows {
            rows.push(render_button_row(row, render_ctx).await);
        }

        if !items.is_empty() {
            let current_idx = selected
                .as_deref()
                .and_then(|selected| {
                    items
                        .iter()
                        .position(|item| (self.id_getter)(item).to_string() == selected)
                })
                .unwrap_or(0);
            let next_idx = (current_idx + 1) % items.len();
            let current_item = &items[current_idx];
            let next_item_id = (self.id_getter)(&items[next_idx]).to_string();
            rows.push(
                [
                    InlineKeyboardButton::new((self.item_renderer)(current_item, data))
                        .callback_data(format_callback_data(ctx, &self.id, Some(&next_item_id))),
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
        debug!(
            context_id = %ctx.id,
            widget_id = %self.id,
            item_id = payload,
            "Resolved toggle selection callback"
        );
        Some(ButtonAction::set_widget_value(self.id.to_string(), payload))
    }
}
