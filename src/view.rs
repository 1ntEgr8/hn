use crate::fetcher::Story;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

// TODOS:
//      * pagination
//      * determine the number of urls you can display per page
//      * integration with the Pocket API

struct Block {
    data: String,
    url: String, 
    height: u16,
}

impl Block {
    fn new(data: String, url: String, height: u16) -> Block {
        Block { data, url, height }
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

        writeln!(
            self.stdout,
            "{frame}{bold}{color}{title}",
            frame = style::Framed,
            bold = style::Bold,
            color = color::Fg(color::Rgb(255, 132, 2)),
            title = "╔══════════════╗\n\r   HACKERNEWS \n\r╚══════════════╝"
        ).unwrap();

        self.cursor_y += 5;

        for i in 0..10 {
            let block = self.print_block(&stories[i]);
            self.cursor_y += block.height;
            self.blocks.push(block)
        }
        self.stdout.flush().unwrap();
    }

    pub fn handle_input(&mut self) {
        self.move_cursor_to(self.init_x, self.init_y + 5);
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
                Key::Char('\n') => {
                    Command::new("open")
                        .arg(self.blocks[self.current_block].url.as_str())
                        .output()
                        .expect("Something went wrong with opening the page");
                }
                _ => continue,
            }
        }
    }

    fn print_block(&mut self, story: &Story) -> Block {
        let output = format!(
            "{goto1}{bold}{blue}{rank}. {yellow}{data}{goto2}{sub}",
            goto1 = cursor::Goto(self.cursor_x, self.cursor_y),
            bold = style::Bold,
            blue = color::Fg(color::Blue),
            rank = story.data.rank,
            yellow = color::Fg(color::Yellow),
            data = story.data.title,
            goto2 = cursor::Goto(self.cursor_x + 3, self.cursor_y + 1),
            sub = format!(
                "{}{} points {}| {}by {} {}| {}",
                color::Fg(color::Rgb(0, 224, 157)),
                story.sub.score,
                color::Fg(color::LightBlack),
                color::Fg(color::Rgb(183, 183, 183)),
                story.sub.by,
                color::Fg(color::LightBlack),
                story.sub.age
            )
        );
        let url = story.data.url.clone();
        let block = Block::new(output, url, 3);
        writeln!(self.stdout, "{}", block.data).unwrap();
        block
    }

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
