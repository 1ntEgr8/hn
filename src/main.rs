mod fetcher;
mod view;

use fetcher::HnFetcher;
use view::BlockContainer;

/*
    create an instance of a fetcher
        fetch_stories
            will download the html of the page
            parse the html
            return a vec of story structs
    fetcher has the base url of the api
        will keep track of other stats as well
    
    today you are going to finish this project!!!!
*/

fn main() {
    let fetcher = HnFetcher::new(String::from(""));
    let mut display = BlockContainer::new(1, 1);
    let stories = fetcher.fetch_stories();

    display.display_stories(stories);
    display.handle_input();
}

