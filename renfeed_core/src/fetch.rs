use std::io::BufReader;     // wrap the reader of hyper's Response

use hyper::Client;
use rss;


pub type RssChannel = rss::Channel;


pub fn get_rss(url: &str) -> RssChannel {
    let client = Client::new();
    let response = client.get(url).send().unwrap();
    let reader = BufReader::new(response);
    rss::Channel::read_from(reader).unwrap()
}
