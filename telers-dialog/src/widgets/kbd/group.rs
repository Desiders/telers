use async_trait::async_trait;
use bon::bon;
use telers::types::{InlineKeyboardMarkup, ReplyMarkup};

use super::{when::is_allowed, ButtonAction, ClickContext, Keyboard, MultiKeyboard, WhenCondition};
use crate::entities::{Context, DataMap, RenderContext};

/// Layout wrapper that regroups inline keyboard buttons into fixed-width rows.
///
/// The wrapped keyboard renders normally; `Group` flattens the resulting inline
/// markup and re-chunks the buttons into rows of `items_per_row` (clamped to
/// at least `1`). Non-inline reply markups pass through unchanged.
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::{Button, ButtonAction, Group, InlineKeyboard};
///
/// let grouped = Group::builder(
///     InlineKeyboard::builder()
///         .row([
///             Button::action("a", "A", ButtonAction::noop()),
///             Button::action("b", "B", ButtonAction::noop()),
///             Button::action("c", "C", ButtonAction::noop()),
///         ])
///         .build(),
/// )
/// .items_per_row(2)
/// .build();
/// ```
pub struct Group<Kbd> {
    kbd: Kbd,
    items_per_row: usize,
    when: Option<WhenCondition>,
}

#[bon]
impl<Kbd> Group<Kbd> {
    /// Wrap a keyboard and regroup its inline buttons by `items_per_row`.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] kbd: Kbd,
        #[builder(default = 1)] items_per_row: usize,
        when: Option<WhenCondition>,
    ) -> Self {
        Self {
            kbd,
            items_per_row: items_per_row.max(1),
            when,
        }
    }
}

#[async_trait]
impl<Kbd> Keyboard for Group<Kbd>
where
    Kbd: Keyboard,
{
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context.as_ref();
        let data = render_ctx.data.as_ref();
        if !self.is_visible(ctx, data).await {
            return None;
        }
        if !self.kbd.is_visible(ctx, data).await {
            return None;
        }
        let markup = self.kbd.render_keyboard(render_ctx).await?;
        let ReplyMarkup::InlineKeyboardMarkup(markup) = markup else {
            return Some(markup);
        };

        regroup_rows(markup, self.items_per_row)
    }

    #[inline]
    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        let data = click.data.as_ref();
        if !self.is_visible(ctx, data).await {
            return None;
        }
        if !self.kbd.is_visible(ctx, data).await {
            return None;
        }
        self.kbd.handle_callback(click).await
    }
}

/// Compose arbitrary keyboard widgets, optionally arranging them in fixed-width rows.
pub struct KeyboardGroup {
    keyboards: MultiKeyboard,
    items_per_row: Option<usize>,
    when: Option<WhenCondition>,
}

#[bon]
impl KeyboardGroup {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field)] keyboards: Vec<Box<dyn Keyboard>>,
        items_per_row: Option<usize>,
        when: Option<WhenCondition>,
    ) -> Self {
        let keyboards = keyboards
            .into_iter()
            .fold(MultiKeyboard::new(), |group, keyboard| {
                group.kbd_boxed(keyboard)
            });
        Self {
            keyboards,
            items_per_row,
            when,
        }
    }
}

impl<S> KeyboardGroupBuilder<S>
where
    S: keyboard_group_builder::State,
{
    /// Append a keyboard, preserving its rows unless `items_per_row` is set.
    pub fn keyboard(mut self, keyboard: impl Keyboard) -> Self {
        self.keyboards.push(Box::new(keyboard));
        self
    }
}

#[async_trait]
impl Keyboard for KeyboardGroup {
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        if !self
            .is_visible(render_ctx.context.as_ref(), render_ctx.data.as_ref())
            .await
        {
            return None;
        }
        let markup = self.keyboards.render_keyboard(render_ctx).await?;
        let Some(items_per_row) = self.items_per_row else {
            return Some(markup);
        };
        let ReplyMarkup::InlineKeyboardMarkup(markup) = markup else {
            return Some(markup);
        };
        regroup_rows(markup, items_per_row)
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        if !self
            .is_visible(click.context.as_ref(), click.data.as_ref())
            .await
        {
            return None;
        }
        self.keyboards.handle_callback(click).await
    }
}

fn regroup_rows(markup: InlineKeyboardMarkup, items_per_row: usize) -> Option<ReplyMarkup> {
    let items_per_row = items_per_row.max(1);
    let capacity = items_per_row.min(
        markup
            .inline_keyboard
            .iter()
            .map(|row| row.len())
            .sum(),
    );
    let mut rows = Vec::new();
    let mut current_row = Vec::with_capacity(capacity);
    for button in markup.inline_keyboard.into_vec().into_iter().flatten() {
        current_row.push(button);
        if current_row.len() == items_per_row {
            rows.push(current_row.into_boxed_slice());
            current_row = Vec::with_capacity(capacity);
        }
    }
    if !current_row.is_empty() {
        rows.push(current_row.into_boxed_slice());
    }
    if rows.is_empty() {
        None
    } else {
        Some(InlineKeyboardMarkup::new(rows).into())
    }
}
