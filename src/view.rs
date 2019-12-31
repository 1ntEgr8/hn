use crate::fetcher::Story;
use crate::App;
use std::io::{stdin, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, color, cursor, style};

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn display(stdout: &mut RawTerminal, app: &App) {
    clear(stdout);

    print_title(stdout); // printing "hackernews"

    let term_dimensions = termion::terminal_size().unwrap();
    let height = term_dimensions.1 - 6; // 5 for title "hackernews" + padding; 1 for status bar
    let page_capacity = (height as usize / 3) - 1; // 3 is the height of each story

    // determine which page to show
    let page = (app.current_story_index / page_capacity) + 1;
    let start_index = page_capacity * (page - 1);

    // bounds on stories index
    let cap = if start_index + page_capacity < app.stories.len() {
        start_index + page_capacity
    } else {
        app.stories.len()
    };

    // create one big string to print out
    let mut out = String::new();
    for i in start_index..cap {
        out.push_str(get_story_string(&app.stories[i]).as_str());
    }
    out.push_str(get_status_bar(&app).as_str());
    writeln!(stdout, "{}", out).unwrap();
    writeln!(
        stdout,
        "{goto}",
        goto = cursor::Goto(1, 5 + 3 * (app.current_story_index - start_index) as u16)
    )
    .unwrap();
    stdout.flush().unwrap();
}

pub fn process_key_press(stdout: &mut RawTerminal, app: &mut App) {
    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('q') => {
                clear(stdout);
                break;
            }
            Key::Char('j') | Key::Down => {
                if app.current_story_index < app.stories.len() - 1 {
                    app.current_story_index += 1;
                }
            }
            Key::Char('k') | Key::Up => {
                if app.current_story_index > 0 {
                    app.current_story_index -= 1;
                }
            }
            Key::Char('s') => {}
            Key::Char('\n') => {
                Command::new("open")
                        .arg(app.stories[app.current_story_index].data.url.as_str())
                        .output()
                        .expect("Something went wrong with opening the page");
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
    )
    .unwrap()
}

fn print_title(stdout: &mut RawTerminal) {
    writeln!(
        stdout,
        "{frame}{bold}{color}{title}",
        frame = style::Framed,
        bold = style::Bold,
        color = color::Fg(color::Rgb(255, 132, 2)),
        title = "╔══════════════╗\n\r   HACKERNEWS \n\r╚══════════════╝\n\r"
    )
    .unwrap();
}

fn get_story_string(story: &Story) -> String {
    format!(
        "{bold}{blue}{rank}. {yellow}{data}\n\r   {sub}\n\r\n\r",
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
    )
}

fn get_status_bar(app: &App) -> String {
    format!(
        "{white}[{num}/{denom}] | Last refresh: {time} | 'q' to exit | 's' to save for later{reset}",
        white = color::Fg(color::White),
        time = app.last_refresh.format("%a %b %e %T %Y"),
        num = app.current_story_index + 1,
        denom = app.stories.len(),
        reset = style::Reset
    )
}