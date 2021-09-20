use crate::*;

#[derive(FromForm, Clone)]
pub struct QueryForm {
    query: String,
}

#[get("/")]
pub async fn page(user: Option<User>) -> ApiResult<Html<String>> {
    let template = template::Search { user };
    Ok(Html(template.render_once()?))
}

#[post("/", data = "<query>")]
pub async fn view_query(db: Db, user: Option<User>, query: Form<QueryForm>) -> ApiResult<Html<String>> {
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

    let template = template::Index { user, threads };
    Ok(Html(template.render_once()?))
}
