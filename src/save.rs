use crate::fetcher::Story;
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct StorySave {
    title: String,
    url: String,
}

fn establish_connection() -> Connection {
    Connection::open("hn_history.db").unwrap()
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

pub fn add_story(conn: &Connection, story: &Story) -> Result<usize> {
    // TODO: check if the record already exists
    conn.execute(
        "INSERT INTO stories (title, url, is_visited, is_saved)
        VALUES (?1, ?2, ?3, ?4)",
        params![
            &story.data.title,
            &story.data.url,
            if story.is_visited { 1 } else { 0 },
            if story.is_saved { 1 } else { 0 },
        ],
    )
}

pub fn get_visited_stories(conn: &Connection) -> Vec<StorySave> {
    let mut stmt = conn
        .prepare("SELECT title, url FROM stories WHERE is_saved = 0 AND is_visited = 0")
        .unwrap();
    let result = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(StorySave {
                title: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
            })
        })
        .unwrap();
    let mut stories = Vec::new();
    for story in result {
        stories.push(story.unwrap())
    }
    stories
}

fn get_saved_stories(conn: &Connection) -> Vec<StorySave> {
    let mut stmt = conn
        .prepare("SELECT title, url FROM stories WHERE is_visited = 1 AND is_saved = 0")
        .unwrap();
        let result = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(StorySave {
                title: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
            })
        })
        .unwrap();
    let mut stories = Vec::new();
    for story in result {
        stories.push(story.unwrap())
    }
    stories
}

// a story is "archived" if it is both saved and visited
fn get_archived_stories(conn: &Connection) -> Vec<StorySave>{
    let mut stmt = conn
        .prepare("SELECT title, url FROM stories WHERE is_visited = 1 AND is_saved = 1")
        .unwrap();
        let result = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(StorySave {
                title: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
            })
        })
        .unwrap();
    let mut stories = Vec::new();
    for story in result {
        stories.push(story.unwrap())
    }
    stories
}
