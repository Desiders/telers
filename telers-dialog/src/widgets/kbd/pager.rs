//! Pagination widgets and page-change helpers.
//!
//! The pager module contains:
//! - [`ScrollingGroup`] for paged keyboard layouts
//! - standalone pager controls such as [`SwitchPage`] and [`NumberedPager`]
//! - synchronization helpers like [`sync_scroll`]

mod common;
mod scrolling_group;
mod standalone;

use common::{
    build_pager_row, handle_pager_callback, page_count_from_rows, read_page,
    render_fixed_direction_button, render_fixed_width_page, resolve_page_target,
};

pub use common::{sync_scroll, sync_scrolls, OnPageChanged, PageChange, PageDirection};
pub use scrolling_group::ScrollingGroup;
pub use standalone::{
    CurrentPage, FirstPage, LastPage, NextPage, NumberedPager, PrevPage, SwitchPage,
};

#[cfg(test)]
mod tests;
