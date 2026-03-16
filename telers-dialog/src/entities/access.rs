use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AccessSettings {
    pub user_ids: Vec<i64>,
    pub custom: Option<serde_json::Value>,
}
