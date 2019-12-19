mod fetcher;
mod view;

use fetcher::HnFetcher;
use view::BlockContainer;

fn main() {
    let fetcher = HnFetcher::new(String::from("https://news.ycombinator.com/"));
    let mut display = BlockContainer::new(1, 1);
    let stories = fetcher.fetch_stories();

    display.display_stories(stories);
    display.handle_input();
}

