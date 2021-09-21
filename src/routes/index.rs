use crate::*;

#[get("/")]
pub async fn page(db: Db, user: Option<User>) -> ApiResult<Html<String>> {
    let threads = db
        .run(|conn| {
            let mut stmt = conn
                .prepare(
                    "SELECT title, users.email, categories, published, threads.id
                     FROM threads 
                     INNER JOIN users
                     ON threads.user_id = users.id;",
                )
                .unwrap();
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

#[get("/login")]
pub fn login_page() -> ApiResult<Html<String>> {
    let template = template::Login { user: None };
    Ok(Html(template.render_once()?))
}

#[post("/login", data = "<form>")]
pub async fn login(mut auth: Auth<'_>, form: Form<Login>) -> ApiResult<Redirect> {
    if form.email.chars().any(|c| !c.is_ascii_alphanumeric()) {
        return Err(ApiError::InvalidInput);
    }
    auth.login(&form).await?;
    Ok(Redirect::to("/"))
}

#[get("/register")]
pub fn register_page() -> ApiResult<Html<String>> {
    let template = template::Register { user: None };
    Ok(Html(template.render_once()?))
}

#[post("/register", data = "<form>")]
pub async fn register(mut auth: Auth<'_>, form: Form<Signup>) -> ApiResult<Redirect> {
    if form.email.chars().any(|c| !c.is_ascii_alphanumeric()) {
        return Err(ApiError::InvalidInput);
    }
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok(Redirect::to("/"))
}
