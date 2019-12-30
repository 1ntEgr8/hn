mod app;
mod fetcher;
mod view;

use app::App;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>; 

fn main() {
    let stdout = &mut enter_raw_mode(); // getting stdout for display function

    let app = App::init(); // initializing app

    display(stdout, &app);

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('q') => {
                clear(stdout);
                break;
            }
            _ => continue,
        }
    }
}

fn clear(stdout: &mut RawTerminal) {
    writeln!(
        stdout,
        "{clear}{goto}",
        clear = clear::All,
        goto = cursor::Goto(1, 1)
    ).unwrap()
}

fn display(stdout: &mut RawTerminal, app: &App) {
    // takes in stdout, and state
    // displays it based on flags
    writeln!(stdout, "Testing").unwrap();
}

fn enter_raw_mode() -> RawTerminal {
    stdout().into_raw_mode().unwrap()
}
