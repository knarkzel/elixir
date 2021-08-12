use crate::*;

#[derive(FromForm, Clone)]
pub struct Thread {
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
pub async fn create(db: Db, user: User, thread: Form<Thread>) -> ApiResult<Redirect> {
    let user_id = user.id();
    let time = Utc::now();
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

    Ok(Redirect::to(uri!("/")))
}
