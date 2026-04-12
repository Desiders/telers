use bon::bon;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fmt::Display, sync::Arc};
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use time::{Date, Duration, Month, OffsetDateTime, UtcOffset, Weekday};
use tracing::debug;

use super::{
    format_callback_data, parse_callback_data, when::is_allowed, ButtonAction, ClickContext,
    Keyboard, WhenCondition,
};
use crate::entities::{Context, DataMap, RenderContext};

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
    dyn for<'a> Fn(&ClickContext<'a>, CalendarDate) -> ButtonAction + Send + Sync + 'static;
type CalendarConfigGetter =
    dyn Fn(&Context, &DataMap) -> CalendarUserConfig + Send + Sync + 'static;
type CalendarTextRenderer =
    dyn for<'a> Fn(CalendarButtonKind, &RenderContext<'a>) -> String + Send + Sync + 'static;
type CalendarScopeView = dyn for<'a> Fn(&CalendarViewContext<'a>) -> Vec<Box<[InlineKeyboardButton]>>
    + Send
    + Sync
    + 'static;

/// Date type used by [`Calendar`] callbacks and configuration.
pub type CalendarDate = Date;

/// Button role rendered by the default calendar views.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarButtonKind {
    Empty,
    Weekday {
        weekday: Weekday,
    },
    DaysHeader {
        month: CalendarDate,
    },
    Day {
        date: CalendarDate,
        is_today: bool,
    },
    DaysPrevMonth {
        month: CalendarDate,
    },
    DaysZoom {
        month: CalendarDate,
    },
    DaysNextMonth {
        month: CalendarDate,
    },
    MonthsHeader {
        year: i32,
    },
    Month {
        month: CalendarDate,
        is_current: bool,
    },
    MonthsPrevYear {
        year: i32,
    },
    MonthsZoom {
        year: i32,
    },
    MonthsNextYear {
        year: i32,
    },
    Year {
        year: i32,
        is_current: bool,
    },
    YearsPrevPage {
        year: i32,
    },
    YearsNextPage {
        year: i32,
    },
}

/// Text renderer used by the built-in calendar views.
#[derive(Clone)]
pub struct CalendarAppearance {
    text_renderer: Arc<CalendarTextRenderer>,
}

impl Default for CalendarAppearance {
    fn default() -> Self {
        Self {
            text_renderer: Arc::new(default_calendar_text),
        }
    }
}

#[bon]
impl CalendarAppearance {
    /// Create calendar appearance hooks.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(
            default = Arc::new(default_calendar_text),
            with = |text_renderer: impl for<'a> Fn(CalendarButtonKind, &RenderContext<'a>) -> String + Send + Sync + 'static| {
                Arc::new(text_renderer)
            }
        )]
        text_renderer: Arc<CalendarTextRenderer>,
    ) -> Self {
        Self {
            text_renderer,
        }
    }
}

/// Context passed into custom calendar scope renderers.
pub struct CalendarViewContext<'a> {
    pub context: &'a Context,
    pub data: &'a DataMap,
    pub config: &'a CalendarConfig,
    pub state: CalendarState,
    pub widget_id: &'a str,
}

impl CalendarViewContext<'_> {
    #[must_use]
    pub fn button(&self, text: impl Into<Box<str>>, payload: &str) -> InlineKeyboardButton {
        InlineKeyboardButton::new(text).callback_data(format_callback_data(
            self.context,
            self.widget_id,
            Some(payload),
        ))
    }

    #[must_use]
    pub fn noop_button(&self, text: impl Into<Box<str>>) -> InlineKeyboardButton {
        self.button(text, CALLBACK_NOOP)
    }
}

/// Optional custom renderers for complete calendar scopes.
#[derive(Clone, Default)]
pub struct CalendarViews {
    days: Option<Arc<CalendarScopeView>>,
    months: Option<Arc<CalendarScopeView>>,
    years: Option<Arc<CalendarScopeView>>,
}

#[bon]
impl CalendarViews {
    /// Create custom calendar scope renderers.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(with = |days: impl for<'a> Fn(&CalendarViewContext<'a>) -> Vec<Box<[InlineKeyboardButton]>> + Send + Sync + 'static| {
            Arc::new(days)
        })]
        days: Option<Arc<CalendarScopeView>>,
        #[builder(with = |months: impl for<'a> Fn(&CalendarViewContext<'a>) -> Vec<Box<[InlineKeyboardButton]>> + Send + Sync + 'static| {
            Arc::new(months)
        })]
        months: Option<Arc<CalendarScopeView>>,
        #[builder(with = |years: impl for<'a> Fn(&CalendarViewContext<'a>) -> Vec<Box<[InlineKeyboardButton]>> + Send + Sync + 'static| {
            Arc::new(years)
        })]
        years: Option<Arc<CalendarScopeView>>,
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

/// Calendar view currently rendered by [`Calendar`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarScope {
    Days,
    Months,
    Years,
}

/// Rendering and range settings for [`Calendar`].
#[derive(Clone, Debug)]
pub struct CalendarConfig {
    pub first_weekday: Weekday,
    pub timezone: UtcOffset,
    pub min_date: CalendarDate,
    pub max_date: CalendarDate,
    pub month_columns: usize,
    pub years_per_page: usize,
    pub years_columns: usize,
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

    fn merge_user_config(&self, user_config: CalendarUserConfig) -> Self {
        let min_date = user_config.min_date.unwrap_or(self.min_date);
        let max_date = user_config.max_date.unwrap_or(self.max_date);
        let (min_date, max_date) = if min_date <= max_date {
            (min_date, max_date)
        } else {
            (max_date, min_date)
        };
        Self {
            first_weekday: user_config.first_weekday.unwrap_or(self.first_weekday),
            timezone: user_config.timezone.unwrap_or(self.timezone),
            min_date,
            max_date,
            month_columns: user_config
                .month_columns
                .unwrap_or(self.month_columns)
                .max(1),
            years_per_page: user_config
                .years_per_page
                .unwrap_or(self.years_per_page)
                .max(1),
            years_columns: user_config
                .years_columns
                .unwrap_or(self.years_columns)
                .max(1),
            today: user_config.today.or(self.today),
        }
    }
}

/// Per-render calendar config overrides.
#[derive(Clone, Debug, Default)]
pub struct CalendarUserConfig {
    pub first_weekday: Option<Weekday>,
    pub timezone: Option<UtcOffset>,
    pub min_date: Option<CalendarDate>,
    pub max_date: Option<CalendarDate>,
    pub month_columns: Option<usize>,
    pub years_per_page: Option<usize>,
    pub years_columns: Option<usize>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarState {
    pub current_scope: CalendarScope,
    pub current_offset: CalendarDate,
}

impl CalendarState {
    #[must_use]
    pub const fn new(current_scope: CalendarScope, current_offset: CalendarDate) -> Self {
        Self {
            current_scope,
            current_offset,
        }
    }

    #[must_use]
    pub fn to_value(&self) -> Value {
        json!({
            "current_scope": self.current_scope,
            "current_offset": self.current_offset.to_string(),
        })
    }

    #[must_use]
    pub fn from_value(value: &Value) -> Option<Self> {
        let scope = serde_json::from_value(value.get("current_scope")?.clone()).ok()?;
        let offset = parse_date(value.get("current_offset")?.as_str()?)?;
        Some(Self::new(scope, offset))
    }
}

/// Date selection keyboard with days, months, and years views.
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
    /// Create a calendar widget.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(default)] config: CalendarConfig,
        #[builder(with = |config_getter: impl Fn(&Context, &DataMap) -> CalendarUserConfig + Send + Sync + 'static| {
            Arc::new(config_getter)
        })]
        config_getter: Option<Arc<CalendarConfigGetter>>,
        #[builder(default)] appearance: CalendarAppearance,
        #[builder(default)] views: CalendarViews,
        #[builder(with = |on_click: impl for<'a> Fn(&ClickContext<'a>, CalendarDate) -> ButtonAction + Send + Sync + 'static| {
            Arc::new(on_click)
        })]
        on_click: Option<Arc<CalendarClickHandler>>,
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

impl<WidgetId> Calendar<WidgetId>
where
    WidgetId: Display,
{
    fn widget_id(&self) -> String {
        self.id.to_string()
    }

    fn config_for(&self, ctx: &Context, data: &DataMap) -> CalendarConfig {
        self.config_getter.as_ref().map_or_else(
            || self.config.clone(),
            |getter| self.config.merge_user_config(getter(ctx, data)),
        )
    }

    fn read_state(&self, ctx: &Context, config: &CalendarConfig) -> CalendarState {
        ctx.widget_value(&self.widget_id())
            .and_then(CalendarState::from_value)
            .unwrap_or_else(|| CalendarState::new(CalendarScope::Days, config.today()))
    }

    fn state_action(&self, state: CalendarState) -> ButtonAction {
        ButtonAction::set_widget_value(self.widget_id(), state.to_value())
    }

    fn button(
        &self,
        render_ctx: &RenderContext<'_>,
        kind: CalendarButtonKind,
        payload: &str,
    ) -> InlineKeyboardButton {
        let text = (self.appearance.text_renderer)(kind, render_ctx);
        InlineKeyboardButton::new(text).callback_data(format_callback_data(
            render_ctx.context,
            &self.id,
            Some(payload),
        ))
    }

    fn empty_button(&self, render_ctx: &RenderContext<'_>) -> InlineKeyboardButton {
        self.button(render_ctx, CalendarButtonKind::Empty, CALLBACK_NOOP)
    }

    fn render_days(
        &self,
        render_ctx: &RenderContext<'_>,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Vec<Box<[InlineKeyboardButton]>> {
        let month = month_begin(offset);
        let today = config.today();
        let mut rows = vec![
            [self.button(
                render_ctx,
                CalendarButtonKind::DaysHeader {
                    month,
                },
                CALLBACK_SCOPE_MONTHS,
            )]
            .into(),
            self.render_week_header(render_ctx, config)
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
                    row.push(self.empty_button(render_ctx));
                } else {
                    row.push(self.button(
                        render_ctx,
                        CalendarButtonKind::Day {
                            date: current_date,
                            is_today: current_date == today,
                        },
                        &format!("{CALLBACK_PREFIX_DATE}{current_date}"),
                    ));
                }
            }
            rows.push(row.into_boxed_slice());
        }

        if let Some(pager) = self.render_month_pager(render_ctx, config, month) {
            rows.push(pager.into_boxed_slice());
        }
        rows
    }

    fn render_week_header(
        &self,
        render_ctx: &RenderContext<'_>,
        config: &CalendarConfig,
    ) -> Vec<InlineKeyboardButton> {
        let mut row = Vec::new();
        let first = config.first_weekday.number_days_from_monday();
        for offset in 0..7 {
            let weekday = weekday_from_monday_offset((first + offset) % 7);
            row.push(self.button(
                render_ctx,
                CalendarButtonKind::Weekday {
                    weekday,
                },
                CALLBACK_NOOP,
            ));
        }
        row
    }

    fn render_month_pager(
        &self,
        render_ctx: &RenderContext<'_>,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Option<Vec<InlineKeyboardButton>> {
        let prev = prev_month_begin(offset);
        let next = next_month_begin(offset);
        let can_go_prev = last_day_of_month(prev) >= config.min_date;
        let can_go_next = next <= config.max_date;
        if !can_go_prev && !can_go_next {
            return None;
        }

        Some(vec![
            if can_go_prev {
                self.button(
                    render_ctx,
                    CalendarButtonKind::DaysPrevMonth {
                        month: prev,
                    },
                    CALLBACK_PREV_MONTH,
                )
            } else {
                self.empty_button(render_ctx)
            },
            self.button(
                render_ctx,
                CalendarButtonKind::DaysZoom {
                    month: offset,
                },
                CALLBACK_SCOPE_MONTHS,
            ),
            if can_go_next {
                self.button(
                    render_ctx,
                    CalendarButtonKind::DaysNextMonth {
                        month: next,
                    },
                    CALLBACK_NEXT_MONTH,
                )
            } else {
                self.empty_button(render_ctx)
            },
        ])
    }

    fn render_months(
        &self,
        render_ctx: &RenderContext<'_>,
        config: &CalendarConfig,
        offset: CalendarDate,
    ) -> Vec<Box<[InlineKeyboardButton]>> {
        let today = config.today();
        let mut rows = vec![[self.button(
            render_ctx,
            CalendarButtonKind::MonthsHeader {
                year: offset.year(),
            },
            CALLBACK_SCOPE_YEARS,
        )]
        .into()];

        let month_columns = config.month_columns;
        for month_start in (1..=12).step_by(month_columns) {
            let mut row = Vec::new();
            for month in month_start..month_start + month_columns {
                if month > 12 {
                    break;
                }
                let month_date = date(offset.year(), month as u8, 1);
                if last_day_of_month(month_date) < config.min_date || month_date > config.max_date {
                    row.push(self.empty_button(render_ctx));
                } else {
                    row.push(self.button(
                        render_ctx,
                        CalendarButtonKind::Month {
                            month: month_date,
                            is_current: month_date.year() == today.year()
                                && month_date.month() == today.month(),
                        },
                        &format!("{CALLBACK_PREFIX_MONTH}{month}"),
                    ));
                }
            }
            rows.push(row.into_boxed_slice());
        }

        if let Some(pager) = self.render_year_pager(render_ctx, config, offset) {
            rows.push(pager.into_boxed_slice());
        }
        rows
    }

    fn render_year_pager(
        &self,
        render_ctx: &RenderContext<'_>,
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
                    CalendarButtonKind::MonthsPrevYear {
                        year: year - 1,
                    },
                    CALLBACK_PREV_YEAR,
                )
            } else {
                self.empty_button(render_ctx)
            },
            self.button(
                render_ctx,
                CalendarButtonKind::MonthsZoom {
                    year,
                },
                CALLBACK_SCOPE_YEARS,
            ),
            if can_go_next {
                self.button(
                    render_ctx,
                    CalendarButtonKind::MonthsNextYear {
                        year: year + 1,
                    },
                    CALLBACK_NEXT_YEAR,
                )
            } else {
                self.empty_button(render_ctx)
            },
        ])
    }

    fn render_years(
        &self,
        render_ctx: &RenderContext<'_>,
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
                let year = offset.year() + (row_start + column) as i32;
                if row_start + column >= years_per_page {
                    break;
                }
                if year < config.min_date.year() || year > config.max_date.year() {
                    row.push(self.empty_button(render_ctx));
                } else {
                    row.push(self.button(
                        render_ctx,
                        CalendarButtonKind::Year {
                            year,
                            is_current: year == today_year,
                        },
                        &format!("{CALLBACK_PREFIX_YEAR}{year}"),
                    ));
                }
            }
            rows.push(row.into_boxed_slice());
        }

        let prev_year = offset.year() - years_per_page as i32;
        let next_year = offset.year() + years_per_page as i32;
        let can_go_prev = offset.year() > config.min_date.year();
        let can_go_next = next_year <= config.max_date.year();
        if can_go_prev || can_go_next {
            rows.push(
                [
                    if can_go_prev {
                        self.button(
                            render_ctx,
                            CalendarButtonKind::YearsPrevPage {
                                year: prev_year,
                            },
                            CALLBACK_PREV_YEARS_PAGE,
                        )
                    } else {
                        self.empty_button(render_ctx)
                    },
                    if can_go_next {
                        self.button(
                            render_ctx,
                            CalendarButtonKind::YearsNextPage {
                                year: next_year,
                            },
                            CALLBACK_NEXT_YEARS_PAGE,
                        )
                    } else {
                        self.empty_button(render_ctx)
                    },
                ]
                .into(),
            );
        }
        rows
    }
}

impl<WidgetId> Keyboard for Calendar<WidgetId>
where
    WidgetId: Display + Send + Sync + 'static,
{
    fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data)
    }

    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        let ctx = render_ctx.context;
        let data = render_ctx.data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        let config = self.config_for(ctx, data);
        let state = self.read_state(ctx, &config);
        let rows = if let Some(view) = self.views.get(state.current_scope) {
            let widget_id = self.widget_id();
            let view_ctx = CalendarViewContext {
                context: ctx,
                data,
                config: &config,
                state,
                widget_id: &widget_id,
            };
            view(&view_ctx)
        } else {
            match state.current_scope {
                CalendarScope::Days => self.render_days(render_ctx, &config, state.current_offset),
                CalendarScope::Months => {
                    self.render_months(render_ctx, &config, state.current_offset)
                }
                CalendarScope::Years => {
                    self.render_years(render_ctx, &config, state.current_offset)
                }
            }
        };
        Some(InlineKeyboardMarkup::new(rows).into())
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        let ctx = click.context;
        let data = &ctx.dialog_data;
        if !self.is_visible(ctx, data) {
            return None;
        }
        let parsed = parse_callback_data(ctx, click.callback_data)?;
        if parsed.target_id != self.widget_id() {
            return None;
        }
        let payload = parsed.payload?;
        let config = self.config_for(ctx, data);
        let mut state = self.read_state(ctx, &config);

        match payload {
            CALLBACK_SCOPE_MONTHS => state.current_scope = CalendarScope::Months,
            CALLBACK_SCOPE_YEARS => state.current_scope = CalendarScope::Years,
            CALLBACK_NOOP => return Some(ButtonAction::noop()),
            CALLBACK_PREV_MONTH => state.current_offset = prev_month_begin(state.current_offset),
            CALLBACK_NEXT_MONTH => state.current_offset = next_month_begin(state.current_offset),
            CALLBACK_PREV_YEAR => state.current_offset = shift_years(state.current_offset, -1),
            CALLBACK_NEXT_YEAR => state.current_offset = shift_years(state.current_offset, 1),
            CALLBACK_PREV_YEARS_PAGE => {
                state.current_offset =
                    shift_years(state.current_offset, -(config.years_per_page as i32));
            }
            CALLBACK_NEXT_YEARS_PAGE => {
                state.current_offset =
                    shift_years(state.current_offset, config.years_per_page as i32);
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
                return Some(
                    self.on_click
                        .as_ref()
                        .map_or_else(ButtonAction::noop, |handler| handler(click, selected_date)),
                );
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

fn next_month_begin(date: CalendarDate) -> CalendarDate {
    let date = month_begin(date);
    if date.month() == Month::December {
        Date::from_calendar_date(date.year() + 1, Month::January, 1).expect("valid next month")
    } else {
        Date::from_calendar_date(date.year(), date.month().next(), 1).expect("valid next month")
    }
}

fn prev_month_begin(date: CalendarDate) -> CalendarDate {
    let date = month_begin(date);
    if date.month() == Month::January {
        Date::from_calendar_date(date.year() - 1, Month::December, 1).expect("valid previous month")
    } else {
        Date::from_calendar_date(date.year(), date.month().previous(), 1)
            .expect("valid previous month")
    }
}

fn last_day_of_month(date: CalendarDate) -> CalendarDate {
    next_month_begin(date) - Duration::days(1)
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

fn shift_years(date: CalendarDate, years: i32) -> CalendarDate {
    let year = date.year() + years;
    let day = date.day().min(date.month().length(year));
    Date::from_calendar_date(year, date.month(), day).expect("valid shifted year")
}

fn default_calendar_text(kind: CalendarButtonKind, _render_ctx: &RenderContext<'_>) -> String {
    match kind {
        CalendarButtonKind::Empty => " ".to_owned(),
        CalendarButtonKind::Weekday {
            weekday,
        } => weekday_label(weekday).to_owned(),
        CalendarButtonKind::DaysHeader {
            month,
        } => month_label(month),
        CalendarButtonKind::Day {
            date,
            is_today,
        } => {
            if is_today {
                format!("[{:02}]", date.day())
            } else {
                format!("{:02}", date.day())
            }
        }
        CalendarButtonKind::DaysPrevMonth {
            month,
        } => {
            format!("<< {}", month_year_label(month))
        }
        CalendarButtonKind::DaysZoom {
            ..
        } => "Zoom".to_owned(),
        CalendarButtonKind::DaysNextMonth {
            month,
        } => {
            format!("{} >>", month_year_label(month))
        }
        CalendarButtonKind::MonthsHeader {
            year,
        } => format!("{year}"),
        CalendarButtonKind::Month {
            month,
            is_current,
        } => {
            if is_current {
                format!("[{}]", month.month())
            } else {
                month.month().to_string()
            }
        }
        CalendarButtonKind::MonthsPrevYear {
            year,
        } => format!("<< {year}"),
        CalendarButtonKind::MonthsZoom {
            ..
        } => "Zoom".to_owned(),
        CalendarButtonKind::MonthsNextYear {
            year,
        } => format!("{year} >>"),
        CalendarButtonKind::Year {
            year,
            is_current,
        } => {
            if is_current {
                format!("[ {year} ]")
            } else {
                format!("{year}")
            }
        }
        CalendarButtonKind::YearsPrevPage {
            year,
        } => format!("<< {year}"),
        CalendarButtonKind::YearsNextPage {
            year,
        } => format!("{year} >>"),
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
        date, Calendar, CalendarAppearance, CalendarButtonKind, CalendarConfig, CalendarScope,
        CalendarUserConfig, CalendarViews,
    };
    use crate::{
        entities::{Context, DataMap},
        widgets::{ButtonAction, CalendarState, Keyboard},
    };

    fn test_config() -> CalendarConfig {
        CalendarConfig::builder().today(date(2026, 4, 12)).build()
    }

    #[test]
    fn calendar_renders_days_by_default() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
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

    #[test]
    fn calendar_header_and_filler_buttons_use_noop_callbacks() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
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
            calendar.handle_callback_for_test(&ctx, &noop_callback),
            Some(ButtonAction::Noop)
        ));
    }

    #[test]
    fn calendar_supports_dynamic_user_config() {
        let ctx = Context::new("", "state", Value::Null);
        let mut data = DataMap::new();
        data.insert("starts_on_sunday".into(), json!(true));
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .config_getter(|_ctx, data| {
                if data
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
            })
            .build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &data)
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[1][0].text, "Sun");
        assert_eq!(&*rows[1][1].text, "Mon");
    }

    #[test]
    fn calendar_supports_custom_text_renderer() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .appearance(
                CalendarAppearance::builder()
                    .text_renderer(|kind, _render_ctx| match kind {
                        CalendarButtonKind::Day {
                            date, ..
                        } => format!("D{}", date.day()),
                        CalendarButtonKind::Weekday {
                            weekday,
                        } => {
                            format!("W{}", weekday.number_from_monday())
                        }
                        _ => "x".to_owned(),
                    })
                    .build(),
            )
            .build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
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

    #[test]
    fn calendar_supports_custom_scope_views() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .views(
                CalendarViews::builder()
                    .days(|view_ctx| vec![[view_ctx.button("Use today", "DATE2026-04-12")].into()])
                    .build(),
            )
            .build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
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

    #[test]
    fn calendar_hides_pager_row_when_navigation_is_unavailable() {
        let ctx = Context::new("", "state", Value::Null);
        let config = CalendarConfig::builder()
            .today(date(2026, 4, 12))
            .min_date(date(2026, 4, 12))
            .max_date(date(2026, 4, 12))
            .build();
        let calendar = Calendar::builder("calendar").config(config).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert!(!rows
            .iter()
            .flatten()
            .any(|button| button.text.as_ref() == "Zoom"));
    }

    #[test]
    fn calendar_can_start_week_from_sunday() {
        let ctx = Context::new("", "state", Value::Null);
        let config = CalendarConfig::builder()
            .today(date(2026, 4, 12))
            .first_weekday(Weekday::Sunday)
            .build();
        let calendar = Calendar::builder("calendar").config(config).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[1][0].text, "Sun");
        assert_eq!(&*rows[1][1].text, "Mon");
    }

    #[test]
    fn calendar_renders_month_scope_from_widget_data() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert(
            "calendar".into(),
            CalendarState::new(CalendarScope::Months, date(2026, 4, 1)).to_value(),
        );
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[0][0].text, "2026");
        assert_eq!(&*rows[1][0].text, "January");
        assert_eq!(&*rows[2][0].text, "[April]");
    }

    #[test]
    fn calendar_renders_year_scope_from_widget_data() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert(
            "calendar".into(),
            CalendarState::new(CalendarScope::Years, date(2020, 1, 1)).to_value(),
        );
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let markup = calendar
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .expect("calendar markup");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(&*rows[0][0].text, "2020");
        assert_eq!(&*rows[1][0].text, "2025");
        assert_eq!(&*rows[1][1].text, "[ 2026 ]");
    }

    #[test]
    fn calendar_navigation_callback_updates_scope() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:M", ctx.id))
            .expect("calendar action");

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "calendar"
                    && value["current_scope"] == json!("MONTHS")
                    && value["current_offset"] == json!("2026-04-12")
        ));
    }

    #[test]
    fn calendar_month_callback_returns_to_days_scope() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert(
            "calendar".into(),
            CalendarState::new(CalendarScope::Months, date(2026, 4, 1)).to_value(),
        );
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:MONTH5", ctx.id))
            .expect("calendar action");

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "calendar"
                    && value["current_scope"] == json!("DAYS")
                    && value["current_offset"] == json!("2026-05-01")
        ));
    }

    #[test]
    fn calendar_date_callback_uses_on_click_handler() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar")
            .config(test_config())
            .on_click(|_click, selected_date| {
                ButtonAction::set_dialog_value("selected_date", selected_date.to_string())
            })
            .build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:DATE2026-04-13", ctx.id))
            .expect("calendar action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "selected_date" && value == "2026-04-13"
        ));
    }

    #[test]
    fn calendar_without_on_click_consumes_date_callback() {
        let ctx = Context::new("", "state", Value::Null);
        let calendar = Calendar::builder("calendar").config(test_config()).build();

        let action = calendar
            .handle_callback_for_test(&ctx, &format!("td:{}:calendar:DATE2026-04-13", ctx.id))
            .expect("calendar action");

        assert!(matches!(action, ButtonAction::Noop));
    }
}
