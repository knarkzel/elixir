use crate::*;

#[derive(FromForm, Clone)]
pub struct Comment {
    body: String,
}

#[post("/create/<thread_id>", data = "<comment>")]
pub async fn create(db: Db, user: User, thread_id: i64, comment: Form<Comment>) -> ApiResult<Redirect> {
    let time = Utc::now();
    let user_id = user.id();

    // Create comment.
    let (body, published) = {
        (comment.into_inner().body, time.to_string())
    };
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO comments (thread_id, user_id, body, published) VALUES (?1, ?2, ?3, ?4)",
            params![thread_id, user_id, body, published],
        )
    })
    .await?;

    let thread_url = format!("/thread/{}", thread_id);
    Ok(Redirect::to(thread_url))
}
