use crate::*;

#[get("/")]
pub async fn index_page(user: Option<User>) -> Result<Html<String>> {
    let template = template::Index { user };
    Ok(Html(template.render_once()?))
}

#[get("/login")]
pub fn login_page() -> Result<Html<String>> {
    let template = template::Login { user: None };
    Ok(Html(template.render_once()?))
}

#[post("/login", data = "<form>")]
pub async fn login(mut auth: Auth<'_>, form: Form<Login>) -> Result<Redirect> {
    auth.login(&form).await.context("Failed to login")?;
    Ok(Redirect::to(uri!("/")))
}

#[get("/register")]
pub fn register_page() -> Result<Html<String>> {
    let template = template::Register { user: None };
    Ok(Html(template.render_once()?))
}

#[post("/register", data = "<form>")]
pub async fn register(mut auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok(Redirect::to(uri!("/")))
}
