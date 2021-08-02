#[macro_use]
extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
}
