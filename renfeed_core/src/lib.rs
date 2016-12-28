#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;      // for Serialize/Deserialize
extern crate serde_yaml;        // for read YAML feed settings
extern crate rss;               // for RSS feeds
extern crate atom_syndication;  // for Atom feeds


mod feed;
mod config;

pub use feed::*;
pub use config::*;
