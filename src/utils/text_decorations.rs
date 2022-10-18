mod html;
mod markdown;
mod text;

pub use html::{HtmlDecoration, HTML_DECORATION};
pub use markdown::{MarkdownDecoration, MARKDOWN_DECORATION};
pub use text::{add_surrogates, remove_surrogates, TextDecoration};
