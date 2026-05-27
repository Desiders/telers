//! Inline button styles and dynamic (data-rendered) payloads.

use telers_dialog::{
    widgets::{format_text, keyboard, text, Button, ButtonAction, FormatText, InlineKeyboard},
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "buttons_styles";

fn back_nav() -> InlineKeyboard {
    InlineKeyboard::builder()
        .row([Button::switch_to("back", "Back", STATE)])
        .row([main_menu_button()])
        .build()
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                text(
                    "Button styles\n\nThe style helpers map to Telegram's coloured callback \
                     buttons.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "delete",
                            "Delete draft",
                            ButtonAction::switch_to("buttons_destructive"),
                        )
                        .danger()])
                        .row([Button::action(
                            "confirm",
                            "Confirm changes",
                            ButtonAction::switch_to("buttons_confirmed"),
                        )
                        .success()])
                        .row([Button::action(
                            "dynamic",
                            "Dynamic payloads",
                            ButtonAction::chain([
                                ButtonAction::extend_dialog_data([
                                    ("docs_url", "https://core.telegram.org/bots/api"),
                                    ("copy_payload", "td_promo_2026"),
                                    ("inline_query", "@gif weekend"),
                                    ("web_app_url", "https://example.com/mini-app"),
                                ]),
                                ButtonAction::switch_to("buttons_dynamic"),
                            ]),
                        )
                        .primary()])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "buttons_destructive",
            [
                text("Confirm deletion\n\nThe red button led here."),
                keyboard(back_nav()),
            ],
        ),
        window(
            "buttons_confirmed",
            [
                text("Changes confirmed\n\nThe green button led here."),
                keyboard(back_nav()),
            ],
        ),
        window(
            "buttons_dynamic",
            [
                format_text(
                    "Dynamic payloads\n\nEach button renders its payload from dialog \
                     data.\n\nDocs URL: {docs_url}\nCopy payload: {copy_payload}\nInline query: \
                     {inline_query}\nWeb app URL: {web_app_url}",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::url_dynamic(
                            "Open docs",
                            FormatText::new("{docs_url}"),
                        )])
                        .row([Button::copy_text_dynamic(
                            "Copy promo code",
                            FormatText::new("{copy_payload}"),
                        )])
                        .row([Button::switch_inline_query_dynamic(
                            "Share inline",
                            FormatText::new("{inline_query}"),
                        )])
                        .row([Button::web_app_dynamic(
                            "Open mini-app",
                            FormatText::new("{web_app_url}"),
                        )])
                        .row([Button::switch_to("back", "Back", STATE)])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
    ])
}
