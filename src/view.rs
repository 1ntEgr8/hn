use crate::fetcher::{Story, SubtextData, TitleData};
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

// TODOS:
//      * pagination
//      * determine the number of urls you can display per page
//      * styling the urls
//      * support for different commands

// bounds checking for move_up and move_down commands
// pagination support
// styling?

// lets work on styling right now

// print_title_data()
// print_subtext_data()
// calculate_height()

/*
    compute the maximum height of the terminal
*/

struct Block {
    data: String,
    x: u16,
    y: u16,
    height: u16,
}

impl Block {
    fn new(data: String, x: u16, y: u16, height: u16) -> Block {
        Block { data, x, y, height }
    }
}

pub struct BlockContainer {
    init_x: u16,
    init_y: u16,
    cursor_x: u16,
    cursor_y: u16,
    current_block: usize,
    blocks: Vec<Block>,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
    dimensions: (u16, u16),
}

impl BlockContainer {
    pub fn new(init_x: u16, init_y: u16) -> BlockContainer {
        BlockContainer {
            init_x,
            init_y,
            cursor_x: init_x,
            cursor_y: init_y,
            current_block: 0,
            blocks: Vec::new(),
            stdout: stdout().into_raw_mode().unwrap(),
            dimensions: termion::terminal_size().unwrap(),
        }
    }

    fn clear_tty(&mut self) {
        writeln!(
            self.stdout,
            "{clear}{goto}",
            clear = clear::All,
            goto = cursor::Goto(self.init_x, self.init_y)
        )
        .unwrap();
        self.cursor_x = self.init_x;
        self.cursor_y = self.init_y;
    }

    pub fn display_stories(&mut self, stories: Vec<Story>) {
        self.clear_tty();

        for i in 0..5 {
            let block = self.print_title_data(&stories[i].data);
            self.cursor_y += block.height;
            self.blocks.push(block)
        }
        self.stdout.flush().unwrap();
    }

    pub fn handle_input(&mut self) {
        self.move_cursor_to(self.init_x, self.init_y);
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    self.clear_tty();
                    break;
                }
                Key::Char('j') => {
                    self.move_cursor_down();
                }
                Key::Char('k') => {
                    self.move_cursor_up();
                }
                _ => continue,
            }
        }
    }

    fn print_title_data(&mut self, data: &TitleData) -> Block {
        let output = format!(
            "{goto}{bold}{blue}{rank}. {yellow}{data}",
            goto = cursor::Goto(self.cursor_x, self.cursor_y),
            bold = style::Bold,
            blue = color::Fg(color::Blue),
            rank = data.rank,
            yellow = color::Fg(color::Yellow),
            data = data.title
        );
        let block = Block::new(output, self.cursor_x, self.cursor_y, 5);
        writeln!(self.stdout, "{}", block.data).unwrap();
        block
    }

    fn print_subtext_data(&mut self) {}

    fn move_cursor_to(&mut self, x: u16, y: u16) {
        if x > self.dimensions.0 || y > self.dimensions.1 || x <= 0 || y <= 0 {
            return;
        }
        self.cursor_x = x;
        self.cursor_y = y;
        writeln!(
            self.stdout,
            "{goto}",
            goto = cursor::Goto(self.cursor_x, self.cursor_y)
        )
        .unwrap();
    }

    fn move_cursor_up(&mut self) {
        if self.blocks.len() == 0 || self.current_block == 0 {
            return;
        }
        self.move_cursor_to(
            self.cursor_x,
            self.cursor_y - self.blocks[self.current_block].height,
        );
        self.current_block -= 1;
    }
    fn move_cursor_down(&mut self) {
        if self.blocks.len() == 0 || self.current_block >= self.blocks.len() - 1 {
            return;
        }
        self.move_cursor_to(
            self.cursor_x,
            self.cursor_y + self.blocks[self.current_block].height,
        );
        self.current_block += 1;
    }
}
