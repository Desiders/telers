use crate::entities::{Data, DataMap};

pub trait Text: Send + Sync + 'static {
    #[must_use]
    fn render_text(&self, data: &DataMap) -> Box<str>;
}

impl<T> Text for T
where
    T: ToString + Send + Sync + 'static,
{
    fn render_text(&self, _data: &DataMap) -> Box<str> {
        self.to_string().into_boxed_str()
    }
}

pub struct FnText<F> {
    renderer: F,
}

impl<F> FnText<F> {
    #[inline]
    #[must_use]
    pub const fn new(renderer: F) -> Self {
        Self { renderer }
    }
}

impl<F, T> Text for FnText<F>
where
    F: Fn(&DataMap) -> T + Send + Sync + 'static,
    T: Into<Box<str>>,
{
    fn render_text(&self, data: &DataMap) -> Box<str> {
        (self.renderer)(data).into()
    }
}

pub struct FormatText {
    template: Box<str>,
}

impl FormatText {
    #[must_use]
    pub fn new(template: impl Into<Box<str>>) -> Self {
        Self {
            template: template.into(),
        }
    }
}

impl Text for FormatText {
    fn render_text(&self, data: &DataMap) -> Box<str> {
        render_template(&self.template, data).into_boxed_str()
    }
}

pub struct MultiText {
    texts: Vec<Box<dyn Text>>,
    separator: Box<str>,
}

impl MultiText {
    #[must_use]
    pub fn new() -> Self {
        Self {
            texts: Vec::new(),
            separator: "\n".into(),
        }
    }

    #[must_use]
    pub fn text(mut self, item: impl Text) -> Self {
        self.texts.push(Box::new(item));
        self
    }

    #[must_use]
    pub(crate) fn text_boxed(mut self, item: Box<dyn Text>) -> Self {
        self.texts.push(item);
        self
    }

    #[must_use]
    pub fn with_separator(mut self, separator: impl Into<Box<str>>) -> Self {
        self.separator = separator.into();
        self
    }
}

impl Text for MultiText {
    fn render_text(&self, data: &DataMap) -> Box<str> {
        self.texts
            .iter()
            .map(|item| item.render_text(data).into_string())
            .collect::<Vec<_>>()
            .join(&self.separator)
            .into_boxed_str()
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

#[cfg(test)]
mod tests {
    use super::{FormatText, MultiText, Text};
    use crate::entities::DataMap;

    #[test]
    fn format_text_replaces_known_keys() {
        let mut data = DataMap::new();
        data.insert("name".into(), "telers".into());

        let text = FormatText::new("hello {name}");

        assert_eq!(&*text.render_text(&data), "hello telers");
    }

    #[test]
    fn format_text_keeps_unknown_keys_visible() {
        let text = FormatText::new("hello {name}");

        assert_eq!(&*text.render_text(&DataMap::new()), "hello {name}");
    }

    #[test]
    fn multi_text_joins_items() {
        let text = MultiText::new()
            .text("one")
            .text("two")
            .with_separator(" | ");

        assert_eq!(&*text.render_text(&DataMap::new()), "one | two");
    }
}
