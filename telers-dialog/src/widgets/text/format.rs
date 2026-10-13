use std::borrow::Cow;

use super::Text;
use crate::entities::{Data, DataMap};
use async_trait::async_trait;

/// Text widget that renders a `{field}`-style template against render data.
///
/// Useful as the dynamic argument for button helpers like
/// [`Button::url_dynamic`] and [`Button::copy_text_dynamic`].
///
/// [`Button::url_dynamic`]: crate::widgets::Button::url_dynamic
/// [`Button::copy_text_dynamic`]: crate::widgets::Button::copy_text_dynamic
pub struct FormatText {
    template: Cow<'static, str>,
}

impl FormatText {
    /// Create a new format-text widget from a template string.
    #[must_use]
    pub fn new(template: impl Into<Cow<'static, str>>) -> Self {
        Self {
            template: template.into(),
        }
    }
}

#[async_trait]
impl Text for FormatText {
    #[inline]
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        render_template(&self.template, data).into_boxed_str()
    }
}

fn render_template(template: &str, data: &DataMap) -> String {
    let mut output = String::with_capacity(template.len());
    let mut chars = template.char_indices().peekable();
    while let Some((offset, character)) = chars.next() {
        if matches!(character, '{' | '}') && chars.peek().is_some_and(|(_, next)| *next == character) {
            output.push(character);
            chars.next();
        } else if character == '{' {
            let end = template[offset + 1..].find('}').map(|end| offset + 1 + end);
            if let Some(end) = end {
                let key = &template[offset + 1..end];
                if let Some(value) = resolve_field(data, key) {
                    output.push_str(&render_data_value(value));
                } else {
                    output.push_str(&template[offset..=end]);
                }
                while chars.peek().is_some_and(|(index, _)| *index <= end) {
                    chars.next();
                }
            } else {
                output.push_str(&template[offset..]);
                break;
            }
        } else {
            output.push(character);
        }
    }
    output
}

fn resolve_field<'a>(data: &'a DataMap, field: &str) -> Option<&'a Data> {
    // Literal keys take precedence over nested paths.
    if let Some(value) = data.get(field) {
        return Some(value);
    }
    let root_end = field.find(['.', '[']).unwrap_or(field.len());
    let mut value = data.get(&field[..root_end])?;
    let mut path = &field[root_end..];
    while !path.is_empty() {
        let component;
        if let Some(rest) = path.strip_prefix('[') {
            let end = rest.find(']')?;
            component = &rest[..end];
            path = &rest[end + 1..];
        } else {
            let rest = path.strip_prefix('.')?;
            let end = rest.find(['.', '[']).unwrap_or(rest.len());
            component = &rest[..end];
            path = &rest[end..];
        }
        if component.is_empty() {
            return None;
        }
        value = if let Some(array) = telers::serialization::as_array(value) {
            array.get(component.parse::<usize>().ok()?)?
        } else {
            value.get(component)?
        };
    }
    Some(value)
}

fn render_data_value(value: &Data) -> String {
    if let Some(text) = value.as_str() {
        text.to_owned()
    } else if value.is_null() {
        String::new()
    } else {
        telers::serialization::to_string(value).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::render_template;
    use crate::entities::DataMap;
    use telers::serialization::json;

    #[test]
    fn nested_maps_arrays_and_escaped_braces() {
        let data = DataMap::from([("data".into(), json!({"items": [{"name": "кофе"}]}))]);
        assert_eq!(render_template("{{{data.items[0].name}}} {data[items][0][name]}", &data), "{кофе} кофе");
    }

    #[test]
    fn malformed_or_missing_fields_preserve_the_template() {
        let data = DataMap::from([("data".into(), json!([1]))]);
        for template in ["{missing}", "{data[5]}", "{data[-1]}", "{data.}", "{data[}", "unclosed {"] {
            assert_eq!(render_template(template, &data), template);
        }
    }

    #[test]
    fn literal_keys_take_precedence_over_nested_paths() {
        let data = DataMap::from([("a.b".into(), json!("literal")), ("a".into(), json!({"b": "nested"}))]);
        assert_eq!(render_template("{a.b} {a[b]}", &data), "literal nested");
    }

    #[test]
    fn unicode_paths_values_and_adjacent_fields_are_preserved() {
        let data = DataMap::from([
            ("данные".into(), json!({"товары": [{"имя": "кофе ☕"}, {"имя": "茶"}]})),
            ("🐈".into(), json!("猫")),
        ]);
        assert_eq!(render_template("Привет {{мир}}: {данные.товары[0].имя} / {данные[товары][1][имя]}{🐈}", &data), "Привет {мир}: кофе ☕ / 茶猫");
    }

    #[test]
    fn substituted_values_are_not_reformatted_or_treated_as_escapes() {
        let data = DataMap::from([("message".into(), json!("{name} {{literal}}")), ("name".into(), json!("Alice"))]);
        assert_eq!(render_template("{{{message}}}", &data), "{{name} {{literal}}}");
        assert_eq!(render_template("{{name}}/{name}", &data), "{name}/Alice");
    }

    #[test]
    fn malformed_nested_paths_and_unclosed_fields_preserve_unicode_templates() {
        let data = DataMap::from([("данные".into(), json!({"items": [1]}))]);
        for field in [
            "данные..items", "данные.items[]", "данные.items[-1]", "данные.items[999999999999999999999999999999999999]",
            "данные.items[0]suffix", "данные.items[0]]", "данные.items.[0]", "данные[items][0", "данные[items]].x",
        ] {
            let template = format!("☕ {{{field}}} 茶");
            assert_eq!(render_template(&template, &data), template);
        }
        assert_eq!(render_template("☕ {данные.items[0]", &data), "☕ {данные.items[0]");
    }

    #[test]
    fn null_and_nonstring_values_render_consistently_inside_nested_fields() {
        let data = DataMap::from([("data".into(), json!({"empty": null, "number": 42, "flag": true, "list": [1, 2], "map": {"a": 1}}))]);
        assert_eq!(render_template("{data.empty}|{data.number}|{data.flag}|{data.list}|{data.map}", &data), "|42|true|[1,2]|{\"a\":1}");
    }

    #[test]
    fn literal_bracket_keys_override_nested_fields_and_empty_names_remain_visible() {
        let data = DataMap::from([("data[0]".into(), json!("literal")), ("data".into(), json!(["nested"]))]);
        assert_eq!(render_template("{data[0]} {} {{{unknown}}} }}}}", &data), "literal {} {{unknown}} }}");
    }
}
