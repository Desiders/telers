//! Plain reply-keyboard rows, native sharing buttons, and keyboard removal.
//!
//! A request widget owns the message's reply markup, so these windows do not
//! also place an inline keyboard; the user advances by sending the requested
//! payload. The final window uses an inline `☰ Main menu` button.

use telers::{
    enums::PollType,
    types::{KeyboardButtonRequestChat, KeyboardButtonRequestUsers, Message, MessageContact,
        MessageLocation, MessagePoll},
};
use telers_dialog::{
    widgets::{
        format_text, input, keyboard, text, ButtonAction, ButtonStyle, FormatText, InlineKeyboard,
        MessageInput, ReplyButton, ReplyKeyboard, ReplyKeyboardRemove, RequestContact,
        RequestLocation, RequestPoll,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "reply_choices";

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                format_text(
                    "Reply keyboard\n\nPlain buttons send text messages; native buttons send \
                     matching Telegram payloads. Select a user or group to try the native \
                     choices, then tap Continue for contact/location/poll requests.\n\n\
                     {shared_selection}",
                ),
                keyboard(
                    ReplyKeyboard::builder()
                        .row([
                            ReplyButton::text("Continue").style(ButtonStyle::Primary),
                            ReplyButton::text("☰ Main menu"),
                        ])
                        .row([
                            ReplyButton::request_users(
                                "Select user", KeyboardButtonRequestUsers::new(1).user_is_bot(false),
                            ),
                            ReplyButton::request_chat(
                                "Select group", KeyboardButtonRequestChat::new(2, false),
                            ),
                        ])
                        .resize_keyboard(true)
                        .input_field_placeholder(FormatText::new("Pick an option. {shared_selection}"))
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: Message| async move {
                    match message.text() {
                        Some("Continue") => return ButtonAction::next(),
                        Some("☰ Main menu") => return ButtonAction::done(),
                        _ => {}
                    }
                    let selection = if let Some(shared) = message.users_shared() {
                        format!("Shared {} user(s).", shared.users.len())
                    } else if let Some(shared) = message.chat_shared() {
                        format!("Shared group {}.", shared.chat_id)
                    } else {
                        "Tap Continue or choose a sharing button.".to_owned()
                    };
                    ButtonAction::set_dialog_value("shared_selection", selection)
                })),
            ],
        ),
        window(
            "reply_contact",
            [
                format_text(
                    "Reply keyboard\n\nStep 1 of 3. Tap the button to share a contact. \
                     `RequestContact` shows a reply-keyboard button; `MessageInput` receives it.",
                ),
                keyboard(
                    RequestContact::builder("Share contact")
                        .resize_keyboard(true)
                        .one_time_keyboard(true)
                        .style(ButtonStyle::Success)
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
            "reply_remove",
            [
                text("The reply keyboard has been removed. Send any message to see the summary."),
                keyboard(ReplyKeyboardRemove::builder().build()),
                input(MessageInput::new(|_ctx, _message: Message| async move {
                    ButtonAction::next()
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
