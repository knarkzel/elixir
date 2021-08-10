#[macro_use]
extern crate rocket;

use elixir::*;
use rocket::{form::Form, fs::FileServer, response::Redirect};
use rocket_auth::{Auth, Login, Signup, User, Users};
use rocket_sync_db_pools::database;

#[database("main")]
struct DbConn(rusqlite::Connection);

#[get("/")]
async fn index_page(user: Option<User>) -> Html<String> {
    let template = template::Index { user };
    Html(template.render_once().unwrap())
}

#[get("/error/<cause>")]
fn error_page(cause: String, user: Option<User>) -> Html<String> {
    let template = template::Error { user, cause };
    Html(template.render_once().unwrap())
}

macro_rules! redirect_error {
    ($value:expr) => {
        if let Err(e) = $value {
            return Redirect::to(uri!(error_page(e.to_string())));
        }
    };
}

#[get("/login")]
fn login_page() -> Html<String> {
    let template = template::Login { user: None };
    Html(template.render_once().unwrap())
}

#[post("/login", data = "<form>")]
async fn login(mut auth: Auth<'_>, form: Form<Login>) -> Redirect {
    redirect_error!(auth.login(&form).await);
    Redirect::to(uri!("/"))
}

#[get("/register")]
fn register_page() -> Html<String> {
    let template = template::Register { user: None };
    Html(template.render_once().unwrap())
}

#[post("/register", data = "<form>")]
async fn register(mut auth: Auth<'_>, form: Form<Signup>) -> Redirect {
    redirect_error!(auth.signup(&form).await);
    redirect_error!(auth.login(&form.into()).await);
    Redirect::to(uri!("/"))
}

#[get("/create")]
fn thread_create_page(user: User) -> Html<String> {
    let template = template::ThreadCreate { user: Some(user) };
    Html(template.render_once().unwrap())
}

// #[post("/new", data = "<new_post>")]
// async fn thread_create(db: DbConn, new_thread: Form<models::PostForm>) {
//     use schema::thread;
//     db.run(|conn| {
//         diesel::insert_into(post::table)
//             .values(new_post.into_inner())
//             .execute(conn)
//             .unwrap()
//     })
//     .await;
// }

#[launch]
async fn rocket() -> _ {
    color_backtrace::install();
    migrations::install().expect("Error when migrating");

    let users = Users::open_sqlite(crate::URL)
        .await
        .expect(&format!("Error connecting to {}", &crate::URL));

    rocket::build()
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![
                index_page,
                error_page,
                register,
                register_page,
                login,
                login_page
            ],
        )
        .mount("/thread", routes![thread_create_page])
        .mount("/public", FileServer::from("public"))
        .manage(users)
}
