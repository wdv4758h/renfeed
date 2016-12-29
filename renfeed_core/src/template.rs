use std::collections::HashMap;

use ::feed::Feed;
use tera::{Tera, Context};


type Template = Tera;

pub fn load_template(template_path: &str) -> Template {
    let mut template = Tera::new(template_path)
                            .expect("Could not compile HTML template :(");
    // disable autoescaping completely
    template.autoescape_on(vec![]);
    template
}

pub fn render(template: &Template, template_name: &str, data: &HashMap<String, Feed>) -> String {
    let mut context = Context::new();
    context.add("feeds", &data.values().collect::<Vec<_>>());
    template.render(template_name, context)
            .expect("Template Engine couldn't render output")
}
