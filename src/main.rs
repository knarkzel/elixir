use elixir::*;

#[launch]
async fn rocket() -> _ {
    // Migrations
    utils::migrations().expect("Failed to run migrations");

    // Users
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
