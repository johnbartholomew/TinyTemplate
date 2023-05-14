extern crate tinytemplate;

use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(serde_derive::Serialize)]
struct Context {
    value: String,
}

static TEMPLATE: &'static str = r"
Hello, world!
value: {value | prefix}
";

fn render_template(
    tt: &TinyTemplate,
    s: &str,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    use std::io::Write;
    let ctx = Context {
        value: s.to_owned(),
    };
    match tt.render("template", &ctx) {
        Ok(s) => {
            if let Err(e) = std::io::stdout().lock().write_all(s.as_bytes()) {
                eprintln!("io err: {e}");
            }
        }
        Err(e) => eprintln!("err: {e}"),
    }
    Ok(())
}

fn main() {
    let val = std::rc::Rc::new("nonthreadable");
    let mut tt = TinyTemplate::new();
    // Because this closure references an Rc value, and Rc values
    // are !Send and !Sync, this requires the non-sync TinyTemplate interface.
    tt.add_formatter("prefix", move |v, s| {
        use std::fmt::Write;
        write!(s, "{}: {}", *val, v).unwrap();
        Ok(())
    });
    tt.add_template("template", TEMPLATE).unwrap();
    render_template(&tt, "one").unwrap();
}
