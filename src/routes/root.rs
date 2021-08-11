use crate::*;

#[get("/")]
pub async fn index_page(user: Option<User>) -> Html<String> {
    let template = template::Index { user };
    Html(template.render_once().unwrap())
}

#[get("/login")]
pub fn login_page() -> Html<String> {
    let template = template::Login { user: None };
    Html(template.render_once().unwrap())
}

#[post("/login", data = "<form>")]
pub async fn login(mut auth: Auth<'_>, form: Form<Login>) -> Redirect {
    redirect_error!(auth.login(&form).await);
    Redirect::to(uri!("/"))
}

#[get("/register")]
pub fn register_page() -> Html<String> {
    let template = template::Register { user: None };
    Html(template.render_once().unwrap())
}

#[post("/register", data = "<form>")]
pub async fn register(mut auth: Auth<'_>, form: Form<Signup>) -> Redirect {
    redirect_error!(auth.signup(&form).await);
    redirect_error!(auth.login(&form.into()).await);
    Redirect::to(uri!("/"))
}
