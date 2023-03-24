pub mod html;
pub mod markdown;
pub mod text;

pub use html::{Decoration as HtmlDecoration, DECORATION as HTML_DECORATION};
pub use markdown::{Decoration as MarkdownDecoration, DECORATION as MARKDOWN_DECORATION};
pub use text::{add_surrogates, remove_surrogates, Decoration as TextDecoration};
