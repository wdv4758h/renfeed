#![feature(proc_macro)]

#[macro_use]
extern crate clap;              // CLI arguments
#[macro_use]
extern crate serde_derive;      // for Serialize/Deserialize
extern crate serde_yaml;        // for read YAML feed settings
extern crate rss;               // for RSS feeds
extern crate atom_syndication;  // for Atom feeds


use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use clap::App;


#[derive(Debug, Default, Serialize, Deserialize)]
struct FeedSetting {
    name: String,
    feedurl: String,
    homepage: String,
    categories: Option<Vec<String>>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeedSettings {
    feeds: HashMap<String, FeedSetting>,    // id, setting
}


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

    let file = File::open("share/feed.yml")
                        .expect("Failed to read YAML feed settings");
    let file = BufReader::new(file);
    let feeds = serde_yaml::from_reader::<_, FeedSettings>(file)
                           .expect("Could not decode YAML feed settings");

    println!("{:#?}", feeds);
}
