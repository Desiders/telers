use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub is_untagged: bool,
    pub tag_field: Option<String>,
    pub exclude_tag_field: bool,
    pub variants: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaggedInfo {
    pub version: String,
    pub docs_tagged_types: HashMap<String, Type>,
}

pub fn parse_json_to_tagged_info(content: &str) -> Result<TaggedInfo, serde_json::Error> {
    serde_json::from_str(content)
}
