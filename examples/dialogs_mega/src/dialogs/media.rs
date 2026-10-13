//! Media widgets: static and dynamic photos, galleries, dice and reusable Telegram media.
//!
//! The URLs below point at `picsum.photos`, which returns real JPEG images that
//! Telegram can fetch. Telegram rejects URLs that do not resolve to a raster
//! image (for example an SVG placeholder), so swap in your own public image URL
//! or a `file_id` your bot owns when adapting this example.

use telers::{serialization::json, types::{MessageSticker, MessageVideoNote, MessageVoice}};
use telers_dialog::{
    widgets::{
        format_text, input, keyboard, media, text, Button, ButtonAction, DynamicMedia, InlineKeyboard,
        MediaContentType, MediaScroll, MessageInput, MessageInputContext, NumberedPager, StaticMedia,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "media_static";

const STATIC_URL: &str = "https://picsum.photos/seed/telers-static/600/400";
const DYNAMIC_A: &str = "https://picsum.photos/seed/telers-a/600/400";
const DYNAMIC_B: &str = "https://picsum.photos/seed/telers-b/600/400";
const GALLERY: &[&str] = &[
    "https://picsum.photos/seed/telers-1/600/400",
    "https://picsum.photos/seed/telers-2/600/400",
    "https://picsum.photos/seed/telers-3/600/400",
    "https://picsum.photos/seed/telers-4/600/400",
];

async fn use_sticker(_ctx: MessageInputContext, message: MessageSticker) -> ButtonAction {
    ButtonAction::set_dialog_value("sticker_file_id", message.sticker.file_id().to_owned())
}

async fn use_voice(_ctx: MessageInputContext, message: MessageVoice) -> ButtonAction {
    ButtonAction::set_dialog_value("voice_file_id", message.voice.file_id.to_string())
}

async fn use_video_note(_ctx: MessageInputContext, message: MessageVideoNote) -> ButtonAction {
    ButtonAction::set_dialog_value("video_note_file_id", message.video_note.file_id.to_string())
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                text("Static media\n\nA fixed photo from a URL with a caption."),
                media(
                    StaticMedia::builder(MediaContentType::Photo)
                        .url(STATIC_URL)
                        .caption("Photo loaded from a public URL")
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "to_dynamic",
                            "Dynamic media",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("photo_url", DYNAMIC_A),
                                ButtonAction::switch_to("media_dynamic"),
                            ]),
                        )])
                        .row([Button::switch_to("to_dice", "Roll dice", "media_dice")])
                        .row([Button::switch_to("to_sticker", "Reuse a sticker", "media_sticker")])
                        .row([Button::switch_to("to_voice", "Reuse a voice message", "media_voice")])
                        .row([Button::switch_to("to_note", "Reuse a video note", "media_video_note")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "media_dynamic",
            [
                format_text(
                    "Dynamic media\n\nThe photo URL is read from \
                     `dialog_data[photo_url]`.\nCurrent: {photo_url}",
                ),
                media(DynamicMedia::from_url_field(
                    MediaContentType::Photo,
                    "photo_url",
                )),
                keyboard(
                    InlineKeyboard::builder()
                        .row([
                            Button::action(
                                "use_a",
                                "Use A",
                                ButtonAction::set_dialog_value("photo_url", DYNAMIC_A),
                            ),
                            Button::action(
                                "use_b",
                                "Use B",
                                ButtonAction::set_dialog_value("photo_url", DYNAMIC_B),
                            ),
                        ])
                        .row([Button::action(
                            "to_scroll",
                            "Open gallery scroll",
                            ButtonAction::chain([
                                ButtonAction::set_dialog_value("gallery_urls", json!(GALLERY)),
                                ButtonAction::switch_to("media_scroll"),
                            ]),
                        )])
                        .row([Button::switch_to("back", "Back", STATE)])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "media_scroll",
            [
                text(
                    "Media scroll\n\n`NumberedPager` writes the current page into `widget_data`; \
                     `MediaScroll` renders the attachment for that page from the URL array in \
                     dialog data.",
                ),
                media(MediaScroll::from_url_array_field(
                    "gallery",
                    MediaContentType::Photo,
                    "gallery_urls",
                )),
                keyboard(
                    NumberedPager::builder(MediaScroll::from_url_array_field(
                        "gallery",
                        MediaContentType::Photo,
                        "gallery_urls",
                    ))
                    .page_renderer(|page, _data| format!("{}", page + 1))
                    .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                    .length(5)
                    .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("back", "Back", "media_dynamic")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "media_dice",
            [
                text(""),
                media(StaticMedia::builder(MediaContentType::Dice).emoji("🎲").build()),
                keyboard(InlineKeyboard::builder()
                    .row([Button::switch_to("roll", "Roll again", "media_dice")])
                    .row([Button::switch_to("back", "Back", STATE)])
                    .row([main_menu_button()]).build()),
            ],
        ),
        window(
            "media_sticker",
            [
                text("Send a sticker to reuse it in this dialog."),
                input(MessageInput::new(use_sticker)),
                media(DynamicMedia::from_file_id_field(MediaContentType::Sticker, "sticker_file_id")),
                keyboard(InlineKeyboard::builder()
                    .row([Button::switch_to("back", "Back", STATE)])
                    .row([main_menu_button()]).build()),
            ],
        ),
        window(
            "media_voice",
            [
                text("Send a voice message to reuse it in this dialog."),
                input(MessageInput::new(use_voice)),
                media(DynamicMedia::from_file_id_field(MediaContentType::Voice, "voice_file_id")),
                keyboard(InlineKeyboard::builder()
                    .row([Button::switch_to("back", "Back", STATE)])
                    .row([main_menu_button()]).build()),
            ],
        ),
        window(
            "media_video_note",
            [
                text("Send a video note to reuse it in this dialog."),
                input(MessageInput::new(use_video_note)),
                media(DynamicMedia::from_file_id_field(MediaContentType::VideoNote, "video_note_file_id")),
                keyboard(InlineKeyboard::builder()
                    .row([Button::switch_to("back", "Back", STATE)])
                    .row([main_menu_button()]).build()),
            ],
        ),
    ])
}
