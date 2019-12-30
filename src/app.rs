use crate::fetcher::{Story, HnFetcher};

pub struct App {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub current_story_index: usize,
    pub stories: Vec<Story>,
}

impl App {
    pub fn init() -> App {
        let fetcher = HnFetcher::new(String::from("https://news.ycombinator.com/"));
        let stories = fetcher.fetch_stories();
        App {
            cursor_x: 1,
            cursor_y: 4, // padding for title "hackernews",
            current_story_index: 1,
            stories
        }
    }
}
