#[macro_use]
extern crate rocket;

use chrono::prelude::*;
use elixir::*;
use rocket::{form::Form, fs::FileServer, response::Redirect};
use rocket_auth::{Auth, Login, Signup, User, Users};
use rocket_sync_db_pools::database;
use rusqlite::params;

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

// THREADS
#[get("/create")]
fn thread_create_page(user: User) -> Html<String> {
    let template = template::ThreadCreate { user: Some(user) };
    Html(template.render_once().unwrap())
}

#[derive(FromForm, Clone)]
struct Thread {
    title: String,
    categories: String,
    body: String,
}

#[post("/create", data = "<new_thread>")]
async fn thread_create(db: DbConn, user: User, new_thread: Form<Thread>) -> Redirect {
    let utc: DateTime<Utc> = Utc::now();
    let inner = new_thread.into_inner();
    let user_id = user.id();

    // create thread
    let thread = inner.clone();
    let err = db
        .run(move |conn| {
            conn.execute(
                "INSERT INTO threads (title, categories) VALUES (?1, ?2)",
                params![thread.title, thread.categories],
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
                "INSERT INTO comments (thread_id, body, created_by, published) VALUES (?1, ?2, ?3, ?4)",
                params![thread_id, thread.body, user.id(), utc.to_string()],
            )
        })
        .await;
    redirect_error!(err);

    Redirect::to(uri!("/"))
}

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
        .mount("/thread", routes![thread_create_page, thread_create])
        .mount("/public", FileServer::from("public"))
        .manage(users)
}
