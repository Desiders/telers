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

#[inline]
#[must_use]
pub fn text(t: impl Text) -> WidgetKind {
    WidgetKind::Text(Box::new(t))
}

#[inline]
#[must_use]
pub fn keyboard(k: impl Keyboard) -> WidgetKind {
    WidgetKind::Keyboard(Box::new(k))
}

#[inline]
#[must_use]
pub fn input(i: impl Input) -> WidgetKind {
    WidgetKind::Input(Box::new(i))
}

/// Normalize a window widget list into the concrete slots used by `WindowImpl`.
///
/// # Panics
/// Panics if no text widget is present.
pub(crate) fn ensure_widgets(widgets: impl IntoIterator<Item = WidgetKind>) -> WindowWidgets {
    let mut texts = Vec::new();
    let mut kbds = Vec::new();
    let mut inputs = Vec::new();
    for widget in widgets {
        match widget {
            WidgetKind::Text(val) => texts.push(val),
            WidgetKind::Keyboard(val) => kbds.push(val),
            WidgetKind::Input(val) => inputs.push(val),
        }
    }
    let text = match texts.len() {
        0 => panic!("`Window` must have at least one `Text` widget"),
        1 => texts.pop().unwrap(),
        _ => Box::new(
            texts
                .into_iter()
                .fold(MultiText::new(), MultiText::text_boxed),
        ),
    };
    let kbd: Option<_> = match kbds.len() {
        0 => None,
        1 => Some(kbds.pop().unwrap()),
        _ => Some(Box::new(
            kbds.into_iter()
                .fold(MultiKeyboard::new(), MultiKeyboard::kbd_boxed),
        )),
    };
    let input: Option<_> = match inputs.len() {
        0 => None,
        1 => Some(inputs.pop().expect("single input")),
        _ => Some(Box::new(
            inputs
                .into_iter()
                .fold(MultiInput::new(), MultiInput::input_boxed),
        )),
    };
    (text, kbd, input)
}

#[cfg(test)]
mod tests {
    use super::{input, keyboard, text, WidgetKind};
    use crate::widgets::{Button, InlineKeyboard, MessageInput};

    #[test]
    fn shortcut_builders_return_expected_widget_kinds() {
        assert!(matches!(text("hello"), WidgetKind::Text(_)));
        assert!(matches!(
            keyboard(InlineKeyboard::new().row([Button::done("done", "Done")])),
            WidgetKind::Keyboard(_)
        ));
        assert!(matches!(
            input(MessageInput::new(|_ctx, _message| None)),
            WidgetKind::Input(_)
        ));
    }
}
