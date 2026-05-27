//! Scrolling widgets: `ScrollingGroup`, `ScrollingText`, `StubScroll`, and
//! synced scrolls.

use telers_dialog::{
    async_trait,
    entities::{DataMap, RenderContext},
    widgets::{
        format_text, keyboard, sync_scroll, text, Button, ButtonAction, InlineKeyboard,
        NumberedPager, ScrollingGroup, ScrollingText, Select, StubScroll, Text,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "scrolls_menu";

const STUB_SCROLL_ID: &str = "stub_page";
const STUB_PAGES: usize = 5;

/// Renders the current `StubScroll` page as text.
///
/// `StubScroll` itself draws nothing: it only stores the page in `widget_data`
/// and reports the page count, letting a `NumberedPager` drive custom content.
struct StubPreview;

#[async_trait]
impl Text for StubPreview {
    async fn render_text(&self, _data: &DataMap) -> Box<str> {
        "Page 1 of ...".into()
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        let page = render_ctx
            .context
            .widget_value_as::<usize>(STUB_SCROLL_ID)
            .unwrap_or(0);
        format!("This is page {} of {STUB_PAGES}.", page + 1).into_boxed_str()
    }
}

const PRODUCTS: &[(&str, &str)] = &[
    ("Espresso", "$2.50"),
    ("Americano", "$3.00"),
    ("Latte", "$4.50"),
    ("Cappuccino", "$4.00"),
    ("Flat White", "$4.20"),
    ("Mocha", "$4.80"),
    ("Raf", "$4.90"),
    ("Tea", "$2.80"),
    ("Matcha", "$4.70"),
    ("Cocoa", "$4.30"),
    ("Cold Brew", "$3.90"),
    ("Affogato", "$5.10"),
];

const LONG_TEXT: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut \
     labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco \
     laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
     voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
     cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. \
     Curabitur pretium tincidunt lacus, eu rutrum nisl imperdiet a. Nulla facilisi. Pellentesque \
     habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.";

fn back_nav() -> InlineKeyboard {
    InlineKeyboard::builder()
        .row([Button::switch_to("back", "Back", STATE)])
        .row([main_menu_button()])
        .build()
}

pub fn dialog() -> impl Dialog {
    let preview = ScrollingText::builder("scroll_text")
        .text(LONG_TEXT)
        .page_size(160)
        .build();

    let sync_grid = ScrollingGroup::builder("sync_grid")
        .height(2)
        .width(2)
        .hide_pager(true)
        .on_page_changed(sync_scroll("sync_details"))
        .kbd(
            Select::builder("sync_grid_items")
                .items_getter(|_data| PRODUCTS)
                .item_renderer(|item, _data| item.0.to_owned())
                .id_getter(|item| item.0)
                .action(|value| async move { ButtonAction::set_dialog_value("scroll_pick", value) })
                .build(),
        )
        .build();
    let sync_details = ScrollingGroup::builder("sync_details")
        .height(2)
        .width(2)
        .hide_pager(true)
        .kbd(
            Select::builder("sync_detail_items")
                .items_getter(|_data| PRODUCTS)
                .item_renderer(|item, _data| item.1.to_owned())
                .id_getter(|item| item.0)
                .action(|_value| async move { ButtonAction::noop() })
                .build(),
        )
        .build();

    telers_dialog::dialog([
        window(
            STATE,
            [
                text("Scrolling widgets\n\nPick a demo."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to(
                            "s_group",
                            "📜 Scrolling group",
                            "scrolls_group",
                        )])
                        .row([Button::switch_to(
                            "s_text",
                            "📄 Scrolling text",
                            "scrolls_text",
                        )])
                        .row([Button::switch_to(
                            "s_sync",
                            "📜 & 📜 Synced",
                            "scrolls_sync",
                        )])
                        .row([Button::switch_to(
                            "s_stub",
                            "📟 Stub scroll",
                            "scrolls_stub",
                        )])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "scrolls_group",
            [
                format_text(
                    "Scrolling group\n\nA long product list paginated by the built-in \
                     pager.\nPicked: {scroll_pick}",
                ),
                keyboard(
                    ScrollingGroup::builder("scroll_group")
                        .height(4)
                        .width(1)
                        .kbd(
                            Select::builder("scroll_group_items")
                                .items_getter(|_data| PRODUCTS)
                                .item_renderer(|item, _data| format!("{} {}", item.0, item.1))
                                .id_getter(|item| item.0)
                                .action(|value| async move {
                                    ButtonAction::set_dialog_value("scroll_pick", value)
                                })
                                .build(),
                        )
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "scrolls_text",
            [
                text("Scrolling text\n"),
                text(preview.clone()),
                keyboard(
                    NumberedPager::builder(preview)
                        .page_renderer(|page, _data| format!("{}", page + 1))
                        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                        .length(5)
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "scrolls_sync",
            [
                format_text(
                    "Synced scroll\n\nOne pager moves both blocks together.\nPicked: {scroll_pick}",
                ),
                keyboard(sync_grid.clone()),
                keyboard(sync_details),
                keyboard(
                    NumberedPager::builder(sync_grid)
                        .page_renderer(|page, _data| format!("{}", page + 1))
                        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                        .length(5)
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "scrolls_stub",
            [
                text(
                    "Stub scroll\n\n`StubScroll` renders nothing itself; it stores the page and \
                     lets the pager drive the custom text below.\n",
                ),
                text(StubPreview),
                keyboard(
                    NumberedPager::builder(
                        StubScroll::builder(STUB_SCROLL_ID)
                            .pages(STUB_PAGES)
                            .build(),
                    )
                    .page_renderer(|page, _data| format!("{}", page + 1))
                    .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                    .length(5)
                    .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
    ])
}
