use std::fs::File;
use std::io::BufReader;

use ::feed::FeedSettings;

use serde_yaml;


pub fn parse_yaml_file(path: &str) -> FeedSettings {
    let file = File::open(path)
                    .expect("Failed to read YAML feed settings");
    let file = BufReader::new(file);
    serde_yaml::from_reader::<_, FeedSettings>(file)
               .expect("Could not decode YAML feed settings")
}
