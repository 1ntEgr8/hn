use crate::fetcher::Story;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

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
    current_page_index: usize,
    page_capacity: usize,
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
            current_page_index: 0,
            page_capacity: 0,
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
        self.bookkeeping();
    }

    fn bookkeeping(&mut self) {
        self.dimensions = termion::terminal_size().unwrap();

        let height = self.dimensions.1 - 5;
        self.page_capacity = height as usize / 3;
    }

    pub fn init_display(&mut self, stories: Vec<Story>) {
        for story in stories {
            self.blocks.push(self.get_block(&story));
        }
        self.display_page(self.current_page_index);
        self.handle_input();
    }

    pub fn display_page(&mut self, page: usize) {
        self.clear_tty();

        writeln!(
            self.stdout,
            "{frame}{bold}{color}{title}",
            frame = style::Framed,
            bold = style::Bold,
            color = color::Fg(color::Rgb(255, 132, 2)),
            title = "╔══════════════╗\n\r   HACKERNEWS \n\r╚══════════════╝"
        )
        .unwrap();

        self.cursor_y += 4; // padding after title

        self.current_page_index = page * self.page_capacity;
        self.current_block = self.current_page_index; // is modified by cursor movements

        let mut cap = self.blocks.len();
        if self.current_page_index + self.page_capacity < self.blocks.len() {
            cap = self.current_page_index + self.page_capacity;
        }

        for i in self.current_page_index..cap {
            self.move_cursor_to(self.cursor_x, self.cursor_y);
            writeln!(self.stdout, "{}", self.blocks[i].data).unwrap();
            self.cursor_y += self.blocks[i].height;
        }

        self.stdout.flush().unwrap();
    }

    pub fn handle_input(&mut self) {
        self.move_cursor_to(self.init_x, self.init_y + 4);
        let stdin = stdin();
        let mut curr_dim;
        for c in stdin.keys() {
            curr_dim = termion::terminal_size().unwrap();

            if curr_dim != self.dimensions {
                self.bookkeeping();
                self.display_page(0);
                self.move_cursor_to(
                    self.init_x,
                    self.init_y + 4
                ); 
            }

            match c.unwrap() {
                Key::Char('q') => {
                    self.clear_tty();
                    break;
                }
                Key::Char('j') | Key::Down => {
                    self.move_cursor_down();
                }
                Key::Char('k') | Key::Up => {
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
        if self.current_block == self.current_page_index {
            self.display_page(self.current_page_index / self.page_capacity - 1);
            self.move_cursor_to(
                self.init_x,
                self.init_y + 4
            ); 
        } else {
            self.move_cursor_to(
                self.cursor_x,
                self.cursor_y - self.blocks[self.current_block].height,
            );
            self.current_block -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.blocks.len() == 0 || self.current_block >= self.blocks.len() - 1 {
            return;
        }

        if self.current_block == self.current_page_index + self.page_capacity - 1 {
            self.display_page(self.current_page_index / self.page_capacity + 1);
            self.move_cursor_to(
                self.init_x,
                self.init_y + 4
            ); 
        } else {
            self.move_cursor_to(
                self.cursor_x,
                self.cursor_y + self.blocks[self.current_block].height,
            );
            self.current_block += 1;
        }
    }

    fn get_block(&self, story: &Story) -> Block {
        let output = format!(
            "{bold}{blue}{rank}. {yellow}{data}\n\r   {sub}",
            bold = style::Bold,
            blue = color::Fg(color::Blue),
            rank = story.data.rank,
            yellow = color::Fg(color::Yellow),
            data = story.data.title,
            sub = format!(
                "{}{} {}| {}by {} {}| {}",
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
        Block::new(output, url, 3)
    }
}
