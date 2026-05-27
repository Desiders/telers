//! `TextInput`: typed parsing of a sent message with an error branch.
//!
//! `TextInput` parses the message text into the type expected by `on_success`;
//! a value that fails to parse takes the `on_error` branch instead. (Free-form
//! message handling is shown by `MessageInput` in the reply-keyboard dialog.)

use telers_dialog::{
    widgets::{format_text, input, keyboard, ButtonAction, InlineKeyboard, TextInput},
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
                    "Text input\n\nSend a whole number (an age). `TextInput` parses it into an \
                     `i64`; text that is not a number takes the error branch.\n\nAge: \
                     {age}\n{input_error}",
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
                keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
            ],
        ),
        window(
            "inputs_done",
            [
                format_text(
                    "Text input\n\nStored age: {age}\n\n`TextInput` parsed the message into a \
                     typed value, and the chained action advanced the dialog.",
                ),
                keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
            ],
        ),
    ])
}
