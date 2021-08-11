use crate::*;

#[derive(FromForm, Clone)]
pub struct Thread {
    title: String,
    categories: String,
    body: String,
}

#[get("/create")]
pub fn thread_create_page(user: User) -> Html<String> {
    let template = template::ThreadCreate { user: Some(user) };
    Html(template.render_once().unwrap())
}

#[post("/create", data = "<new_thread>")]
pub async fn thread_create(db: DbConn, user: User, new_thread: Form<Thread>) -> Redirect {
    let utc: DateTime<Utc> = Utc::now();
    let inner = new_thread.into_inner();
    let user_id = user.id();

    // create thread
    let thread = inner.clone();
    let err = db
        .run(move |conn| {
            conn.execute(
                "INSERT INTO threads (title, categories, user_id, published) VALUES (?1, ?2, ?3, ?4)",
                params![thread.title, thread.categories, user_id, utc.to_string()],
            )
        })
        .await;
    redirect_error!(err);

    // create comment
    let thread = inner;
    let thread_id = db.run(|conn| conn.last_insert_rowid()).await;
    let err = db
        .run(move |conn| {
            conn.execute(
                "INSERT INTO comments (thread_id, user_id, body, published) VALUES (?1, ?2, ?3, ?4)",
                params![thread_id, user.id(), thread.body, utc.to_string()],
            )
        })
        .await;
    redirect_error!(err);

    Redirect::to(uri!("/"))
}
