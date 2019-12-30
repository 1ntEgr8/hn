use crate::App;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn processKeyPress(stdout: &mut RawTerminal, app: &mut App) {
    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('q') => {
                clear(stdout);
                break;
            }
            Key::Char('j') | Key::Down => {
                if app.current_story_index < app.stories.len() {
                    app.cursor_y += 4;
                    app.current_story_index += 1;
                }
            }
            Key::Char('k') | Key::Up => {
                if app.current_story_index > 0 {
                    app.cursor_y -= 4;
                    app.current_story_index -= 1;
                }
            }
            Key::Char('s') => {

            }
            _ => continue,
        }
        display(stdout, &app);
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

pub fn display(stdout: &mut RawTerminal, app: &App) {
    // takes in stdout, and state
    // displays it based on flags
    clear(stdout);
    writeln!(stdout, "x:{}, y:{}", app.cursor_x, app.cursor_y).unwrap();
    writeln!(stdout, "Testing").unwrap();
}

