#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;      // for Serialize/Deserialize
extern crate serde_yaml;        // for read YAML feed settings
extern crate rss;               // for RSS feeds
extern crate atom_syndication;  // for Atom feeds


use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FeedSetting {
    name: String,
    feedurl: String,
    homepage: String,
    categories: Option<Vec<String>>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedSettings {
    feeds: HashMap<String, FeedSetting>,    // id, setting
}


pub fn parse_yaml_file(path: &str) -> FeedSettings {
    let file = File::open(path)
                    .expect("Failed to read YAML feed settings");
    let file = BufReader::new(file);
    serde_yaml::from_reader::<_, FeedSettings>(file)
               .expect("Could not decode YAML feed settings")
}
