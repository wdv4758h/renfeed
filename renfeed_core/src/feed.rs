use std::collections::HashMap;

use rss;
use atom_syndication;


////////////////////////////////////////
// Config
////////////////////////////////////////

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedSetting {
    pub name: String,
    pub feedurl: String,
    pub homepage: String,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedSettings {
    pub feeds: HashMap<String, FeedSetting>,    // id, setting
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
