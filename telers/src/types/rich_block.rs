use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichBlock {
    Paragraph(crate::types::RichBlockParagraph),
    Heading(crate::types::RichBlockSectionHeading),
    Pre(crate::types::RichBlockPreformatted),
    Footer(crate::types::RichBlockFooter),
    Divider(crate::types::RichBlockDivider),
    MathematicalExpression(crate::types::RichBlockMathematicalExpression),
    Anchor(crate::types::RichBlockAnchor),
    List(crate::types::RichBlockList),
    Blockquote(crate::types::RichBlockBlockQuotation),
    Pullquote(crate::types::RichBlockPullQuotation),
    Collage(crate::types::RichBlockCollage),
    Slideshow(crate::types::RichBlockSlideshow),
    Table(crate::types::RichBlockTable),
    Details(crate::types::RichBlockDetails),
    Map(crate::types::RichBlockMap),
    Animation(crate::types::RichBlockAnimation),
    Audio(crate::types::RichBlockAudio),
    Photo(crate::types::RichBlockPhoto),
    Video(crate::types::RichBlockVideo),
    VoiceNote(crate::types::RichBlockVoiceNote),
    Thinking(crate::types::RichBlockThinking),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::RichBlockUnknown),
}
impl RichBlock {
    /// Helper method for field `animation`.
    ///
    /// The animation
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::Animation(val) => Some(val.animation.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `audio`.
    ///
    /// The audio
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Audio(val) => Some(val.audio.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `blocks`.
    ///
    /// # Variants
    /// - `RichBlockBlockQuotation`, `RichBlockDetails`. Content of the block
    /// - `RichBlockCollage`. Elements of the collage
    /// - `RichBlockSlideshow`. Elements of the slideshow
    #[must_use]
    pub fn blocks(&self) -> Option<&[crate::types::RichBlock]> {
        match self {
            Self::Blockquote(val) => Some(val.blocks.as_ref()),
            Self::Collage(val) => Some(val.blocks.as_ref()),
            Self::Slideshow(val) => Some(val.blocks.as_ref()),
            Self::Details(val) => Some(val.blocks.as_ref()),
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
            Self::Pullquote(val) => val.credit.as_deref(),
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

    /// Helper method for field `has_spoiler`.
    ///
    /// `true`, if the media preview is covered by a spoiler animation
    #[must_use]
    pub fn has_spoiler(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_spoiler,
            Self::Photo(val) => val.has_spoiler,
            Self::Video(val) => val.has_spoiler,
            _ => None,
        }
    }

    /// Helper method for field `height`.
    ///
    /// Expected height of the map
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Map(val) => Some(val.height),
            _ => None,
        }
    }

    /// Helper method for field `is_bordered`.
    ///
    /// `true`, if the table has borders
    #[must_use]
    pub fn is_bordered(&self) -> Option<bool> {
        match self {
            Self::Table(val) => val.is_bordered,
            _ => None,
        }
    }

    /// Helper method for field `is_open`.
    ///
    /// `true`, if the content of the block is visible by default
    #[must_use]
    pub fn is_open(&self) -> Option<bool> {
        match self {
            Self::Details(val) => val.is_open,
            _ => None,
        }
    }

    /// Helper method for field `is_striped`.
    ///
    /// `true`, if the table is striped
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
    pub fn items(&self) -> Option<&[crate::types::RichBlockListItem]> {
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
    /// Available sizes of the photo
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Photo(val) => Some(val.photo.as_ref()),
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
    /// - `RichBlockParagraph`, `RichBlockSectionHeading`, `RichBlockPreformatted`, `RichBlockFooter`, `RichBlockPullQuotation`. Text of the block
    /// - `RichBlockThinking`. Text of the block. See <https://t.me/addemoji/AIActions> for examples of custom emoji that are recommended for usage in the block.
    #[must_use]
    pub fn text(&self) -> Option<&crate::types::RichText> {
        match self {
            Self::Paragraph(val) => Some(val.text.as_ref()),
            Self::Heading(val) => Some(val.text.as_ref()),
            Self::Pre(val) => Some(val.text.as_ref()),
            Self::Footer(val) => Some(val.text.as_ref()),
            Self::Pullquote(val) => Some(val.text.as_ref()),
            Self::Thinking(val) => Some(val.text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `video`.
    ///
    /// The video
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::Video(val) => Some(val.video.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `voice_note`.
    ///
    /// The voice note
    #[must_use]
    pub fn voice_note(&self) -> Option<&crate::types::Voice> {
        match self {
            Self::VoiceNote(val) => Some(&val.voice_note),
            _ => None,
        }
    }

    /// Helper method for field `width`.
    ///
    /// Expected width of the map
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Map(val) => Some(val.width),
            _ => None,
        }
    }

    /// Helper method for field `zoom`.
    ///
    /// Map zoom level; 13-20
    #[must_use]
    pub fn zoom(&self) -> Option<u8> {
        match self {
            Self::Map(val) => Some(val.zoom),
            _ => None,
        }
    }

    /// Helper method for nested field `cover`.
    #[must_use]
    pub fn cover(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.cover.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `duration`.
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.duration)
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.duration)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.duration)
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                Some(inner.duration)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_id`.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                Some(inner.file_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_name`.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_name.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_name.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_size`.
    #[must_use]
    pub fn file_size(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_size
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_size
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_size
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                inner.file_size
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_unique_id`.
    #[must_use]
    pub fn file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                Some(inner.file_unique_id.as_ref())
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

    /// Helper method for nested field `mime_type`.
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.mime_type.as_deref()
            }
            Self::VoiceNote(val) => {
                let inner = &val.voice_note;
                inner.mime_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `performer`.
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
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

    /// Helper method for nested field `qualities`.
    #[must_use]
    pub fn qualities(&self) -> Option<&[crate::types::VideoQuality]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.qualities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `start_timestamp`.
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.start_timestamp
            }
            _ => None,
        }
    }

    /// Helper method for nested field `thumbnail`.
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::PhotoSize> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
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
                let inner = val.audio.as_ref();
                inner.title.as_deref()
            }
            _ => None,
        }
    }
}
impl From<crate::types::RichBlockParagraph> for RichBlock {
    fn from(val: crate::types::RichBlockParagraph) -> Self {
        Self::Paragraph(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockParagraph {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Paragraph(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockParagraph),
            ))
        }
    }
}
impl From<crate::types::RichBlockSectionHeading> for RichBlock {
    fn from(val: crate::types::RichBlockSectionHeading) -> Self {
        Self::Heading(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockSectionHeading {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Heading(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockSectionHeading),
            ))
        }
    }
}
impl From<crate::types::RichBlockPreformatted> for RichBlock {
    fn from(val: crate::types::RichBlockPreformatted) -> Self {
        Self::Pre(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockPreformatted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Pre(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockPreformatted),
            ))
        }
    }
}
impl From<crate::types::RichBlockFooter> for RichBlock {
    fn from(val: crate::types::RichBlockFooter) -> Self {
        Self::Footer(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockFooter {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Footer(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockFooter),
            ))
        }
    }
}
impl From<crate::types::RichBlockDivider> for RichBlock {
    fn from(val: crate::types::RichBlockDivider) -> Self {
        Self::Divider(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockDivider {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Divider(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockDivider),
            ))
        }
    }
}
impl From<crate::types::RichBlockMathematicalExpression> for RichBlock {
    fn from(val: crate::types::RichBlockMathematicalExpression) -> Self {
        Self::MathematicalExpression(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockMathematicalExpression {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::MathematicalExpression(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockMathematicalExpression),
            ))
        }
    }
}
impl From<crate::types::RichBlockAnchor> for RichBlock {
    fn from(val: crate::types::RichBlockAnchor) -> Self {
        Self::Anchor(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockAnchor {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Anchor(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockAnchor),
            ))
        }
    }
}
impl From<crate::types::RichBlockList> for RichBlock {
    fn from(val: crate::types::RichBlockList) -> Self {
        Self::List(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockList {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::List(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockList),
            ))
        }
    }
}
impl From<crate::types::RichBlockBlockQuotation> for RichBlock {
    fn from(val: crate::types::RichBlockBlockQuotation) -> Self {
        Self::Blockquote(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockBlockQuotation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Blockquote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockBlockQuotation),
            ))
        }
    }
}
impl From<crate::types::RichBlockPullQuotation> for RichBlock {
    fn from(val: crate::types::RichBlockPullQuotation) -> Self {
        Self::Pullquote(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockPullQuotation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Pullquote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockPullQuotation),
            ))
        }
    }
}
impl From<crate::types::RichBlockCollage> for RichBlock {
    fn from(val: crate::types::RichBlockCollage) -> Self {
        Self::Collage(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockCollage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Collage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockCollage),
            ))
        }
    }
}
impl From<crate::types::RichBlockSlideshow> for RichBlock {
    fn from(val: crate::types::RichBlockSlideshow) -> Self {
        Self::Slideshow(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockSlideshow {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Slideshow(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockSlideshow),
            ))
        }
    }
}
impl From<crate::types::RichBlockTable> for RichBlock {
    fn from(val: crate::types::RichBlockTable) -> Self {
        Self::Table(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockTable {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Table(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockTable),
            ))
        }
    }
}
impl From<crate::types::RichBlockDetails> for RichBlock {
    fn from(val: crate::types::RichBlockDetails) -> Self {
        Self::Details(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockDetails {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Details(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockDetails),
            ))
        }
    }
}
impl From<crate::types::RichBlockMap> for RichBlock {
    fn from(val: crate::types::RichBlockMap) -> Self {
        Self::Map(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockMap {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Map(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockMap),
            ))
        }
    }
}
impl From<crate::types::RichBlockAnimation> for RichBlock {
    fn from(val: crate::types::RichBlockAnimation) -> Self {
        Self::Animation(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Animation(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockAnimation),
            ))
        }
    }
}
impl From<crate::types::RichBlockAudio> for RichBlock {
    fn from(val: crate::types::RichBlockAudio) -> Self {
        Self::Audio(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Audio(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockAudio),
            ))
        }
    }
}
impl From<crate::types::RichBlockPhoto> for RichBlock {
    fn from(val: crate::types::RichBlockPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockPhoto),
            ))
        }
    }
}
impl From<crate::types::RichBlockVideo> for RichBlock {
    fn from(val: crate::types::RichBlockVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockVideo),
            ))
        }
    }
}
impl From<crate::types::RichBlockVoiceNote> for RichBlock {
    fn from(val: crate::types::RichBlockVoiceNote) -> Self {
        Self::VoiceNote(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockVoiceNote {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::VoiceNote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockVoiceNote),
            ))
        }
    }
}
impl From<crate::types::RichBlockThinking> for RichBlock {
    fn from(val: crate::types::RichBlockThinking) -> Self {
        Self::Thinking(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockThinking {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Thinking(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockThinking),
            ))
        }
    }
}
impl From<crate::types::RichBlockUnknown> for RichBlock {
    fn from(val: crate::types::RichBlockUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<RichBlock> for crate::types::RichBlockUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichBlock) -> Result<Self, Self::Error> {
        if let RichBlock::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichBlock),
                stringify!(RichBlockUnknown),
            ))
        }
    }
}
