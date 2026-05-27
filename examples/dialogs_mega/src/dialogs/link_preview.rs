//! `LinkPreview` options: disable, small/large media, and show-above-text.

use telers_dialog::{
    widgets::{keyboard, link_preview, text, Button, InlineKeyboard, LinkPreview},
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "linkpreview_menu";

const COMMON_TEXT: &str =
    "Link preview\n\nLink in text: https://www.youtube.com/watch?v=dQw4w9WgXcQ\nThe preview below \
     shows the current mode:";
const PHOTO_URL: &str = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";

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
                text(COMMON_TEXT),
                text("Default (no override)"),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("d", "Disabled", "linkpreview_disabled")])
                        .row([Button::switch_to(
                            "s",
                            "Prefer small media",
                            "linkpreview_small",
                        )])
                        .row([Button::switch_to(
                            "l",
                            "Prefer large media",
                            "linkpreview_large",
                        )])
                        .row([Button::switch_to(
                            "a",
                            "Show above text",
                            "linkpreview_above",
                        )])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "linkpreview_disabled",
            [
                text(COMMON_TEXT),
                text("is_disabled = true"),
                link_preview(LinkPreview::builder().is_disabled(true).build()),
                keyboard(back_nav()),
            ],
        ),
        window(
            "linkpreview_small",
            [
                text(COMMON_TEXT),
                text("prefer_small_media = true"),
                link_preview(
                    LinkPreview::builder()
                        .url(PHOTO_URL)
                        .prefer_small_media(true)
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "linkpreview_large",
            [
                text(COMMON_TEXT),
                text("prefer_large_media = true"),
                link_preview(
                    LinkPreview::builder()
                        .url(PHOTO_URL)
                        .prefer_large_media(true)
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "linkpreview_above",
            [
                text(COMMON_TEXT),
                text("show_above_text = true"),
                link_preview(
                    LinkPreview::builder()
                        .url(PHOTO_URL)
                        .show_above_text(true)
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
    ])
}
