//! Root menu dialog.
//!
//! Each button starts a feature dialog on top of the menu with
//! [`StartMode::Normal`]; the feature dialogs return here via [`Button::done`].

use serde_json::Value;
use telers_dialog::{
    widgets::{keyboard, text, Button, InlineKeyboard},
    window, Dialog, LaunchMode, StartMode,
};

use crate::{
    common::MAIN_MENU_STATE,
    dialogs::{
        button_actions, buttons, calendar, counter, inputs, layouts, link_preview, media,
        multiwidget, reply_kbd, scrolls, selects, switch, template, text_widgets,
    },
};

/// A menu button that starts the feature dialog owning `state`.
fn link(id: &'static str, label: impl telers_dialog::widgets::Text, state: &'static str) -> Button {
    Button::start(id, label, state, Value::Null, StartMode::Normal)
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(
        MAIN_MENU_STATE,
        [
            text(
                "telers-dialog demo\n\nThis bot combines the individual dialog examples. Pick a \
                 feature below; every screen has a \"☰ Main menu\" button to come back here.",
            ),
            keyboard(
                InlineKeyboard::builder()
                    .row([link("m_text", "📝 Text widgets", text_widgets::STATE)])
                    .row([link("m_template", "🧩 Template text", template::STATE)])
                    .row([link("m_scrolls", "📜 Scrolling widgets", scrolls::STATE)])
                    .row([link("m_layouts", "📐 Keyboard layouts", layouts::STATE)])
                    .row([link("m_selects", "☑️ Selection widgets", selects::STATE)])
                    .row([link("m_multi", "🎛 Combined widgets", multiwidget::STATE)])
                    .row([link("m_counter", "💯 Counter & progress", counter::STATE)])
                    .row([link("m_calendar", "📅 Calendar & time", calendar::STATE)])
                    .row([link("m_switch", "🔢 Multi-step input", switch::STATE)])
                    .row([link("m_reply", "⌨️ Reply keyboard", reply_kbd::STATE)])
                    .row([link("m_inputs", "✍️ Text input", inputs::STATE)])
                    .row([link("m_buttons", "🎨 Button styles", buttons::STATE)])
                    .row([link(
                        "m_actions",
                        "🧭 Button actions",
                        button_actions::STATE,
                    )])
                    .row([link("m_link", "🔗 Link preview", link_preview::STATE)])
                    .row([link("m_media", "🖼 Media widgets", media::STATE)])
                    .build(),
            ),
        ],
    )])
    // Always reset the stack when this dialog is started, so `/start` (and any
    // explicit start) returns to a clean menu.
    .with_launch_mode(LaunchMode::Root)
}
