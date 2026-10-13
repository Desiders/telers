use std::sync::Arc;

use async_fn_traits::AsyncFn1;
use bon::bon;
use telers::types::{InlineKeyboardButton, KeyboardButton};

use super::{when::is_allowed, ButtonStyle, WhenCondition};
use crate::{entities::RenderContext, future::BoxFuture, widgets::Text};

type StyleRenderer = dyn Fn(RenderContext) -> BoxFuture<'static, Option<ButtonStyle>> + Send + Sync;

#[derive(Clone)]
enum StyleSource {
    Static(ButtonStyle),
    Dynamic(Arc<StyleRenderer>),
}

/// Reusable conditional button style and custom emoji.
#[derive(Clone, Default)]
pub struct ButtonAppearance {
    style: Option<StyleSource>,
    icon_custom_emoji_id: Option<Arc<dyn Text>>,
    when: Option<WhenCondition>,
    fallbacks: Vec<Self>,
}

#[bon]
impl ButtonAppearance {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field)] style: Option<StyleSource>,
        #[builder(field)] icon_custom_emoji_id: Option<Arc<dyn Text>>,
        #[builder(field)] fallbacks: Vec<Self>,
        when: Option<WhenCondition>,
    ) -> Self {
        Self {
            style,
            icon_custom_emoji_id,
            when,
            fallbacks,
        }
    }

    #[must_use]
    pub fn or(mut self, fallback: Self) -> Self {
        self.fallbacks.push(fallback);
        self
    }

    pub(crate) fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(StyleSource::Static(style));
        self
    }

    pub(crate) fn with_style_dynamic<F>(mut self, renderer: F) -> Self
    where
        F: AsyncFn(RenderContext) -> Option<ButtonStyle>
            + AsyncFn1<RenderContext, Output = Option<ButtonStyle>>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<RenderContext>>::OutputFuture: Send + 'static,
    {
        self.style = Some(style_source(renderer));
        self
    }

    pub(crate) fn with_emoji(mut self, emoji: impl Text) -> Self {
        self.icon_custom_emoji_id = Some(Arc::new(emoji));
        self
    }

    fn render<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
        render_style: bool,
        render_emoji: bool,
    ) -> BoxFuture<'a, (Option<ButtonStyle>, Option<Box<str>>)> {
        Box::pin(async move {
            let (mut style, mut emoji) = if is_allowed(
                self.when.as_ref(),
                &render_ctx.context,
                &render_ctx.data,
            )
            .await
            {
                let style = match (render_style, &self.style) {
                    (true, Some(StyleSource::Static(style))) => Some(*style),
                    (true, Some(StyleSource::Dynamic(renderer))) => {
                        renderer(render_ctx.clone()).await
                    }
                    _ => None,
                };
                let emoji = match (render_emoji, &self.icon_custom_emoji_id) {
                    (true, Some(text)) => {
                        let emoji = text.render_text_in_context(render_ctx).await;
                        (!emoji.is_empty()).then_some(emoji)
                    }
                    _ => None,
                };
                (style, emoji)
            } else {
                (None, None)
            };
            for fallback in &self.fallbacks {
                let need_style = render_style && style.is_none();
                let need_emoji = render_emoji && emoji.is_none();
                if !need_style && !need_emoji {
                    break;
                }
                let (fallback_style, fallback_emoji) = fallback
                    .render(render_ctx, need_style, need_emoji)
                    .await;
                style = style.or(fallback_style);
                emoji = emoji.or(fallback_emoji);
            }
            (style, emoji)
        })
    }

    pub(crate) async fn apply(
        &self,
        button: InlineKeyboardButton,
        render_ctx: &RenderContext,
    ) -> InlineKeyboardButton {
        let (style, emoji) = self.render(render_ctx, true, true).await;
        button
            .style_option(style.map(|style| style.as_str()))
            .icon_custom_emoji_id_option(emoji)
    }

    pub(crate) async fn apply_reply(
        &self,
        button: KeyboardButton,
        render_ctx: &RenderContext,
    ) -> KeyboardButton {
        let (style, emoji) = self.render(render_ctx, true, true).await;
        button
            .style_option(style.map(|style| style.as_str()))
            .icon_custom_emoji_id_option(emoji)
    }
}

impl<S> ButtonAppearanceBuilder<S>
where
    S: button_appearance_builder::State,
{
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(StyleSource::Static(style));
        self
    }

    pub fn style_dynamic<F>(mut self, renderer: F) -> Self
    where
        F: AsyncFn(RenderContext) -> Option<ButtonStyle>
            + AsyncFn1<RenderContext, Output = Option<ButtonStyle>>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<RenderContext>>::OutputFuture: Send + 'static,
    {
        self.style = Some(style_source(renderer));
        self
    }

    pub fn icon_custom_emoji_id(mut self, emoji: impl Text) -> Self {
        self.icon_custom_emoji_id = Some(Arc::new(emoji));
        self
    }
}

fn style_source<F>(renderer: F) -> StyleSource
where
    F: AsyncFn(RenderContext) -> Option<ButtonStyle>
        + AsyncFn1<RenderContext, Output = Option<ButtonStyle>>
        + Send
        + Sync
        + 'static,
    <F as AsyncFn1<RenderContext>>::OutputFuture: Send + 'static,
{
    let renderer = Arc::new(renderer);
    StyleSource::Dynamic(Arc::new(move |ctx| {
        let renderer = renderer.clone();
        Box::pin(async move { renderer(ctx).await })
    }))
}

#[cfg(test)]
mod tests {
    use telers::serialization::{json, Value};

    use super::ButtonAppearance;
    use crate::{
        entities::{Context, DataMap, RenderContext},
        widgets::{Button, ButtonStyle, FnText, InlineKeyboard, Keyboard, WhenCondition},
    };

    async fn render(
        appearance: ButtonAppearance,
        data: &DataMap,
    ) -> telers::types::InlineKeyboardButton {
        let keyboard = InlineKeyboard::builder()
            .row([Button::done("a", "A").appearance(appearance)])
            .build();
        let markup = keyboard
            .render_keyboard_for_test(&Context::new("", "state", Value::default()), data)
            .await
            .unwrap();
        markup.inline_keyboard().unwrap()[0][0].clone()
    }

    #[tokio::test]
    async fn conditional_appearance_uses_fallback_when_hidden() {
        let appearance = ButtonAppearance::builder()
            .style(ButtonStyle::Danger)
            .icon_custom_emoji_id("111")
            .when(WhenCondition::data_field("danger"))
            .build()
            .or(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Primary)
                    .icon_custom_emoji_id("222")
                    .build(),
            );
        let button = render(appearance.clone(), &DataMap::new()).await;
        assert_eq!(button.style.as_deref(), Some("primary"));
        assert_eq!(button.icon_custom_emoji_id.as_deref(), Some("222"));
        let button = render(appearance, &DataMap::from([("danger".into(), json!(true))])).await;
        assert_eq!(button.style.as_deref(), Some("danger"));
        assert_eq!(button.icon_custom_emoji_id.as_deref(), Some("111"));
    }

    #[tokio::test]
    async fn appearance_fallback_fills_style_and_emoji_independently() {
        let appearance = ButtonAppearance::builder()
            .style(ButtonStyle::Success)
            .build()
            .or(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Danger)
                    .icon_custom_emoji_id("123")
                    .build(),
            );
        let button = render(appearance, &DataMap::new()).await;
        assert_eq!(button.style.as_deref(), Some("success"));
        assert_eq!(button.icon_custom_emoji_id.as_deref(), Some("123"));
    }

    #[tokio::test]
    async fn appearance_fallback_only_renders_missing_fields() {
        let appearance = ButtonAppearance::builder()
            .style(ButtonStyle::Success)
            .build()
            .or(
                ButtonAppearance::builder()
                    .style_dynamic(|_: RenderContext| async {
                        panic!("already selected style rendered")
                    })
                    .icon_custom_emoji_id("123")
                    .build(),
            )
            .or(
                ButtonAppearance::builder()
                    .style_dynamic(|_: RenderContext| async { panic!("unused fallback rendered") })
                    .build(),
            );
        let button = render(appearance, &DataMap::new()).await;
        assert_eq!(button.style.as_deref(), Some("success"));
        assert_eq!(button.icon_custom_emoji_id.as_deref(), Some("123"));
    }

    #[tokio::test]
    async fn empty_dynamic_appearance_values_use_fallback() {
        let appearance = ButtonAppearance::builder()
            .style_dynamic(|_: RenderContext| async { None })
            .icon_custom_emoji_id(FnText::new(|_: &DataMap| String::new()))
            .build()
            .or(
                ButtonAppearance::builder()
                    .style(ButtonStyle::Primary)
                    .icon_custom_emoji_id("123")
                    .build(),
            );
        let button = render(appearance, &DataMap::new()).await;
        assert_eq!(button.style.as_deref(), Some("primary"));
        assert_eq!(button.icon_custom_emoji_id.as_deref(), Some("123"));
    }

    #[tokio::test]
    async fn hidden_appearance_does_not_run_dynamic_renderers() {
        let appearance = ButtonAppearance::builder()
            .style_dynamic(|_: RenderContext| async { panic!("hidden style rendered") })
            .icon_custom_emoji_id(FnText::new(|_: &DataMap| -> String {
                panic!("hidden emoji rendered")
            }))
            .when(WhenCondition::data_field("missing"))
            .build();
        let button = render(appearance, &DataMap::new()).await;
        assert!(button.style.is_none());
        assert!(button.icon_custom_emoji_id.is_none());
    }
}
