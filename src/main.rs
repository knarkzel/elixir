#[macro_use]
extern crate rocket;

use elixir::*;
use rocket::fs::FileServer;
use rocket_sync_db_pools::{database, diesel};

#[database("elixir")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> Html<String> {
    let messages = vec![String::from("hello"), String::from("world")];
    let index = template::Index { messages };
    Html(index.render_once().unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
}
