use telers::types::{Chat, User};

use super::{AccessSettings, ShowMode, StartMode};

pub const DIALOG_EVENT_NAME: &str = "td_update";

#[derive(Clone, Debug)]
pub enum DialogAction {
    Done,
    Start,
    Update,
    Switch,
}

#[derive(Clone, Debug)]
pub struct DialogUpdateEvent {
    pub user: User,
    pub chat: Chat,
    pub action: DialogAction,
    pub data: serde_json::Value,
    pub intent_id: Option<String>,
    pub stack_id: Option<String>,
    pub thread_id: Option<i64>,
    pub business_connection_id: Option<String>,
    pub show_mode: Option<ShowMode>,
}

#[derive(Clone, Debug)]
pub struct DialogStartEvent {
    pub update_event: DialogUpdateEvent,
    pub state: String,
    pub mode: StartMode,
    pub access_settings: Option<AccessSettings>,
}

#[derive(Clone, Debug)]
pub struct DialogSwitchEvent {
    pub update_event: DialogUpdateEvent,
    pub state: String,
}
