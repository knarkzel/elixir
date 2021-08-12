use crate::*;

pub fn install() -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open(crate::URL)?;

    conn.execute(sql::CREATE_THREADS, [])?;
    conn.execute(sql::CREATE_COMMENTS, [])?;

    Ok(())
}
