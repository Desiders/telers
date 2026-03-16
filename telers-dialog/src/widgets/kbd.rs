use crate::entities::{Context, Data, DataMap, StartMode};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

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
    Chain(Box<[ButtonAction]>),
}

pub trait Keyboard: Send + Sync + 'static {
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup>;

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction>;
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
            ButtonKind::Callback(_) => button.callback_data(self.callback_data(ctx)),
            ButtonKind::Url(url) => button.url(url.clone()),
        }
    }

    fn resolve_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        let button_id = parse_callback_data(ctx, callback_data)?;
        if button_id != self.id.as_ref() {
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

    fn callback_data(&self, ctx: &Context) -> String {
        format!("{CALLBACK_PREFIX}:{}:{}", ctx.id, self.id)
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

    #[must_use]
    pub fn row(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.rows.push(buttons.into_iter().collect());
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

fn parse_callback_data<'a>(ctx: &Context, callback_data: &'a str) -> Option<&'a str> {
    let mut parts = callback_data.splitn(3, ':');
    if parts.next()? != CALLBACK_PREFIX {
        return None;
    }
    if parts.next()? != ctx.id {
        return None;
    }
    parts.next()
}

#[cfg(test)]
mod tests {
    use super::{Button, ButtonAction, InlineKeyboard, Keyboard};
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
}
