use std::fmt;
use std::error::Error;
use std::convert::From;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    by: String,
    descendants: Option<i32>,
    id: i32,
    kids: Option<Vec<i32>>,
    score: i32, 
    title: String,
    url: Option<String>
}

#[derive(Debug)]
pub struct FetcherError { 
    cause: String
}

impl fmt::Display for FetcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fetcher failed to fetch data from hackernews")
    }
}

impl Error for FetcherError { }

impl From<reqwest::Error> for FetcherError {
    fn from(_error: reqwest::Error) -> Self {
        FetcherError { cause: String::from("something went wrong") }
    }
}

pub struct Fetcher {
    base_url: String,
}

impl Fetcher {
    pub fn new(base_url: String) -> Fetcher {
        Fetcher {
            base_url
        }
    }

    pub fn fetch_stories(&self) -> Result<Vec<Story>, FetcherError> {
        let url = format!("{}{}", self.base_url, "topstories.json");
        
        let body = reqwest::get(url.as_str())?
            .text()?;
        
        let ids = Fetcher::parse_top_stories_body(&body);
        let stories = self.get_stories(ids);
        Ok(stories)
    }

    pub fn fetch_story(&self, id: &str) -> Result<Story, FetcherError> {
        let url = format!("{}{}{}{}", self.base_url, "item/", id, ".json");
        let body = reqwest::get(url.as_str())?
            .text()?;
        let story: Story = serde_json::from_str(body.as_str()).unwrap();

        Ok(story)
    }

    fn parse_top_stories_body(body: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut it = body.trim().chars();
        
        if it.next().unwrap() != '[' {
            panic!("Invalid body received");
        }

        loop {
            let mut ch = it.next().unwrap();
            if ch.is_digit(10) {
                let mut id = String::new();
                while ch != ',' && ch != ']' {
                    id.push(ch);
                    ch = it.next().unwrap();
                }
                result.push(id);
            }
            if ch == ']' {
                break;
            }
        }

        result
    }

    fn get_stories(&self, ids: Vec<String>) -> Vec<Story> {
        let mut stories: Vec<Story> = Vec::new();
        
        for id in ids {
            let story = self.fetch_story(&id).unwrap();
            stories.push(story);
        }

        stories
    }
}
