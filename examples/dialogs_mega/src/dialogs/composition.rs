use telers_dialog::{
    entities::{DataMap, RenderContext},
    widgets::{
        input, keyboard, text, Button, ButtonAction, ButtonAppearance, ButtonStyle, CombinedInput, FirstText,
        FnText, InlineKeyboard, Input, InputFilter, InputFilterContext, MultiText, Text,
        Select, TextInput, WhenCondition,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "widget_composition";

struct Nickname(String);

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(
        STATE,
        [
            text("Conditional text, fallback text, input groups, and dynamic button styles"),
            text(
                MultiText::builder()
                    .text("Send a whole number for your age, or a word for your nickname.")
                    .text(
                        FnText::new(|data: &DataMap| {
                            format!("Age: {}", data.get("age").and_then(|value| value.as_u64()).unwrap_or(0))
                        })
                        .when(WhenCondition::data_field("age")),
                    )
                    .text(
                        FirstText::builder()
                            .text(
                                FnText::new(|data: &DataMap| {
                                    format!("Nickname: {}", data.get("nickname").and_then(|value| value.as_str()).unwrap_or(""))
                                })
                                .when(WhenCondition::data_field("nickname")),
                            )
                            .text("No nickname entered yet.")
                            .build(),
                    )
                    .build(),
            ),
            keyboard(
                InlineKeyboard::builder()
                    .row([
                        Button::on_click("highlight", "Toggle highlight", |click| async move {
                            ButtonAction::set_dialog_value(
                                "highlight",
                                !click.context.dialog_value_as::<bool>("highlight").unwrap_or(false),
                            )
                        })
                        .style_dynamic(|ctx: RenderContext| async move {
                            ctx.data.get("highlight").and_then(|value| value.as_bool())
                                .unwrap_or(false).then_some(ButtonStyle::Success)
                        })
                        .icon_custom_emoji_id_dynamic(FnText::new(|_: &DataMap| {
                            std::env::var("DIALOG_EMOJI_ID").unwrap_or_default()
                        })),
                    ])
                    .row([
                        Button::set_dialog_value("clear", "Clear nickname", "nickname", "")
                            .when(WhenCondition::data_field("nickname")),
                    ])
                    .row([main_menu_button()])
                    .build(),
            ),
            keyboard(
                Select::builder("color")
                    .items_getter(|_: &DataMap| ["Blue", "Green", "Red"])
                    .item_renderer(|item: &&str, _: &DataMap| (*item).to_owned())
                    .id_getter(|item| item)
                    .action(|color: String| async move { ButtonAction::set_dialog_value("color", color) })
                    .appearance(
                        ButtonAppearance::builder().style_dynamic(|ctx: RenderContext| async move {
                            match ctx.data.get("item_id").and_then(|value| value.as_str()) {
                                Some("Green") => Some(ButtonStyle::Success),
                                Some("Red") => Some(ButtonStyle::Danger),
                                _ => Some(ButtonStyle::Primary),
                            }
                        })
                            .when(WhenCondition::data_field("highlight")).build()
                            .or(ButtonAppearance::builder().style(ButtonStyle::Primary).build()),
                    )
                    .build(),
            ),
            input(
                CombinedInput::builder()
                    .input(
                        TextInput::builder("age")
                            .on_success(|_ctx, age: u32| async move {
                                ButtonAction::set_dialog_value("age", age)
                            })
                            .build()
                            .filter(InputFilter::new(|ctx: InputFilterContext| async move {
                                ctx.message.text().is_some_and(|text| !text.is_empty() && text.chars().all(|ch| ch.is_ascii_digit()))
                            })),
                    )
                    .input(
                        TextInput::builder_with_parser("nickname", |text: &str| {
                            if text.trim().is_empty() {
                                Err("Send a nonempty nickname.")
                            } else {
                                Ok(Nickname(text.trim().to_owned()))
                            }
                        })
                            .on_success(|_ctx, name: Nickname| async move {
                                ButtonAction::set_dialog_value("nickname", name.0)
                            })
                            .build(),
                    )
                    .filter(InputFilter::new(|ctx: InputFilterContext| async move {
                        ctx.message.text().is_some_and(|text| !text.starts_with('/'))
                    }))
                    .build(),
            ),
        ],
    )])
}
