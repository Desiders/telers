use bon::bon;
use std::{borrow::Cow, fmt::Display, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::{debug, warn};

use super::Text;
use crate::entities::{Context, Data, DataMap, StartMode};

const CALLBACK_PREFIX: &str = "td";

#[derive(Clone, Debug)]
pub enum ButtonAction {
    Noop,
    Next,
    Back,
    SwitchTo(Cow<'static, str>),
    Start {
        state: Cow<'static, str>,
        data: Data,
        mode: StartMode,
    },
    Done,
    SetDialogData(DataMap),
    SetDialogValue {
        key: Cow<'static, str>,
        value: Data,
    },
    SetWidgetData(DataMap),
    SetWidgetValue {
        key: Cow<'static, str>,
        value: Data,
    },
    Chain(Box<[ButtonAction]>),
}

impl ButtonAction {
    #[inline]
    #[must_use]
    pub const fn noop() -> Self {
        Self::Noop
    }

    #[inline]
    #[must_use]
    pub const fn next() -> Self {
        Self::Next
    }

    #[inline]
    #[must_use]
    pub const fn back() -> Self {
        Self::Back
    }

    #[must_use]
    pub fn switch_to(state: impl Into<Cow<'static, str>>) -> Self {
        Self::SwitchTo(state.into())
    }

    #[must_use]
    pub fn start(
        state: impl Into<Cow<'static, str>>,
        data: impl Into<Data>,
        mode: StartMode,
    ) -> Self {
        Self::Start {
            state: state.into(),
            data: data.into(),
            mode,
        }
    }

    #[inline]
    #[must_use]
    pub const fn done() -> Self {
        Self::Done
    }

    #[must_use]
    pub fn set_dialog_data(data: DataMap) -> Self {
        Self::SetDialogData(data)
    }

    #[must_use]
    pub fn set_dialog_value(key: impl Into<Cow<'static, str>>, value: impl Into<Data>) -> Self {
        Self::SetDialogValue {
            key: key.into(),
            value: value.into(),
        }
    }

    #[must_use]
    pub fn set_widget_data(data: DataMap) -> Self {
        Self::SetWidgetData(data)
    }

    #[must_use]
    pub fn set_widget_value(key: impl Into<Cow<'static, str>>, value: impl Into<Data>) -> Self {
        Self::SetWidgetValue {
            key: key.into(),
            value: value.into(),
        }
    }

    #[must_use]
    pub fn chain(actions: impl IntoIterator<Item = ButtonAction>) -> Self {
        Self::Chain(actions.into_iter().collect())
    }
}

pub trait Keyboard: Send + Sync + 'static {
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup>;

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction>;
}

pub(crate) struct MultiKeyboard {
    keyboards: Vec<Box<dyn Keyboard>>,
}

impl MultiKeyboard {
    #[inline]
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            keyboards: Vec::new(),
        }
    }

    #[must_use]
    pub(crate) fn kbd_boxed(mut self, keyboard: Box<dyn Keyboard>) -> Self {
        self.keyboards.push(keyboard);
        self
    }
}

impl Keyboard for MultiKeyboard {
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let mut inline_rows = Vec::new();
        let mut non_inline_markup = None;

        for keyboard in &self.keyboards {
            let Some(markup) = keyboard.render_keyboard(ctx, data) else {
                continue;
            };
            match markup {
                ReplyMarkup::InlineKeyboardMarkup(markup) => {
                    if non_inline_markup.is_some() {
                        warn!("Cannot combine inline and non-inline reply markups in one window");
                        continue;
                    }
                    inline_rows.extend(markup.inline_keyboard.into_vec());
                }
                other_markup => {
                    if !inline_rows.is_empty() {
                        warn!("Cannot combine non-inline reply markup with inline keyboard rows");
                        continue;
                    }
                    if non_inline_markup.is_some() {
                        warn!("Only one non-inline reply markup can be used in a window");
                        continue;
                    }
                    non_inline_markup = Some(other_markup);
                }
            }
        }

        if inline_rows.is_empty() {
            non_inline_markup
        } else {
            Some(ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
                inline_keyboard: inline_rows.into_boxed_slice(),
            }))
        }
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        self.keyboards
            .iter()
            .find_map(|keyboard| keyboard.handle_callback(ctx, callback_data))
    }
}

enum ButtonKind {
    Callback(ButtonAction),
    Url(Cow<'static, str>),
}

pub struct Button {
    id: Cow<'static, str>,
    text: Box<dyn Text>,
    kind: ButtonKind,
}

impl Button {
    #[must_use]
    pub fn action(id: impl Into<Cow<'static, str>>, text: impl Text, action: ButtonAction) -> Self {
        Self {
            id: id.into(),
            text: Box::new(text),
            kind: ButtonKind::Callback(action),
        }
    }

    #[must_use]
    pub fn next(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::next())
    }

    #[must_use]
    pub fn back(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::back())
    }

    #[must_use]
    pub fn switch_to(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        state: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::action(id, text, ButtonAction::switch_to(state))
    }

    #[must_use]
    pub fn start(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        state: impl Into<Cow<'static, str>>,
        data: impl Into<Data>,
        mode: StartMode,
    ) -> Self {
        Self::action(id, text, ButtonAction::start(state, data, mode))
    }

    #[must_use]
    pub fn done(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::done())
    }

    #[must_use]
    pub fn set_dialog_value(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Data>,
    ) -> Self {
        Self::action(id, text, ButtonAction::set_dialog_value(key, value))
    }

    #[must_use]
    pub fn url(text: impl Text, url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: String::new().into(),
            text: Box::new(text),
            kind: ButtonKind::Url(url.into()),
        }
    }

    fn render(&self, ctx: &Context, data: &DataMap) -> InlineKeyboardButton {
        let button = InlineKeyboardButton::new(self.text.render_text(data));
        match &self.kind {
            ButtonKind::Callback(_) => {
                button.callback_data(format_callback_data(ctx, &self.id, None))
            }
            ButtonKind::Url(url) => button.url(url.clone()),
        }
    }

    fn resolve_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.as_ref() || parsed.payload.is_some() {
            return None;
        }
        match &self.kind {
            ButtonKind::Callback(action) => {
                debug!(context_id = %ctx.id, button_id = %self.id, "Resolved button callback");
                Some(action.clone())
            }
            ButtonKind::Url(_) => None,
        }
    }
}

pub struct InlineKeyboard {
    rows: Vec<Vec<Button>>,
}

impl InlineKeyboard {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            rows: Vec::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn row<T, I>(mut self, row: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Button>,
    {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    /// Add a button to the last row or create a new row if the last row not found
    #[must_use]
    pub fn push(mut self, button: Button) -> Self {
        match self.rows.last_mut() {
            Some(row) => row.push(button),
            None => self.rows.push(vec![button]),
        }
        self
    }
}

impl Keyboard for InlineKeyboard {
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        if self.rows.is_empty() {
            return None;
        }

        let rows = self.rows.iter().map(|row| {
            row.iter()
                .map(|button| button.render(ctx, data))
                .collect::<Box<[_]>>()
        });

        Some(ReplyMarkup::InlineKeyboardMarkup(
            InlineKeyboardMarkup::new(rows),
        ))
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .find_map(|button| button.resolve_callback(ctx, callback_data))
    }
}

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
    items_per_row: usize,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
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
        #[builder(default = 1)] items_per_row: usize,
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
            items_per_row,
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
    #[must_use]
    pub fn header_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.header_rows.push(buttons.into_iter().collect());
        self
    }

    #[must_use]
    pub fn header_push(mut self, button: Button) -> Self {
        match self.header_rows.last_mut() {
            Some(row) => row.push(button),
            None => self.header_rows.push(vec![button]),
        }
        self
    }

    #[must_use]
    pub fn footer_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.footer_rows.push(buttons.into_iter().collect());
        self
    }

    #[must_use]
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

        let mut current_row = Vec::with_capacity(self.items_per_row);
        for item in (self.items_getter)(data) {
            let text = (self.item_renderer)(&item, data);
            let payload = (self.id_getter)(item).to_string();
            current_row.push(
                InlineKeyboardButton::new(text).callback_data(format_callback_data(
                    ctx,
                    &self.id,
                    Some(&payload),
                )),
            );
            if current_row.len() == self.items_per_row {
                rows.push(current_row.into_boxed_slice());
                current_row = Vec::with_capacity(self.items_per_row);
            }
        }
        if !current_row.is_empty() {
            rows.push(current_row.into_boxed_slice());
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

fn render_button_row(row: &[Button], ctx: &Context, data: &DataMap) -> Box<[InlineKeyboardButton]> {
    row.iter().map(|button| button.render(ctx, data)).collect()
}

fn format_callback_data(ctx: &Context, target_id: impl Display, payload: Option<&str>) -> String {
    match payload {
        Some(payload) => format!("{CALLBACK_PREFIX}:{}:{target_id}:{payload}", ctx.id),
        None => format!("{CALLBACK_PREFIX}:{}:{target_id}", ctx.id),
    }
}

struct ParsedCallbackData<'a> {
    target_id: &'a str,
    payload: Option<&'a str>,
}

fn parse_callback_data<'a>(
    ctx: &Context,
    callback_data: &'a str,
) -> Option<ParsedCallbackData<'a>> {
    let mut parts = callback_data.splitn(4, ':');
    if parts.next()? != CALLBACK_PREFIX {
        return None;
    }
    if parts.next()? != ctx.id {
        return None;
    }
    Some(ParsedCallbackData {
        target_id: parts.next()?,
        payload: parts.next(),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{Button, ButtonAction, InlineKeyboard, Keyboard, Select};
    use crate::entities::{Context, DataMap, StartMode};

    #[test]
    fn inline_keyboard_renders_callback_data_with_intent() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::new().row([Button::action("go", "Go", ButtonAction::Next)]);

        let markup = keyboard
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let callback_data = markup
            .inline_keyboard()
            .and_then(|rows| rows.first())
            .and_then(|row| row.first())
            .and_then(|button| button.callback_data.as_deref())
            .expect("callback data");

        assert_eq!(callback_data, format!("td:{}:go", ctx.id));
    }

    #[test]
    fn inline_keyboard_ignores_foreign_intent_callbacks() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::new().row([Button::action(
            "go",
            "Go",
            ButtonAction::Start {
                state: "next".into(),
                data: Value::Null,
                mode: StartMode::Normal,
            },
        )]);

        assert!(keyboard.handle_callback(&ctx, "td:another:go").is_none());
    }

    #[test]
    fn select_renders_and_resolves_string_payloads() {
        let ctx = Context::new("", "state", Value::Null);

        let select = Select::builder("fruit")
            .items_getter(|_data| ["red:apple", "pear"])
            .item_renderer(|item, _data| item.to_owned())
            .id_getter(|item| item)
            .action(|value| ButtonAction::set_dialog_value("fruit", value))
            .items_per_row(2)
            .build();

        let markup = select
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let callback_data = markup
            .inline_keyboard()
            .and_then(|rows| rows.first())
            .and_then(|row| row.first())
            .and_then(|button| button.callback_data.as_deref())
            .expect("callback data");

        assert_eq!(callback_data, format!("td:{}:fruit:red:apple", ctx.id));

        let action = select
            .handle_callback(&ctx, callback_data)
            .expect("select action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "fruit" && value == "red:apple"
        ));
    }

    #[test]
    fn select_allows_static_footer_buttons() {
        let ctx = Context::new("", "state", Value::Null);

        let select = Select::builder("fruit")
            .items_getter(|_data| ["pear"])
            .item_renderer(|item, _data| item.to_owned())
            .id_getter(|item| item)
            .action(|value| ButtonAction::set_dialog_value("fruit", value))
            .footer_push(Button::done("done", "Done"))
            .build();

        let action = select
            .handle_callback(&ctx, &format!("td:{}:done", ctx.id))
            .expect("footer action");

        assert!(matches!(action, ButtonAction::Done));
    }
}
