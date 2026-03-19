use super::{
    input::{Input, MultiInput},
    kbd::{Keyboard, MultiKeyboard},
    text::{MultiText, Text},
};

pub type WindowWidgets = (
    Box<dyn Text>,
    Option<Box<dyn Keyboard>>,
    Option<Box<dyn Input>>,
);

pub enum WidgetKind {
    Text(Box<dyn Text>),
    Keyboard(Box<dyn Keyboard>),
    Input(Box<dyn Input>),
}

impl WidgetKind {
    pub fn text(t: impl Text) -> Self {
        Self::Text(Box::new(t))
    }

    pub fn keyboard(k: impl Keyboard) -> Self {
        Self::Keyboard(Box::new(k))
    }

    pub fn input(i: impl Input) -> Self {
        Self::Input(Box::new(i))
    }
}

#[inline]
#[must_use]
pub fn text(t: impl Text) -> WidgetKind {
    WidgetKind::text(t)
}

#[inline]
#[must_use]
pub fn keyboard(k: impl Keyboard) -> WidgetKind {
    WidgetKind::keyboard(k)
}

#[inline]
#[must_use]
pub fn input(i: impl Input) -> WidgetKind {
    WidgetKind::input(i)
}

/// Normalize a window widget list into the concrete slots used by `WindowImpl`.
///
/// # Panics
/// Panics if no text widget is present.
pub(crate) fn ensure_widgets(widgets: impl IntoIterator<Item = WidgetKind>) -> WindowWidgets {
    let mut texts: Vec<Box<dyn Text>> = Vec::new();
    let mut keyboards: Vec<Box<dyn Keyboard>> = Vec::new();
    let mut inputs: Vec<Box<dyn Input>> = Vec::new();
    for widget in widgets {
        match widget {
            WidgetKind::Text(val) => texts.push(val),
            WidgetKind::Keyboard(val) => keyboards.push(val),
            WidgetKind::Input(val) => inputs.push(val),
        }
    }
    let text: Box<dyn Text> = match texts.len() {
        0 => panic!("Window must contain Text widget"),
        1 => texts.pop().expect("single text"),
        _ => Box::new(MultiText::new(texts)),
    };
    let keyboard = match keyboards.len() {
        0 => None,
        1 => Some(keyboards.pop().expect("single keyboard")),
        _ => Some(Box::new(MultiKeyboard::new(keyboards)) as Box<dyn Keyboard>),
    };
    let input = match inputs.len() {
        0 => None,
        1 => Some(inputs.pop().expect("single input")),
        _ => Some(Box::new(MultiInput::new(inputs)) as Box<dyn Input>),
    };
    (text, keyboard, input)
}

#[cfg(test)]
mod tests {
    use super::{input, keyboard, text, WidgetKind};
    use crate::widgets::{Button, InlineKeyboard, MessageInput};

    #[test]
    fn shortcut_builders_return_expected_widget_kinds() {
        assert!(matches!(text("hello"), WidgetKind::Text(_)));
        assert!(matches!(
            keyboard(InlineKeyboard::new([[Button::done("done", "Done")]])),
            WidgetKind::Keyboard(_)
        ));
        assert!(matches!(
            input(MessageInput::new(|_ctx, _message| None)),
            WidgetKind::Input(_)
        ));
    }
}
