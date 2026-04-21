use async_fn_traits::AsyncFn1;
use bon::bon;
use std::{fmt::Display, marker::PhantomData, sync::Arc};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::{
    format_callback_data, parse_callback_data, when::is_allowed, Button, ButtonAction,
    ClickContext, Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

type SelectActionHandler =
    dyn Fn(String) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;
type SelectClickHandler =
    dyn Fn(SelectClickContext) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;

/// Runtime inputs available to [`Select`] click handlers.
#[derive(Clone, Debug)]
pub struct SelectClickContext {
    /// Callback context for the selected item.
    pub click: ClickContext,
    /// Payload rendered by the select item id getter.
    pub payload: String,
}

#[derive(Clone)]
enum SelectAction {
    Action(Arc<SelectActionHandler>),
    OnClick(Arc<SelectClickHandler>),
}

impl SelectAction {
    fn action<F>(handler: F) -> Self
    where
        F: AsyncFn(String) -> ButtonAction
            + AsyncFn1<String, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<String>>::OutputFuture: Send + 'static,
    {
        let handler = Arc::new(handler);
        Self::Action(Arc::new(move |payload| {
            let handler = handler.clone();
            Box::pin(async move { handler(payload).await })
        }))
    }

    fn click<F>(handler: F) -> Self
    where
        F: AsyncFn(SelectClickContext) -> ButtonAction
            + AsyncFn1<SelectClickContext, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<SelectClickContext>>::OutputFuture: Send + 'static,
    {
        let handler = Arc::new(handler);
        Self::OnClick(Arc::new(move |select_click| {
            let handler = handler.clone();
            Box::pin(async move { handler(select_click).await })
        }))
    }

    fn call<'a>(
        &'a self,
        click: &'a ClickContext,
        payload: &'a str,
    ) -> BoxFuture<'a, ButtonAction> {
        match self {
            Self::Action(handler) => handler(payload.to_owned()),
            Self::OnClick(handler) => handler(SelectClickContext {
                click: click.clone(),
                payload: payload.to_owned(),
            }),
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
        #[builder(field)] action: Option<SelectAction>,
        #[builder(field)] on_click: Option<SelectAction>,
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
    pub fn action<F>(mut self, action: F) -> Self
    where
        F: AsyncFn(String) -> ButtonAction
            + AsyncFn1<String, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<String>>::OutputFuture: Send + 'static,
    {
        self.action = Some(SelectAction::action(action));
        self
    }

    pub fn on_click<F>(mut self, on_click: F) -> Self
    where
        F: AsyncFn(SelectClickContext) -> ButtonAction
            + AsyncFn1<SelectClickContext, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<SelectClickContext>>::OutputFuture: Send + 'static,
    {
        self.on_click = Some(SelectAction::click(on_click));
        self
    }

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
            let mut rows = Vec::with_capacity(self.header_rows.len() + self.footer_rows.len());
            for row in &self.header_rows {
                rows.push(render_button_row(row, render_ctx).await);
            }

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
            debug!(context_id = %ctx.id, widget_id = %self.id, "Resolved select callback");
            Some(self.action.call(click, payload).await)
        })
    }
}

pub(crate) fn render_button_row<'a>(
    row: &'a [Button],
    render_ctx: &'a RenderContext,
) -> BoxFuture<'a, Box<[InlineKeyboardButton]>> {
    Box::pin(async move {
        let mut buttons = Vec::with_capacity(row.len());
        for button in row {
            buttons.push(button.render(render_ctx).await);
        }
        buttons.into_boxed_slice()
    })
}
