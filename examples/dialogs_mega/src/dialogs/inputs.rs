//! Text-driven inputs: `TextInput` (typed parsing with an error branch) and
//! `ForceReply` (a reply-markup widget paired with a `MessageInput`).
//!
//! `ForceReply` only opens the reply UI when its message is *sent* (Telegram
//! ignores it on an edit), so it is reached through a message transition: the
//! `TextInput` window carries no inline keyboard, and sending the number sends
//! the `ForceReply` window fresh.

use telers::types::MessageText;
use telers_dialog::{
    widgets::{
        format_text, input, keyboard, text, ButtonAction, ForceReply, InlineKeyboard, MessageInput,
        TextInput,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "inputs_text";

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                format_text(
                    "Text & force reply\n\nStep 1 of 2. Send a whole number (an age). `TextInput` \
                     parses it into an `i64`; text that is not a number takes the error \
                     branch.\n\nAge: {age}\n{input_error}",
                ),
                input(
                    TextInput::builder("age_input")
                        .on_success(|_ctx, age: i64| async move {
                            ButtonAction::chain([
                                ButtonAction::extend_dialog_data([
                                    ("age", age.to_string()),
                                    ("input_error", String::new()),
                                ]),
                                ButtonAction::next(),
                            ])
                        })
                        .on_error(|_ctx, _err| async move {
                            ButtonAction::set_dialog_value(
                                "input_error",
                                "That was not a whole number, try again.",
                            )
                        })
                        .build(),
                ),
            ],
        ),
        window(
            "inputs_force",
            [
                text(
                    "Text & force reply\n\nStep 2 of 2. Telegram auto-opens the reply UI with the \
                     placeholder. Send a nickname; the `MessageInput` stores it.",
                ),
                keyboard(
                    ForceReply::builder()
                        .input_field_placeholder("Type a nickname")
                        .build(),
                ),
                input(MessageInput::new(|_ctx, message: MessageText| async move {
                    ButtonAction::chain([
                        ButtonAction::set_dialog_value("nickname", message.text.to_string()),
                        ButtonAction::next(),
                    ])
                })),
            ],
        ),
        window(
            "inputs_done",
            [
                format_text(
                    "Text & force reply\n\nAge: {age}\nNickname: {nickname}\n\n`TextInput` parses \
                     typed values with an error branch; `ForceReply` only owns the reply markup \
                     while a `MessageInput` consumes the reply.",
                ),
                keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
            ],
        ),
    ])
}
