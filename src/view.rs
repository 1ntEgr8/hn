use crate::fetcher::Story;
use std::io::{Write, stdout, stdin};
use termion::{clear, cursor};

struct Block {
    data: String,
    x: u16,
    y: u16,
    height: u16 
}

impl Block {
    fn new(data: String, x: u16, y: u16, height: u16) -> Block {
        Block {
            data, 
            x,
            y, 
            height
        }
    }
}

pub struct BlockContainer {
    // current location
    // functions to handle moving across locations 
    init_x: u16,
    init_y: u16,
    cursor_x: u16,
    cursor_y: u16,
    blocks: Vec<Block>,
    stdout: std::io::Stdout,
    stdin: std::io::Stdin
}

impl BlockContainer {
    pub fn new(init_x: u16, init_y: u16) -> BlockContainer {
        BlockContainer {
            init_x,
            init_y, 
            cursor_x: init_x,
            cursor_y: init_y,
            blocks: Vec::new(),
            stdout: stdout(),
            stdin: stdin()
        }
    }

    fn clear_tty(&mut self) {
        writeln!(self.stdout, "{clear}{goto}", 
            clear = clear::All,
            goto = cursor::Goto(self.init_x, self.init_y)).unwrap();
            self.cursor_x = self.init_x;
            self.cursor_y = self.init_y;
    } 

    pub fn display_stories(&mut self, stories: Vec<Story>) {
        self.clear_tty();
        for story in stories {
            let output = format!("> {}. {}\n", story.data.rank, story.data.title);
            let block = Block::new(output, self.cursor_x, self.cursor_y, 3);
            
            // writeln!(self.stdout, "{}", block.data).unwrap();
            self.blocks.push(block);
            // calculate the height of the block
            // create a new block
            // add the block to the vector
            // update the current x and current y values
            // repeat
        }
    }
}

