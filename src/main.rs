mod app;
mod fetcher;
mod save;
mod view;

use app::App;
use std::io::stdout;
use termion::raw::IntoRawMode;
use view::*;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

fn main() {
    let stdout = &mut enter_raw_mode(); // getting stdout for display function

    let mut app = App::init(); // initializing app

    display(stdout, &app);
    process_key_press(stdout, &mut app);
}

fn enter_raw_mode() -> RawTerminal {
    stdout().into_raw_mode().unwrap()
}
