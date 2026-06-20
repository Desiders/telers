//! Step-by-step input with `Next`/`Back` and a `Case` summary.

use serde_json::json;
use telers_dialog::{
    entities::DataMap,
    widgets::{keyboard, text, Button, ButtonAction, Case, InlineKeyboard},
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "switch_step1";

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                text(
                    "Multi-step input\n\nMultiple windows in one dialog make a step-by-step \
                     flow.\n\nStep 1. Press Next.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::next("next", "Next ▶️")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "switch_step2",
            [
                text("Step 2. Choose a plan. The choice is stored in dialog data."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "basic",
                            "Basic",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("plan", "basic"),
                                ButtonAction::next(),
                            ]),
                        )])
                        .row([Button::action(
                            "pro",
                            "Pro",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("plan", "pro"),
                                ButtonAction::next(),
                            ]),
                        )])
                        .row([Button::back("back", "◀️ Back")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "switch_step3",
            [
                text("Step 3. Your selection:"),
                text(
                    Case::builder(|data: &DataMap| data.get("plan").cloned())
                        .when(Some(json!("basic")), "Plan: Basic — the essentials.")
                        .when(Some(json!("pro")), "Plan: Pro — every feature.")
                        .default("Plan: none chosen.")
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "◀️ Back")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
    ])
}
