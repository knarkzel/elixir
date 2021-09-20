use elixir::*;

#[launch]
async fn rocket() -> _ {
    color_backtrace::install();
    migrations::install().expect("Error when migrating");

    let users = Users::open_sqlite(crate::URL)
        .await
        .expect(&format!("Error connecting to {}", &crate::URL));

    rocket::build()
        .attach(Db::fairing())
        .mount(
            "/",
            routes![
                index::page,
                index::register_page,
                index::login_page,
                index::register,
                index::login,
            ],
        )
        .mount(
            "/thread",
            routes![thread::create_page, thread::create, thread::view_page],
        )
        .mount("/search", routes![search::page, search::view_query])
        .mount("/comment", routes![comment::create])
        .mount("/public", rocket::fs::FileServer::from("public"))
        .manage(users)
}
