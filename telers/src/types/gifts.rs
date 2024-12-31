use super::Gift;

use serde::{Deserialize, Serialize};

/// This object represent a list of gifts.
/// # Documentation
/// <https://core.telegram.org/bots/api#gifts>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Gifts {
    /// The list of gifts
    pub gifts: Box<[Gift]>,
}
