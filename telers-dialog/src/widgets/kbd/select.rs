use bon::bon;
use std::{fmt::Display, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::{format_callback_data, parse_callback_data, Button, ButtonAction, Keyboard};
use crate::entities::{Context, DataMap};

pub struct Select<
    WidgetId,
    ItemsGetter,
    ItemsIter,
    Item,
    ItemRenderer,
    ItemStr,
    IdGetter,
    Id,
    Action,
> {
    id: WidgetId,
    items_getter: ItemsGetter,
    item_renderer: ItemRenderer,
    id_getter: IdGetter,
    action: Action,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr, Id)>,
}

#[bon]
impl<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id, Action>
    Select<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id, Action>
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        items_getter: ItemsGetter,
        item_renderer: ItemRenderer,
        id_getter: IdGetter,
        action: Action,
    ) -> Self
    where
        WidgetId: Display,
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        ItemRenderer: Fn(&Item, &DataMap) -> ItemStr,
        ItemStr: Into<Box<str>>,
        IdGetter: Fn(Item) -> Id,
        Id: Display,
        Action: Fn(&str) -> ButtonAction,
    {
        Self {
            id,
            items_getter,
            item_renderer,
            id_getter,
            action,
            header_rows,
            footer_rows,
            marker: PhantomData,
        }
    }
}

impl<S, WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id, Action>
    SelectBuilder<
        WidgetId,
        ItemsGetter,
        ItemsIter,
        Item,
        ItemRenderer,
        ItemStr,
        IdGetter,
        Id,
        Action,
        S,
    >
where
    S: select_builder::State,
    WidgetId: Display,
    ItemsGetter: Fn(&DataMap) -> ItemsIter,
    ItemsIter: IntoIterator<Item = Item>,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr,
    ItemStr: Into<Box<str>>,
    IdGetter: Fn(Item) -> Id,
    Id: Display,
    Action: Fn(&str) -> ButtonAction,
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

impl<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id, Action> Keyboard
    for Select<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id, Action>
where
    WidgetId: Display + Send + Sync + 'static,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
    IdGetter: Fn(Item) -> Id + Send + Sync + 'static,
    Id: Display + Send + Sync + 'static,
    Action: Fn(&str) -> ButtonAction + Send + Sync + 'static,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let mut rows: Vec<_> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, ctx, data))
            .collect();

        for item in (self.items_getter)(data) {
            let text = (self.item_renderer)(&item, data);
            let payload = (self.id_getter)(item).to_string();
            rows.push(
                [
                    InlineKeyboardButton::new(text).callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some(&payload),
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
        debug!(context_id = %ctx.id, widget_id = %self.id, "Resolved select callback");
        Some((self.action)(payload))
    }
}

pub(crate) fn render_button_row(
    row: &[Button],
    ctx: &Context,
    data: &DataMap,
) -> Box<[InlineKeyboardButton]> {
    row.iter().map(|button| button.render(ctx, data)).collect()
}
