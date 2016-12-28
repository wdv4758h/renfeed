extern crate renfeed_core;

#[macro_use]
extern crate clap;              // CLI arguments


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

    let feeds = renfeed_core::parse_yaml_file("share/feed.yml");
    println!("{:#?}", feeds);
}
