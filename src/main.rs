#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use elixir::*;
use rocket::{form::Form, fs::FileServer, response::Redirect, Request};
use rocket_auth::{Auth, Error, Login, Signup, Users};
use rocket_sync_db_pools::{database, diesel};

#[database("elixir")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
async fn index_page(db: DbConn) -> Html<String> {
    use models::Post;
    use schema::post::dsl::*;

    let posts = db.run(|conn| post.load::<Post>(conn)).await.unwrap();
    let template = template::Index { posts };
    Html(template.render_once().unwrap())
}

#[get("/error/<cause>")]
fn error_page(cause: String) -> Html<String> {
    let template = template::Error { cause };
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
    let template = template::Login {};
    Html(template.render_once().unwrap())
}

#[post("/login", data = "<form>")]
async fn login(mut auth: Auth<'_>, form: Form<Login>) -> Redirect {
    redirect_error!(auth.login(&form).await);
    Redirect::to(uri!("/"))
}

#[get("/register")]
fn register_page() -> Html<String> {
    let template = template::Register {};
    Html(template.render_once().unwrap())
}

#[post("/register", data = "<form>")]
async fn register(mut auth: Auth<'_>, form: Form<Signup>) -> Redirect {
    redirect_error!(auth.signup(&form).await);
    redirect_error!(auth.login(&form.into()).await);
    Redirect::to(uri!("/"))
}

#[post("/new", data = "<new_post>")]
async fn new_post(db: DbConn, new_post: Form<models::PostForm>) {
    use schema::post;
    db.run(|conn| {
        diesel::insert_into(post::table)
            .values(new_post.into_inner())
            .execute(conn)
            .unwrap()
    })
    .await;
}

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
        .mount("/post", routes![new_post])
        .mount("/public", FileServer::from("public"))
        .manage(users)
}
