use super::{kbd::Keyboard, text::Text};

pub enum WidgetKind {
    Text(Box<dyn Text>),
    Keyboard(Box<dyn Keyboard>),
}

impl WidgetKind {
    pub fn text(t: impl Text) -> Self {
        Self::Text(Box::new(t))
    }
    pub fn keyboard(k: impl Keyboard) -> Self {
        Self::Keyboard(Box::new(k))
    }
}

pub fn ensure_widgets(
    widgets: impl IntoIterator<Item = WidgetKind>,
) -> (Box<dyn Text>, Option<Box<dyn Keyboard>>) {
    let mut text: Option<Box<dyn Text>> = None;
    let mut keyboard: Option<Box<dyn Keyboard>> = None;
    for widget in widgets {
        match widget {
            WidgetKind::Text(val) => {
                if text.is_none() {
                    text = Some(val);
                }
            }
            WidgetKind::Keyboard(val) => {
                if keyboard.is_none() {
                    keyboard = Some(val);
                }
            }
        }
    }
    let text = text.expect("Window must contain Text widget");
    (text, keyboard)
}
