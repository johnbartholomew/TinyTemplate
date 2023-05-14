use super::template::Template;
use super::{Error, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// The TinyTemplate struct is the entry point for the TinyTemplate library. It contains the
/// template and formatter registries and provides functions to render templates as well as to
/// register templates and formatters.
pub(crate) struct TinyTemplate<'template, VF: ?Sized> {
    templates: HashMap<&'template str, Template<'template>>,
    formatters: HashMap<&'template str, Box<VF>>,
    default_formatter: &'template VF,
}

impl<'template, VF> TinyTemplate<'template, VF>
where
    VF: ?Sized + for<'a, 'b> Fn(&'a Value, &'b mut String) -> Result<()>,
{
    pub(crate) fn new(default_formatter: &'template VF) -> Self {
        TinyTemplate {
            templates: HashMap::default(),
            formatters: HashMap::default(),
            default_formatter,
        }
    }
    /// Parse and compile the given template, then register it under the given name.
    pub(crate) fn add_template(
        &mut self,
        name: &'template str,
        text: &'template str,
    ) -> Result<()> {
        let template = Template::compile(text)?;
        self.templates.insert(name, template);
        Ok(())
    }

    /// Render the template with the given name using the given context object. The context
    /// object must implement `serde::Serialize` as it will be converted to `serde_json::Value`.
    pub(crate) fn render<C>(&self, template: &str, context: &C) -> Result<String>
    where
        C: Serialize,
    {
        let value = serde_json::to_value(context)?;
        match self.templates.get(template) {
            Some(tmpl) => tmpl.render(
                &value,
                &self.templates,
                &self.formatters,
                self.default_formatter,
            ),
            None => Err(Error::GenericError {
                msg: format!("Unknown template '{}'", template),
            }),
        }
    }

    pub(crate) fn set_default_formatter(&mut self, formatter: &'template VF) {
        self.default_formatter = formatter;
    }
    pub(crate) fn add_formatter(&mut self, name: &'template str, formatter: Box<VF>) {
        self.formatters.insert(name, formatter);
    }
}
