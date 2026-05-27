//! Text widgets and text rendering helpers.

mod base;
mod case;
mod format;
mod list;
mod multi;
mod progress;
mod scrolling;

#[cfg(feature = "template")]
mod template;

pub use base::{FnText, Text};
pub use case::Case;
pub use format::FormatText;
pub use list::ListText;
pub use multi::{MultiText, MultiTextBuilder};
pub use progress::Progress;
pub use scrolling::ScrollingText;

#[cfg(feature = "template")]
pub use template::{TemplateEnvBuilder, TemplateText};

#[cfg(test)]
mod tests;
