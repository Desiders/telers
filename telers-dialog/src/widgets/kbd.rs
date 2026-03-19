use crate::entities::{Context, Data, DataMap, StartMode};
use std::{fmt::Write, marker::PhantomData};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::{debug, warn};

use super::Text;

const CALLBACK_PREFIX: &str = "td";

#[derive(Clone, Debug)]
pub enum ButtonAction {
    Noop,
    Next,
    Back,
    SwitchTo(Box<str>),
    Start {
        state: Box<str>,
        data: Data,
        mode: StartMode,
    },
    Done,
    SetDialogData(DataMap),
    SetDialogValue {
        key: Box<str>,
        value: Data,
    },
    SetWidgetData(DataMap),
    SetWidgetValue {
        key: Box<str>,
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
    pub fn switch_to(state: impl Into<Box<str>>) -> Self {
        Self::SwitchTo(state.into())
    }

    #[must_use]
    pub fn start(state: impl Into<Box<str>>, data: impl Into<Data>, mode: StartMode) -> Self {
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
    pub fn set_dialog_value(key: impl Into<Box<str>>, value: impl Into<Data>) -> Self {
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
    pub fn set_widget_value(key: impl Into<Box<str>>, value: impl Into<Data>) -> Self {
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

pub trait CallbackPayload: Clone + Send + Sync + 'static {
    fn encode_payload(&self) -> String;
    fn decode_payload(payload: &str) -> Option<Self>;
}

impl CallbackPayload for String {
    fn encode_payload(&self) -> String {
        encode_string_payload(self)
    }

    fn decode_payload(payload: &str) -> Option<Self> {
        decode_string_payload(payload)
    }
}

impl CallbackPayload for Box<str> {
    fn encode_payload(&self) -> String {
        encode_string_payload(self)
    }

    fn decode_payload(payload: &str) -> Option<Self> {
        String::decode_payload(payload).map(String::into_boxed_str)
    }
}

macro_rules! impl_callback_payload_from_parse {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl CallbackPayload for $ty {
                fn encode_payload(&self) -> String {
                    self.to_string()
                }

                fn decode_payload(payload: &str) -> Option<Self> {
                    payload.parse().ok()
                }
            }
        )+
    };
}

impl_callback_payload_from_parse!(bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl CallbackPayload for serde_json::Value {
    fn encode_payload(&self) -> String {
        encode_string_payload(&self.to_string())
    }

    fn decode_payload(payload: &str) -> Option<Self> {
        serde_json::from_str(&decode_string_payload(payload)?).ok()
    }
}

pub trait Keyboard: Send + Sync + 'static {
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup>;

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction>;
}

pub(super) struct MultiKeyboard {
    keyboards: Vec<Box<dyn Keyboard>>,
}

impl MultiKeyboard {
    pub(super) fn new(keyboards: Vec<Box<dyn Keyboard>>) -> Self {
        Self {
            keyboards,
        }
    }
}

impl Keyboard for MultiKeyboard {
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let mut inline_rows: Vec<Box<[InlineKeyboardButton]>> = Vec::new();
        let mut non_inline_markup: Option<ReplyMarkup> = None;

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
    Url(Box<str>),
}

pub struct Button {
    id: Box<str>,
    text: Box<dyn Text>,
    kind: ButtonKind,
}

impl Button {
    #[must_use]
    pub fn action(id: impl Into<Box<str>>, text: impl Text, action: ButtonAction) -> Self {
        Self {
            id: id.into(),
            text: Box::new(text),
            kind: ButtonKind::Callback(action),
        }
    }

    #[must_use]
    pub fn next(id: impl Into<Box<str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::next())
    }

    #[must_use]
    pub fn back(id: impl Into<Box<str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::back())
    }

    #[must_use]
    pub fn switch_to(id: impl Into<Box<str>>, text: impl Text, state: impl Into<Box<str>>) -> Self {
        Self::action(id, text, ButtonAction::switch_to(state))
    }

    #[must_use]
    pub fn start(
        id: impl Into<Box<str>>,
        text: impl Text,
        state: impl Into<Box<str>>,
        data: impl Into<Data>,
        mode: StartMode,
    ) -> Self {
        Self::action(id, text, ButtonAction::start(state, data, mode))
    }

    #[must_use]
    pub fn done(id: impl Into<Box<str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::done())
    }

    #[must_use]
    pub fn set_dialog_value(
        id: impl Into<Box<str>>,
        text: impl Text,
        key: impl Into<Box<str>>,
        value: impl Into<Data>,
    ) -> Self {
        Self::action(id, text, ButtonAction::set_dialog_value(key, value))
    }

    #[must_use]
    pub fn url(text: impl Text, url: impl Into<Box<str>>) -> Self {
        Self {
            id: String::new().into_boxed_str(),
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
    #[must_use]
    pub fn new<Row, Rows>(rows: Rows) -> Self
    where
        Row: IntoIterator<Item = Button>,
        Rows: IntoIterator<Item = Row>,
    {
        Self {
            rows: rows
                .into_iter()
                .map(|row| row.into_iter().collect())
                .collect(),
        }
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

pub struct Select<T, V, I, R, P, A> {
    id: Box<str>,
    items: I,
    render_item: R,
    item_value: P,
    action: A,
    items_per_row: usize,
    header_rows: Vec<Vec<Button>>,
    footer_rows: Vec<Vec<Button>>,
    marker: PhantomData<fn() -> (T, V)>,
}

impl<T, V, I, R, P, A> Select<T, V, I, R, P, A> {
    #[must_use]
    pub fn new(
        id: impl Into<Box<str>>,
        items: I,
        render_item: R,
        item_value: P,
        action: A,
    ) -> Self {
        Self {
            id: id.into(),
            items,
            render_item,
            item_value,
            action,
            items_per_row: 1,
            header_rows: Vec::new(),
            footer_rows: Vec::new(),
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn items_per_row(mut self, width: usize) -> Self {
        self.items_per_row = width.max(1);
        self
    }

    #[must_use]
    pub fn header_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.header_rows.push(buttons.into_iter().collect());
        self
    }

    #[must_use]
    pub fn footer_row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.footer_rows.push(buttons.into_iter().collect());
        self
    }
}

impl<T, V, I, R, P, A, S> Keyboard for Select<T, V, I, R, P, A>
where
    T: Send + Sync + 'static,
    V: CallbackPayload,
    I: Fn(&DataMap) -> Vec<T> + Send + Sync + 'static,
    R: Fn(&T, &DataMap) -> S + Send + Sync + 'static,
    P: Fn(&T) -> V + Send + Sync + 'static,
    A: Fn(V) -> ButtonAction + Send + Sync + 'static,
    S: Into<Box<str>>,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let mut rows: Vec<Box<[InlineKeyboardButton]>> = self
            .header_rows
            .iter()
            .map(|row| render_button_row(row, ctx, data))
            .collect();

        let mut current_row = Vec::with_capacity(self.items_per_row);
        for item in (self.items)(data) {
            let payload = (self.item_value)(&item).encode_payload();
            current_row.push(
                InlineKeyboardButton::new((self.render_item)(&item, data))
                    .callback_data(format_callback_data(ctx, &self.id, Some(&payload))),
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
            Some(ReplyMarkup::InlineKeyboardMarkup(
                InlineKeyboardMarkup::new(rows),
            ))
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
        if parsed.target_id != self.id.as_ref() {
            return None;
        }
        let payload = parsed.payload?;
        let value = V::decode_payload(payload)?;
        debug!(context_id = %ctx.id, widget_id = %self.id, "Resolved select callback");
        Some((self.action)(value))
    }
}

fn render_button_row(row: &[Button], ctx: &Context, data: &DataMap) -> Box<[InlineKeyboardButton]> {
    row.iter()
        .map(|button| button.render(ctx, data))
        .collect::<Box<[_]>>()
}

fn format_callback_data(ctx: &Context, target_id: &str, payload: Option<&str>) -> String {
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

fn encode_string_payload(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for &byte in value.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            let _ = write!(&mut encoded, "%{byte:02X}");
        }
    }
    encoded
}

fn decode_string_payload(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut idx = 0;
    while idx < bytes.len() {
        if bytes[idx] == b'%' {
            let hex = std::str::from_utf8(bytes.get(idx + 1..idx + 3)?).ok()?;
            decoded.push(u8::from_str_radix(hex, 16).ok()?);
            idx += 3;
        } else {
            decoded.push(bytes[idx]);
            idx += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

#[cfg(test)]
mod tests {
    use super::{Button, ButtonAction, InlineKeyboard, Keyboard, Select};
    use crate::entities::{Context, DataMap, StartMode};

    #[test]
    fn inline_keyboard_renders_callback_data_with_intent() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let keyboard = InlineKeyboard::new([[Button::action("go", "Go", ButtonAction::Next)]]);

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
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let keyboard = InlineKeyboard::new([[Button::action(
            "go",
            "Go",
            ButtonAction::Start {
                state: "next".into(),
                data: serde_json::Value::Null,
                mode: StartMode::Normal,
            },
        )]]);

        assert!(keyboard.handle_callback(&ctx, "td:another:go").is_none());
    }

    #[test]
    fn select_renders_and_resolves_string_payloads() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let select = Select::<String, String, _, _, _, _>::new(
            "fruit",
            |_data: &DataMap| vec!["red:apple".to_owned(), "pear".to_owned()],
            |item: &String, _data: &DataMap| item.clone(),
            |item: &String| item.clone(),
            |value: String| ButtonAction::set_dialog_value("fruit", value),
        )
        .items_per_row(2);

        let markup = select
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let callback_data = markup
            .inline_keyboard()
            .and_then(|rows: &[Box<[telers::types::InlineKeyboardButton]>]| rows.first())
            .and_then(|row: &Box<[telers::types::InlineKeyboardButton]>| row.first())
            .and_then(|button| button.callback_data.as_deref())
            .expect("callback data");

        assert_eq!(callback_data, format!("td:{}:fruit:red%3Aapple", ctx.id));

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
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let select = Select::<String, String, _, _, _, _>::new(
            "fruit",
            |_data: &DataMap| vec!["pear".to_owned()],
            |item: &String, _data: &DataMap| item.clone(),
            |item: &String| item.clone(),
            |value: String| ButtonAction::set_dialog_value("fruit", value),
        )
        .footer_row([Button::done("done", "Done")]);

        let action = select
            .handle_callback(&ctx, &format!("td:{}:done", ctx.id))
            .expect("footer action");

        assert!(matches!(action, ButtonAction::Done));
    }
}
