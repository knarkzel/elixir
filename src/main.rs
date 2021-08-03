#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use elixir::*;
use rocket::fs::FileServer;
use rocket_sync_db_pools::{database, diesel};

#[database("elixir")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
async fn index(db: DbConn) -> Html<String> {
    use models::Post;
    use schema::post::dsl::*;

    let posts = db.run(|conn| post.load::<Post>(conn)).await.unwrap();
    let index = template::Index { posts };
    Html(index.render_once().unwrap())
}

embed_migrations!();

#[launch]
fn rocket() -> _ {
    // Get connection to database and load migrations.
    let conn = {
        use diesel::SqliteConnection;
        const URL: &str = "database/elixir.sqlite";
        SqliteConnection::establish(&URL).expect(&format!("Error connecting to {}", URL))
    };
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Failed to run migrations");

    // Launch!
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
}
