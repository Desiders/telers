mod input;
mod kbd;
mod list;
mod text;
mod widget;

pub(super) use input::MultiInput;
pub(super) use kbd::MultiKeyboard;
pub(crate) use text::{FnText, FormatText};
pub(crate) use widget::ensure_widgets;

pub use input::{Input, MessageInput, TextInput};
pub use kbd::{
    sync_scroll, sync_scrolls, Button, ButtonAction, CurrentPage, FirstPage, Group, InlineKeyboard,
    Keyboard, LastPage, Multiselect, NextPage, NumberedPager, OnPageChanged, PageChange,
    PageDirection, PrevPage, Radio, ScrollingGroup, Select, SwitchPage, Toggle,
};
pub use list::ListText;
pub use text::{MultiText, Text};
pub use widget::{fn_text, format_text, input, keyboard, text, WidgetKind};
