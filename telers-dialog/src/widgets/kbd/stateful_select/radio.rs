use bon::bon;
use std::{fmt::Display, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{
    format_callback_data, parse_callback_data, render_button_row, Button, ButtonAction, Keyboard,
};
use crate::entities::{Context, DataMap};

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
    pub fn header_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.header_rows.push(buttons.into_iter().collect());
        self
    }

    pub fn header_push(mut self, button: Button) -> Self {
        match self.header_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.header_rows.push(vec![button]),
        }
        self
    }

    pub fn footer_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.footer_rows.push(buttons.into_iter().collect());
        self
    }

    pub fn footer_push(mut self, button: Button) -> Self {
        match self.footer_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.footer_rows.push(vec![button]),
        }
        self
    }
}

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
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let widget_id = self.id.to_string();
        let checked: Option<String> = ctx.widget_value_as(&widget_id);

        let mut rows: Vec<_> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, ctx, data))
            .collect();

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

        rows.extend(
            self.footer_rows
                .iter()
                .map(|row| render_button_row(row, ctx, data)),
        );

        if rows.is_empty() {
            None
        } else {
            Some(InlineKeyboardMarkup::new(rows).into())
        }
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        if let Some(action) = self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(ctx, callback_data))
        {
            return Some(action);
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
