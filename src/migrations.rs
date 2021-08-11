use rusqlite::{Connection, Result};

pub fn install() -> Result<()> {
    let conn = Connection::open(crate::URL)?;

    // THREADS
    conn.execute(
        "CREATE TABLE IF NOT EXISTS threads (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            categories TEXT,
            title TEXT NOT NULL,
            created_by INTEGER NOT NULL,
            published TEXT NOT NULL
        );",
        [],
    )?;

    Ok(())
}

fn uninstall() -> Result<()> {
    let conn = Connection::open(crate::URL)?;

    // THREADS
    conn.execute("DROP TABLE IF EXISTS threads;", [])?;

    Ok(())
}
