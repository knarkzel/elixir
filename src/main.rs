#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use elixir::*;
use rocket::{form::Form, fs::FileServer, response::Redirect};
use rocket_auth::{Auth, Login, Signup, User, Users};
use rocket_sync_db_pools::{database, diesel};

#[database("elixir")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
// async fn index_page(db: DbConn, user: Option<User>) -> Html<String> {
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
fn thread_create(user: User) -> Html<String> {
    let template = template::ThreadCreate { user: Some(user) };
    Html(template.render_once().unwrap())
}

// #[post("/new", data = "<new_post>")]
// async fn new_post(db: DbConn, new_post: Form<models::PostForm>) {
//     use schema::post;
//     db.run(|conn| {
//         diesel::insert_into(post::table)
//             .values(new_post.into_inner())
//             .execute(conn)
//             .unwrap()
//     })
//     .await;
// }

embed_migrations!();

#[launch]
async fn rocket() -> _ {
    color_backtrace::install();

    const URL: &str = "database/elixir.sqlite";

    // Get connection to database and load migrations.
    let conn = {
        use diesel::SqliteConnection;
        SqliteConnection::establish(&URL).expect(&format!("Error connecting to {}", &URL))
    };
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Failed to run migrations");

    // Load users from rocket_auth
    let users = Users::open_sqlite(URL)
        .await
        .expect(&format!("Error connecting to {}", &URL));

    // Launch!
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
        .mount("/thread", routes![thread_create])
        .mount("/public", FileServer::from("public"))
        .manage(users)
}
