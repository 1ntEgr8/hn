mod actions;
use actions::Fetcher;

fn main() {
    let fetcher = Fetcher::new(String::from("https://hacker-news.firebaseio.com/v0/"));

    let res = fetcher.fetch_stories().unwrap();
}
