mod fetcher;
mod view;
mod app;

use app::App;

fn main() {
    let app = App::init();
    // loop that handles user input and displays stuff
        // pass in stdout to the display function
    

    // display.init_display(stories);
}

fn display(stdout: &termion::raw::RawTerminal<std::io::Stdout>, state: &State) {
    // takes in stdout, and state
    // displays it based on flags
}

fn enter_raw_mode() {

}
