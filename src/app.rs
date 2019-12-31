use crate::fetcher::{Story, HnFetcher};
use crate::save::{get_store, does_exist, get_story};
use rusqlite::Connection;
use chrono::{DateTime, Utc};

pub struct App {
    pub current_story_index: usize,
    pub stories: Vec<Story>,
    pub last_refresh: DateTime<Utc>,
    pub connection: Connection
}

impl App {
    pub fn init() -> App {
        let conn = get_store();
        let fetcher = HnFetcher::new(String::from("https://news.ycombinator.com/"));
        let stories = fetcher.fetch_stories().iter().map(|story| {
            let mut is_visited  = false;
            let mut is_saved = false;
            if does_exist(&conn, &story) {
                let story_save = get_story(&conn, &story);
                is_visited = story_save.is_visited;
                is_saved = story_save.is_saved;
            }
            Story {
                data: story.data.clone(),
                sub: story.sub.clone(),
                is_visited,
                is_saved
            }
        }).collect();
        let last_refresh = Utc::now();
        App {
            current_story_index: 0,
            stories,
            last_refresh,
            connection: conn
        }
    }
}
