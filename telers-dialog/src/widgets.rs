mod input;
mod kbd;
mod link_preview;
pub mod media;
mod text;
mod widget;

pub(super) use input::MultiInput;
pub(super) use kbd::MultiKeyboard;
pub(crate) use widget::ensure_widgets;

pub use input::{Input, MessageInput, MessageInputContext, TextInput, TextInputContext};
pub use kbd::{
    sync_scroll, sync_scrolls, BaseScroll, Button, ButtonAction, ButtonStyle, Calendar,
    CalendarAppearance, CalendarAppearanceBuilder, CalendarBuilder, CalendarButtonKind,
    CalendarConfig, CalendarDate, CalendarScope, CalendarScopeRows, CalendarState,
    CalendarUserConfig, CalendarViewContext, CalendarViews, CalendarViewsBuilder, Checkbox,
    ClickContext, Counter, CurrentPage, FirstPage, ForceReply, Group, InlineKeyboard, Keyboard,
    LastPage, Multiselect, NextPage, NumberedPager, OnPageChanged, PageChange, PageDirection,
    PagerBinding, PrevPage, Radio, RequestContact, RequestLocation, RequestPoll, Scroll,
    ScrollingGroup, Select, SelectClickContext, StubScroll, StubScrollPages, SwitchPage,
    TimeSelect, Toggle, WhenCondition, WhenContext,
};
pub use link_preview::{LinkPreview, LinkPreviewWidget};
pub use media::{
    DynamicMedia, InMemoryMediaIdStorage, Media, MediaAttachment, MediaContentType, MediaId,
    MediaIdStorage, MediaScroll, StaticMedia,
};
pub use text::{Case, FnText, FormatText, ListText, MultiText, Progress, ScrollingText, Text};
#[cfg(feature = "template")]
pub use text::{TemplateEnvBuilder, TemplateText};
pub use widget::{fn_text, format_text, input, keyboard, link_preview, media, text, WidgetKind};
