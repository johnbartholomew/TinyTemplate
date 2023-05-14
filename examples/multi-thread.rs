extern crate tinytemplate;

use std::error::Error;
use std::sync::Arc;
use tinytemplate::sync::*;

#[derive(serde_derive::Serialize)]
struct Context {
    value: String,
}

static TEMPLATE: &'static str = r"
Hello, world!
value: {value}
";

fn render_template(
    tt: Arc<TinyTemplate>,
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
    let mut tt = TinyTemplate::new();
    tt.add_template("template", TEMPLATE).unwrap();
    let tt = Arc::new(tt);
    let th1 = {
        let tt = tt.clone();
        std::thread::spawn(move || render_template(tt, "one").unwrap())
    };
    let th2 = {
        let tt = tt.clone();
        std::thread::spawn(move || render_template(tt, "two").unwrap())
    };
    th1.join().unwrap();
    th2.join().unwrap();
}
