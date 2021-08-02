#[macro_use]
extern crate rocket;

use elixir::*;
use rocket::fs::FileServer;

#[get("/")]
fn index() -> Html<String> {
    let messages = vec![String::from("hello"), String::from("world")];
    let index = template::Index { messages };
    Html(index.render_once().unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
}
