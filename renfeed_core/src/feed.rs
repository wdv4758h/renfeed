use std::collections::HashMap;


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
