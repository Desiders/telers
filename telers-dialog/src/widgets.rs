mod input;
mod kbd;
mod link_preview;
mod list;
mod text;
mod widget;

pub(super) use input::MultiInput;
pub(super) use kbd::MultiKeyboard;
pub(crate) use text::{FnText, FormatText};
pub(crate) use widget::ensure_widgets;

pub use input::{Input, MessageInput, TextInput};
pub use kbd::{
    sync_scroll, sync_scrolls, BaseScroll, Button, ButtonAction, Checkbox, Counter, CurrentPage,
    FirstPage, Group, InlineKeyboard, Keyboard, LastPage, Multiselect, NextPage, NumberedPager,
    OnPageChanged, PageChange, PageDirection, PagerBinding, PrevPage, Radio, RequestContact,
    RequestLocation, RequestPoll, Scroll, ScrollingGroup, Select, SwitchPage, TimeSelect, Toggle,
};
pub use link_preview::{LinkPreview, LinkPreviewWidget};
pub use list::ListText;
pub use text::{Case, MultiText, Progress, ScrollingText, Text};
pub use widget::{fn_text, format_text, input, keyboard, link_preview, text, WidgetKind};
