extern crate renfeed_core;

#[macro_use]
extern crate clap;              // CLI arguments


use std::fs::File;
use std::io::Write;

use clap::App;


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("cli.yml");
    let arguments = App::from_yaml(yml).get_matches();

    println!("{:#?}", arguments);

    ////////////////////
    // Parse Feed Settings
    ////////////////////

    let settings = renfeed_core::parse_yaml_file("share/feeds/basic.yml");
    let feeds = renfeed_core::get_feeds(&settings);
    let template = renfeed_core::load_template("share/templates/**/*");
    let html = renfeed_core::render(&template, "basic.tera.html", &feeds);
    let mut f = File::create("share/index.html").unwrap();
    let _ = f.write_all(html.as_bytes());
}
