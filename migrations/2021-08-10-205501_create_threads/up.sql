CREATE TABLE IF NOT EXISTS threads (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    categories TEXT,
    title TEXT NOT NULL,
    created_by INTEGER NOT NULL,
    published TEXT
);
