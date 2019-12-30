use crate::fetcher::{Story, HnFetcher};
use chrono::{DateTime, Utc};

pub struct App {
    pub current_story_index: usize,
    pub stories: Vec<Story>,
    pub last_refresh: DateTime<Utc>
}

impl App {
    pub fn init() -> App {
        let fetcher = HnFetcher::new(String::from("https://news.ycombinator.com/"));
        let stories = fetcher.fetch_stories();
        let last_refresh = Utc::now();
        App {
            current_story_index: 0,
            stories,
            last_refresh
        }
    }
}
