use telers::serialization::{json, Value};
use telers_dialog::{
    entities::Context,
    widgets::{fn_text, keyboard, Button, ButtonAction, InlineKeyboard},
    window, Dialog, DialogControl, DialogError, ResultContext, StartMode,
};

use crate::common::main_menu_button;

pub const STATE: &str = "lifecycle_parent";
const SUMMARY: &str = "lifecycle_summary";
const CHILD: &str = "lifecycle_color";

async fn initialize(_ctx: Context, manager: &dyn DialogControl) -> Result<Option<ButtonAction>, DialogError> {
    manager.set_dialog_value("selection", json!("No color selected")).await?;
    Ok(None)
}

async fn receive_color(ctx: ResultContext, manager: &dyn DialogControl) -> Result<Option<ButtonAction>, DialogError> {
    let color = ctx.result().as_str().unwrap_or("Selection cancelled");
    manager.set_dialog_value("selection", json!(color)).await?;
    manager.switch_to(SUMMARY).await?;
    Ok(None)
}

async fn close_color(ctx: Context, result: Option<Value>, _manager: &dyn DialogControl) -> Result<(), DialogError> {
    tracing::info!(state = %ctx.state, result = ?result, "Color picker closed");
    Ok(())
}

fn choose_color_button() -> Button {
    Button::start("choose", "Choose color", CHILD, json!({"source": "lifecycle"}), StartMode::Normal)
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(STATE, [
            fn_text(|data| format!("Color preference: {}", data.get("selection").and_then(|value| value.as_str()).unwrap_or_default())),
            keyboard(InlineKeyboard::builder().row([choose_color_button()]).row([main_menu_button()]).build()),
        ]),
        window(SUMMARY, [
            fn_text(|data| format!("Your selection: {}", data.get("selection").and_then(|value| value.as_str()).unwrap_or_default())),
            keyboard(InlineKeyboard::builder().row([choose_color_button()]).row([main_menu_button()]).build()),
        ]),
    ])
    .on_start(initialize)
    .on_process_result_with_manager(receive_color)
}

pub fn child_dialog() -> impl Dialog {
    telers_dialog::dialog([window(CHILD, [
        telers_dialog::widgets::text("Pick a color."),
        keyboard(InlineKeyboard::builder()
            .row([
                Button::action("blue", "Blue", ButtonAction::done_with_result(json!("Blue"))),
                Button::action("green", "Green", ButtonAction::done_with_result(json!("Green"))),
            ])
            .row([Button::done("cancel", "Cancel")])
            .build()),
    ])])
    .on_close(close_color)
}
