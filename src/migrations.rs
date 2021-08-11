use rusqlite::{Connection, Result};

pub fn install() -> Result<()> {
    let conn = Connection::open(crate::URL)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS threads (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER NOT NULL,
            categories TEXT,
            title TEXT NOT NULL,
            published TEXT NOT NULL
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            thread_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            body TEXT NOT NULL,
            published TEXT NOT NULL
        );",
        [],
    )?;

    Ok(())
}

// fn uninstall() -> Result<()> {
//     let conn = Connection::open(crate::URL)?;
//     conn.execute("DROP TABLE IF EXISTS threads;", [])?;
//     conn.execute("DROP TABLE IF EXISTS comments;", [])?;
//     Ok(())
// }
