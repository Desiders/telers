use telers::{
    errors::HandlerError,
    event::telegram::HandlerResult,
    fsm::MemoryStorage,
    serialization::json,
    Bot,
};
use telers_dialog::{
    entities::{Context, DataMap},
    widgets::{format_text, keyboard, InlineKeyboard},
    window, AccessSettings, Dialog, DialogControl, DialogError, DialogManager, StartMode,
};

use crate::common::main_menu_button;

pub const STATE: &str = "background_progress";

async fn initialize(_context: Context, manager: &dyn DialogControl) -> Result<Option<telers_dialog::widgets::ButtonAction>, DialogError> {
    manager.set_dialog_value("progress", json!(0)).await?;
    Ok(None)
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(STATE, [
        format_text("Background work: {progress}%\nSend /background to run it. Closing this screen cancels further updates."),
        keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
    ])]).on_start(initialize)
}

pub async fn run(bot: Bot, manager: DialogManager<MemoryStorage>) -> HandlerResult<()> {
    let user_id = match manager.event() {
        telers_dialog::entities::ChatEvent::Message(message) => message.from().map(|user| user.id),
        _ => None,
    };
    manager.start_with_access_settings(&bot, STATE, json!({}), StartMode::Normal,
        Some(AccessSettings { user_ids: user_id.into_iter().collect(), custom: None }))
        .await.map_err(HandlerError::new)?;
    let background = manager.background(&bot).await.map_err(HandlerError::new)?;
    tokio::spawn(async move {
        for progress in [0, 25, 50, 75, 100] {
            if progress != 0 {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            match background.update(DataMap::from([("progress".into(), json!(progress))]), None).await {
                Ok(_) => {}
                Err(telers_dialog::DialogError::NoContext) => break,
                Err(error) => {
                    tracing::error!(%error, "Background dialog update failed");
                    break;
                }
            }
        }
    });
    Ok(())
}
