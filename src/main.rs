mod app;
mod fetcher;
mod view;
mod actions;

use app::App;
use actions::*;
use std::io::stdout;
use termion::raw::IntoRawMode;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>; 

fn main() {
    let stdout = &mut enter_raw_mode(); // getting stdout for display function

    let mut app = App::init(); // initializing app

    display(stdout, &app);
    processKeyPress(stdout, &mut app);
}

fn enter_raw_mode() -> RawTerminal {
    stdout().into_raw_mode().unwrap()
}
