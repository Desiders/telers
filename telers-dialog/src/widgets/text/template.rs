//! Jinja-like templated text widget using minijinja.
//!
//! This module is only available with the `template` feature.

use async_trait::async_trait;
use bon::bon;
use minijinja::{Environment, Value};
use std::{borrow::Cow, sync::Arc};

use super::base::Text;
use crate::entities::DataMap;

/// A text widget that renders Jinja-like templates.
///
/// Templates are rendered using the [`minijinja`] crate, which provides a
/// subset of Jinja2 syntax.
///
/// Construct via [`TemplateText::builder`], passing the template as the start
/// argument. A custom [`Environment`] can be supplied with
/// [`TemplateEnvBuilder`].
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::TemplateText;
///
/// let text = TemplateText::builder("Hello, {{ name }}! You have {{ count }} items.").build();
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

#[bon]
impl TemplateText {
    /// Build a template text widget from a template source.
    ///
    /// When `env` is omitted, a default environment with `trim_blocks` and
    /// `lstrip_blocks` enabled is used.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] template: Cow<'static, str>,
        #[builder(with = |env: Environment<'static>| Arc::new(env))] env: Option<
            Arc<Environment<'static>>,
        >,
    ) -> Self {
        Self {
            template,
            env: env.unwrap_or_else(|| Arc::new(default_env())),
        }
    }
}

#[async_trait]
impl Text for TemplateText {
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        let ctx = Value::from_serialize(data);
        match self.env.render_str(&self.template, ctx) {
            Ok(rendered) => rendered.into_boxed_str(),
            Err(err) => {
                tracing::warn!(error = %err, template = %self.template, "Template rendering failed");
                self.template.to_string().into_boxed_str()
            }
        }
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

/// Builder for a customized template [`Environment`].
///
/// Use this to register custom filters, globals, or auto-escape rules, then
/// pass the result to [`TemplateText::builder`].
pub struct TemplateEnvBuilder {
    env: Environment<'static>,
}

impl Default for TemplateEnvBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEnvBuilder {
    /// Create a new environment builder seeded with the default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            env: default_env(),
        }
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
    pub fn auto_escape(mut self, escape: bool) -> Self {
        if escape {
            self.env
                .set_auto_escape_callback(|_| minijinja::AutoEscape::Html);
        } else {
            self.env
                .set_auto_escape_callback(|_| minijinja::AutoEscape::None);
        }
        self
    }

    /// Finalize and return the configured environment.
    #[must_use]
    pub fn build(self) -> Environment<'static> {
        self.env
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

        let text = TemplateText::builder("Hello, {{ name }}!").build();
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Hello, Alice!");
    }

    #[tokio::test]
    async fn renders_nested_values() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert(
            "user".into(),
            json!({ "name": "Bob", "email": "bob@example.com" }),
        );

        let text = TemplateText::builder("User: {{ user.name }} <{{ user.email }}>").build();
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "User: Bob <bob@example.com>");
    }

    #[tokio::test]
    async fn renders_with_filters() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("name".into(), json!("alice"));

        let text = TemplateText::builder("Hello, {{ name | upper }}!").build();
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Hello, ALICE!");
    }

    #[tokio::test]
    async fn renders_with_conditionals() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("premium".into(), json!(true));

        let text =
            TemplateText::builder("{% if premium %}Premium user{% else %}Free user{% endif %}")
                .build();
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Premium user");
    }

    #[tokio::test]
    async fn renders_with_loops() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("items".into(), json!(["apple", "banana", "cherry"]));

        let text = TemplateText::builder(
            "Items: {% for item in items %}{{ item }}{% if not loop.last %}, {% endif %}{% endfor \
             %}",
        )
        .build();
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Items: apple, banana, cherry");
    }

    #[tokio::test]
    async fn falls_back_to_template_on_error() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let data = DataMap::new();

        let text = TemplateText::builder("Hello, {{ name }").build();
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Hello, {{ name }");
    }

    #[tokio::test]
    async fn uses_default_filter_for_missing_values() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let data = DataMap::new();

        let text = TemplateText::builder("Count: {{ count | default(0) }}").build();
        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Count: 0");
    }

    #[tokio::test]
    async fn custom_env_with_filter() {
        let ctx = Context::new("", "state", serde_json::Value::Null);
        let mut data = DataMap::new();
        data.insert("price".into(), json!(42.5));

        let env = TemplateEnvBuilder::new()
            .add_filter("currency", |v: f64| format!("${:.2}", v))
            .build();
        let text = TemplateText::builder("Price: {{ price | currency }}")
            .env(env)
            .build();

        let rendered = text.render_text_in_context_for_test(&ctx, &data).await;

        assert_eq!(rendered.as_ref(), "Price: $42.50");
    }
}
