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

/// Inline-keyboard radio group that stores one selected item id in `widget_data`.
///
/// On click the widget writes the picked item id under its own `widget_id`
/// using [`ButtonAction::SetWidgetValue`]. The next render compares stored id
/// against each item's id and picks either `checked_renderer` or
/// `unchecked_renderer` for the label.
///
/// [`ButtonAction::SetWidgetValue`]: crate::widgets::ButtonAction::SetWidgetValue
pub struct Radio<
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
    Radio<
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
    /// Build a [`Radio`] widget.
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
    RadioBuilder<
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
    S: radio_builder::State,
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
    for Radio<
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
        let checked: Option<String> = ctx.widget_value_as(&widget_id);

        let mut rows = Vec::new();
        for row in &self.header_rows {
            rows.push(render_button_row(row, render_ctx).await);
        }

        for item in (self.items_getter)(data) {
            let item_id = (self.id_getter)(&item).to_string();
            let is_checked = checked.as_deref() == Some(item_id.as_str());
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
        debug!(
            context_id = %ctx.id,
            widget_id = %self.id,
            item_id = payload,
            "Resolved radio selection callback"
        );
        Some(ButtonAction::set_widget_value(self.id.to_string(), payload))
    }
}
