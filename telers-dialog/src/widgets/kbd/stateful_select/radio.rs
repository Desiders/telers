use bon::bon;
use std::{fmt::Display, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::super::{
    format_callback_data, parse_callback_data, render_button_row, when::is_allowed, Button,
    ButtonAction, ClickContext, Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

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
    fn is_visible<'a>(&'a self, ctx: &'a Context, data: &'a DataMap) -> BoxFuture<'a, bool> {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, Option<ReplyMarkup>> {
        Box::pin(async move {
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
        })
    }

    fn handle_callback<'a>(
        &'a self,
        click: &'a ClickContext,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
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
        })
    }
}
