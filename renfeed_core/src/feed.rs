use std::collections::HashMap;

use rss;
use atom_syndication;


////////////////////////////////////////
// Config
////////////////////////////////////////

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

////////////////////////////////////////
// Raw Feed
////////////////////////////////////////

pub type RssFeed = rss::Channel;
pub type AtomFeed = atom_syndication::Feed;

#[derive(Debug)]
pub enum RawFeed {
    RssFeed(RssFeed),
    AtomFeed(AtomFeed),
}

////////////////////////////////////////
// Internal Handled Feed
////////////////////////////////////////

pub struct Feed;
