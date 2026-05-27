use bon::bon;

use super::Text;
use crate::entities::{DataMap, RenderContext};
use async_trait::async_trait;

/// Render one of several text variants chosen by a selector closure.
///
/// The selector receives the current render data and returns a key. The first
/// variant whose registered key equals the selector's result is rendered.
/// When no variant matches, the optional [`default`](CaseBuilder::default)
/// variant renders; otherwise the widget produces an empty string.
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::Case;
///
/// let case = Case::builder(|data| data.get("plan").is_some())
///     .when(true, "Active subscription")
///     .when(false, "Free tier")
///     .build();
/// ```
pub struct Case<Selector, Key> {
    selector: Selector,
    variants: Vec<(Key, Box<dyn Text>)>,
    default: Option<Box<dyn Text>>,
}

#[bon]
impl<Selector, Key> Case<Selector, Key> {
    /// Build a [`Case`] widget bound to the given selector.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] selector: Selector,
        #[builder(field)] variants: Vec<(Key, Box<dyn Text>)>,
        #[builder(field)] default: Option<Box<dyn Text>>,
    ) -> Self
    where
        Selector: Fn(&DataMap) -> Key,
        Key: PartialEq,
    {
        Self {
            selector,
            variants,
            default,
        }
    }
}

impl<Selector, Key, S> CaseBuilder<Selector, Key, S>
where
    S: case_builder::State,
    Selector: Fn(&DataMap) -> Key,
    Key: PartialEq,
{
    /// Register a keyed text variant; rendered when the selector returns `key`.
    pub fn when(mut self, key: Key, text: impl Text) -> Self {
        self.variants.push((key, Box::new(text)));
        self
    }

    /// Register the fallback text rendered when no keyed variant matches.
    pub fn default(mut self, text: impl Text) -> Self {
        self.default = Some(Box::new(text));
        self
    }
}

#[async_trait]
impl<Selector, Key> Text for Case<Selector, Key>
where
    Selector: Fn(&DataMap) -> Key + Send + Sync + 'static,
    Key: PartialEq + Send + Sync + 'static,
{
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        let selected = (self.selector)(data);
        if let Some((_, text)) = self.variants.iter().find(|(key, _)| *key == selected) {
            return text.render_text(data).await;
        }
        match &self.default {
            Some(text) => text.render_text(data).await,
            None => Box::<str>::default(),
        }
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        let selected = (self.selector)(render_ctx.data.as_ref());
        if let Some((_, text)) = self.variants.iter().find(|(key, _)| *key == selected) {
            return text.render_text_in_context(render_ctx).await;
        }
        match &self.default {
            Some(text) => text.render_text_in_context(render_ctx).await,
            None => Box::<str>::default(),
        }
    }
}
