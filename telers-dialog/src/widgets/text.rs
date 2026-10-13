//! Text widgets and text rendering helpers.

mod base;
mod case;
mod format;
mod fallback;
mod list;
mod multi;
mod progress;
mod scrolling;

#[cfg(feature = "template")]
mod template;

pub use base::{ConditionalText, FnText, Text};
pub use fallback::FirstText;
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
