use std::io::Read;          // tait and read_to_string

use hyper::Client;
use rss;
use atom_syndication;


pub type RssFeed = rss::Channel;
pub type AtomFeed = atom_syndication::Feed;


#[derive(Debug)]
pub enum Feed {
    RssFeed(RssFeed),
    AtomFeed(AtomFeed),
}


pub fn get_feed(url: &str) -> Feed {
    let client = Client::new();
    let mut reader = client.get(url).send().unwrap();

    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);

    // TODO: need a better way to recognize RSS or Atom
    if let Ok(feed) = data.parse::<RssFeed>() {
        return Feed::RssFeed(feed);
    } else if let Ok(feed) = data.parse::<AtomFeed>() {
        return Feed::AtomFeed(feed);
    } else {
        panic!("WTF, neither RSS or Atom :(");
    }
}
