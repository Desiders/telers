use async_fn_traits::AsyncFn1;
use async_trait::async_trait;
use bon::bon;
use std::{fmt::Display, marker::PhantomData, sync::Arc};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::{
    format_callback_data, macros::impl_button_row_helpers, parse_callback_data, when::is_allowed,
    Button, ButtonAction, ClickContext, Keyboard, WhenCondition,
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
/// that payload is converted into a [`ButtonAction`] by either `action` (which
/// receives only the payload) or `on_click` (which also receives the click
/// context). The widget does not persist any state itself — write any
/// selected value back into `dialog_data` or `widget_data` from the handler.
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::{ButtonAction, Select};
///
/// let select = Select::builder("fruit")
///     .items_getter(|_data| ["apple", "pear", "plum"])
///     .item_renderer(|item, _data| item.to_owned())
///     .id_getter(|item| item)
///     .action(|payload: String| async move {
///         ButtonAction::set_dialog_value("fruit", payload)
///     })
///     .build();
/// ```
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
    /// Build a [`Select`] widget.
    ///
    /// At least one of [`action`](SelectBuilder::action) or
    /// [`on_click`](SelectBuilder::on_click) must be provided; `on_click`
    /// wins when both are set. Header and footer rows render around the
    /// dynamic item buttons.
    ///
    /// # Panics
    /// Builds will panic at runtime if neither `action` nor `on_click` is supplied.
    #[builder]
    #[must_use]
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
    impl_button_row_helpers!();

    /// Register an async handler that receives the selected item's payload string.
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

    /// Register an async handler that receives the full click context plus the
    /// selected item's payload string.
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
}

#[async_trait]
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
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
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
        debug!(context_id = %ctx.id, widget_id = %self.id, "Resolved select callback");
        Some(self.action.call(click, payload).await)
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
