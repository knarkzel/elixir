use crate::*;
use pulldown_cmark::{html, Options, Parser};

#[derive(Debug)]
pub struct Comment {
    pub email: String,
    pub body: String,
    pub published: String,
    pub id: i64,
    pub thread_id: i64,
}

impl Comment {
    pub fn parse_markdown(&mut self) {
        let mut buffer = String::new();
        let parser = Parser::new_ext(&self.body, Options::all());
        html::push_html(&mut buffer, parser);
        self.body = buffer;
    }
}

#[derive(FromForm, Clone)]
pub struct CommentForm {
    body: String,
}

#[post("/create/<thread_id>", data = "<comment>")]
pub async fn create(
    db: Db,
    user: User,
    thread_id: i64,
    comment: Form<CommentForm>,
) -> ApiResult<Redirect> {
    let time = Utc::now();
    let user_id = user.id();

    // Create comment
    let (body, published) = { (comment.into_inner().body, time.to_string()) };
    // Clean input
    let body = clean(&body);
    // Insert into database
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO comments (thread_id, user_id, body, published) VALUES (?1, ?2, ?3, ?4)",
            params![thread_id, user_id, body, published],
        )
    })
    .await?;

    Ok(Redirect::to(format!("/thread/{}", thread_id)))
}
