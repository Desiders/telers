//! Jinja-like templated text widget using minijinja.
//!
//! This module is only available with the `template` feature.

use async_trait::async_trait;
use minijinja::{Environment, Value};
use std::borrow::Cow;
use std::sync::Arc;

use super::base::Text;
use crate::entities::DataMap;

/// A text widget that renders Jinja-like templates.
///
/// Templates are rendered using the [`minijinja`] crate, which provides
/// a subset of Jinja2 syntax.
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::TemplateText;
///
/// let text = TemplateText::new("Hello, {{ name }}! You have {{ count }} items.");
/// ```
///
/// # Template Syntax
///
/// - Variables: `{{ name }}`, `{{ user.email }}`
/// - Filters: `{{ name | upper }}`, `{{ count | default(0) }}`
/// - Control flow: `{% if ... %} ... {% endif %}`, `{% for ... %} ... {% endfor %}`
///
/// See minijinja documentation for full syntax reference.
pub struct TemplateText {
    template: Cow<'static, str>,
    env: Arc<Environment<'static>>,
}

impl Clone for TemplateText {
    fn clone(&self) -> Self {
        Self {
            template: self.template.clone(),
            env: Arc::clone(&self.env),
        }
    }
}

impl TemplateText {
    /// Create a new template text widget.
    ///
    /// The template string uses Jinja2-like syntax and will be rendered
    /// against the dialog's render data.
    #[must_use]
    pub fn new(template: impl Into<Cow<'static, str>>) -> Self {
        Self {
            template: template.into(),
            env: Arc::new(Self::default_env()),
        }
    }

    /// Create a template text widget with a custom environment.
    ///
    /// Use this to add custom filters or configure template behavior.
    #[must_use]
    pub fn with_env(template: impl Into<Cow<'static, str>>, env: Environment<'static>) -> Self {
        Self {
            template: template.into(),
            env: Arc::new(env),
        }
    }

    /// Create a default minijinja environment with common settings.
    #[must_use]
    pub fn default_env() -> Environment<'static> {
        let mut env = Environment::new();
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        env
    }

    /// Render the template with the given data.
    fn render_template(&self, data: &DataMap) -> String {
        // Convert DataMap to minijinja Value
        let ctx = datamap_to_value(data);

        // Create an inline template and render
        match self.env.render_str(&self.template, ctx) {
            Ok(rendered) => rendered,
            Err(e) => {
                tracing::warn!(error = %e, template = %self.template, "Template rendering failed");
                // Return template source as fallback (like preview mode)
                self.template.to_string()
            }
        }
    }
}

#[async_trait]
impl Text for TemplateText {
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        self.render_template(data).into_boxed_str()
    }
}

/// Convert a DataMap to a minijinja Value for template rendering.
fn datamap_to_value(data: &DataMap) -> Value {
    // DataMap is BTreeMap<String, serde_json::Value>
    // We can convert it to a minijinja Value directly
    Value::from_serialize(data)
}

/// Builder for creating a custom template environment.
#[derive(Default)]
pub struct TemplateEnvBuilder {
    env: Environment<'static>,
}

impl TemplateEnvBuilder {
    /// Create a new environment builder.
    #[must_use]
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        Self { env }
    }

    /// Add a custom filter function.
    #[must_use]
    pub fn add_filter<F, Rv, Args>(mut self, name: &'static str, f: F) -> Self
    where
        F: minijinja::filters::Filter<Rv, Args>,
        Rv: Into<Value>,
        Args: for<'a> minijinja::value::FunctionArgs<'a>,
    {
        self.env.add_filter(name, f);
        self
    }

    /// Add a global value accessible in all templates.
    #[must_use]
    pub fn add_global(mut self, name: &'static str, value: impl Into<Value>) -> Self {
        self.env.add_global(name, value);
        self
    }

    /// Set whether to auto-escape HTML.
    #[must_use]
    pub fn set_auto_escape(mut self, escape: bool) -> Self {
        if escape {
            self.env.set_auto_escape_callback(|_| minijinja::AutoEscape::Html);
        } else {
            self.env.set_auto_escape_callback(|_| minijinja::AutoEscape::None);
        }
        self
    }

    /// Build the environment.
    #[must_use]
    pub fn build(self) -> Environment<'static> {
        self.env
    }

    /// Build a template text widget with this environment.
    #[must_use]
    pub fn build_template(self, template: impl Into<Cow<'static, str>>) -> TemplateText {
        TemplateText::with_env(template, self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::Context;
    use serde_json::json;

    #[tokio::test]
    async fn renders_simple_variable() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("name".into(), json!("Alice"));

        let text = TemplateText::new("Hello, {{ name }}!");
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Hello, Alice!");
    }

    #[tokio::test]
    async fn renders_nested_values() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("user".into(), json!({ "name": "Bob", "email": "bob@example.com" }));

        let text = TemplateText::new("User: {{ user.name }} <{{ user.email }}>");
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "User: Bob <bob@example.com>");
    }

    #[tokio::test]
    async fn renders_with_filters() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("name".into(), json!("alice"));

        let text = TemplateText::new("Hello, {{ name | upper }}!");
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Hello, ALICE!");
    }

    #[tokio::test]
    async fn renders_with_conditionals() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("premium".into(), json!(true));

        let text = TemplateText::new("{% if premium %}Premium user{% else %}Free user{% endif %}");
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Premium user");
    }

    #[tokio::test]
    async fn renders_with_loops() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("items".into(), json!(["apple", "banana", "cherry"]));

        let text = TemplateText::new("Items: {% for item in items %}{{ item }}{% if not loop.last %}, {% endif %}{% endfor %}");
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Items: apple, banana, cherry");
    }

    #[tokio::test]
    async fn falls_back_to_template_on_error() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let data = DataMap::new();

        // Invalid template syntax
        let text = TemplateText::new("Hello, {{ name }");
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        // Should return template source on error
        assert_eq!(rendered.as_ref(), "Hello, {{ name }");
    }

    #[tokio::test]
    async fn uses_default_filter_for_missing_values() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let data = DataMap::new();

        let text = TemplateText::new("Count: {{ count | default(0) }}");
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Count: 0");
    }

    #[tokio::test]
    async fn custom_env_with_filter() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("price".into(), json!(42.5));

        let text = TemplateEnvBuilder::new()
            .add_filter("currency", |v: f64| format!("${:.2}", v))
            .build_template("Price: {{ price | currency }}");

        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Price: $42.50");
    }
}
