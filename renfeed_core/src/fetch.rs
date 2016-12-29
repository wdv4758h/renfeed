use std::io::Read;          // read_to_string
use std::collections::HashMap;

use ::feed::{FeedSettings, Feed, RawFeed, RssFeed, AtomFeed};
use hyper::Client;


pub fn get_feed(url: &str) -> RawFeed {
    let client = Client::new();
    let mut reader = client.get(url).send().unwrap();

    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);

    // TODO: need a better way to recognize RSS or Atom
    if let Ok(feed) = data.parse::<RssFeed>() {
        return RawFeed::RssFeed(feed);
    } else if let Ok(feed) = data.parse::<AtomFeed>() {
        return RawFeed::AtomFeed(feed);
    } else {
        panic!("WTF, neither RSS or Atom :(");
    }
}

pub fn get_feeds(settings: &FeedSettings) -> HashMap<String, Feed> {
    let mut data = HashMap::new();
    for (id, setting) in settings.feeds.iter() {
        let mut feed = Feed::from_feed_setting(setting);
        match get_feed(&setting.feedurl) {
            RawFeed::RssFeed(f) => { feed.content_from_rss(f); },
            RawFeed::AtomFeed(f) => { feed.content_from_atom(f); },
        }
        data.insert(id.clone(), feed);
    }
    data
}
