use crate::*;
use crate::comment::Comment;
use crate::thread::Thread;

#[derive(FromForm, Clone)]
pub struct QueryForm {
    filter: String,
    query: String,
}

#[get("/")]
pub async fn page(user: Option<User>) -> ApiResult<Html<String>> {
    let template = template::Search { user };
    Ok(Html(template.render_once()?))
}

pub async fn query_threads(db: Db, query: Form<QueryForm>) -> ApiResult<Vec<Thread>> {
    let threads = db
        .run(|conn| {
            let sql = format!(
                "SELECT title, users.email, categories, published, threads.id
                 FROM threads 
                 INNER JOIN users
                 ON threads.user_id = users.id
                 WHERE title LIKE \"%{}%\"",
                query.into_inner().query
            );
            let mut stmt = conn.prepare(&sql).unwrap();
            stmt.query_map([], |row| {
                let published: String = row.get(3)?;
                Ok(thread::Thread {
                    title: row.get(0)?,
                    email: row.get(1)?,
                    categories: row.get(2)?,
                    published: utils::time_ago(&published),
                    id: row.get(4)?,
                })
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
        })
        .await?;
    Ok(threads)
}

pub async fn query_comments(db: Db, query: Form<QueryForm>) -> ApiResult<Vec<Comment>> {
    let mut comments = db
        .run(move |conn| {
            let sql = format!(
                "SELECT users.email, comments.body, comments.published, comments.id, comments.thread_id
                FROM comments
                INNER JOIN users
                ON users.id = comments.user_id
                WHERE comments.body like \"%{}%\";",
                query.into_inner().query,
            );
            let mut stmt = conn.prepare(&sql).unwrap();
            stmt.query_map([], |row| {
                let published: String = row.get(2)?;
                Ok(comment::Comment {
                    email: row.get(0)?,
                    body: row.get(1)?,
                    published: utils::time_ago(&published),
                    id: row.get(3)?,
                    thread_id: row.get(4)?,
                })
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
        })
        .await?;
    comments.iter_mut().for_each(Comment::parse_markdown);

    Ok(comments)
}

pub async fn query_categories(db: Db, query: Form<QueryForm>) -> ApiResult<Vec<Thread>> {
    let threads = db
        .run(|conn| {
            let sql = format!(
                "SELECT title, users.email, categories, published, threads.id
                 FROM threads 
                 INNER JOIN users
                 ON threads.user_id = users.id
                 WHERE threads.categories LIKE \"%{}%\"",
                query.into_inner().query
            );
            let mut stmt = conn.prepare(&sql).unwrap();
            stmt.query_map([], |row| {
                let published: String = row.get(3)?;
                Ok(thread::Thread {
                    title: row.get(0)?,
                    email: row.get(1)?,
                    categories: row.get(2)?,
                    published: utils::time_ago(&published),
                    id: row.get(4)?,
                })
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
        })
        .await?;
    Ok(threads)
}

#[post("/", data = "<query>")]
pub async fn view_query(db: Db, user: Option<User>, mut query: Form<QueryForm>) -> ApiResult<Html<String>> {
    // Clean input
    query.query = clean(&query.query);
    if query.query.is_empty() {
        return Err(ApiError::InvalidInput);
    }
    let filter = &query.filter;
    match filter.as_str() {
        "comments" => {
            let comments = query_comments(db, query).await?;
            let template = template::SearchComments { user, comments };
            Ok(Html(template.render_once()?))
        }
        "categories" => {
            let threads = query_categories(db, query).await?;
            let template = template::Index { user, threads };
            Ok(Html(template.render_once()?))
        }
        "threads" | _ => {
            let threads = query_threads(db, query).await?;
            let template = template::Index { user, threads };
            Ok(Html(template.render_once()?))
        }
    }
}
