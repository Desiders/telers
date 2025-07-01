use serde::{Deserialize, Serialize};

/// This object describes the colors of the backdrop of a unique gift
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftbackdropcolors>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: u32,
    /// The color on the edges of the backdrop in RGB format
    pub edge_color: u32,
    /// The color to be applied to the symbol in RGB format
    pub symbol_color: u32,
    /// The color for the text on the backdrop in RGB format
    pub text_color: u32,
}
