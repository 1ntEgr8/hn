use crate::fetcher::Story;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

/*
    ViewConfig

        Should be responsible for handling display only!!
        
        There should be another struct that stores 
            the state of 
                blocks
                cursor
            ViewConfig should take in the state, and display that
            shouldnt be modifying blocks
*/

/*
    YAYYY
    we are going to be using a database to store information

    id -> i32
    title -> story title
    url -> url of the story
    time_stamp -> refer documentation
    is_visited -> bool

    implementing save story feature
        * create a separate file (if it doesnt exist)
            that stores a list of saved stories
            * basically store all meta data that you need
        * append stories, have flags on whether visited or not
        * a story is visited if
            * the user presses enter
        * a story is saved if
            * the user presses s
        
        If a story is visited
            * muted color, with green check emoji
        If a story is saved
            * emoji for saved, and no bolding
*/

/*
    Lifecyle of the application
    
        * startup
            * load stories
                * validate stories in database
                * style based on response
            * upon interaction
                * add to database
                    * set flag is_visited to true if enter
                    * false if saved

    Each View consists of pages
        Each page consists of Blocks

    Flow
        when user presses key
            process key press
            'j' => move cursor down
            'k' => move cursor up
            'enter' => {
                open page in browser
                add block to database
                mark block as visited
            }
            's' => {
                add block to database
                mark block as not visited
            }
            pass state to display function
                displays the stories

    Refactor steps:
        Create a new struct that stores the current state of the app
        handle input from the main function
            
    });
*/

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

pub struct ViewConfig {
    cursor_x: u16,
    cursor_y: u16,
    current_block_index: usize,
    blocks: Vec<Block>,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
    dimensions: (u16, u16),
    start_block_index: usize,
    page_capacity: usize,
}

impl ViewConfig {
    pub fn new(init_x: u16, init_y: u16) -> ViewConfig {
        ViewConfig {
            cursor_x: init_x,
            cursor_y: init_y,
            current_block_index: 0,
            blocks: Vec::new(),
            stdout: stdout().into_raw_mode().unwrap(),
            dimensions: termion::terminal_size().unwrap(),
            start_block_index: 0,
            page_capacity: 0,
        }
    }

    pub fn init_display(&mut self, stories: Vec<Story>) {
        for story in stories {
            self.blocks.push(self.get_block(&story));
        }
        self.display_page(self.start_block_index);
        self.handle_input();
    }

    fn clear(&mut self) {
        writeln!(
            self.stdout,
            "{clear}{goto}",
            clear = clear::All,
            goto = cursor::Goto(1, 1)
        )
        .unwrap();
        self.cursor_x = 1;
        self.cursor_y = 1;

        self.update_dimensions();
    }

    fn update_dimensions(&mut self) {
        self.dimensions = termion::terminal_size().unwrap();

        let height = self.dimensions.1 - 5;
        self.page_capacity = height as usize / 3;
    }

    fn display_page(&mut self, page: usize) {
        self.clear();

        self.print_title();

        self.start_block_index = page * self.page_capacity;
        self.current_block_index = self.start_block_index; // is modified by cursor movements

        let mut cap = self.blocks.len();
        if self.start_block_index + self.page_capacity < self.blocks.len() {
            cap = self.start_block_index + self.page_capacity;
        }

        for i in self.start_block_index..cap {
            self.move_cursor_to(self.cursor_x, self.cursor_y);
            writeln!(self.stdout, "{}", self.blocks[i].data).unwrap();
            self.cursor_y += self.blocks[i].height;
        }

        self.stdout.flush().unwrap();
    }

    fn handle_input(&mut self) {
        self.move_cursor_to(1, 1 + 4);

        let stdin = stdin();
        let mut curr_dim;

        for c in stdin.keys() {
            curr_dim = termion::terminal_size().unwrap();

            if curr_dim != self.dimensions {
                self.update_dimensions();
                self.display_page(0);
                self.move_cursor_to(
                    1,
                    1 + 4
                ); 
            }

            match c.unwrap() {
                Key::Char('q') => {
                    self.clear();
                    break;
                }
                Key::Char('j') | Key::Down => {
                    self.move_cursor_down();
                }
                Key::Char('k') | Key::Up => {
                    self.move_cursor_up();
                }
                Key::Char('s') => {
                    // save story

                }
                Key::Char('\n') => {
                    Command::new("open")
                        .arg(self.blocks[self.current_block_index].url.as_str())
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
        if self.blocks.len() == 0 || self.current_block_index == 0 {
            return;
        }
        if self.current_block_index == self.start_block_index {
            self.display_page(self.start_block_index / self.page_capacity - 1);
            self.move_cursor_to(
                1,
                1 + 4
            ); 
        } else {
            self.move_cursor_to(
                self.cursor_x,
                self.cursor_y - self.blocks[self.current_block_index].height,
            );
            self.current_block_index -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.blocks.len() == 0 || self.current_block_index >= self.blocks.len() - 1 {
            return;
        }

        if self.current_block_index == self.start_block_index + self.page_capacity - 1 {
            self.display_page(self.start_block_index / self.page_capacity + 1);
            self.move_cursor_to(
                1,
                1 + 4
            ); 
        } else {
            self.move_cursor_to(
                self.cursor_x,
                self.cursor_y + self.blocks[self.current_block_index].height,
            );
            self.current_block_index += 1;
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

    fn print_title(&mut self) {
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
    }
}
