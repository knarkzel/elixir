pub const CREATE_THREADS: &str = "CREATE TABLE IF NOT EXISTS threads (
                                    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                                    user_id INTEGER NOT NULL,
                                    categories TEXT,
                                    title TEXT NOT NULL,
                                    published TEXT NOT NULL
                                );";

pub const CREATE_COMMENTS: &str = "CREATE TABLE IF NOT EXISTS comments (
                                      id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                                      thread_id INTEGER NOT NULL,
                                      user_id INTEGER NOT NULL,
                                      body TEXT NOT NULL,
                                      published TEXT NOT NULL
                                  );";
