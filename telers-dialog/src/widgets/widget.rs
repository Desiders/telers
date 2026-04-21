use std::borrow::Cow;

use super::{Input, Keyboard, LinkPreviewWidget, MultiInput, MultiKeyboard, MultiText, Text};
use crate::{
    entities::DataMap,
    widgets::{text::MultiTextBuilder, FnText, FormatText},
};

pub type WindowWidgets = (
    Box<dyn Text>,
    Option<Box<dyn Keyboard>>,
    Option<Box<dyn Input>>,
    Option<Box<dyn LinkPreviewWidget>>,
);

pub enum WidgetKind {
    Text(Box<dyn Text>),
    Keyboard(Box<dyn Keyboard>),
    Input(Box<dyn Input>),
    LinkPreview(Box<dyn LinkPreviewWidget>),
}

#[inline]
#[must_use]
pub fn text(val: impl Text) -> WidgetKind {
    WidgetKind::Text(Box::new(val))
}

#[inline]
#[must_use]
pub fn fn_text<Renderer, Item>(renderer: Renderer) -> WidgetKind
where
    Renderer: Fn(&DataMap) -> Item + Send + Sync + 'static,
    Item: Into<Box<str>>,
{
    WidgetKind::Text(Box::new(FnText::new(renderer)))
}

#[inline]
#[must_use]
pub fn format_text(template: impl Into<Cow<'static, str>>) -> WidgetKind {
    WidgetKind::Text(Box::new(FormatText::new(template)))
}

#[inline]
#[must_use]
pub fn keyboard(val: impl Keyboard) -> WidgetKind {
    WidgetKind::Keyboard(Box::new(val))
}

#[inline]
#[must_use]
pub fn input(val: impl Input) -> WidgetKind {
    WidgetKind::Input(Box::new(val))
}

#[inline]
#[must_use]
pub fn link_preview(val: impl LinkPreviewWidget) -> WidgetKind {
    WidgetKind::LinkPreview(Box::new(val))
}

/// Normalize a window widget list into the concrete slots used by `WindowImpl`.
///
/// # Panics
/// Panics if no text widget is present.
pub(crate) fn ensure_widgets(widgets: impl IntoIterator<Item = WidgetKind>) -> WindowWidgets {
    let mut texts = Vec::new();
    let mut kbds = Vec::new();
    let mut inputs = Vec::new();
    let mut link_previews = Vec::new();
    for widget in widgets {
        match widget {
            WidgetKind::Text(val) => texts.push(val),
            WidgetKind::Keyboard(val) => kbds.push(val),
            WidgetKind::Input(val) => inputs.push(val),
            WidgetKind::LinkPreview(val) => link_previews.push(val),
        }
    }
    let text = match texts.len() {
        0 => panic!("`Window` must have at least one `Text` widget"),
        1 => texts.pop().unwrap(),
        _ => Box::new(
            texts
                .into_iter()
                .fold(MultiText::builder(), MultiTextBuilder::text_boxed)
                .build(),
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
    let link_preview = link_previews.pop();
    (text, kbd, input, link_preview)
}

#[cfg(test)]
mod tests {
    use super::{input, keyboard, link_preview, text, WidgetKind};
    use crate::widgets::{
        Button, ButtonAction, InlineKeyboard, LinkPreview, TextInput, TextInputContext,
    };

    async fn noop_input(_ctx: TextInputContext, _val: i64) -> ButtonAction {
        ButtonAction::noop()
    }

    #[tokio::test]
    async fn shortcut_builders_return_expected_widget_kinds() {
        assert!(matches!(text("hello"), WidgetKind::Text(_)));
        assert!(matches!(
            keyboard(
                InlineKeyboard::builder()
                    .row([Button::done("done", "Done")])
                    .build()
            ),
            WidgetKind::Keyboard(_)
        ));
        assert!(matches!(
            input(TextInput::builder("id").on_success(noop_input).build()),
            WidgetKind::Input(_)
        ));
        assert!(matches!(
            link_preview(LinkPreview::builder().is_disabled(true).build()),
            WidgetKind::LinkPreview(_)
        ));
    }
}
