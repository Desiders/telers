use async_trait::async_trait;
use bon::bon;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{borrow::Cow, fmt::Display, future::Future, sync::Arc};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use time::{Date, Duration, Month, OffsetDateTime, UtcOffset, Weekday};
use tracing::debug;

use super::{
    format_callback_data, parse_callback_data,
    when::{is_allowed, WhenContext},
    ButtonAction, ClickContext, Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

const CALLBACK_NEXT_MONTH: &str = "+";
const CALLBACK_PREV_MONTH: &str = "-";
const CALLBACK_NEXT_YEAR: &str = "+Y";
const CALLBACK_PREV_YEAR: &str = "-Y";
const CALLBACK_NEXT_YEARS_PAGE: &str = "+YY";
const CALLBACK_PREV_YEARS_PAGE: &str = "-YY";
const CALLBACK_SCOPE_MONTHS: &str = "M";
const CALLBACK_SCOPE_YEARS: &str = "Y";
const CALLBACK_NOOP: &str = "noop";
const CALLBACK_PREFIX_MONTH: &str = "MONTH";
const CALLBACK_PREFIX_YEAR: &str = "YEAR";
const CALLBACK_PREFIX_DATE: &str = "DATE";

type CalendarClickHandler =
    dyn Fn(ClickContext, CalendarDate) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;
type CalendarConfigGetter =
    dyn Fn(WhenContext) -> BoxFuture<'static, CalendarUserConfig> + Send + Sync + 'static;
type CalendarTextRenderer =
    dyn Fn(CalendarButtonKind, RenderContext) -> BoxFuture<'static, String> + Send + Sync + 'static;
/// Inline-keyboard rows returned by custom calendar scope renderers.
pub type CalendarScopeRows = Vec<Box<[InlineKeyboardButton]>>;
type CalendarScopeView =
    dyn Fn(CalendarViewContext) -> BoxFuture<'static, CalendarScopeRows> + Send + Sync + 'static;

/// Date type used by [`Calendar`] callbacks and configuration.
pub type CalendarDate = Date;

/// Button role rendered by the built-in calendar views.
///
/// Use this in [`CalendarAppearance`] to customize labels without replacing the
/// complete days, months, or years view. Navigation buttons and inert header or
/// filler buttons are included so one renderer can style the whole widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarButtonKind {
    /// Empty filler cell used to keep the calendar grid rectangular.
    Empty,
    /// Weekday header cell in the days view.
    Weekday {
        /// Weekday represented by this header cell.
        weekday: Weekday,
    },
    /// Top header button for the days view.
    DaysHeader {
        /// First day of the currently rendered month.
        month: CalendarDate,
    },
    /// Selectable date button in the days view.
    Day {
        /// Date selected when this button is clicked.
        date: CalendarDate,
        /// Whether `date` matches the configured "today" value.
        is_today: bool,
    },
    /// Previous-month navigation button in the days view.
    DaysPrevMonth {
        /// First day of the month that will be shown after clicking.
        month: CalendarDate,
    },
    /// Zoom button that switches from days to months.
    DaysZoom {
        /// First day of the month currently rendered in the days view.
        month: CalendarDate,
    },
    /// Next-month navigation button in the days view.
    DaysNextMonth {
        /// First day of the month that will be shown after clicking.
        month: CalendarDate,
    },
    /// Top header button for the months view.
    MonthsHeader {
        /// Year currently rendered in the months view.
        year: i32,
    },
    /// Selectable month button in the months view.
    Month {
        /// First day of the represented month.
        month: CalendarDate,
        /// Whether this month contains the configured "today" date.
        is_current: bool,
    },
    /// Previous-year navigation button in the months view.
    MonthsPrevYear {
        /// Year that will be shown after clicking.
        year: i32,
    },
    /// Zoom button that switches from months to years.
    MonthsZoom {
        /// Year currently rendered in the months view.
        year: i32,
    },
    /// Next-year navigation button in the months view.
    MonthsNextYear {
        /// Year that will be shown after clicking.
        year: i32,
    },
    /// Selectable year button in the years view.
    Year {
        /// Year represented by this button.
        year: i32,
        /// Whether this year contains the configured "today" date.
        is_current: bool,
    },
    /// Previous years-page navigation button in the years view.
    YearsPrevPage {
        /// First year that will be shown after clicking.
        year: i32,
    },
    /// Next years-page navigation button in the years view.
    YearsNextPage {
        /// First year that will be shown after clicking.
        year: i32,
    },
}

/// Label customization for the built-in calendar views.
///
/// The default renderer prints short weekday names, month/year headers,
/// navigation labels, and wraps the configured "today" date in brackets.
/// Provide `text_renderer(...)` when you only need to change button text while
/// keeping the standard days, months, and years layout.
#[derive(Clone)]
pub struct CalendarAppearance {
    text_renderer: Arc<CalendarTextRenderer>,
}

impl Default for CalendarAppearance {
    fn default() -> Self {
        Self {
            text_renderer: Arc::new(default_calendar_text_renderer),
        }
    }
}

#[bon]
impl CalendarAppearance {
    /// Create calendar appearance hooks.
    ///
    /// Use [`CalendarAppearanceBuilder::text_renderer`] to customize labels
    /// rendered by the built-in views.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field = Arc::new(default_calendar_text_renderer))] text_renderer: Arc<
            CalendarTextRenderer,
        >,
    ) -> Self {
        Self { text_renderer }
    }
}

impl<S> CalendarAppearanceBuilder<S>
where
    S: calendar_appearance_builder::State,
{
    /// Customize labels rendered by built-in calendar views.
    pub fn text_renderer<F, Fut>(mut self, text_renderer: F) -> Self
    where
        F: Fn(CalendarButtonKind, RenderContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = String> + Send + 'static,
    {
        let text_renderer = Arc::new(text_renderer);
        self.text_renderer = Arc::new(move |kind, render_ctx| {
            let text_renderer = text_renderer.clone();
            Box::pin(async move { text_renderer(kind, render_ctx).await })
        });
        self
    }
}

/// Context passed into custom calendar scope renderers.
///
/// Custom views can use this context to inspect dialog data, current calendar
/// state, and config while producing [`InlineKeyboardButton`] rows. Prefer
/// [`CalendarViewContext::button`] and [`CalendarViewContext::noop_button`] so
/// callbacks keep the same `td:{intent_id}:{widget_id}:{payload}` contract as
/// the built-in views.
#[derive(Clone, Debug)]
pub struct CalendarViewContext {
    /// Dialog context currently being rendered.
    pub context: Arc<Context>,
    /// Data passed to the current window render.
    pub data: Arc<DataMap>,
    /// Fully resolved calendar config after dynamic user overrides are merged.
    pub config: CalendarConfig,
    /// Persisted calendar scope and offset currently being rendered.
    pub state: CalendarState,
    /// Widget id used by the calendar callback payloads.
    pub widget_id: Cow<'static, str>,
}

impl CalendarViewContext {
    /// Build a callback button targeted at this calendar widget.
    ///
    /// The payload must use the same payload grammar as the built-in calendar
    /// handler when you want default callback handling. For example,
    /// `DATE2026-04-13` selects a date and `noop` consumes a callback without
    /// changing state.
    #[must_use]
    pub fn button(&self, text: impl Into<Box<str>>, payload: &str) -> InlineKeyboardButton {
        InlineKeyboardButton::new(text).callback_data(format_callback_data(
            self.context.as_ref(),
            self.widget_id.as_ref(),
            Some(payload),
        ))
    }

    /// Build an inert callback button targeted at this calendar widget.
    #[must_use]
    pub fn noop_button(&self, text: impl Into<Box<str>>) -> InlineKeyboardButton {
        self.button(text, CALLBACK_NOOP)
    }
}

/// Optional custom renderers for complete calendar scopes.
///
/// Use this when label-level customization is not enough and a scope needs a
/// different row layout. Any omitted scope falls back to the built-in renderer.
#[derive(Clone, Default)]
pub struct CalendarViews {
    days: Option<Arc<CalendarScopeView>>,
    months: Option<Arc<CalendarScopeView>>,
    years: Option<Arc<CalendarScopeView>>,
}

#[bon]
impl CalendarViews {
    /// Build custom calendar scope renderers.
    ///
    /// Each omitted scope falls back to the built-in renderer.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field)] days: Option<Arc<CalendarScopeView>>,
        #[builder(field)] months: Option<Arc<CalendarScopeView>>,
        #[builder(field)] years: Option<Arc<CalendarScopeView>>,
    ) -> Self {
        Self {
            days,
            months,
            years,
        }
    }

    fn get(&self, scope: CalendarScope) -> Option<&Arc<CalendarScopeView>> {
        match scope {
            CalendarScope::Days => self.days.as_ref(),
            CalendarScope::Months => self.months.as_ref(),
            CalendarScope::Years => self.years.as_ref(),
        }
    }
}

impl<S> CalendarViewsBuilder<S>
where
    S: calendar_views_builder::State,
{
    /// Replace the days scope renderer.
    pub fn days<F, Fut>(mut self, days: F) -> Self
    where
        F: Fn(CalendarViewContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CalendarScopeRows> + Send + 'static,
    {
        self.days = Some(scope_view(days));
        self
    }

    /// Replace the months scope renderer.
    pub fn months<F, Fut>(mut self, months: F) -> Self
    where
        F: Fn(CalendarViewContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CalendarScopeRows> + Send + 'static,
    {
        self.months = Some(scope_view(months));
        self
    }

    /// Replace the years scope renderer.
    pub fn years<F, Fut>(mut self, years: F) -> Self
    where
        F: Fn(CalendarViewContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CalendarScopeRows> + Send + 'static,
    {
        self.years = Some(scope_view(years));
        self
    }
}

fn scope_view<F, Fut>(view: F) -> Arc<CalendarScopeView>
where
    F: Fn(CalendarViewContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = CalendarScopeRows> + Send + 'static,
{
    let view = Arc::new(view);
    Arc::new(move |view_ctx| {
        let view = view.clone();
        Box::pin(async move { view(view_ctx).await })
    })
}

/// Calendar view currently rendered by [`Calendar`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarScope {
    /// Day grid for a single month.
    Days,
    /// Month grid for a single year.
    Months,
    /// Year grid for a page of years.
    Years,
}

/// Rendering and range settings for [`Calendar`].
#[derive(Clone, Debug)]
pub struct CalendarConfig {
    /// First weekday shown in the days view header.
    pub first_weekday: Weekday,
    /// Offset used to calculate the default "today" date.
    pub timezone: UtcOffset,
    /// Earliest date that can be selected or navigated to.
    pub min_date: CalendarDate,
    /// Latest date that can be selected or navigated to.
    pub max_date: CalendarDate,
    /// Number of month buttons per row in the months view.
    pub month_columns: usize,
    /// Number of years shown in one years-view page.
    pub years_per_page: usize,
    /// Number of year buttons per row in the years view.
    pub years_columns: usize,
    /// Fixed "today" date used for rendering, or `None` to compute it from
    /// [`CalendarConfig::timezone`] at render time.
    pub today: Option<CalendarDate>,
}

impl Default for CalendarConfig {
    fn default() -> Self {
        Self {
            first_weekday: Weekday::Monday,
            timezone: UtcOffset::UTC,
            min_date: date(1900, 1, 1),
            max_date: date(2100, 12, 31),
            month_columns: 3,
            years_per_page: 20,
            years_columns: 5,
            today: None,
        }
    }
}

#[bon]
impl CalendarConfig {
    /// Create calendar configuration.
    ///
    /// `min_date` and `max_date` are sorted automatically when they are passed
    /// in reverse order. Column and page counts are clamped to at least `1`.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(default = Weekday::Monday)] first_weekday: Weekday,
        #[builder(default = UtcOffset::UTC)] timezone: UtcOffset,
        #[builder(default = date(1900, 1, 1))] min_date: CalendarDate,
        #[builder(default = date(2100, 12, 31))] max_date: CalendarDate,
        #[builder(default = 3)] month_columns: usize,
        #[builder(default = 20)] years_per_page: usize,
        #[builder(default = 5)] years_columns: usize,
        today: Option<CalendarDate>,
    ) -> Self {
        let (min_date, max_date) = if min_date <= max_date {
            (min_date, max_date)
        } else {
            (max_date, min_date)
        };
        Self {
            first_weekday,
            timezone,
            min_date,
            max_date,
            month_columns: month_columns.max(1),
            years_per_page: years_per_page.max(1),
            years_columns: years_columns.max(1),
            today,
        }
    }

    fn today(&self) -> CalendarDate {
        self.today
            .unwrap_or_else(|| OffsetDateTime::now_utc().to_offset(self.timezone).date())
    }

    fn merge_user_config(&self, cfg: &CalendarUserConfig) -> Self {
        let min_date = cfg.min_date.unwrap_or(self.min_date);
        let max_date = cfg.max_date.unwrap_or(self.max_date);
        let (min_date, max_date) = if min_date <= max_date {
            (min_date, max_date)
        } else {
            (max_date, min_date)
        };
        Self {
            first_weekday: cfg.first_weekday.unwrap_or(self.first_weekday),
            timezone: cfg.timezone.unwrap_or(self.timezone),
            min_date,
            max_date,
            month_columns: cfg.month_columns.unwrap_or(self.month_columns).max(1),
            years_per_page: cfg.years_per_page.unwrap_or(self.years_per_page).max(1),
            years_columns: cfg.years_columns.unwrap_or(self.years_columns).max(1),
            today: cfg.today.or(self.today),
        }
    }
}

/// Per-render calendar config overrides.
///
/// Values returned from a calendar `config_getter(...)` are merged over the
/// base [`CalendarConfig`] for a single render/callback pass. Leave a field as
/// `None` to keep the base configuration.
#[derive(Clone, Debug, Default)]
pub struct CalendarUserConfig {
    /// Override for [`CalendarConfig::first_weekday`].
    pub first_weekday: Option<Weekday>,
    /// Override for [`CalendarConfig::timezone`].
    pub timezone: Option<UtcOffset>,
    /// Override for [`CalendarConfig::min_date`].
    pub min_date: Option<CalendarDate>,
    /// Override for [`CalendarConfig::max_date`].
    pub max_date: Option<CalendarDate>,
    /// Override for [`CalendarConfig::month_columns`].
    pub month_columns: Option<usize>,
    /// Override for [`CalendarConfig::years_per_page`].
    pub years_per_page: Option<usize>,
    /// Override for [`CalendarConfig::years_columns`].
    pub years_columns: Option<usize>,
    /// Override for the [`CalendarConfig`] `today` field.
    pub today: Option<CalendarDate>,
}

#[bon]
impl CalendarUserConfig {
    /// Create per-render calendar config overrides.
    #[builder]
    #[must_use]
    pub fn new(
        first_weekday: Option<Weekday>,
        timezone: Option<UtcOffset>,
        min_date: Option<CalendarDate>,
        max_date: Option<CalendarDate>,
        month_columns: Option<usize>,
        years_per_page: Option<usize>,
        years_columns: Option<usize>,
        today: Option<CalendarDate>,
    ) -> Self {
        Self {
            first_weekday,
            timezone,
            min_date,
            max_date,
            month_columns,
            years_per_page,
            years_columns,
            today,
        }
    }
}

/// Persisted state stored in `widget_data` for [`Calendar`].
///
/// Calendar navigation callbacks update this state with
/// [`ButtonAction::SetWidgetValue`](crate::widgets::ButtonAction::SetWidgetValue).
/// Date-selection callbacks do not mutate it automatically; they call the
/// calendar `on_click(...)` handler or return [`ButtonAction::Noop`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarState {
    /// Scope currently rendered by the calendar.
    pub current_scope: CalendarScope,
    /// Date used as the current month, year, or years-page offset.
    pub current_offset: CalendarDate,
}

impl CalendarState {
    /// Create persisted calendar state.
    #[must_use]
    pub const fn new(current_scope: CalendarScope, current_offset: CalendarDate) -> Self {
        Self {
            current_scope,
            current_offset,
        }
    }

    /// Convert state to the JSON representation stored in `widget_data`.
    #[must_use]
    pub fn to_value(&self) -> Value {
        json!({
            "current_scope": self.current_scope,
            "current_offset": self.current_offset.to_string(),
        })
    }

    /// Decode calendar state from a `widget_data` value.
    #[must_use]
    pub fn from_value(value: &Value) -> Option<Self> {
        let scope = serde_json::from_value(value.get("current_scope")?.clone()).ok()?;
        let offset = parse_date(value.get("current_offset")?.as_str()?)?;
        Some(Self::new(scope, offset))
    }
}

/// Date selection keyboard with days, months, and years views.
///
/// The calendar stores navigation state in `widget_data` under its widget id.
/// Clicking a day calls `on_click(...)` with a [`CalendarDate`]; clicking month,
/// year, and pager controls updates the stored [`CalendarState`].
///
/// ```
/// use telers_dialog::widgets::{ButtonAction, Calendar};
///
/// let _calendar = Calendar::builder("reservation_date")
///     .on_click(|_click, selected_date| async move {
///         ButtonAction::set_dialog_value("selected_date", selected_date.to_string())
///     })
///     .build();
/// ```
pub struct Calendar<WidgetId> {
    id: WidgetId,
    config: CalendarConfig,
    config_getter: Option<Arc<CalendarConfigGetter>>,
    appearance: CalendarAppearance,
    views: CalendarViews,
    on_click: Option<Arc<CalendarClickHandler>>,
    when: Option<WhenCondition>,
}

#[bon]
impl<WidgetId> Calendar<WidgetId> {
    /// Build a calendar widget.
    ///
    /// Use `config(...)` for static range/layout settings, `config_getter(...)`
    /// for per-render overrides, `appearance(...)` for label customization,
    /// `views(...)` for full scope replacement, and `on_click(...)` to handle
    /// selected dates.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(field)] config_getter: Option<Arc<CalendarConfigGetter>>,
        #[builder(field)] on_click: Option<Arc<CalendarClickHandler>>,
        #[builder(default)] config: CalendarConfig,
        #[builder(default)] appearance: CalendarAppearance,
        #[builder(default)] views: CalendarViews,
        when: Option<WhenCondition>,
    ) -> Self
    where
        WidgetId: Display,
    {
        Self {
            id,
            config,
            config_getter,
            appearance,
            views,
            on_click,
            when,
        }
    }
}

impl<S, WidgetId> CalendarBuilder<WidgetId, S>
where
    S: calendar_builder::State,
    WidgetId: Display,
{
    /// Provide async per-render configuration overrides.
    pub fn config_getter<F, Fut>(mut self, config_getter: F) -> Self
    where
        F: Fn(WhenContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CalendarUserConfig> + Send + 'static,
    {
        let config_getter = Arc::new(config_getter);
        self.config_getter = Some(Arc::new(move |when_ctx| {
            let config_getter = config_getter.clone();
            Box::pin(async move { config_getter(when_ctx).await })
        }));
        self
    }

    /// Handle selected dates asynchronously.
    pub fn on_click<F, Fut>(mut self, on_click: F) -> Self
    where
        F: Fn(ClickContext, CalendarDate) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ButtonAction> + Send + 'static,
    {
        let on_click = Arc::new(on_click);
        self.on_click = Some(Arc::new(move |click, selected_date| {
            let on_click = on_click.clone();
            Box::pin(async move { on_click(click, selected_date).await })
        }));
        self
    }
}

impl<WidgetId> Calendar<WidgetId>
where
    WidgetId: Display,
{
    fn widget_id(&self) -> String {
        self.id.to_string()
    }

    async fn config_for(&self, ctx: &Context, data: &DataMap) -> CalendarConfig {
        match &self.config_getter {
            Some(getter) => self
                .config
                .merge_user_config(&getter(WhenContext::new(ctx, data)).await),
            None => self.config.clone(),
        }
    }

    fn read_state(&self, ctx: &Context, config: &CalendarConfig) -> CalendarState {
        ctx.widget_value(&self.widget_id())
            .and_then(CalendarState::from_value)
            .unwrap_or_else(|| CalendarState::new(CalendarScope::Days, config.today()))
    }

    fn state_action(&self, state: CalendarState) -> ButtonAction {
        ButtonAction::set_widget_value(self.widget_id(), state.to_value())
    }

    async fn button(
        &self,
        render_ctx: &RenderContext,
        kind: CalendarButtonKind,
        payload: &str,
    ) -> InlineKeyboardButton {
        let text = (self.appearance.text_renderer)(kind, render_ctx.clone()).await;
        InlineKeyboardButton::new(text).callback_data(format_callback_data(
            render_ctx.context.as_ref(),
            &self.id,
            Some(payload),
        ))
    }

    async fn empty_button(&self, render_ctx: &RenderContext) -> InlineKeyboardButton {
        self.button(render_ctx, CalendarButtonKind::Empty, CALLBACK_NOOP)
            .await
    }

    async fn render_days(
        &self,
        render_ctx: &RenderContext,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Vec<Box<[InlineKeyboardButton]>> {
        let month = month_begin(offset);
        let today = config.today();
        let mut rows = vec![
            [self
                .button(
                    render_ctx,
                    CalendarButtonKind::DaysHeader { month },
                    CALLBACK_SCOPE_MONTHS,
                )
                .await]
            .into(),
            self.render_week_header(render_ctx, config)
                .await
                .into_boxed_slice(),
        ];
        let start = calendar_start(month, config.first_weekday);
        let end = calendar_end(month, config.first_weekday);
        let weeks = ((end - start).whole_days() / 7) + 1;

        for week in 0..weeks {
            let mut row = Vec::new();
            for weekday in 0..7 {
                let current_date = start + Duration::days(week * 7 + weekday);
                if current_date.month() != month.month()
                    || current_date < config.min_date
                    || current_date > config.max_date
                {
                    row.push(self.empty_button(render_ctx).await);
                } else {
                    row.push(
                        self.button(
                            render_ctx,
                            CalendarButtonKind::Day {
                                date: current_date,
                                is_today: current_date == today,
                            },
                            &format!("{CALLBACK_PREFIX_DATE}{current_date}"),
                        )
                        .await,
                    );
                }
            }
            rows.push(row.into_boxed_slice());
        }

        if let Some(pager) = self.render_month_pager(render_ctx, config, month).await {
            rows.push(pager.into_boxed_slice());
        }
        rows
    }

    async fn render_week_header(
        &self,
        render_ctx: &RenderContext,
        config: &CalendarConfig,
    ) -> Vec<InlineKeyboardButton> {
        let mut row = Vec::new();
        let first = config.first_weekday.number_days_from_monday();
        for offset in 0..7 {
            let weekday = weekday_from_monday_offset((first + offset) % 7);
            row.push(
                self.button(
                    render_ctx,
                    CalendarButtonKind::Weekday { weekday },
                    CALLBACK_NOOP,
                )
                .await,
            );
        }
        row
    }

    async fn render_month_pager(
        &self,
        render_ctx: &RenderContext,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Option<Vec<InlineKeyboardButton>> {
        let prev =
            prev_month_begin(offset).filter(|prev| last_day_of_month(*prev) >= config.min_date);
        let next = next_month_begin(offset).filter(|next| *next <= config.max_date);
        if prev.is_none() && next.is_none() {
            return None;
        }

        Some(vec![
            if let Some(prev) = prev {
                self.button(
                    render_ctx,
                    CalendarButtonKind::DaysPrevMonth { month: prev },
                    CALLBACK_PREV_MONTH,
                )
                .await
            } else {
                self.empty_button(render_ctx).await
            },
            self.button(
                render_ctx,
                CalendarButtonKind::DaysZoom { month: offset },
                CALLBACK_SCOPE_MONTHS,
            )
            .await,
            if let Some(next) = next {
                self.button(
                    render_ctx,
                    CalendarButtonKind::DaysNextMonth { month: next },
                    CALLBACK_NEXT_MONTH,
                )
                .await
            } else {
                self.empty_button(render_ctx).await
            },
        ])
    }

    async fn render_months(
        &self,
        render_ctx: &RenderContext,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Vec<Box<[InlineKeyboardButton]>> {
        let today = config.today();
        let mut rows = vec![[self
            .button(
                render_ctx,
                CalendarButtonKind::MonthsHeader {
                    year: offset.year(),
                },
                CALLBACK_SCOPE_YEARS,
            )
            .await]
        .into()];

        let month_columns = config.month_columns;
        for month_start in (1..=12).step_by(month_columns) {
            let mut row = Vec::new();
            for month in month_start..month_start + month_columns {
                if month > 12 {
                    break;
                }
                let month_date = date(offset.year(), u8::try_from(month).unwrap_or(12), 1);
                if last_day_of_month(month_date) < config.min_date || month_date > config.max_date {
                    row.push(self.empty_button(render_ctx).await);
                } else {
                    row.push(
                        self.button(
                            render_ctx,
                            CalendarButtonKind::Month {
                                month: month_date,
                                is_current: month_date.year() == today.year()
                                    && month_date.month() == today.month(),
                            },
                            &format!("{CALLBACK_PREFIX_MONTH}{month}"),
                        )
                        .await,
                    );
                }
            }
            rows.push(row.into_boxed_slice());
        }

        if let Some(pager) = self.render_year_pager(render_ctx, config, offset).await {
            rows.push(pager.into_boxed_slice());
        }
        rows
    }

    async fn render_year_pager(
        &self,
        render_ctx: &RenderContext,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Option<Vec<InlineKeyboardButton>> {
        let year = offset.year();
        let can_go_prev = year > config.min_date.year();
        let can_go_next = year < config.max_date.year();
        if !can_go_prev && !can_go_next {
            return None;
        }

        Some(vec![
            if can_go_prev {
                self.button(
                    render_ctx,
                    CalendarButtonKind::MonthsPrevYear { year: year - 1 },
                    CALLBACK_PREV_YEAR,
                )
                .await
            } else {
                self.empty_button(render_ctx).await
            },
            self.button(
                render_ctx,
                CalendarButtonKind::MonthsZoom { year },
                CALLBACK_SCOPE_YEARS,
            )
            .await,
            if can_go_next {
                self.button(
                    render_ctx,
                    CalendarButtonKind::MonthsNextYear { year: year + 1 },
                    CALLBACK_NEXT_YEAR,
                )
                .await
            } else {
                self.empty_button(render_ctx).await
            },
        ])
    }

    async fn render_years(
        &self,
        render_ctx: &RenderContext,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Vec<Box<[InlineKeyboardButton]>> {
        let today_year = config.today().year();
        let years_per_page = config.years_per_page;
        let years_columns = config.years_columns;
        let mut rows = Vec::new();

        for row_start in (0..years_per_page).step_by(years_columns) {
            let mut row = Vec::new();
            for column in 0..years_columns {
                let year = offset.year() + i32::try_from(row_start + column).unwrap_or(i32::MAX);
                if row_start + column >= years_per_page {
                    break;
                }
                if year < config.min_date.year() || year > config.max_date.year() {
                    row.push(self.empty_button(render_ctx).await);
                } else {
                    row.push(
                        self.button(
                            render_ctx,
                            CalendarButtonKind::Year {
                                year,
                                is_current: year == today_year,
                            },
                            &format!("{CALLBACK_PREFIX_YEAR}{year}"),
                        )
                        .await,
                    );
                }
            }
            rows.push(row.into_boxed_slice());
        }

        let years_per_page = i32::try_from(years_per_page).unwrap_or(i32::MAX);
        let prev_year = offset.year() - years_per_page;
        let next_year = offset.year() + years_per_page;
        let can_go_prev = offset.year() > config.min_date.year();
        let can_go_next = next_year <= config.max_date.year();
        if can_go_prev || can_go_next {
            rows.push(
                [
                    if can_go_prev {
                        self.button(
                            render_ctx,
                            CalendarButtonKind::YearsPrevPage { year: prev_year },
                            CALLBACK_PREV_YEARS_PAGE,
                        )
                        .await
                    } else {
                        self.empty_button(render_ctx).await
                    },
                    if can_go_next {
                        self.button(
                            render_ctx,
                            CalendarButtonKind::YearsNextPage { year: next_year },
                            CALLBACK_NEXT_YEARS_PAGE,
                        )
                        .await
                    } else {
                        self.empty_button(render_ctx).await
                    },
                ]
                .into(),
            );
        }
        rows
    }
}

#[async_trait]
impl<WidgetId> Keyboard for Calendar<WidgetId>
where
    WidgetId: Display + Send + Sync + 'static,
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
        let config = self.config_for(ctx, data).await;
        let state = self.read_state(ctx, &config);
        let rows = if let Some(view) = self.views.get(state.current_scope) {
            let widget_id = self.widget_id();
            let view_ctx = CalendarViewContext {
                context: render_ctx.context.clone(),
                data: render_ctx.data.clone(),
                config: config.clone(),
                state,
                widget_id: Cow::Owned(widget_id),
            };
            view(view_ctx).await
        } else {
            match state.current_scope {
                CalendarScope::Days => {
                    self.render_days(render_ctx, &config, state.current_offset)
                        .await
                }
                CalendarScope::Months => {
                    self.render_months(render_ctx, &config, state.current_offset)
                        .await
                }
                CalendarScope::Years => {
                    self.render_years(render_ctx, &config, state.current_offset)
                        .await
                }
            }
        };
        Some(InlineKeyboardMarkup::new(rows).into())
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        let ctx = click.context.as_ref();
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data).await {
            return None;
        }
        let parsed = parse_callback_data(ctx, click.callback_data.as_str())?;
        if parsed.target_id != self.widget_id() {
            return None;
        }
        let payload = parsed.payload?;
        let config = self.config_for(ctx, data).await;
        let mut state = self.read_state(ctx, &config);

        match payload {
            CALLBACK_SCOPE_MONTHS => state.current_scope = CalendarScope::Months,
            CALLBACK_SCOPE_YEARS => state.current_scope = CalendarScope::Years,
            CALLBACK_NOOP => return Some(ButtonAction::noop()),
            // A callback can be stale or crafted, so it may ask to step past the first/last date
            // `Date` can represent. There is nowhere to go, so the offset simply stays put.
            CALLBACK_PREV_MONTH => {
                if let Some(prev) = prev_month_begin(state.current_offset) {
                    state.current_offset = prev;
                }
            }
            CALLBACK_NEXT_MONTH => {
                if let Some(next) = next_month_begin(state.current_offset) {
                    state.current_offset = next;
                }
            }
            CALLBACK_PREV_YEAR => {
                if let Some(shifted) = shift_years(state.current_offset, -1) {
                    state.current_offset = shifted;
                }
            }
            CALLBACK_NEXT_YEAR => {
                if let Some(shifted) = shift_years(state.current_offset, 1) {
                    state.current_offset = shifted;
                }
            }
            CALLBACK_PREV_YEARS_PAGE => {
                if let Some(shifted) = shift_years(
                    state.current_offset,
                    -i32::try_from(config.years_per_page).unwrap_or(i32::MAX),
                ) {
                    state.current_offset = shifted;
                }
            }
            CALLBACK_NEXT_YEARS_PAGE => {
                if let Some(shifted) = shift_years(
                    state.current_offset,
                    i32::try_from(config.years_per_page).unwrap_or(i32::MAX),
                ) {
                    state.current_offset = shifted;
                }
            }
            payload if payload.starts_with(CALLBACK_PREFIX_MONTH) => {
                let month = payload[CALLBACK_PREFIX_MONTH.len()..].parse::<u8>().ok()?;
                let month = Month::try_from(month).ok()?;
                state.current_offset =
                    Date::from_calendar_date(state.current_offset.year(), month, 1).ok()?;
                state.current_scope = CalendarScope::Days;
            }
            payload if payload.starts_with(CALLBACK_PREFIX_YEAR) => {
                let year = payload[CALLBACK_PREFIX_YEAR.len()..].parse::<i32>().ok()?;
                state.current_offset = Date::from_calendar_date(year, Month::January, 1).ok()?;
                state.current_scope = CalendarScope::Months;
            }
            payload if payload.starts_with(CALLBACK_PREFIX_DATE) => {
                let selected_date = parse_date(&payload[CALLBACK_PREFIX_DATE.len()..])?;
                if selected_date < config.min_date || selected_date > config.max_date {
                    return None;
                }
                debug!(
                    context_id = %ctx.id,
                    widget_id = %self.id,
                    selected_date = %selected_date,
                    "Resolved calendar date callback"
                );
                return Some(match &self.on_click {
                    Some(handler) => handler(click.clone(), selected_date).await,
                    None => ButtonAction::noop(),
                });
            }
            _ => return None,
        }

        debug!(
            context_id = %ctx.id,
            widget_id = %self.id,
            scope = ?state.current_scope,
            offset = %state.current_offset,
            "Resolved calendar navigation callback"
        );
        Some(self.state_action(state))
    }
}

fn date(year: i32, month: u8, day: u8) -> CalendarDate {
    Date::from_calendar_date(year, Month::try_from(month).expect("valid month"), day)
        .expect("valid calendar date")
}

fn month_begin(date: CalendarDate) -> CalendarDate {
    Date::from_calendar_date(date.year(), date.month(), 1).expect("valid month begin")
}

/// The first day of the month after `date`, or `None` when there is none because `date` is in the
/// last month [`Date`] can represent.
fn next_month_begin(date: CalendarDate) -> Option<CalendarDate> {
    let date = month_begin(date);
    if date.month() == Month::December {
        Date::from_calendar_date(date.year().checked_add(1)?, Month::January, 1).ok()
    } else {
        Date::from_calendar_date(date.year(), date.month().next(), 1).ok()
    }
}

/// The first day of the month before `date`, or `None` when there is none because `date` is in the
/// first month [`Date`] can represent.
fn prev_month_begin(date: CalendarDate) -> Option<CalendarDate> {
    let date = month_begin(date);
    if date.month() == Month::January {
        Date::from_calendar_date(date.year().checked_sub(1)?, Month::December, 1).ok()
    } else {
        Date::from_calendar_date(date.year(), date.month().previous(), 1).ok()
    }
}

fn last_day_of_month(date: CalendarDate) -> CalendarDate {
    let (year, month) = (date.year(), date.month());

    Date::from_calendar_date(year, month, month.length(year))
        .expect("the last day of an existing month is always a valid date")
}

fn calendar_start(month: CalendarDate, first_weekday: Weekday) -> CalendarDate {
    let days_since_week_start = (month.weekday().number_days_from_monday() + 7
        - first_weekday.number_days_from_monday())
        % 7;
    month - Duration::days(i64::from(days_since_week_start))
}

fn calendar_end(month: CalendarDate, first_weekday: Weekday) -> CalendarDate {
    let end = last_day_of_month(month);
    let days_since_week_start =
        (end.weekday().number_days_from_monday() + 7 - first_weekday.number_days_from_monday()) % 7;
    let days_till_week_end = (6 - days_since_week_start) % 7;
    end + Duration::days(i64::from(days_till_week_end))
}

/// `date` shifted by `years`, or `None` when the result is not representable by [`Date`].
fn shift_years(date: CalendarDate, years: i32) -> Option<CalendarDate> {
    let year = date.year().checked_add(years)?;
    let day = date.day().min(date.month().length(year));

    Date::from_calendar_date(year, date.month(), day).ok()
}

fn default_calendar_text_renderer<'a>(
    kind: CalendarButtonKind,
    render_ctx: RenderContext,
) -> BoxFuture<'a, String> {
    Box::pin(async move { default_calendar_text(kind, &render_ctx) })
}

fn default_calendar_text(kind: CalendarButtonKind, _render_ctx: &RenderContext) -> String {
    match kind {
        CalendarButtonKind::Empty => " ".to_owned(),
        CalendarButtonKind::Weekday { weekday } => weekday_label(weekday).to_owned(),
        CalendarButtonKind::DaysHeader { month } => month_label(month),
        CalendarButtonKind::Day { date, is_today } => {
            if is_today {
                format!("[{:02}]", date.day())
            } else {
                format!("{:02}", date.day())
            }
        }
        CalendarButtonKind::DaysPrevMonth { month } => {
            format!("<< {}", month_year_label(month))
        }
        CalendarButtonKind::DaysZoom { .. } | CalendarButtonKind::MonthsZoom { .. } => {
            "Zoom".to_owned()
        }
        CalendarButtonKind::DaysNextMonth { month } => {
            format!("{} >>", month_year_label(month))
        }
        CalendarButtonKind::MonthsHeader { year } => format!("{year}"),
        CalendarButtonKind::Month { month, is_current } => {
            if is_current {
                format!("[{}]", month.month())
            } else {
                month.month().to_string()
            }
        }
        CalendarButtonKind::MonthsPrevYear { year }
        | CalendarButtonKind::YearsPrevPage { year } => format!("<< {year}"),
        CalendarButtonKind::MonthsNextYear { year }
        | CalendarButtonKind::YearsNextPage { year } => format!("{year} >>"),
        CalendarButtonKind::Year { year, is_current } => {
            if is_current {
                format!("[ {year} ]")
            } else {
                format!("{year}")
            }
        }
    }
}

fn month_label(date: CalendarDate) -> String {
    format!("{} {}", date.month(), date.year())
}

fn month_year_label(date: CalendarDate) -> String {
    format!("{} {}", date.month(), date.year())
}

fn weekday_label(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Mon",
        Weekday::Tuesday => "Tue",
        Weekday::Wednesday => "Wed",
        Weekday::Thursday => "Thu",
        Weekday::Friday => "Fri",
        Weekday::Saturday => "Sat",
        Weekday::Sunday => "Sun",
    }
}

fn weekday_from_monday_offset(offset: u8) -> Weekday {
    match offset {
        0 => Weekday::Monday,
        1 => Weekday::Tuesday,
        2 => Weekday::Wednesday,
        3 => Weekday::Thursday,
        4 => Weekday::Friday,
        5 => Weekday::Saturday,
        _ => Weekday::Sunday,
    }
}

fn parse_date(value: &str) -> Option<CalendarDate> {
    let mut parts = value.splitn(3, '-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u8>().ok()?;
    let day = parts.next()?.parse::<u8>().ok()?;
    Date::from_calendar_date(year, Month::try_from(month).ok()?, day).ok()
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use time::Weekday;

    use super::{
        date, last_day_of_month, next_month_begin, prev_month_begin, shift_years, Calendar,
        CalendarAppearance, CalendarButtonKind, CalendarConfig, CalendarDate, CalendarScope,
        CalendarUserConfig, CalendarViewContext, CalendarViews, WhenContext,
    };
    use crate::{
        entities::{Context, DataMap, RenderContext},
        widgets::{ButtonAction, CalendarState, ClickContext, Keyboard},
    };

    fn test_config() -> CalendarConfig {
        CalendarConfig::builder().today(date(2026, 4, 12)).build()
    }

    #[test]
    fn date_helpers_have_no_neighbour_at_the_representable_boundary() {
        // `time::Date` (without the `large-dates` feature) spans years -9999..=9999, so the first
        // and last months have no neighbour. These used to `.expect(...)` and panic inside the
        // async render/callback path, which a year-9999 `max_date` (accepted by the config) or a
        // stale callback could reach.
        let last = date(9999, 12, 31);
        assert_eq!(next_month_begin(last), None);
        assert_eq!(shift_years(last, 1), None);
        // The year shift itself must not overflow `i32` either.
        assert_eq!(shift_years(last, i32::MAX), None);
        // ...but the last day of that month is still perfectly representable.
        assert_eq!(last_day_of_month(last), date(9999, 12, 31));

        let first = date(-9999, 1, 1);
        assert_eq!(prev_month_begin(first), None);
        assert_eq!(shift_years(first, -1), None);
        assert_eq!(shift_years(first, i32::MIN), None);
        assert_eq!(last_day_of_month(first), date(-9999, 1, 31));
    }

    #[test]
    fn date_helpers_step_normally_away_from_the_boundary() {
        assert_eq!(next_month_begin(date(2026, 4, 5)), Some(date(2026, 5, 1)));
        assert_eq!(prev_month_begin(date(2026, 4, 5)), Some(date(2026, 3, 1)));
        // Across a year boundary.
        assert_eq!(next_month_begin(date(2026, 12, 5)), Some(date(2027, 1, 1)));
        assert_eq!(prev_month_begin(date(2026, 1, 5)), Some(date(2025, 12, 1)));

        // `last_day_of_month` is leap-year aware.
        assert_eq!(last_day_of_month(date(2024, 2, 10)), date(2024, 2, 29));
        assert_eq!(last_day_of_month(date(2025, 2, 10)), date(2025, 2, 28));

        // Shifting off a leap day clamps to the shorter month.
        assert_eq!(shift_years(date(2024, 2, 29), 1), Some(date(2025, 2, 28)));
        assert_eq!(shift_years(date(2026, 4, 12), -1), Some(date(2025, 4, 12)));
    }

    async fn sunday_config(when_ctx: WhenContext) -> CalendarUserConfig {
        if when_ctx
            .data
            .get("starts_on_sunday")
            .and_then(Value::as_bool)
            .unwrap_or_default()
        {
            CalendarUserConfig::builder()
                .first_weekday(Weekday::Sunday)
                .build()
        } else {
            CalendarUserConfig::default()
        }
    }

    async fn custom_text_renderer(kind: CalendarButtonKind, _render_ctx: RenderContext) -> String {
        match kind {
            CalendarButtonKind::Day { date, .. } => format!("D{}", date.day()),
            CalendarButtonKind::Weekday { weekday } => {
                format!("W{}", weekday.number_from_monday())
            }
            _ => "x".to_owned(),
        }
    }

    async fn custom_days_view(
        view_ctx: CalendarViewContext,
    ) -> Vec<Box<[telers::types::InlineKeyboardButton]>> {
        vec![[view_ctx.button("Use today", "DATE2026-04-12")].into()]
    }

    async fn store_selected_date(
        _click: ClickContext,
        selected_date: CalendarDate,
    ) -> ButtonAction {
        ButtonAction::set_dialog_value("selected_date", selected_date.to_string())
    }

    #[tokio::test]
    async fn calendar_renders_days_by_default() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[0][0].text, "April 2026");
        assert_eq!(&*rows[1][0].text, "Mon");
        assert_eq!(&*rows[1][6].text, "Sun");
        assert!(rows
            .iter()
            .flatten()
            .any(|button| button.text.as_ref() == "[12]"));
    }

    #[tokio::test]
    async fn calendar_header_and_filler_buttons_use_noop_callbacks() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");
        let weekday = &rows[1][0];
        let filler = rows
            .iter()
            .flatten()
            .find(|button| button.text.as_ref() == " ")
            .expect("filler button");
        let noop_callback = format!("td:{}:calendar:noop", ctx.id);

        assert_eq!(
            weekday.callback_data.as_deref(),
            Some(noop_callback.as_str())
        );
        assert_eq!(
            filler.callback_data.as_deref(),
            Some(noop_callback.as_str())
        );
        assert!(matches!(
            calendar
                .handle_callback_for_test(&ctx, &noop_callback)
                .await,
            Some(ButtonAction::Noop)
        ));
    }

    #[tokio::test]
    async fn calendar_supports_dynamic_user_config() {
        let ctx = Context::new("", "state", Value::Null);
        let mut data = DataMap::new();
        data.insert("starts_on_sunday".into(), json!(true));
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .config_getter(sunday_config)
            .build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &data)
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[1][0].text, "Sun");
        assert_eq!(&*rows[1][1].text, "Mon");
    }

    #[tokio::test]
    async fn calendar_supports_custom_text_renderer() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .appearance(
                CalendarAppearance::builder()
                    .text_renderer(custom_text_renderer)
                    .build(),
            )
            .build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");
        let today_button = rows
            .iter()
            .flatten()
            .find(|button| button.text.as_ref() == "D12")
            .expect("today button");

        assert_eq!(&*rows[1][0].text, "W1");
        assert_eq!(&*today_button.text, "D12");
    }

    #[tokio::test]
    async fn calendar_supports_custom_scope_views() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .views(CalendarViews::builder().days(custom_days_view).build())
            .build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");
        let expected_callback = format!("td:{}:calendar:DATE2026-04-12", ctx.id);

        assert_eq!(rows.len(), 1);
        assert_eq!(&*rows[0][0].text, "Use today");
        assert_eq!(
            rows[0][0].callback_data.as_deref(),
            Some(expected_callback.as_str())
        );
    }

    #[tokio::test]
    async fn calendar_hides_pager_row_when_navigation_is_unavailable() {
        let ctx = Context::new("", "state", Value::Null);
        let config = CalendarConfig::builder()
            .today(date(2026, 4, 12))
            .min_date(date(2026, 4, 12))
            .max_date(date(2026, 4, 12))
            .build();
        let calendar = Calendar::builder("calendar").config(config).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert!(!rows
            .iter()
            .flatten()
            .any(|button| button.text.as_ref() == "Zoom"));
    }

    #[tokio::test]
    async fn calendar_can_start_week_from_sunday() {
        let ctx = Context::new("", "state", Value::Null);
        let config = CalendarConfig::builder()
            .today(date(2026, 4, 12))
            .first_weekday(Weekday::Sunday)
            .build();
        let calendar = Calendar::builder("calendar").config(config).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[1][0].text, "Sun");
        assert_eq!(&*rows[1][1].text, "Mon");
    }

    #[tokio::test]
    async fn calendar_renders_month_scope_from_widget_data() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert(
            "calendar".into(),
            CalendarState::new(CalendarScope::Months, date(2026, 4, 1)).to_value(),
        );
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[0][0].text, "2026");
        assert_eq!(&*rows[1][0].text, "January");
        assert_eq!(&*rows[2][0].text, "[April]");
    }

    #[tokio::test]
    async fn calendar_renders_year_scope_from_widget_data() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert(
            "calendar".into(),
            CalendarState::new(CalendarScope::Years, date(2020, 1, 1)).to_value(),
        );
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[0][0].text, "2020");
        assert_eq!(&*rows[1][0].text, "2025");
        assert_eq!(&*rows[1][1].text, "[ 2026 ]");
    }

    #[tokio::test]
    async fn calendar_navigation_callback_updates_scope() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:M", ctx.id))
            .await
            .expect("calendar action");

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "calendar"
                    && value["current_scope"] == json!("MONTHS")
                    && value["current_offset"] == json!("2026-04-12")
        ));
    }

    #[tokio::test]
    async fn calendar_month_callback_returns_to_days_scope() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert(
            "calendar".into(),
            CalendarState::new(CalendarScope::Months, date(2026, 4, 1)).to_value(),
        );
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:MONTH5", ctx.id))
            .await
            .expect("calendar action");

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "calendar"
                    && value["current_scope"] == json!("DAYS")
                    && value["current_offset"] == json!("2026-05-01")
        ));
    }

    #[tokio::test]
    async fn calendar_date_callback_uses_on_click_handler() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .on_click(store_selected_date)
            .build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:DATE2026-04-13", ctx.id))
            .await
            .expect("calendar action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "selected_date" && value == "2026-04-13"
        ));
    }

    #[tokio::test]
    async fn calendar_without_on_click_consumes_date_callback() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:DATE2026-04-13", ctx.id))
            .await
            .expect("calendar action");

        assert!(matches!(action, ButtonAction::Noop));
    }
}
