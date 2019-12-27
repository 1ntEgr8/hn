use std::error::Error;
use select::document::Document;
use select::predicate::{Attr, Name};

#[derive(Debug)]
pub struct TitleData {
    pub rank: i32,
    pub title: String,
    pub url: String,
}

#[derive(Debug)]
pub struct SubtextData {
    pub score: String,
    pub by: String,
    pub age: String,
}

#[derive(Debug)]
pub struct Story {
    pub data: TitleData,
    pub sub: SubtextData,
}

pub struct HnFetcher {
    base_url: String,
}

impl HnFetcher {
    pub fn new(base_url: String) -> HnFetcher {
        HnFetcher { base_url }
    }

    pub fn get_page(&self) -> Result<String, Box<dyn Error>> {
        let body = reqwest::get(self.base_url.as_str())?.text()?;
        Ok(body)
    }

    pub fn fetch_stories(&self) -> Vec<Story> {
        let mut stories = Vec::new();
        let document = Document::from(self.get_page().expect("Could not fetch data from hackernews").as_str());
        // let document = Document::from(include_str!("hn.html"));
        let title_nodes = document.find(Attr("class", "athing")).collect::<Vec<_>>();
        let subtext_nodes = document.find(Attr("class", "subtext")).collect::<Vec<_>>();

        if title_nodes.len() == subtext_nodes.len() {
            let n = title_nodes.len();

            for i in 0..n {
                stories.push(Story {
                    data: self.get_title_data(title_nodes[i]),
                    sub: self.get_subtext_data(subtext_nodes[i]),
                });
            }
        } else {
            // TERROR
        }

        stories
    }

    fn get_title_data(&self, node: select::node::Node<'_>) -> TitleData {
        let title_node = node.find(Attr("class", "storylink")).next().unwrap();
        // parse the rank field so that it is an integer!
        let rank_text = node.find(Attr("class", "rank")).next().unwrap().text();
        let rank: i32 = rank_text
            .trim()
            .get(0..(rank_text.len() - 1))
            .unwrap()
            .parse()
            .unwrap();
        // TODO: process the title to remove the awkward whitespace
        let title = title_node.text();
        let url = title_node.attr("href").unwrap();
        TitleData {
            rank,
            title,
            url: String::from(url),
        }
    }

    fn get_subtext_data(&self, node: select::node::Node<'_>) -> SubtextData {
        // TODO: parse scores
        let mut score = String::new();
        let mut by = String::new();
        if let Some(score_res) = node.find(Attr("class", "score")).next() {
            score = score_res.text();
        }
        if let Some(by_res) = node.find(Attr("class", "hnuser")).next() {
            by = by_res.text();
        }
        let age = node
            .find(Attr("class", "age"))
            .next()
            .unwrap()
            .find(Name("a"))
            .next()
            .unwrap()
            .text();

        SubtextData { score, by, age }
    }
}
