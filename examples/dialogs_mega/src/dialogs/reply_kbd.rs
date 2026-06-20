//! Reply-keyboard request widgets: `RequestContact`, `RequestLocation`,
//! `RequestPoll`, each paired with a `MessageInput` that advances the flow.
//!
//! A request widget owns the message's reply markup, so these windows do not
//! also place an inline keyboard; the user advances by sending the requested
//! payload. The final window uses an inline `☰ Main menu` button.

use telers::{
    enums::PollType,
    types::{MessageContact, MessageLocation, MessagePoll},
};
use telers_dialog::{
    widgets::{
        format_text, input, keyboard, ButtonAction, InlineKeyboard, MessageInput, RequestContact,
        RequestLocation, RequestPoll,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "reply_contact";

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                format_text(
                    "Reply keyboard\n\nStep 1 of 3. Tap the button to share a contact. \
                     `RequestContact` shows a reply-keyboard button; `MessageInput` receives it.",
                ),
                keyboard(
                    RequestContact::builder("Share contact")
                        .resize_keyboard(true)
                        .one_time_keyboard(true)
                        .build(),
                ),
                input(MessageInput::new(
                    |_ctx, message: MessageContact| async move {
                        ButtonAction::chain([
                            ButtonAction::set_dialog_value(
                                "contact_name",
                                message.contact.first_name.to_string(),
                            ),
                            ButtonAction::next(),
                        ])
                    },
                )),
            ],
        ),
        window(
            "reply_location",
            [
                format_text(
                    "Reply keyboard\n\nStep 2 of 3. Share a location. `RequestLocation` gives a \
                     location button.",
                ),
                keyboard(
                    RequestLocation::builder("Share location")
                        .resize_keyboard(true)
                        .one_time_keyboard(true)
                        .build(),
                ),
                input(MessageInput::new(
                    |_ctx, message: MessageLocation| async move {
                        ButtonAction::chain([
                            ButtonAction::set_dialog_value(
                                "location",
                                format!(
                                    "{:.4}, {:.4}",
                                    message.location.latitude, message.location.longitude
                                ),
                            ),
                            ButtonAction::next(),
                        ])
                    },
                )),
            ],
        ),
        window(
            "reply_poll",
            [
                format_text(
                    "Reply keyboard\n\nStep 3 of 3. Create a poll. `RequestPoll` asks Telegram to \
                     build a native poll.",
                ),
                keyboard(
                    RequestPoll::builder("Create poll")
                        .poll_type(PollType::Regular)
                        .resize_keyboard(true)
                        .one_time_keyboard(true)
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: MessagePoll| async move {
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value(
                            "poll_question",
                            message.poll.question().to_string(),
                        ),
                        ButtonAction::next(),
                    ])
                })),
            ],
        ),
        window(
            "reply_done",
            [
                format_text(
                    "Reply keyboard\n\nContact: {contact_name}\nLocation: {location}\nPoll: \
                     {poll_question}\n\nReply-keyboard request widgets collect Telegram-native \
                     payloads that a matching `MessageInput` stores.",
                ),
                keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
            ],
        ),
    ])
}
