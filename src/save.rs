use crate::fetcher::Story;
use std::path::Path;
use std::fs;
use rusqlite::{params, Connection, Result, Statement, NO_PARAMS};

#[derive(Debug)]
pub struct StorySave {
    pub title: String,
    pub url: String,
    pub is_visited: bool,
    pub is_saved: bool,
}

pub fn get_store() -> Connection {
    let conn = establish_connection();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS stories (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL UNIQUE, 
            url TEXT NOT NULL UNIQUE,
            is_visited INTEGER NOT NULL,
            is_saved INTEGER NOT NULL
        )",
        NO_PARAMS,
    )
    .unwrap();
    conn
}

pub fn does_exist(conn: &Connection, story: &Story) -> bool {
    let temp = conn.prepare(
        format!(
            "SELECT * FROM stories where title = \"{}\" and url = \"{}\"",
            story.data.title.replace("\"", "\"\""),
            story.data.url
        )
        .as_str(),
    );
    match temp {
        Ok(mut stmt) => stmt.exists(NO_PARAMS).unwrap(),
        _ => {
            println!("ERROR\n{:#?}", story);
            false
        }
    }
}

pub fn add_story(conn: &Connection, story: &Story) -> Result<usize> {
    if !does_exist(conn, story) {
        conn.execute(
            "INSERT INTO stories (title, url, is_visited, is_saved)
        VALUES (?1, ?2, ?3, ?4)",
            params![
                &story.data.title.replace("\"", "\"\""),
                &story.data.url,
                if story.is_visited { 1 } else { 0 },
                if story.is_saved { 1 } else { 0 },
            ],
        )
    } else {
        conn.execute(
            "UPDATE stories
        SET title = (?1), url = (?2), is_visited = (?3), is_saved = (?4)
        WHERE title = (?1) AND url = (?2)",
            params![
                &story.data.title.replace("\"", "\"\""),
                &story.data.url,
                if story.is_visited { 1 } else { 0 },
                if story.is_saved { 1 } else { 0 },
            ],
        )
    }
}

pub fn get_visited_stories(conn: &Connection) -> Vec<StorySave> {
    query(
        conn,
        "SELECT title, url, is_visited, is_saved FROM stories WHERE is_visited = 1",
    )
}

pub fn get_saved_stories(conn: &Connection) -> Vec<StorySave> {
    query(
        conn,
        "SELECT title, url, is_visited, is_saved FROM stories WHERE is_saved = 1",
    )
}

pub fn get_all_interacted_stories(conn: &Connection) -> Vec<StorySave> {
    query(conn,"SELECT title, url, is_visited, is_saved FROM stories")
}

pub fn get_saved_stories_exclusive(conn: &Connection) -> Vec<StorySave> {
    query(conn,"SELECT title, url, is_visited, is_saved FROM stories WHERE is_visited = 0 AND is_saved = 1")
}

pub fn get_story(conn: &Connection, story: &Story) -> StorySave {
    let story = &query(conn, format!("SELECT title, url, is_visited, is_saved FROM stories where title = \"{}\" and url = \"{}\"", story.data.title.replace("\"", "\"\""), story.data.url).as_str())[0];
    StorySave {
        title: story.title.clone().replace("\"\"", "\""),
        url: story.url.clone(),
        is_visited: story.is_visited,
        is_saved: story.is_saved,
    }
}

pub fn story_save_to_stories(stories_save: Vec<StorySave>) -> Vec<Story> {
    let stories = stories_save.iter().map(|story_save| {
        Story::from_story_save(story_save)
    }).collect();
    stories
}

fn establish_connection() -> Connection {
    if !Path::new("~/.hn_data").is_dir() {
        fs::create_dir("~/.hn_data").unwrap(); 
    }
    Connection::open(".hn_data/hn_history.db").unwrap()
}

fn query(conn: &Connection, command: &str) -> Vec<StorySave> {
    collect_query(&mut conn.prepare(command).unwrap())
}

fn collect_query(stmt: &mut Statement) -> Vec<StorySave> {
    let result = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(StorySave {
                title: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
                is_visited: row.get::<_, i32>(2).unwrap() == 1,
                is_saved: row.get::<_, i32>(3).unwrap() == 1,
            })
        })
        .unwrap();
    let mut stories = Vec::new();
    for story in result {
        stories.push(story.unwrap())
    }
    stories
}
