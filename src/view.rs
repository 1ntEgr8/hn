use crate::fetcher::Story;
use crate::save;
use crate::App;
use std::io::{stdin, Write};
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, color, cursor, style};

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn display(stdout: &mut RawTerminal, app: &App) {
    clear(stdout);

    if app.is_main_screen {
        print_title(stdout); // printing "hackernews"
    } else {
        print_header(stdout, app);
    }

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
    out.push_str(get_message_bar(&app).as_str());
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
            Key::Char('l') => {
                // show reading list
                app.header = String::from("Reading List");
                if app.is_main_screen {
                    app.is_main_screen = false;
                    let stories_save = save::get_saved_stories_exclusive(&app.conn);
                    let stories = save::story_save_to_stories(stories_save);
                    app.stories = stories;
                }
            }
            Key::Char('h') => {
                // show history
                app.header = String::from("History");
                if app.is_main_screen {
                    app.is_main_screen = false;
                    app.message = String::from("All stories you've interacted with");
                    let stories_save = save::get_all_interacted_stories(&app.conn);
                    let stories = save::story_save_to_stories(stories_save);
                    app.stories = stories;
                }
            }
            Key::Char('f') => {
                // show all saved stories
                app.header = String::from("Saved List");
                if app.is_main_screen {
                    app.is_main_screen = false;
                    app.message = String::from("All stories you've saved");
                    let stories_save = save::get_saved_stories(&app.conn);
                    let stories = save::story_save_to_stories(stories_save);
                    app.stories = stories;
                }
            }
            Key::Char('v') => {
                // show all visited stories
                app.header = String::from("Visited List");
                if app.is_main_screen {
                    app.is_main_screen = false;
                    app.message = String::from("All stories you've visited");
                    let stories_save = save::get_visited_stories(&app.conn);
                    let stories = save::story_save_to_stories(stories_save);
                    app.stories = stories;
                }
            }
            Key::Char('b') => {
                if !app.is_main_screen {
                    app.message = String::from("Back to home");
                    app.is_main_screen = true;
                    app.refresh();
                }
            }
            Key::Char('s') => {
                if app.is_main_screen {
                    app.stories[app.current_story_index].is_saved = true;
                    app.message = String::from("Saved!");
                    save::add_story(&app.conn, &app.stories[app.current_story_index]).unwrap();
                }
            }
            Key::Char('r') => {
                if app.is_main_screen {
                    app.message = String::from("Refreshed");
                    app.refresh();
                }
            }
            Key::Char('\n') => {
                app.stories[app.current_story_index].is_visited = true;
                app.message = String::from("Visited");
                save::add_story(&app.conn, &app.stories[app.current_story_index]).unwrap();
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

fn print_header(stdout: &mut RawTerminal, app: &App) {
    writeln!(
        stdout,
        "{frame}{bold}{color}{title}",
        frame = style::Framed,
        bold = style::Bold,
        color = color::Fg(color::Rgb(255, 132, 2)),
        title = format!("\n\r( ̲̅:̲̅:̲̅:̲̅[̲̅ ̲̅]̲̅:̲̅:̲̅:̲̅ ) {} ( ̲̅:̲̅:̲̅:̲̅[̲̅ ̲̅]̲̅:̲̅:̲̅:̲̅ )\n\r\n\r", app.header)
    )
    .unwrap(); 
}

fn get_story_string(story: &Story) -> String {
    if story.data.rank == 0 {
        format!(
            " {status} {yellow}{data}\n\r\n\r\n\r",
            status = if story.is_visited {
                "✅"
            } else if story.is_saved {
                "💾"
            } else {
                "  "
            },
            yellow = color::Fg(color::Yellow),
            data = story.data.title,
        )
    } else {
        format!(
            "{bold}{blue} {status} {rank}. {yellow}{data}\n\r       {sub}\n\r\n\r",
            bold = style::Bold,
            blue = color::Fg(color::Blue),
            status = if story.is_visited {
                "✅"
            } else if story.is_saved {
                "💾"
            } else {
                "  "
            },
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
}

fn get_status_bar(app: &App) -> String {
    format!(
        "{white}[{num}/{denom}] | Last refresh: {time} | 'q' to exit {reset}\n\r\n\r",
        white = color::Fg(color::White),
        time = app.last_refresh.format("%a %b %e %T %Y"),
        num = app.current_story_index + 1,
        denom = app.stories.len(),
        reset = style::Reset
    )
}

fn get_message_bar(app: &App) -> String {
    format!(
        "{red}[ {msg} ]{reset}",
        red = color::Fg(color::Red),
        msg = app.message, 
        reset = style::Reset
    ) 
}
