use std::borrow::Cow;

use super::Text;
use crate::entities::{Data, DataMap};

pub(crate) struct FormatText {
    template: Cow<'static, str>,
}

impl FormatText {
    #[must_use]
    pub(crate) fn new(template: impl Into<Cow<'static, str>>) -> Self {
        Self {
            template: template.into(),
        }
    }
}

impl Text for FormatText {
    #[inline]
    fn render_text(&self, data: &DataMap) -> Box<str> {
        render_template(&self.template, data).into_boxed_str()
    }
}

fn render_template(template: &str, data: &DataMap) -> String {
    let mut output = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(start) = rest.find('{') {
        output.push_str(&rest[..start]);
        let after_start = &rest[start + 1..];

        let Some(end) = after_start.find('}') else {
            output.push_str(&rest[start..]);
            return output;
        };

        let key = &after_start[..end];
        if key.is_empty() || key.contains('{') {
            output.push_str(&rest[start..start + end + 2]);
        } else if let Some(value) = data.get(key) {
            output.push_str(&render_data_value(value));
        } else {
            output.push('{');
            output.push_str(key);
            output.push('}');
        }

        rest = &after_start[end + 1..];
    }

    output.push_str(rest);
    output
}

fn render_data_value(value: &Data) -> String {
    match value {
        Data::String(value) => value.clone(),
        Data::Null => String::new(),
        _ => value.to_string(),
    }
}
