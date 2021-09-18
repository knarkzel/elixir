use crate::*;
use pulldown_cmark::{html, Options, Parser};

#[derive(Debug)]
pub struct Thread {
    pub title: String,
    pub email: String,
    pub categories: String,
    pub published: String,
    pub id: i64,
}

#[derive(FromForm, Clone)]
pub struct ThreadForm {
    title: String,
    categories: String,
    body: String,
}

#[get("/create")]
pub fn create_page(user: User) -> ApiResult<Html<String>> {
    let template = template::ThreadCreate { user: Some(user) };
    Ok(Html(template.render_once()?))
}

#[post("/create", data = "<thread>")]
pub async fn create(db: Db, user: User, thread: Form<ThreadForm>) -> ApiResult<Redirect> {
    let time = Utc::now();
    let user_id = user.id();
    let thread = thread.into_inner();

    // Create thread.
    let (title, categories, published) = {
        let thread = thread.clone();
        (thread.title, thread.categories, time.to_string())
    };
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO threads (title, categories, user_id, published) VALUES (?1, ?2, ?3, ?4)",
            params![title, categories, user_id, published,],
        )
    })
    .await?;

    // Create comment.
    let thread_id = db.run(|conn| conn.last_insert_rowid()).await;
    let (body, published) = {
        let thread = thread.clone();
        (thread.body, time.to_string())
    };
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO comments (thread_id, user_id, body, published) VALUES (?1, ?2, ?3, ?4)",
            params![thread_id, user_id, body, published],
        )
    })
    .await?;

    Ok(Redirect::to("/"))
}

#[get("/<id>")]
pub async fn view_page(db: Db, user: Option<User>, id: i64) -> ApiResult<Html<String>> {
    let thread = db
        .run(move |conn| {
            let sql = format!(
                "SELECT title, users.email, categories, published
                 FROM threads 
                 INNER JOIN users
                 ON threads.user_id = users.id
                 WHERE threads.id = {};",
                id,
            );
            let mut stmt = conn.prepare(&sql).unwrap();
            stmt.query_row([], |row| {
                let published: String = row.get(3)?;
                Ok(Thread {
                    title: row.get(0)?,
                    email: row.get(1)?,
                    categories: row.get(2)?,
                    published: utils::time_ago(&published),
                    id,
                })
            })
        })
        .await?;

    let mut comments = db
        .run(move |conn| {
            let sql = format!(
                "SELECT users.email, comments.body, comments.published
                FROM threads
                INNER JOIN comments
                ON threads.id = comments.thread_id
                INNER JOIN users
                ON users.id = comments.user_id
                WHERE threads.id = {};",
                id,
            );
            let mut stmt = conn.prepare(&sql).unwrap();
            stmt.query_map([], |row| {
                let published: String = row.get(2)?;
                Ok(comment::Comment {
                    email: row.get(0)?,
                    body: row.get(1)?,
                    published: utils::time_ago(&published),
                })
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
        })
        .await?;

    // transform for each comments -> comment.body with markdown
    for comment in &mut comments {
        let mut buffer = String::new();
        let parser = Parser::new_ext(&comment.body, Options::all());
        html::push_html(&mut buffer, parser);
        comment.body = buffer;
    }

    let template = template::ThreadView {
        user,
        thread,
        comments,
    };

    Ok(Html(template.render_once()?))
}
