use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputRichBlock {
    Paragraph(crate::types::InputRichBlockParagraph),
    Heading(crate::types::InputRichBlockSectionHeading),
    Pre(crate::types::InputRichBlockPreformatted),
    Footer(crate::types::InputRichBlockFooter),
    Divider(crate::types::InputRichBlockDivider),
    MathematicalExpression(crate::types::InputRichBlockMathematicalExpression),
    Anchor(crate::types::InputRichBlockAnchor),
    List(crate::types::InputRichBlockList),
    Blockquote(crate::types::InputRichBlockBlockQuotation),
    ExpandableBlockquote(crate::types::InputRichBlockExpandableBlockQuotation),
    Pullquote(crate::types::InputRichBlockPullQuotation),
    Collage(crate::types::InputRichBlockCollage),
    Slideshow(crate::types::InputRichBlockSlideshow),
    Table(crate::types::InputRichBlockTable),
    Details(crate::types::InputRichBlockDetails),
    Map(crate::types::InputRichBlockMap),
    Buttons(crate::types::InputRichBlockButtons),
    Animation(crate::types::InputRichBlockAnimation),
    Audio(crate::types::InputRichBlockAudio),
    Document(crate::types::InputRichBlockDocument),
    Photo(crate::types::InputRichBlockPhoto),
    Video(crate::types::InputRichBlockVideo),
    VoiceNote(crate::types::InputRichBlockVoiceNote),
    Thinking(crate::types::InputRichBlockThinking),
}
impl InputRichBlock {
    /// Helper method for field `align`.
    ///
    /// Horizontal alignment of the buttons. Currently, must be one of `left`, `center`, or `right`.
    #[must_use]
    pub fn align(&self) -> Option<&str> {
        match self {
            Self::Buttons(val) => val.align.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `animation`.
    ///
    /// The animation. Caption is ignored.
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::InputMediaAnimation> {
        match self {
            Self::Animation(val) => Some(&val.animation),
            _ => None,
        }
    }

    /// Helper method for field `audio`.
    ///
    /// The audio. Caption is ignored.
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::InputMediaAudio> {
        match self {
            Self::Audio(val) => Some(&val.audio),
            _ => None,
        }
    }

    /// Helper method for field `blocks`.
    ///
    /// # Variants
    /// - `InputRichBlockBlockQuotation`, `InputRichBlockDetails`. Content of the block
    /// - `InputRichBlockCollage`. Elements of the collage
    /// - `InputRichBlockSlideshow`. Elements of the slideshow
    #[must_use]
    pub fn blocks(&self) -> Option<&[crate::types::InputRichBlock]> {
        match self {
            Self::Blockquote(val) => Some(val.blocks.as_ref()),
            Self::Collage(val) => Some(val.blocks.as_ref()),
            Self::Slideshow(val) => Some(val.blocks.as_ref()),
            Self::Details(val) => Some(val.blocks.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `buttons`.
    ///
    /// List of 1-8 buttons to send
    #[must_use]
    pub fn buttons(&self) -> Option<&[crate::types::RichMessageButton]> {
        match self {
            Self::Buttons(val) => Some(val.buttons.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `cells`.
    ///
    /// Cells of the table
    #[must_use]
    pub fn cells(&self) -> Option<&[Box<[crate::types::RichBlockTableCell]>]> {
        match self {
            Self::Table(val) => Some(val.cells.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `credit`.
    ///
    /// Credit of the block
    #[must_use]
    pub fn credit(&self) -> Option<&crate::types::RichText> {
        match self {
            Self::Blockquote(val) => val.credit.as_deref(),
            Self::ExpandableBlockquote(val) => val.credit.as_deref(),
            Self::Pullquote(val) => val.credit.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `document`.
    ///
    /// The document. Caption is ignored.
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::InputMediaDocument> {
        match self {
            Self::Document(val) => Some(&val.document),
            _ => None,
        }
    }

    /// Helper method for field `expression`.
    ///
    /// The mathematical expression in `LaTeX` format
    #[must_use]
    pub fn expression(&self) -> Option<&str> {
        match self {
            Self::MathematicalExpression(val) => Some(val.expression.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `height`.
    ///
    /// Map height; 0-10000
    #[must_use]
    pub fn height(&self) -> Option<u16> {
        match self {
            Self::Map(val) => val.height,
            _ => None,
        }
    }

    /// Helper method for field `is_bordered`.
    ///
    /// Pass `true` if the table has borders
    #[must_use]
    pub fn is_bordered(&self) -> Option<bool> {
        match self {
            Self::Table(val) => val.is_bordered,
            _ => None,
        }
    }

    /// Helper method for field `is_compact`.
    ///
    /// Pass `true` if table cells must have smaller indents
    #[must_use]
    pub fn is_compact(&self) -> Option<bool> {
        match self {
            Self::Table(val) => val.is_compact,
            _ => None,
        }
    }

    /// Helper method for field `is_open`.
    ///
    /// Pass `true` if the content of the block is visible by default
    #[must_use]
    pub fn is_open(&self) -> Option<bool> {
        match self {
            Self::Details(val) => val.is_open,
            _ => None,
        }
    }

    /// Helper method for field `is_striped`.
    ///
    /// Pass `true` if the table is striped
    #[must_use]
    pub fn is_striped(&self) -> Option<bool> {
        match self {
            Self::Table(val) => val.is_striped,
            _ => None,
        }
    }

    /// Helper method for field `items`.
    ///
    /// Items of the list
    #[must_use]
    pub fn items(&self) -> Option<&[crate::types::InputRichBlockListItem]> {
        match self {
            Self::List(val) => Some(val.items.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `language`.
    ///
    /// The programming language of the text
    #[must_use]
    pub fn language(&self) -> Option<&str> {
        match self {
            Self::Pre(val) => val.language.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `location`.
    ///
    /// Location of the center of the map
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::Location> {
        match self {
            Self::Map(val) => Some(&val.location),
            _ => None,
        }
    }

    /// Helper method for field `name`.
    ///
    /// The name of the anchor
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Anchor(val) => Some(val.name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// The photo. Caption is ignored.
    #[must_use]
    pub fn photo(&self) -> Option<&crate::types::InputMediaPhoto> {
        match self {
            Self::Photo(val) => Some(&val.photo),
            _ => None,
        }
    }

    /// Helper method for field `size`.
    ///
    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    #[must_use]
    pub fn size(&self) -> Option<u8> {
        match self {
            Self::Heading(val) => Some(val.size),
            _ => None,
        }
    }

    /// Helper method for field `summary`.
    ///
    /// Always shown summary of the block
    #[must_use]
    pub fn summary(&self) -> Option<&crate::types::RichText> {
        match self {
            Self::Details(val) => Some(val.summary.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `text`.
    ///
    /// # Variants
    /// - `InputRichBlockParagraph`, `InputRichBlockSectionHeading`, `InputRichBlockPreformatted`, `InputRichBlockFooter`, `InputRichBlockPullQuotation`. Text of the block
    /// - `InputRichBlockExpandableBlockQuotation`. Content of the block
    /// - `InputRichBlockThinking`. Text of the block. See <https://t.me/addemoji/AIActions> for examples of custom emoji that are recommended for usage in the block.
    #[must_use]
    pub fn text(&self) -> Option<&crate::types::RichText> {
        match self {
            Self::Paragraph(val) => Some(val.text.as_ref()),
            Self::Heading(val) => Some(val.text.as_ref()),
            Self::Pre(val) => Some(val.text.as_ref()),
            Self::Footer(val) => Some(val.text.as_ref()),
            Self::ExpandableBlockquote(val) => Some(val.text.as_ref()),
            Self::Pullquote(val) => Some(val.text.as_ref()),
            Self::Thinking(val) => Some(val.text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `video`.
    ///
    /// The video. Caption is ignored.
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::InputMediaVideo> {
        match self {
            Self::Video(val) => Some(&val.video),
            _ => None,
        }
    }

    /// Helper method for field `voice_note`.
    ///
    /// The voice note. Caption is ignored.
    #[must_use]
    pub fn voice_note(&self) -> Option<&crate::types::InputMediaVoiceNote> {
        match self {
            Self::VoiceNote(val) => Some(&val.voice_note),
            _ => None,
        }
    }

    /// Helper method for field `width`.
    ///
    /// Map width; 0-10000
    #[must_use]
    pub fn width(&self) -> Option<u16> {
        match self {
            Self::Map(val) => val.width,
            _ => None,
        }
    }

    /// Helper method for field `zoom`.
    ///
    /// Map zoom level; 0-24
    #[must_use]
    pub fn zoom(&self) -> Option<u8> {
        match self {
            Self::Map(val) => val.zoom,
            _ => None,
        }
    }

    /// Helper method for nested field `caption_entities`.
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Animation(val) => {
                let inner = &val.animation;
                inner.caption_entities.as_deref()
            }
            Self::Audio(val) => {
                let inner = &val.audio;
                inner.caption_entities.as_deref()
            }
            Self::Document(val) => {
                let inner = &val.document;
                inner.caption_entities.as_deref()
            }
            Self::Photo(val) => {
                let inner = &val.photo;
                inner.caption_entities.as_deref()
            }
            Self::Video(val) => {
                let inner = &val.video;
                inner.caption_entities.as_deref()
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                inner.caption_entities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `cover`.
    #[must_use]
    pub fn cover(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Video(val) => {
                let inner = &val.video;
                inner.cover.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `disable_content_type_detection`.
    #[must_use]
    pub fn disable_content_type_detection(&self) -> Option<bool> {
        match self {
            Self::Document(val) => {
                let inner = &val.document;
                inner.disable_content_type_detection
            }
            _ => None,
        }
    }

    /// Helper method for nested field `duration`.
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = &val.animation;
                inner.duration
            }
            Self::Audio(val) => {
                let inner = &val.audio;
                inner.duration
            }
            Self::Video(val) => {
                let inner = &val.video;
                inner.duration
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                inner.duration
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_spoiler`.
    #[must_use]
    pub fn has_spoiler(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => {
                let inner = &val.animation;
                inner.has_spoiler
            }
            Self::Photo(val) => {
                let inner = &val.photo;
                inner.has_spoiler
            }
            Self::Video(val) => {
                let inner = &val.video;
                inner.has_spoiler
            }
            _ => None,
        }
    }

    /// Helper method for nested field `heading`.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::Map(val) => {
                let inner = &val.location;
                inner.heading
            }
            _ => None,
        }
    }

    /// Helper method for nested field `horizontal_accuracy`.
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        match self {
            Self::Map(val) => {
                let inner = &val.location;
                inner.horizontal_accuracy
            }
            _ => None,
        }
    }

    /// Helper method for nested field `latitude`.
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Map(val) => {
                let inner = &val.location;
                Some(inner.latitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `live_period`.
    #[must_use]
    pub fn live_period(&self) -> Option<i64> {
        match self {
            Self::Map(val) => {
                let inner = &val.location;
                inner.live_period
            }
            _ => None,
        }
    }

    /// Helper method for nested field `longitude`.
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Map(val) => {
                let inner = &val.location;
                Some(inner.longitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `media`.
    #[must_use]
    pub fn media(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Animation(val) => {
                let inner = &val.animation;
                Some(&inner.media)
            }
            Self::Audio(val) => {
                let inner = &val.audio;
                Some(&inner.media)
            }
            Self::Document(val) => {
                let inner = &val.document;
                Some(&inner.media)
            }
            Self::Photo(val) => {
                let inner = &val.photo;
                Some(&inner.media)
            }
            Self::Video(val) => {
                let inner = &val.video;
                Some(&inner.media)
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                Some(&inner.media)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `parse_mode`.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = &val.animation;
                inner.parse_mode.as_deref()
            }
            Self::Audio(val) => {
                let inner = &val.audio;
                inner.parse_mode.as_deref()
            }
            Self::Document(val) => {
                let inner = &val.document;
                inner.parse_mode.as_deref()
            }
            Self::Photo(val) => {
                let inner = &val.photo;
                inner.parse_mode.as_deref()
            }
            Self::Video(val) => {
                let inner = &val.video;
                inner.parse_mode.as_deref()
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                inner.parse_mode.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `performer`.
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                let inner = &val.audio;
                inner.performer.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `proximity_alert_radius`.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<i64> {
        match self {
            Self::Map(val) => {
                let inner = &val.location;
                inner.proximity_alert_radius
            }
            _ => None,
        }
    }

    /// Helper method for nested field `show_caption_above_media`.
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => {
                let inner = &val.animation;
                inner.show_caption_above_media
            }
            Self::Photo(val) => {
                let inner = &val.photo;
                inner.show_caption_above_media
            }
            Self::Video(val) => {
                let inner = &val.video;
                inner.show_caption_above_media
            }
            _ => None,
        }
    }

    /// Helper method for nested field `start_timestamp`.
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => {
                let inner = &val.video;
                inner.start_timestamp
            }
            _ => None,
        }
    }

    /// Helper method for nested field `supports_streaming`.
    #[must_use]
    pub fn supports_streaming(&self) -> Option<bool> {
        match self {
            Self::Video(val) => {
                let inner = &val.video;
                inner.supports_streaming
            }
            _ => None,
        }
    }

    /// Helper method for nested field `thumbnail`.
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Animation(val) => {
                let inner = &val.animation;
                inner.thumbnail.as_ref()
            }
            Self::Audio(val) => {
                let inner = &val.audio;
                inner.thumbnail.as_ref()
            }
            Self::Document(val) => {
                let inner = &val.document;
                inner.thumbnail.as_ref()
            }
            Self::Video(val) => {
                let inner = &val.video;
                inner.thumbnail.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `title`.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                let inner = &val.audio;
                inner.title.as_deref()
            }
            _ => None,
        }
    }
}
impl From<crate::types::InputRichBlockParagraph> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockParagraph) -> Self {
        Self::Paragraph(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockParagraph {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Paragraph(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockParagraph),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockSectionHeading> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockSectionHeading) -> Self {
        Self::Heading(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockSectionHeading {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Heading(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockSectionHeading),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockPreformatted> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockPreformatted) -> Self {
        Self::Pre(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockPreformatted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Pre(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockPreformatted),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockFooter> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockFooter) -> Self {
        Self::Footer(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockFooter {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Footer(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockFooter),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockDivider> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockDivider) -> Self {
        Self::Divider(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockDivider {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Divider(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockDivider),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockMathematicalExpression> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockMathematicalExpression {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::MathematicalExpression(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockMathematicalExpression),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockAnchor> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockAnchor) -> Self {
        Self::Anchor(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockAnchor {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Anchor(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockAnchor),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockList> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockList) -> Self {
        Self::List(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockList {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::List(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockList),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockBlockQuotation> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockBlockQuotation) -> Self {
        Self::Blockquote(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockBlockQuotation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Blockquote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockBlockQuotation),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockExpandableBlockQuotation> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockExpandableBlockQuotation) -> Self {
        Self::ExpandableBlockquote(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockExpandableBlockQuotation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::ExpandableBlockquote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockExpandableBlockQuotation),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockPullQuotation> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockPullQuotation) -> Self {
        Self::Pullquote(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockPullQuotation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Pullquote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockPullQuotation),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockCollage> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockCollage) -> Self {
        Self::Collage(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockCollage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Collage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockCollage),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockSlideshow> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockSlideshow) -> Self {
        Self::Slideshow(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockSlideshow {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Slideshow(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockSlideshow),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockTable> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockTable) -> Self {
        Self::Table(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockTable {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Table(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockTable),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockDetails> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockDetails) -> Self {
        Self::Details(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockDetails {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Details(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockDetails),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockMap> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockMap) -> Self {
        Self::Map(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockMap {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Map(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockMap),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockButtons> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockButtons) -> Self {
        Self::Buttons(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockButtons {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Buttons(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockButtons),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockAnimation> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockAnimation) -> Self {
        Self::Animation(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Animation(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockAnimation),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockAudio> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockAudio) -> Self {
        Self::Audio(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Audio(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockAudio),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockDocument> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockDocument) -> Self {
        Self::Document(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Document(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockDocument),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockPhoto> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockPhoto),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockVideo> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockVideo),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockVoiceNote> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockVoiceNote) -> Self {
        Self::VoiceNote(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockVoiceNote {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::VoiceNote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockVoiceNote),
            ))
        }
    }
}
impl From<crate::types::InputRichBlockThinking> for InputRichBlock {
    fn from(val: crate::types::InputRichBlockThinking) -> Self {
        Self::Thinking(val)
    }
}
impl TryFrom<InputRichBlock> for crate::types::InputRichBlockThinking {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputRichBlock) -> Result<Self, Self::Error> {
        if let InputRichBlock::Thinking(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputRichBlock),
                stringify!(InputRichBlockThinking),
            ))
        }
    }
}
