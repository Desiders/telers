use serde::{Deserialize, Serialize};
/// This object describes paid media. Currently, it can be one of
/// - [`crate::types::PaidMediaLivePhoto`]
/// - [`crate::types::PaidMediaPhoto`]
/// - [`crate::types::PaidMediaPreview`]
/// - [`crate::types::PaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaidMedia {
    LivePhoto(crate::types::PaidMediaLivePhoto),
    Photo(crate::types::PaidMediaPhoto),
    Preview(crate::types::PaidMediaPreview),
    Video(crate::types::PaidMediaVideo),
}
impl PaidMedia {
    /// Helper method for field `duration`.
    ///
    /// Duration of the media in seconds as defined by the sender
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Preview(val) => val.duration,
            _ => None,
        }
    }

    /// Helper method for field `height`.
    ///
    /// Media height as defined by the sender
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Preview(val) => val.height,
            _ => None,
        }
    }

    /// Helper method for field `live_photo`.
    ///
    /// The photo
    #[must_use]
    pub fn live_photo(&self) -> Option<&crate::types::LivePhoto> {
        match self {
            Self::LivePhoto(val) => Some(&val.live_photo),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// The photo
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Photo(val) => Some(val.photo.as_ref()),
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

    /// Helper method for field `width`.
    ///
    /// Media width as defined by the sender
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Preview(val) => val.width,
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

    /// Helper method for nested field `file_id`.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        match self {
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.file_id.as_ref())
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_name`.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
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
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                inner.file_size
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_size
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_unique_id`.
    #[must_use]
    pub fn file_unique_id(&self) -> Option<&str> {
        match self {
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.file_unique_id.as_ref())
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mime_type`.
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                inner.mime_type.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.mime_type.as_deref()
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
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.thumbnail.as_ref()
            }
            _ => None,
        }
    }
}
impl From<crate::types::PaidMediaLivePhoto> for PaidMedia {
    fn from(val: crate::types::PaidMediaLivePhoto) -> Self {
        Self::LivePhoto(val)
    }
}
impl TryFrom<PaidMedia> for crate::types::PaidMediaLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PaidMedia) -> Result<Self, Self::Error> {
        if let PaidMedia::LivePhoto(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PaidMedia),
                stringify!(PaidMediaLivePhoto),
            ))
        }
    }
}
impl From<crate::types::PaidMediaPhoto> for PaidMedia {
    fn from(val: crate::types::PaidMediaPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<PaidMedia> for crate::types::PaidMediaPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PaidMedia) -> Result<Self, Self::Error> {
        if let PaidMedia::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PaidMedia),
                stringify!(PaidMediaPhoto),
            ))
        }
    }
}
impl From<crate::types::PaidMediaPreview> for PaidMedia {
    fn from(val: crate::types::PaidMediaPreview) -> Self {
        Self::Preview(val)
    }
}
impl TryFrom<PaidMedia> for crate::types::PaidMediaPreview {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PaidMedia) -> Result<Self, Self::Error> {
        if let PaidMedia::Preview(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PaidMedia),
                stringify!(PaidMediaPreview),
            ))
        }
    }
}
impl From<crate::types::PaidMediaVideo> for PaidMedia {
    fn from(val: crate::types::PaidMediaVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<PaidMedia> for crate::types::PaidMediaVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PaidMedia) -> Result<Self, Self::Error> {
        if let PaidMedia::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PaidMedia),
                stringify!(PaidMediaVideo),
            ))
        }
    }
}
