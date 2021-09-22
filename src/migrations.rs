pub fn install() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open(crate::URL)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS threads (
         id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
         user_id INTEGER NOT NULL,
         categories TEXT,
         title TEXT NOT NULL,
         published TEXT NOT NULL);",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS comments (
         id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
         thread_id INTEGER NOT NULL,
         user_id INTEGER NOT NULL,
         body TEXT NOT NULL,
         published TEXT NOT NULL);",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS threads_index ON threads(categories, title);",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS comments_index ON comments(body);",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS users_index ON users(email);",
        [],
    )?;

    Ok(())
}
