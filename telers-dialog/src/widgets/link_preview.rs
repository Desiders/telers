use async_trait::async_trait;
use bon::bon;
use telers::types::LinkPreviewOptions;

#[cfg(test)]
use crate::entities::{ChatEvent, EventContext};
use crate::{entities::RenderContext, widgets::Text};

/// Widget that renders link preview options for a window.
#[async_trait]
pub trait LinkPreviewWidget: Send + Sync + 'static {
    /// Render link preview options for the current data snapshot.
    async fn render_link_preview(&self, render_ctx: &RenderContext) -> Option<LinkPreviewOptions>;

    #[cfg(test)]
    async fn render_link_preview_for_test(
        &self,
        data: &crate::entities::DataMap,
    ) -> Option<LinkPreviewOptions> {
        use telers::{
            client::Reqwest,
            types::{ChatPrivate, MessageText, User},
            Bot,
        };

        let ctx = crate::entities::Context::new("", "state", serde_json::Value::Null);
        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatPrivate::new(10), "/test")
                .from(User::new(10, false, "tester"))
                .into(),
        );
        let event_context = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = RenderContext::new(&ctx, data, &event, &event_context);
        self.render_link_preview(&render_ctx).await
    }
}

/// Configurable link preview widget.
#[allow(clippy::struct_excessive_bools)]
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

#[async_trait]
impl LinkPreviewWidget for LinkPreview {
    async fn render_link_preview(&self, render_ctx: &RenderContext) -> Option<LinkPreviewOptions> {
        let url = if let Some(url) = &self.url {
            Some(url.render_text_in_context(render_ctx).await.into_string())
        } else {
            None
        };
        Some(
            LinkPreviewOptions::new()
                .url_option(url)
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

    #[tokio::test]
    async fn link_preview_renders_options() {
        let preview = LinkPreview::builder()
            .url("https://example.com/menu")
            .prefer_large_media(true)
            .show_above_text(true)
            .build();

        let options = preview
            .render_link_preview_for_test(&DataMap::new())
            .await
            .expect("link preview");

        assert_eq!(options.url.as_deref(), Some("https://example.com/menu"));
        assert_eq!(options.prefer_large_media, Some(true));
        assert_eq!(options.show_above_text, Some(true));
    }
}
