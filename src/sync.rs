//! TinyTemplate with an interface slightly adjusted to allow multi-threaded usage.

use super::Result;
use super::Value;
use super::{format, format_unescaped};
use crate::generic;

/// Type alias for closures which can be used as value formatters.
pub type ValueFormatter = dyn Fn(&Value, &mut String) -> Result<()> + Send + Sync;

pub struct TinyTemplate<'template>(generic::TinyTemplate<'template, ValueFormatter>);

impl<'template> TinyTemplate<'template> {
    /// Create a new TinyTemplate registry. The returned registry contains no templates, and has
    /// [`format_unescaped`](fn.format_unescaped.html) registered as a formatter named "unescaped".
    pub fn new() -> TinyTemplate<'template> {
        let mut tt = TinyTemplate(generic::TinyTemplate::new(&format));
        tt.add_formatter("unescaped", format_unescaped);
        tt
    }

    /// Parse and compile the given template, then register it under the given name.
    pub fn add_template(&mut self, name: &'template str, text: &'template str) -> Result<()> {
        self.0.add_template(name, text)
    }

    /// Changes the default formatter from [`format`](fn.format.html) to `formatter`. Usefull in combination with [`format_unescaped`](fn.format_unescaped.html) to deactivate HTML-escaping
    pub fn set_default_formatter<F>(&mut self, formatter: &'template F)
    where
        F: 'static + Fn(&Value, &mut String) -> Result<()> + Send + Sync,
    {
        self.0.set_default_formatter(formatter);
    }

    /// Register the given formatter function under the given name.
    pub fn add_formatter<F>(&mut self, name: &'template str, formatter: F)
    where
        F: 'static + Fn(&Value, &mut String) -> Result<()> + Send + Sync,
    {
        self.0.add_formatter(name, Box::new(formatter));
    }

    /// Render the template with the given name using the given context object. The context
    /// object must implement `serde::Serialize` as it will be converted to `serde_json::Value`.
    pub fn render<C>(&self, template: &str, context: &C) -> Result<String>
    where
        C: serde::Serialize,
    {
        self.0.render::<C>(template, context)
    }
}
impl<'template> Default for TinyTemplate<'template> {
    fn default() -> TinyTemplate<'template> {
        TinyTemplate::new()
    }
}
