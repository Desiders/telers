use serde::Serialize;

use crate::types::InputFile;

/// Describes a photo to post as a story.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgiftregular>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputStoryContentPhoto<'a> {
    /// The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub photo: InputFile<'a>,
}
