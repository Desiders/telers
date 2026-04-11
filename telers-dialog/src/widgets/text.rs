//! Text widgets and text rendering helpers.

mod base;
mod case;
mod format;
mod list;
mod multi;
mod progress;
mod scrolling;

pub(crate) use base::FnText;
pub(crate) use format::FormatText;

pub use base::Text;
pub use case::Case;
pub use list::ListText;
pub use multi::{MultiText, MultiTextBuilder};
pub use progress::Progress;
pub use scrolling::ScrollingText;

#[cfg(test)]
mod tests;
