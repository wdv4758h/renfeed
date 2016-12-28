#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;      // for Serialize/Deserialize
extern crate serde_yaml;        // for read YAML feed settings
extern crate hyper;             // for fetching RSS feeds
extern crate rss;               // for parsing RSS feeds
extern crate atom_syndication;  // for Atom feeds


pub mod feed;
pub mod config;
pub mod fetch;

pub use feed::*;
pub use config::*;
pub use fetch::*;
