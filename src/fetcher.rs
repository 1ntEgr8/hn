use select::document::Document;
use select::predicate::{Attr, Name};

// forget about using the API for now because it is slow
// work on scraping the webpage, displaying the content
// then work on getting data from the API

#[derive(Debug)]
struct TitleData {
    rank: i32,
    title: String,
    url: String,
}

#[derive(Debug)]
struct SubtextData {
    score: i32,
    by: String,
    age: String,
}

#[derive(Debug)]
pub struct Story {
    data: TitleData,
    sub: SubtextData,
}

pub struct HnFetcher {
    base_url: String,
}

impl HnFetcher {
    pub fn new(base_url: String) -> HnFetcher {
        HnFetcher { base_url }
    }

    pub fn fetch_stories(&self) -> Vec<Story> {
        let mut stories = Vec::new();
        let document = Document::from(include_str!("hn.html"));
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
        let title = title_node.text();
        let url = title_node.attr("href").unwrap();
        TitleData {
            rank,
            title,
            url: String::from(url),
        }
    }

    fn get_subtext_data(&self, node: select::node::Node<'_>) -> SubtextData {
        let score = node.find(Attr("class", "score")).next().unwrap().text();
        let by = node.find(Attr("class", "hnuser")).next().unwrap().text();
        let age = node
            .find(Attr("class", "age"))
            .next()
            .unwrap()
            .find(Name("a"))
            .next()
            .unwrap()
            .text();

        SubtextData { score: 1, by, age }
    }
}
