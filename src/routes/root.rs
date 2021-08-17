use crate::*;

pub struct ThreadListing {
    pub title: String,
    pub email: String,
    pub categories: String,
    pub published: String,
    pub link: String,
}

#[get("/")]
pub async fn index_page(db: Db, user: Option<User>) -> ApiResult<Html<String>> {
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
                let title: String = row.get(0)?;
                let email: String = row.get(1)?;
                let href: i64 = row.get(4)?;
                let link = format!("/thread/{}", href);
                Ok(ThreadListing {
                    title,
                    email,
                    categories: row.get(2)?,
                    published: row.get(3)?,
                    link,
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
    auth.login(&form).await?;
    Ok(Redirect::to(uri!("/")))
}

#[get("/register")]
pub fn register_page() -> ApiResult<Html<String>> {
    let template = template::Register { user: None };
    Ok(Html(template.render_once()?))
}

#[post("/register", data = "<form>")]
pub async fn register(mut auth: Auth<'_>, form: Form<Signup>) -> ApiResult<Redirect> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok(Redirect::to(uri!("/")))
}
