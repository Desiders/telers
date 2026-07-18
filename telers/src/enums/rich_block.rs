use crate::types::RichBlock;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a block in a rich formatted message. Currently, it can be any of the following types:
/// - [`crate::types::RichBlockParagraph`]
/// - [`crate::types::RichBlockSectionHeading`]
/// - [`crate::types::RichBlockPreformatted`]
/// - [`crate::types::RichBlockFooter`]
/// - [`crate::types::RichBlockDivider`]
/// - [`crate::types::RichBlockMathematicalExpression`]
/// - [`crate::types::RichBlockAnchor`]
/// - [`crate::types::RichBlockList`]
/// - [`crate::types::RichBlockBlockQuotation`]
/// - [`crate::types::RichBlockPullQuotation`]
/// - [`crate::types::RichBlockCollage`]
/// - [`crate::types::RichBlockSlideshow`]
/// - [`crate::types::RichBlockTable`]
/// - [`crate::types::RichBlockDetails`]
/// - [`crate::types::RichBlockMap`]
/// - [`crate::types::RichBlockAnimation`]
/// - [`crate::types::RichBlockAudio`]
/// - [`crate::types::RichBlockPhoto`]
/// - [`crate::types::RichBlockVideo`]
/// - [`crate::types::RichBlockVoiceNote`]
/// - [`crate::types::RichBlockThinking`]
/// # Documentation
/// <https://core.telegram.org/bots/api#richblock>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum RichBlockType {
    #[strum(serialize = "paragraph")]
    Paragraph,
    #[strum(serialize = "heading")]
    Heading,
    #[strum(serialize = "pre")]
    Pre,
    #[strum(serialize = "footer")]
    Footer,
    #[strum(serialize = "divider")]
    Divider,
    #[strum(serialize = "mathematical_expression")]
    MathematicalExpression,
    #[strum(serialize = "anchor")]
    Anchor,
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "blockquote")]
    Blockquote,
    #[strum(serialize = "pullquote")]
    Pullquote,
    #[strum(serialize = "collage")]
    Collage,
    #[strum(serialize = "slideshow")]
    Slideshow,
    #[strum(serialize = "table")]
    Table,
    #[strum(serialize = "details")]
    Details,
    #[strum(serialize = "map")]
    Map,
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice_note")]
    VoiceNote,
    #[strum(serialize = "thinking")]
    Thinking,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl RichBlockType {
    #[must_use]
    pub const fn all() -> [RichBlockType; 22usize] {
        [
            RichBlockType::Paragraph,
            RichBlockType::Heading,
            RichBlockType::Pre,
            RichBlockType::Footer,
            RichBlockType::Divider,
            RichBlockType::MathematicalExpression,
            RichBlockType::Anchor,
            RichBlockType::List,
            RichBlockType::Blockquote,
            RichBlockType::Pullquote,
            RichBlockType::Collage,
            RichBlockType::Slideshow,
            RichBlockType::Table,
            RichBlockType::Details,
            RichBlockType::Map,
            RichBlockType::Animation,
            RichBlockType::Audio,
            RichBlockType::Photo,
            RichBlockType::Video,
            RichBlockType::VoiceNote,
            RichBlockType::Thinking,
            RichBlockType::Unknown,
        ]
    }
}
impl From<RichBlockType> for Box<str> {
    fn from(val: RichBlockType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<RichBlockType> for String {
    fn from(val: RichBlockType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for RichBlockType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a RichBlock> for RichBlockType {
    fn from(val: &'a RichBlock) -> Self {
        match val {
            RichBlock::Paragraph(_) => RichBlockType::Paragraph,
            RichBlock::Heading(_) => RichBlockType::Heading,
            RichBlock::Pre(_) => RichBlockType::Pre,
            RichBlock::Footer(_) => RichBlockType::Footer,
            RichBlock::Divider(_) => RichBlockType::Divider,
            RichBlock::MathematicalExpression(_) => RichBlockType::MathematicalExpression,
            RichBlock::Anchor(_) => RichBlockType::Anchor,
            RichBlock::List(_) => RichBlockType::List,
            RichBlock::Blockquote(_) => RichBlockType::Blockquote,
            RichBlock::Pullquote(_) => RichBlockType::Pullquote,
            RichBlock::Collage(_) => RichBlockType::Collage,
            RichBlock::Slideshow(_) => RichBlockType::Slideshow,
            RichBlock::Table(_) => RichBlockType::Table,
            RichBlock::Details(_) => RichBlockType::Details,
            RichBlock::Map(_) => RichBlockType::Map,
            RichBlock::Animation(_) => RichBlockType::Animation,
            RichBlock::Audio(_) => RichBlockType::Audio,
            RichBlock::Photo(_) => RichBlockType::Photo,
            RichBlock::Video(_) => RichBlockType::Video,
            RichBlock::VoiceNote(_) => RichBlockType::VoiceNote,
            RichBlock::Thinking(_) => RichBlockType::Thinking,
            RichBlock::Unknown(_) => RichBlockType::Unknown,
        }
    }
}
impl From<RichBlock> for RichBlockType {
    fn from(val: RichBlock) -> Self {
        RichBlockType::from(&val)
    }
}
