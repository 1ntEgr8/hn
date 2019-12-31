mod app;
mod fetcher;
mod save;
mod view;

use crate::fetcher::{Story, HnFetcher};
use app::App;
use std::io::stdout;
use termion::raw::IntoRawMode;
use view::*;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

fn main() {
    // let stdout = &mut enter_raw_mode(); // getting stdout for display function

    // let mut app = App::init(); // initializing app

    // display(stdout, &app);
    // process_key_press(stdout, &mut app);
    let fetcher = HnFetcher::new(String::from("https://news.ycombinator.com/"));
    let stories = fetcher.fetch_stories();
    let conn = &save::get_store();
    save::add_story(conn, &stories[0]).unwrap();
    save::add_story(conn, &stories[1]).unwrap();
    let stories = save::get_visited_stories(conn);
    for story in stories {
        println!("{:#?}", story);
    }
}

// fn enter_raw_mode() -> RawTerminal {
//     stdout().into_raw_mode().unwrap()
// }

/*
    Next steps
        implement save for later functionality

        on init()
            setup database connection

*/
