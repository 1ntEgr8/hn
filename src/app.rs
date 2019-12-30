use crate::fetcher::{Story, HnFetcher};

pub struct App {
    pub current_story_index: usize,
    pub stories: Vec<Story>,
}

impl App {
    pub fn init() -> App {
        let fetcher = HnFetcher::new(String::from("https://news.ycombinator.com/"));
        let stories = fetcher.fetch_stories();
        App {
            current_story_index: 0,
            stories
        }
    }
}
