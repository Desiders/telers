//! `TemplateText` (minijinja) with the default environment and a custom one.
//!
//! Requires the `template` feature, enabled in this crate's `Cargo.toml`.

use serde_json::json;
use telers_dialog::{
    widgets::{
        keyboard, text, Button, ButtonAction, InlineKeyboard, TemplateEnvBuilder, TemplateText,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "template_intro";

// The default environment enables `trim_blocks`/`lstrip_blocks`, so block tags
// (`{% ... %}`) sit on their own lines: the newline that follows each tag is
// trimmed, which keeps the rendered output cleanly line-broken. `rustfmt::skip`
// keeps this hand-laid-out template readable (the `\` line continuations strip
// the source indentation, so it does not leak into the rendered text).
#[rustfmt::skip]
const DEFAULT_TEMPLATE: &str =
    "Hello, {{ user.name | upper }}!\n\
     {% if user.premium %}\n\
     You have premium access.\n\
     {% else %}\n\
     Upgrade for premium features.\n\
     {% endif %}\n\
     You have {{ user.items | length }} items in your cart.";

#[rustfmt::skip]
const RECEIPT_TEMPLATE: &str =
    "{{ brand }} receipt for {{ user.name | title }}\n\
     {% if user.premium %}\n\
     Status: premium ({{ 10 }}% off applied)\n\
     {% else %}\n\
     Status: regular\n\
     {% endif %}\n\
     Total: {{ price | currency }}\n\
     Items:\n\
     {% for item in user.items %}\n\
     - {{ item }}\n\
     {% endfor %}";

pub fn dialog() -> impl Dialog {
    // Custom environment: a `currency` filter and a `brand` global.
    let env = TemplateEnvBuilder::new()
        .add_filter("currency", |v: f64| format!("${:.2}", v))
        .add_global("brand", "North Roast")
        .build();
    let receipt = TemplateText::builder(RECEIPT_TEMPLATE).env(env).build();

    telers_dialog::dialog([
        window(
            STATE,
            [
                text(
                    "Template text\n\nDialogs started from the menu begin with empty data, so the \
                     button below seeds the render data first, then renders the templates.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "seed",
                            "Render templates",
                            ButtonAction::chain([
                                ButtonAction::extend_dialog_data([
                                    (
                                        "user",
                                        json!({
                                            "name": "alice",
                                            "premium": true,
                                            "items": ["espresso", "croissant", "oat milk"],
                                        }),
                                    ),
                                    ("price", json!(42.5)),
                                ]),
                                ButtonAction::next(),
                            ]),
                        )])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "template_default",
            [
                text(
                    "Default environment\n\nVariables, filters, conditionals, and loops map to \
                     minijinja syntax.\n",
                ),
                text(TemplateText::builder(DEFAULT_TEMPLATE).build()),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::next("next", "Custom environment")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "template_custom",
            [
                text(
                    "Custom environment\n\n`TemplateEnvBuilder` registers a `currency` filter and \
                     a `brand` global before handing the environment to the template.\n",
                ),
                text(receipt),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
    ])
}
