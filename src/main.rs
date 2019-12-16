use reqwest;
use std::error::Error;

fn main() {
    let res = fetch_stories();
    println!("success");
}

fn fetch_stories() -> Result<(), Box<dyn Error>> {
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";

    let body = reqwest::get(url)?
        .text()?;
    let ids = parse_body(&body);
    Ok(())
}

fn parse_body(body: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut it = body.trim()
        .chars();

    if it.next().unwrap() != '[' {
        panic!("invalid body passed in");
    }

    loop {
        let mut ch = it.next()
            .unwrap();
        if ch.is_digit(10) {
            let mut id = String::new();
            while ch != ',' && ch != ']' {
                id.push(ch);
                ch = it.next()
                    .unwrap();
            }
            result.push(id);
        }

        if ch == ']' {
            break
        }
    }

    result
}
