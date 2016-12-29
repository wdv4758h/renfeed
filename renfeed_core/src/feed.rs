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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedContent {
    pub id: String,
    pub title: String,
    pub date: String,
    pub link: String,
    pub description: String,
    pub human_date: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Feed {
    pub name: String,
    pub feedurl: String,
    pub homepage: String,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub content: Vec<FeedContent>,
}


impl Feed {
    pub fn from_feed_setting(setting: &FeedSetting) -> Self {
        Feed {
            name: setting.name.clone(),
            feedurl: setting.feedurl.clone(),
            homepage: setting.homepage.clone(),
            categories: setting.categories.clone(),
            tags: setting.tags.clone(),
            content: vec![],
        }
    }

    pub fn content_from_rss(&mut self, feed: RssFeed) {
        for item in feed.items.iter() {
            let item = item.clone();
            self.content.push(
                FeedContent {
                    id: "TODO".to_string(),
                    title: item.title.unwrap_or(Default::default()),
                    date: item.pub_date.unwrap_or(Default::default()),
                    link: item.link.unwrap_or(Default::default()),
                    description: item.description.unwrap_or(Default::default()),
                    human_date: "TODO".to_string(),
                }
            );
        }
    }

    pub fn content_from_atom(&mut self, feed: AtomFeed) {
        for item in feed.entries.iter() {
            let item = item.clone();
            self.content.push(
                FeedContent {
                    id: "TODO".to_string(),
                    title: item.title,
                    date: item.updated,
                    link: item.links[0].href.clone(),
                    description:
                        match item.content {
                            Some(atom_syndication::Content::Text(text)) => text,
                            Some(atom_syndication::Content::Html(text)) => text,
                            _ => Default::default(),
                        },
                    human_date: "TODO".to_string(),
                }
            );
        }
    }
}
