use bon::bon;
use telers::types::LinkPreviewOptions;

use crate::{entities::DataMap, widgets::Text};

/// Widget that renders link preview options for a window.
pub trait LinkPreviewWidget: Send + Sync + 'static {
    /// Render link preview options for the current data snapshot.
    fn render_link_preview(&self, data: &DataMap) -> Option<LinkPreviewOptions>;
}

/// Configurable link preview widget.
pub struct LinkPreview {
    url: Option<Box<dyn Text>>,
    is_disabled: bool,
    prefer_small_media: bool,
    prefer_large_media: bool,
    show_above_text: bool,
}

#[bon]
impl LinkPreview {
    /// Create a new link preview widget.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(with = |url: impl Text| Box::new(url))] url: Option<Box<dyn Text>>,
        #[builder(default = false)] is_disabled: bool,
        #[builder(default = false)] prefer_small_media: bool,
        #[builder(default = false)] prefer_large_media: bool,
        #[builder(default = false)] show_above_text: bool,
    ) -> Self {
        Self {
            url,
            is_disabled,
            prefer_small_media,
            prefer_large_media,
            show_above_text,
        }
    }
}

impl Default for LinkPreview {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl LinkPreviewWidget for LinkPreview {
    fn render_link_preview(&self, data: &DataMap) -> Option<LinkPreviewOptions> {
        Some(
            LinkPreviewOptions::new()
                .url_option(
                    self.url
                        .as_ref()
                        .map(|url| url.render_text(data).into_string()),
                )
                .is_disabled(self.is_disabled)
                .prefer_small_media(self.prefer_small_media)
                .prefer_large_media(self.prefer_large_media)
                .show_above_text(self.show_above_text),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkPreview, LinkPreviewWidget};
    use crate::entities::DataMap;

    #[test]
    fn link_preview_renders_options() {
        let preview = LinkPreview::builder()
            .url("https://example.com/menu")
            .prefer_large_media(true)
            .show_above_text(true)
            .build();

        let options = preview
            .render_link_preview(&DataMap::new())
            .expect("link preview");

        assert_eq!(options.url.as_deref(), Some("https://example.com/menu"));
        assert_eq!(options.prefer_large_media, Some(true));
        assert_eq!(options.show_above_text, Some(true));
    }
}
