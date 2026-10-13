use async_trait::async_trait;
use bon::bon;
use std::{borrow::Cow, collections::HashSet, fmt::Display, marker::PhantomData, sync::Arc};
use telers::{
    serialization::Serialize,
    types::{InlineKeyboardMarkup, ReplyMarkup},
};
use tracing::warn;

use super::{
    format_callback_data, parse_callback_data, when::is_allowed, BaseScroll, ButtonAction,
    ClickContext, Keyboard, OnPageChanged, Scroll, WhenCondition,
};
use crate::entities::{Context, Data, DataMap, RenderContext, StartMode};

/// Repeat an inline keyboard for each item, isolating the nested widgets' state by item id.
///
/// Nested render and click data retains the parent fields and adds `item`,
/// `item_id`, `pos` (one-based), `pos0`, and `data` (the parent data map).
/// `page_size` limits each page by items rather than by rendered keyboard rows.
/// Use a pager with the same id to switch pages. Item ids must be unique and
/// stable; callbacks for removed items, hidden pages, and other intents are ignored.
/// Widget mutations in action chains remain item-scoped until the chain starts
/// another dialog on its stack or closes it. Starting an independent stack keeps
/// the current item scope.
pub struct ListGroup<Kbd, ItemsGetter, ItemsIter, Item, IdGetter, Id> {
    base_scroll: BaseScroll,
    kbd: Kbd,
    items_getter: ItemsGetter,
    id_getter: IdGetter,
    page_size: usize,
    when: Option<WhenCondition>,
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> (ItemsIter, Item, Id)>,
}

#[bon]
impl<Kbd, ItemsGetter, ItemsIter, Item, IdGetter, Id>
    ListGroup<Kbd, ItemsGetter, ItemsIter, Item, IdGetter, Id>
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] id: Cow<'static, str>,
        kbd: Kbd,
        items_getter: ItemsGetter,
        id_getter: IdGetter,
        #[builder(default = 0)] page_size: usize,
        on_page_changed: Option<OnPageChanged>,
        when: Option<WhenCondition>,
    ) -> Self
    where
        Kbd: Keyboard,
        ItemsGetter: Fn(&DataMap) -> ItemsIter,
        ItemsIter: IntoIterator<Item = Item>,
        Item: Serialize,
        IdGetter: Fn(&Item) -> Id,
        Id: Display,
    {
        Self {
            base_scroll: BaseScroll::new(id, on_page_changed),
            kbd,
            items_getter,
            id_getter,
            page_size,
            when,
            marker: PhantomData,
        }
    }

    /// Resolve the persisted key of a nested widget for a particular item.
    #[must_use]
    pub fn item_widget_key(&self, item_id: &str, widget_id: &str) -> String {
        format!("{}{widget_id}", self.item_prefix(item_id))
    }

    /// Read a nested widget's raw persisted value for an item.
    #[must_use]
    pub fn item_widget_value<'a>(
        &self,
        ctx: &'a Context,
        item_id: &str,
        widget_id: &str,
    ) -> Option<&'a Data> {
        ctx.widget_value(&self.item_widget_key(item_id, widget_id))
    }

    fn item_prefix(&self, item_id: &str) -> String {
        let id = self.base_scroll.widget_id();
        format!("list:{}:{id}:{}:{item_id}:", id.len(), item_id.len())
    }

    fn scoped_context(&self, ctx: &Context, item_id: &str) -> Context {
        let prefix = self.item_prefix(item_id);
        let mut scoped = ctx.clone();
        scoped.widget_data = ctx
            .widget_data
            .iter()
            .filter_map(|(key, value)| {
                key.strip_prefix(&prefix)
                    .map(|key| (key.to_owned(), value.clone()))
            })
            .collect();
        scoped
    }

    fn page_range(&self, total: usize, page: usize) -> std::ops::Range<usize> {
        if self.page_size == 0 {
            return 0..total;
        }
        let pages = total.div_ceil(self.page_size).max(1);
        let start = page.min(pages - 1) * self.page_size;
        start..start.saturating_add(self.page_size).min(total)
    }
}

fn item_data<Item: Serialize>(
    data: &DataMap,
    widgets: &DataMap,
    item: &Item,
    item_id: &str,
    pos: usize,
) -> Option<DataMap> {
    let mut scoped = data.clone();
    scoped.insert("data".into(), telers::serialization::to_value(data).ok()?);
    scoped.insert("item".into(), telers::serialization::to_value(item).ok()?);
    scoped.insert(
        "widget_data".into(),
        telers::serialization::to_value(widgets).ok()?,
    );
    scoped.insert("item_id".into(), Data::from(item_id));
    scoped.insert("pos0".into(), Data::from(pos));
    scoped.insert("pos".into(), Data::from(pos + 1));
    Some(scoped)
}

fn split_item_callback(payload: &str) -> Option<(&str, &str)> {
    let (length, rest) = payload.strip_prefix('@')?.split_once(':')?;
    let length = length.parse::<usize>().ok()?;
    let item_id = rest.get(..length)?;
    let callback = rest.get(length..)?.strip_prefix(':')?;
    Some((item_id, callback))
}

fn scope_action(action: ButtonAction, prefix: &str, parent_widgets: &mut DataMap) -> ButtonAction {
    scope_action_in_context(action, prefix, parent_widgets, &mut true)
}

fn scope_action_in_context(
    action: ButtonAction,
    prefix: &str,
    parent_widgets: &mut DataMap,
    scoped: &mut bool,
) -> ButtonAction {
    match action {
        ButtonAction::Chain(actions) => ButtonAction::chain(
            actions
                .into_vec()
                .into_iter()
                .map(|action| scope_action_in_context(action, prefix, parent_widgets, scoped)),
        ),
        navigation @ (ButtonAction::Start {
            mode: StartMode::Normal | StartMode::ResetStack,
            ..
        }
        | ButtonAction::Done
        | ButtonAction::DoneWithResult(_)) => {
            *scoped = false;
            navigation
        }
        action if !*scoped => action,
        ButtonAction::SetWidgetValue { key, value } => {
            let key = format!("{prefix}{key}");
            parent_widgets.insert(key.clone(), value.clone());
            ButtonAction::set_widget_value(key, value)
        }
        ButtonAction::ExtendWidgetData(data) => {
            let scoped = data
                .into_iter()
                .map(|(key, value)| (format!("{prefix}{key}"), value))
                .collect::<DataMap>();
            parent_widgets.extend(scoped.clone());
            ButtonAction::ExtendWidgetData(scoped)
        }
        ButtonAction::SetWidgetData(data) => {
            parent_widgets.retain(|key, _| !key.starts_with(prefix));
            parent_widgets.extend(
                data.into_iter()
                    .map(|(key, value)| (format!("{prefix}{key}"), value)),
            );
            ButtonAction::SetWidgetData(parent_widgets.clone())
        }
        other => other,
    }
}

#[async_trait]
impl<Kbd, ItemsGetter, ItemsIter, Item, IdGetter, Id> Scroll
    for ListGroup<Kbd, ItemsGetter, ItemsIter, Item, IdGetter, Id>
where
    Kbd: Keyboard,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: Serialize + Send + Sync + 'static,
    IdGetter: Fn(&Item) -> Id + Send + Sync + 'static,
    Id: Display + Send + Sync + 'static,
{
    fn base_scroll(&self) -> &BaseScroll {
        &self.base_scroll
    }

    async fn get_page_count(&self, render_ctx: RenderContext) -> usize {
        if self.page_size == 0 {
            1
        } else {
            (self.items_getter)(render_ctx.data.as_ref())
                .into_iter()
                .count()
                .div_ceil(self.page_size)
                .max(1)
        }
    }
}

#[async_trait]
impl<Kbd, ItemsGetter, ItemsIter, Item, IdGetter, Id> Keyboard
    for ListGroup<Kbd, ItemsGetter, ItemsIter, Item, IdGetter, Id>
where
    Kbd: Keyboard,
    ItemsGetter: Fn(&DataMap) -> ItemsIter + Send + Sync + 'static,
    ItemsIter: IntoIterator<Item = Item> + 'static,
    Item: Serialize + Send + Sync + 'static,
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
        let items = (self.items_getter)(data)
            .into_iter()
            .collect::<Vec<_>>();
        let range = self.page_range(items.len(), self.get_page(ctx));
        let mut rows = Vec::new();
        let mut seen = HashSet::new();
        for (pos, item) in items.iter().enumerate() {
            let item_id = (self.id_getter)(item).to_string();
            if !seen.insert(item_id.clone()) {
                warn!(
                    widget_id = self.widget_id(),
                    item_id,
                    "ListGroup item ids must be unique"
                );
                continue;
            }
            if !range.contains(&pos) {
                continue;
            }
            let scoped_context = self.scoped_context(ctx, &item_id);
            let Some(scoped_data) =
                item_data(data, &scoped_context.widget_data, item, &item_id, pos)
            else {
                warn!(
                    widget_id = self.widget_id(),
                    item_id,
                    "Cannot serialize ListGroup item"
                );
                continue;
            };
            let mut scoped = render_ctx.clone();
            scoped.context = Arc::new(scoped_context);
            scoped.data = Arc::new(scoped_data);
            if !self
                .kbd
                .is_visible(scoped.context.as_ref(), scoped.data.as_ref())
                .await
            {
                continue;
            }
            let Some(markup) = self.kbd.render_keyboard(&scoped).await else {
                continue;
            };
            let ReplyMarkup::InlineKeyboardMarkup(markup) = markup else {
                warn!(
                    widget_id = self.widget_id(),
                    "ListGroup only supports inline keyboards"
                );
                continue;
            };
            for row in markup.inline_keyboard.into_vec() {
                let mut buttons = Vec::with_capacity(row.len());
                for mut button in row.into_vec() {
                    if let Some(callback) = button.callback_data.as_deref() {
                        let Some(parsed) = parse_callback_data(ctx, callback) else {
                            warn!(
                                widget_id = self.widget_id(),
                                "ListGroup requires scoped dialog callbacks"
                            );
                            continue;
                        };
                        let inner = match parsed.payload {
                            Some(payload) => format!("{}:{payload}", parsed.target_id),
                            None => parsed.target_id.to_owned(),
                        };
                        let payload = format!("@{}:{item_id}:{inner}", item_id.len());
                        let callback = format_callback_data(ctx, self.widget_id(), Some(&payload));
                        if callback.len() > 64 {
                            warn!(
                                widget_id = self.widget_id(),
                                item_id,
                                "ListGroup callback exceeds Telegram's 64-byte limit"
                            );
                            continue;
                        }
                        button.callback_data = Some(callback.into_boxed_str());
                    }
                    buttons.push(button);
                }
                if !buttons.is_empty() {
                    rows.push(buttons.into_boxed_slice());
                }
            }
        }
        if rows.is_empty() {
            None
        } else {
            Some(InlineKeyboardMarkup::new(rows).into())
        }
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        let data = click.data.as_ref();
        if click.callback_data.len() > 64 {
            return None;
        }
        if !self.is_visible(ctx, data).await {
            return None;
        }
        let parsed = parse_callback_data(ctx, &click.callback_data)?;
        if parsed.target_id != self.widget_id() {
            return None;
        }
        let payload = parsed.payload?;
        if !payload.starts_with('@') {
            let page = payload.parse::<usize>().ok()?;
            let total = (self.items_getter)(data).into_iter().count();
            let pages = if self.page_size == 0 {
                1
            } else {
                total.div_ceil(self.page_size).max(1)
            };
            let callback = format_callback_data(
                ctx,
                self.widget_id(),
                Some(&page.min(pages - 1).to_string()),
            );
            return self.base_scroll.handle_callback(ctx, &callback).await;
        }
        let (item_id, inner_callback) = split_item_callback(payload)?;
        let items = (self.items_getter)(data)
            .into_iter()
            .collect::<Vec<_>>();
        let range = self.page_range(items.len(), self.get_page(ctx));
        let (pos, item) = items
            .iter()
            .enumerate()
            .find(|(_, item)| (self.id_getter)(item).to_string() == item_id)?;
        if !range.contains(&pos) {
            return None;
        }
        let mut scoped = click.clone();
        scoped.context = Arc::new(self.scoped_context(ctx, item_id));
        scoped.data = Arc::new(item_data(
            data,
            &scoped.context.widget_data,
            item,
            item_id,
            pos,
        )?);
        let mut inner_parts = inner_callback.splitn(2, ':');
        scoped.callback_data = format_callback_data(ctx, inner_parts.next()?, inner_parts.next());
        if !self
            .kbd
            .is_visible(scoped.context.as_ref(), scoped.data.as_ref())
            .await
        {
            return None;
        }
        let action = self.kbd.handle_callback(&scoped).await?;
        Some(scope_action(
            action,
            &self.item_prefix(item_id),
            &mut ctx.widget_data.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{scope_action, split_item_callback, ListGroup};
    use crate::{
        entities::{Context, DataMap, RenderContext},
        widgets::{
            Button, ButtonAction, Checkbox, ClickContext, FormatText, InlineKeyboard, Keyboard,
            KeyboardGroup, Scroll, WhenCondition,
        },
    };
    use telers::serialization::{json, Value};

    fn context() -> Context {
        Context::new("", "list", Value::default())
    }

    fn list() -> impl Keyboard + Scroll {
        ListGroup::builder("fruits")
            .items_getter(|_data| ["apple", "pear"])
            .id_getter(|item| *item)
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::on_click(
                        "pick",
                        FormatText::new("{pos}. {item}"),
                        |click: ClickContext| async move {
                            ButtonAction::set_dialog_value(
                                "picked",
                                click.data.get("item").cloned().unwrap_or_default(),
                            )
                        },
                    )])
                    .build(),
            )
            .build()
    }

    #[tokio::test]
    async fn repeated_rows_render_item_fields_and_scoped_callbacks() {
        let ctx = context();
        let markup = list()
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows[0][0].text.as_ref(), "1. apple");
        assert_eq!(rows[1][0].text.as_ref(), "2. pear");
        assert_eq!(
            rows[0][0].callback_data.as_deref(),
            Some(format!("td:{}:fruits:@5:apple:pick", ctx.id).as_str())
        );
        let action = list()
            .handle_callback_for_test(&ctx, rows[1][0].callback_data.as_deref().unwrap())
            .await
            .unwrap();
        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { key, value }
                if key == "picked" && value == json!("pear")
        ));
        assert!(ctx.dialog_data.is_empty());
    }

    #[tokio::test]
    async fn callbacks_use_merged_getter_data_and_ignore_removed_items() {
        let ctx = context();
        let group = ListGroup::builder("list")
            .items_getter(|data: &DataMap| {
                data.get("items")
                    .and_then(telers::serialization::as_array)
                    .map(<[Value]>::to_vec)
                    .unwrap_or_default()
            })
            .id_getter(|item: &Value| item.as_str().unwrap_or_default().to_owned())
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::next("pick", "Pick")])
                    .build(),
            )
            .build();
        let callback = format!("td:{}:list:@4:pear:pick", ctx.id);
        assert!(group
            .handle_callback_for_test(&ctx, &callback)
            .await
            .is_none());
        ClickContext::with_test(&ctx, &callback, |click| {
            Box::pin(async move {
                let data = DataMap::from([("items".into(), json!(["pear"]))]);
                let click = click.with_data(data);
                assert!(matches!(
                    group.handle_callback(&click).await,
                    Some(ButtonAction::Next)
                ));
            })
        })
        .await;
    }

    #[tokio::test]
    async fn repeated_stateful_widgets_keep_independent_persisted_state() {
        let mut ctx = context();
        let group = ListGroup::builder("list")
            .items_getter(|_data| ["apple", "pear"])
            .id_getter(|item| *item)
            .kbd(
                Checkbox::builder("checked")
                    .checked_text(FormatText::new("✓ {item} {widget_data.checked}"))
                    .unchecked_text(FormatText::new("□ {item}"))
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let callback = markup.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref()
            .unwrap();
        let action = group.handle_callback_for_test(&ctx, callback).await.unwrap();
        let ButtonAction::SetWidgetValue { key, value } = action else {
            panic!("expected checkbox state");
        };
        assert_eq!(key, group.item_widget_key("apple", "checked"));
        ctx.widget_data.insert(key.into_owned(), value);
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows[0][0].text.as_ref(), "✓ apple true");
        assert_eq!(rows[1][0].text.as_ref(), "□ pear");
        assert_eq!(
            group.item_widget_value(&ctx, "apple", "checked"),
            Some(&json!(true))
        );
        assert_eq!(group.item_widget_value(&ctx, "pear", "checked"), None);
    }

    #[tokio::test]
    async fn item_ids_and_inner_payloads_can_contain_colons_and_unicode() {
        let ctx = context();
        let group = ListGroup::builder("list")
            .items_getter(|_data| ["🍐:pear"])
            .id_getter(|item| *item)
            .kbd(
                crate::widgets::Select::builder("select")
                    .items_getter(|_data| ["a:b"])
                    .id_getter(|item| item)
                    .item_renderer(|item, _data| *item)
                    .action(|payload| async move {
                        ButtonAction::set_dialog_value("picked", payload)
                    })
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let callback = markup.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref()
            .unwrap();
        assert!(callback.contains("@9:🍐:pear:select:a:b"));
        assert!(matches!(
            group.handle_callback_for_test(&ctx, callback).await,
            Some(ButtonAction::SetDialogValue { value, .. }) if value == json!("a:b")
        ));
    }

    #[tokio::test]
    async fn pagination_counts_items_keeps_absolute_positions_and_clamps_page() {
        let mut ctx = context();
        ctx.widget_data.insert("list".into(), json!(usize::MAX));
        let group = ListGroup::builder("list")
            .page_size(2)
            .items_getter(|_data| ["a", "b", "c"])
            .id_getter(|item| *item)
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::next("pick", FormatText::new("{pos}/{pos0}: {item}"))])
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0].text.as_ref(), "3/2: c");
        assert!(group
            .handle_callback_for_test(&ctx, &format!("td:{}:list:@1:a:pick", ctx.id))
            .await
            .is_none());
        RenderContext::with_test(&ctx, &DataMap::new(), |render| {
            Box::pin(async move {
                assert_eq!(group.get_page_count(render).await, 2);
            })
        })
        .await;
    }

    #[tokio::test]
    async fn empty_lists_render_nothing_and_have_one_page() {
        let ctx = context();
        let group = ListGroup::builder("list")
            .page_size(2)
            .items_getter(|_data| [] as [&str; 0])
            .id_getter(|item| *item)
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::next("pick", "Pick")])
                    .build(),
            )
            .build();
        assert!(group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .is_none());
        RenderContext::with_test(&ctx, &DataMap::new(), |render| {
            Box::pin(async move {
                assert_eq!(group.get_page_count(render).await, 1);
            })
        })
        .await;
    }

    #[tokio::test]
    async fn page_callbacks_are_normalized_before_persisting_and_notifying() {
        let ctx = context();
        let group = ListGroup::builder("list")
            .page_size(2)
            .items_getter(|_data| ["a", "b", "c"])
            .id_getter(|item| *item)
            .on_page_changed(crate::widgets::OnPageChanged::new(|change| async move {
                ButtonAction::set_dialog_value("page", change.new_page)
            }))
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::next("pick", "Pick")])
                    .build(),
            )
            .build();
        let action = group
            .handle_callback_for_test(&ctx, &format!("td:{}:list:{}", ctx.id, usize::MAX))
            .await
            .unwrap();
        let ButtonAction::Chain(actions) = action else {
            panic!("expected page change hook");
        };
        assert!(matches!(
            &actions[0],
            ButtonAction::SetWidgetValue { key, value } if key == "list" && value == &json!(1)
        ));
        assert!(matches!(
            &actions[1],
            ButtonAction::SetDialogValue { key, value } if key == "page" && value == &json!(1)
        ));
    }

    #[tokio::test]
    async fn stale_foreign_and_malformed_callbacks_do_not_route() {
        let ctx = context();
        for callback in [
            "td:foreign:fruits:@5:apple:pick".to_owned(),
            format!("td:{}:other:@5:apple:pick", ctx.id),
            format!("td:{}:fruits:@x:apple:pick", ctx.id),
            format!("td:{}:fruits:@100:apple:pick", ctx.id),
            format!("td:{}:fruits:@4:apple:pick", ctx.id),
            format!("td:{}:fruits:@4:plum:pick", ctx.id),
        ] {
            assert!(
                list()
                    .handle_callback_for_test(&ctx, &callback)
                    .await
                    .is_none(),
                "{callback}"
            );
        }
        assert_eq!(split_item_callback("@1:🍐:pick"), None);
    }

    #[tokio::test]
    async fn hidden_items_are_ignored_during_render_and_callback() {
        let ctx = context();
        let group = ListGroup::builder("list")
            .items_getter(|_data| ["pear", "apple"])
            .id_getter(|item| *item)
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::next("pick", FormatText::new("{item}"))])
                    .when(WhenCondition::new(
                        |when: crate::widgets::WhenContext| async move {
                            when.data.get("item").is_some_and(|item| item == "pear")
                        },
                    ))
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        assert_eq!(markup.inline_keyboard().unwrap().len(), 1);
        assert!(group
            .handle_callback_for_test(&ctx, &format!("td:{}:list:@5:apple:pick", ctx.id))
            .await
            .is_none());
    }

    #[tokio::test]
    async fn duplicate_item_ids_render_once() {
        let ctx = context();
        let group = ListGroup::builder("list")
            .items_getter(|_data| ["pear", "pear"])
            .id_getter(|item| *item)
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::next("pick", "Pick")])
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        assert_eq!(markup.inline_keyboard().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn oversized_callbacks_are_omitted_but_link_buttons_are_preserved() {
        let ctx = context();
        let group = ListGroup::builder("list")
            .items_getter(|_data| ["a".repeat(64)])
            .id_getter(Clone::clone)
            .kbd(
                InlineKeyboard::builder()
                    .row([
                        Button::next("pick", "Pick"),
                        Button::url("Link", "https://example.com"),
                    ])
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].url.as_deref(), Some("https://example.com"));
    }

    #[tokio::test]
    async fn nested_list_groups_route_and_scope_state_recursively() {
        let mut ctx = context();
        let inner = ListGroup::builder("inner")
            .items_getter(|_data| ["x", "y"])
            .id_getter(|item| *item)
            .kbd(
                Checkbox::builder("flag")
                    .checked_text("Yes")
                    .unchecked_text("No")
                    .build(),
            )
            .build();
        let inner_key = inner.item_widget_key("x", "flag");
        let outer = ListGroup::builder("outer")
            .items_getter(|_data| ["a", "b"])
            .id_getter(|item| *item)
            .kbd(inner)
            .build();
        let markup = outer
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let callback = markup.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref()
            .unwrap();
        let action = outer.handle_callback_for_test(&ctx, callback).await.unwrap();
        let ButtonAction::SetWidgetValue { key, value } = action else {
            panic!("expected scoped state");
        };
        assert_eq!(key, outer.item_widget_key("a", &inner_key));
        ctx.widget_data.insert(key.into_owned(), value);
        let markup = outer
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows[0][0].text.as_ref(), "Yes");
        assert_eq!(rows[1][0].text.as_ref(), "No");
        assert_eq!(rows[2][0].text.as_ref(), "No");
    }

    #[tokio::test]
    async fn nested_getters_receive_the_current_parent_item() {
        let ctx = context();
        let inner = ListGroup::builder("inner")
            .items_getter(|data: &DataMap| {
                data.get("item")
                    .and_then(telers::serialization::as_array)
                    .map(<[Value]>::to_vec)
                    .unwrap_or_default()
            })
            .id_getter(|item: &Value| item.as_str().unwrap_or_default().to_owned())
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::on_click(
                        "pick",
                        FormatText::new("{data.item_id}: {item}"),
                        |click: ClickContext| async move {
                            ButtonAction::set_dialog_value("picked", click.data["item"].clone())
                        },
                    )])
                    .build(),
            )
            .build();
        let outer = ListGroup::builder("outer")
            .items_getter(|_data| [json!(["x", "y"]), json!(["z"])])
            .id_getter(|item: &Value| telers::serialization::as_array(item).unwrap().len())
            .kbd(inner)
            .build();
        let markup = outer
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0].text.as_ref(), "2: x");
        assert_eq!(rows[2][0].text.as_ref(), "1: z");
        let callback = rows[2][0].callback_data.as_deref().unwrap();
        assert!(matches!(
            outer.handle_callback_for_test(&ctx, callback).await,
            Some(ButtonAction::SetDialogValue { value, .. }) if value == json!("z")
        ));
    }

    #[tokio::test]
    async fn nested_lists_keep_each_items_page_independent() {
        let mut ctx = context();
        let inner = ListGroup::builder("inner")
            .page_size(1)
            .items_getter(|_data| ["x", "y"])
            .id_getter(|item| *item)
            .kbd(
                InlineKeyboard::builder()
                    .row([Button::next("pick", FormatText::new("{item}"))])
                    .build(),
            )
            .build();
        let outer = ListGroup::builder("outer")
            .items_getter(|_data| ["a", "b"])
            .id_getter(|item| *item)
            .kbd(inner)
            .build();
        let callback = format!("td:{}:outer:@1:a:inner:1", ctx.id);
        let action = outer.handle_callback_for_test(&ctx, &callback).await.unwrap();
        let ButtonAction::SetWidgetValue { key, value } = action else {
            panic!("expected page state");
        };
        assert_eq!(key, outer.item_widget_key("a", "inner"));
        ctx.widget_data.insert(key.into_owned(), value);
        let markup = outer
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows[0][0].text.as_ref(), "y");
        assert_eq!(rows[1][0].text.as_ref(), "x");
    }

    #[test]
    fn replacement_actions_preserve_other_items_and_preceding_chain_mutations() {
        let mut parent = DataMap::from([("other".into(), json!(1)), ("scope:old".into(), json!(2))]);
        let action = scope_action(
            ButtonAction::chain([
                ButtonAction::set_widget_value("before", true),
                ButtonAction::set_widget_data(DataMap::from([("new".into(), json!(3))])),
                ButtonAction::extend_widget_data([("after", 4)]),
            ]),
            "scope:",
            &mut parent,
        );
        assert_eq!(parent.get("other"), Some(&json!(1)));
        assert_eq!(parent.get("scope:new"), Some(&json!(3)));
        assert_eq!(parent.get("scope:after"), Some(&json!(4)));
        assert!(!parent.contains_key("scope:old"));
        assert!(!parent.contains_key("scope:before"));
        let ButtonAction::Chain(actions) = action else {
            panic!("expected chain");
        };
        let ButtonAction::SetWidgetData(data) = &actions[1] else {
            panic!("expected replacement");
        };
        assert_eq!(data.get("other"), Some(&json!(1)));
        assert!(!data.contains_key("scope:after"));
    }

    #[test]
    fn item_scope_ends_after_navigation_to_another_intent() {
        let mut parent = DataMap::from([("other".into(), json!(1))]);
        let action = scope_action(
            ButtonAction::chain([
                ButtonAction::set_widget_value("before", true),
                ButtonAction::chain([
                    ButtonAction::done(),
                    ButtonAction::set_widget_value("after", 2),
                ]),
                ButtonAction::set_widget_data(DataMap::from([("destination".into(), json!(3))])),
            ]),
            "scope:",
            &mut parent,
        );
        let ButtonAction::Chain(actions) = action else {
            panic!("expected chain");
        };
        assert!(matches!(
            &actions[0],
            ButtonAction::SetWidgetValue { key, .. } if key == "scope:before"
        ));
        let ButtonAction::Chain(nested) = &actions[1] else {
            panic!("expected nested chain");
        };
        assert!(matches!(
            &nested[1],
            ButtonAction::SetWidgetValue { key, .. } if key == "after"
        ));
        let ButtonAction::SetWidgetData(data) = &actions[2] else {
            panic!("expected replacement");
        };
        assert_eq!(data, &DataMap::from([("destination".into(), json!(3))]));
        assert!(!parent.contains_key("after"));
    }

    #[test]
    fn starting_an_independent_stack_keeps_the_current_item_scope() {
        let mut parent = DataMap::new();
        let action = scope_action(
            ButtonAction::chain([
                ButtonAction::start("new", Value::default(), crate::entities::StartMode::NewStack),
                ButtonAction::set_widget_value("after", true),
            ]),
            "scope:",
            &mut parent,
        );
        let ButtonAction::Chain(actions) = action else {
            panic!("expected chain");
        };
        assert!(matches!(
            &actions[1],
            ButtonAction::SetWidgetValue { key, .. } if key == "scope:after"
        ));
        assert_eq!(parent.get("scope:after"), Some(&json!(true)));
    }

    #[tokio::test]
    async fn keyboard_groups_compose_stateful_widgets_and_explicit_rows() {
        let ctx = context();
        let group = KeyboardGroup::builder()
            .items_per_row(2)
            .keyboard(
                Checkbox::builder("flag")
                    .checked_text("Yes")
                    .unchecked_text("No")
                    .build(),
            )
            .keyboard(
                InlineKeyboard::builder()
                    .row([Button::next("next", "Next"), Button::done("done", "Done")])
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let rows = markup.inline_keyboard().unwrap();
        assert_eq!(rows.iter().map(|row| row.len()).collect::<Vec<_>>(), [2, 1]);
        assert!(matches!(
            group
                .handle_callback_for_test(&ctx, &format!("td:{}:flag:true", ctx.id))
                .await,
            Some(ButtonAction::SetWidgetValue { .. })
        ));
        assert!(matches!(
            group
                .handle_callback_for_test(&ctx, &format!("td:{}:next", ctx.id))
                .await,
            Some(ButtonAction::Next)
        ));
    }

    #[tokio::test]
    async fn keyboard_groups_preserve_rows_and_obey_visibility() {
        let ctx = context();
        let group = KeyboardGroup::builder()
            .when(WhenCondition::data_field("show"))
            .keyboard(
                InlineKeyboard::builder()
                    .row([Button::next("next", "Next")])
                    .row([Button::done("done", "Done")])
                    .build(),
            )
            .keyboard(
                InlineKeyboard::builder()
                    .row([Button::back("back", "Back")])
                    .build(),
            )
            .build();
        assert!(group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .is_none());
        assert!(group
            .handle_callback_for_test(&ctx, &format!("td:{}:next", ctx.id))
            .await
            .is_none());
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::from([("show".into(), json!(true))]))
            .await
            .unwrap();
        assert_eq!(
            markup
                .inline_keyboard()
                .unwrap()
                .iter()
                .map(|row| row.len())
                .collect::<Vec<_>>(),
            [1, 1, 1]
        );
    }

    #[tokio::test]
    async fn group_width_larger_than_available_buttons_does_not_overallocate() {
        let ctx = context();
        let group = KeyboardGroup::builder()
            .items_per_row(usize::MAX)
            .keyboard(
                InlineKeyboard::builder()
                    .row([Button::next("next", "Next")])
                    .row([Button::done("done", "Done")])
                    .build(),
            )
            .build();
        let markup = group
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        assert_eq!(markup.inline_keyboard().unwrap()[0].len(), 2);
    }
}
