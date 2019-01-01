extern crate serde;
extern crate serde_json;

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate serde_derive;

mod compiler;
pub mod error;
mod instruction;
mod template;

use error::Error::*;
use error::Result;
use serde::Serialize;
use std::collections::HashMap;
use template::Template;

/*
TODO:
- Implement parsing using Jinja2-like syntax
    - Call {% call macro_name foo.bar %}
    - Formatters {{ foo.bar | my_formatter }}
- Implement error detail handling by calculating the line/column when an error occurs
- HTML escaping?
- Benchmark
- Write documentation
- CI builds
- Build my own clone of serde_json::Value so I can drop serde_json.
*/
pub struct TinyTemplate<'template> {
    templates: HashMap<&'template str, Template<'template>>,
}
impl<'template> TinyTemplate<'template> {
    pub fn new() -> TinyTemplate<'template> {
        TinyTemplate {
            templates: HashMap::default(),
        }
    }

    pub fn add_template(&mut self, name: &'template str, text: &'template str) -> Result<()> {
        let template = Template::compile(text)?;
        self.templates.insert(name, template);
        Ok(())
    }

    pub fn render<C>(&self, template: &str, context: &C) -> Result<String>
    where
        C: Serialize,
    {
        let value = serde_json::to_value(context)?;
        match self.templates.get(template) {
            Some(tmpl) => tmpl.render(&value),
            None => Err(UnknownTemplate {
                msg: format!("Unknown template '{}'", template),
            }),
        }
    }
}
impl<'template> Default for TinyTemplate<'template> {
    fn default() -> TinyTemplate<'template> {
        TinyTemplate::new()
    }
}
