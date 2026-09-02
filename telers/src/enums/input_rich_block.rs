use crate::types::InputRichBlock;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a block in a rich formatted message to be sent. Currently, it can be any of the following types:
/// - [`crate::types::InputRichBlockParagraph`]
/// - [`crate::types::InputRichBlockSectionHeading`]
/// - [`crate::types::InputRichBlockPreformatted`]
/// - [`crate::types::InputRichBlockFooter`]
/// - [`crate::types::InputRichBlockDivider`]
/// - [`crate::types::InputRichBlockMathematicalExpression`]
/// - [`crate::types::InputRichBlockAnchor`]
/// - [`crate::types::InputRichBlockList`]
/// - [`crate::types::InputRichBlockBlockQuotation`]
/// - [`crate::types::InputRichBlockExpandableBlockQuotation`]
/// - [`crate::types::InputRichBlockPullQuotation`]
/// - [`crate::types::InputRichBlockCollage`]
/// - [`crate::types::InputRichBlockSlideshow`]
/// - [`crate::types::InputRichBlockTable`]
/// - [`crate::types::InputRichBlockDetails`]
/// - [`crate::types::InputRichBlockMap`]
/// - [`crate::types::InputRichBlockButtons`]
/// - [`crate::types::InputRichBlockAnimation`]
/// - [`crate::types::InputRichBlockAudio`]
/// - [`crate::types::InputRichBlockDocument`]
/// - [`crate::types::InputRichBlockPhoto`]
/// - [`crate::types::InputRichBlockVideo`]
/// - [`crate::types::InputRichBlockVoiceNote`]
/// - [`crate::types::InputRichBlockThinking`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblock>
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
pub enum InputRichBlockType {
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
    #[strum(serialize = "expandable_blockquote")]
    ExpandableBlockquote,
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
    #[strum(serialize = "buttons")]
    Buttons,
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice_note")]
    VoiceNote,
    #[strum(serialize = "thinking")]
    Thinking,
}
impl InputRichBlockType {
    #[must_use]
    pub const fn all() -> [InputRichBlockType; 24usize] {
        [
            InputRichBlockType::Paragraph,
            InputRichBlockType::Heading,
            InputRichBlockType::Pre,
            InputRichBlockType::Footer,
            InputRichBlockType::Divider,
            InputRichBlockType::MathematicalExpression,
            InputRichBlockType::Anchor,
            InputRichBlockType::List,
            InputRichBlockType::Blockquote,
            InputRichBlockType::ExpandableBlockquote,
            InputRichBlockType::Pullquote,
            InputRichBlockType::Collage,
            InputRichBlockType::Slideshow,
            InputRichBlockType::Table,
            InputRichBlockType::Details,
            InputRichBlockType::Map,
            InputRichBlockType::Buttons,
            InputRichBlockType::Animation,
            InputRichBlockType::Audio,
            InputRichBlockType::Document,
            InputRichBlockType::Photo,
            InputRichBlockType::Video,
            InputRichBlockType::VoiceNote,
            InputRichBlockType::Thinking,
        ]
    }
}
impl From<InputRichBlockType> for Box<str> {
    fn from(val: InputRichBlockType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputRichBlockType> for String {
    fn from(val: InputRichBlockType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputRichBlockType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputRichBlock> for InputRichBlockType {
    fn from(val: &'a InputRichBlock) -> Self {
        match val {
            InputRichBlock::Paragraph(_) => InputRichBlockType::Paragraph,
            InputRichBlock::Heading(_) => InputRichBlockType::Heading,
            InputRichBlock::Pre(_) => InputRichBlockType::Pre,
            InputRichBlock::Footer(_) => InputRichBlockType::Footer,
            InputRichBlock::Divider(_) => InputRichBlockType::Divider,
            InputRichBlock::MathematicalExpression(_) => InputRichBlockType::MathematicalExpression,
            InputRichBlock::Anchor(_) => InputRichBlockType::Anchor,
            InputRichBlock::List(_) => InputRichBlockType::List,
            InputRichBlock::Blockquote(_) => InputRichBlockType::Blockquote,
            InputRichBlock::ExpandableBlockquote(_) => InputRichBlockType::ExpandableBlockquote,
            InputRichBlock::Pullquote(_) => InputRichBlockType::Pullquote,
            InputRichBlock::Collage(_) => InputRichBlockType::Collage,
            InputRichBlock::Slideshow(_) => InputRichBlockType::Slideshow,
            InputRichBlock::Table(_) => InputRichBlockType::Table,
            InputRichBlock::Details(_) => InputRichBlockType::Details,
            InputRichBlock::Map(_) => InputRichBlockType::Map,
            InputRichBlock::Buttons(_) => InputRichBlockType::Buttons,
            InputRichBlock::Animation(_) => InputRichBlockType::Animation,
            InputRichBlock::Audio(_) => InputRichBlockType::Audio,
            InputRichBlock::Document(_) => InputRichBlockType::Document,
            InputRichBlock::Photo(_) => InputRichBlockType::Photo,
            InputRichBlock::Video(_) => InputRichBlockType::Video,
            InputRichBlock::VoiceNote(_) => InputRichBlockType::VoiceNote,
            InputRichBlock::Thinking(_) => InputRichBlockType::Thinking,
        }
    }
}
impl From<InputRichBlock> for InputRichBlockType {
    fn from(val: InputRichBlock) -> Self {
        InputRichBlockType::from(&val)
    }
}
