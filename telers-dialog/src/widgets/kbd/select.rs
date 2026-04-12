use bon::bon;
use std::{fmt::Display, marker::PhantomData, sync::Arc};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::{
    format_callback_data, parse_callback_data, when::is_allowed, Button, ButtonAction,
    ClickContext, Keyboard, WhenCondition,
};
use crate::entities::{Context, DataMap, RenderContext};

type SelectActionHandler = dyn Fn(&str) -> ButtonAction + Send + Sync + 'static;
type SelectClickHandler =
    dyn for<'a> Fn(&ClickContext<'a>, &str) -> ButtonAction + Send + Sync + 'static;

#[derive(Clone)]
enum SelectAction {
    Action(Arc<SelectActionHandler>),
    OnClick(Arc<SelectClickHandler>),
}

impl SelectAction {
    fn action(handler: impl Fn(&str) -> ButtonAction + Send + Sync + 'static) -> Self {
        Self::Action(Arc::new(handler))
    }

    fn on_click(
        handler: impl for<'a> Fn(&ClickContext<'a>, &str) -> ButtonAction + Send + Sync + 'static,
    ) -> Self {
        Self::OnClick(Arc::new(handler))
    }

    fn call(&self, click: &ClickContext<'_>, payload: &str) -> ButtonAction {
        match self {
            Self::Action(handler) => handler(payload),
            Self::OnClick(handler) => handler(click, payload),
        }
    }
}

/// Stateless list of selectable items.
///
/// Each rendered item produces a callback payload derived from `id_getter`, and
/// that payload is converted into a [`ButtonAction`] by `action` or `on_click`.
#[derive(Clone)]
pub struct Select<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id> {
    id: WidgetId,
    items_getter: ItemsGetter,
    item_renderer: ItemRenderer,
    id_getter: IdGetter,
    action: SelectAction,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    when: Option<WhenCondition>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, ItemStr, Id)>,
}

#[bon]
impl<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
    Select<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
{
    #[builder]
    #[must_use]
    /// Build a [`Select`] widget.
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] header_rows: Vec<Vec<Button>>,
        #[builder(field)] footer_rows: Vec<Vec<Button>>,
        items_getter: ItemsGetter,
        item_renderer: ItemRenderer,
        id_getter: IdGetter,
        #[builder(with = |action: impl Fn(&str) -> ButtonAction + Send + Sync + 'static| {
            SelectAction::action(action)
        })]
        action: Option<SelectAction>,
        #[builder(with = |on_click: impl for<'a> Fn(&ClickContext<'a>, &str) -> ButtonAction + Send + Sync + 'static| {
            SelectAction::on_click(on_click)
        })]
        on_click: Option<SelectAction>,
        when: Option<WhenCondition>,
    ) -> Self
    where
        WidgetId: Display,
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        ItemRenderer: Fn(&Item, &DataMap) -> ItemStr,
        ItemStr: Into<Box<str>>,
        IdGetter: Fn(Item) -> Id,
        Id: Display,
    {
        let action = on_click
            .or(action)
            .expect("Select requires `action` or `on_click`");
        Self {
            id,
            items_getter,
            item_renderer,
            id_getter,
            action,
            header_rows,
            footer_rows,
            when,
            marker: PhantomData,
        }
    }
}

impl<S, WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
    SelectBuilder<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id, S>
where
    S: select_builder::State,
    WidgetId: Display,
    ItemsGetter: Fn(&DataMap) -> ItemsIter,
    ItemsIter: IntoIterator<Item = Item>,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr,
    ItemStr: Into<Box<str>>,
    IdGetter: Fn(Item) -> Id,
    Id: Display,
{
    /// Append a full header row before the selectable items.
    pub fn header_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.header_rows.push(buttons.into_iter().collect());
        self
    }

    /// Append a button to the last header row, or create one if absent.
    pub fn header_push(mut self, button: Button) -> Self {
        match self.header_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.header_rows.push(vec![button]),
        }
        self
    }

    /// Append a full footer row after the selectable items.
    pub fn footer_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.footer_rows.push(buttons.into_iter().collect());
        self
    }

    /// Append a button to the last footer row, or create one if absent.
    pub fn footer_push(mut self, button: Button) -> Self {
        match self.footer_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.footer_rows.push(vec![button]),
        }
        self
    }
}

impl<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id> Keyboard
    for Select<WidgetId, ItemsGetter, ItemsIter, Item, ItemRenderer, ItemStr, IdGetter, Id>
where
    WidgetId: Display + Send + Sync + 'static,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: 'static,
    ItemRenderer: Fn(&Item, &DataMap) -> ItemStr + Send + Sync + 'static,
    ItemStr: Into<Box<str>> + 'static,
    IdGetter: Fn(Item) -> Id + Send + Sync + 'static,
    Id: Display + Send + Sync + 'static,
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
        let mut rows: Vec<_> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, render_ctx))
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
                .map(|row| render_button_row(row, render_ctx)),
        );

        if rows.is_empty() {
            None
        } else {
            Some(InlineKeyboardMarkup::new(rows).into())
        }
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        let callback_data = click.callback_data;
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        if let Some(action) = self
            .header_rows
            .iter()
            .chain(self.footer_rows.iter())
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(click))
        {
            return Some(action);
        }

        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.to_string() {
            return None;
        }
        let payload = parsed.payload?;
        debug!(context_id = %ctx.id, widget_id = %self.id, "Resolved select callback");
        Some(self.action.call(click, payload))
    }
}

pub(crate) fn render_button_row(
    row: &[Button],
    render_ctx: &RenderContext<'_>,
) -> Box<[InlineKeyboardButton]> {
    row.iter().map(|button| button.render(render_ctx)).collect()
}
