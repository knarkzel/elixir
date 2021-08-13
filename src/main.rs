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
                root::index_page,
                root::register_page,
                root::login_page,
                root::register,
                root::login,
            ],
        )
        .mount("/thread", routes![thread::create_page, thread::create, thread::view_page])
        .mount("/public", rocket::fs::FileServer::from("public"))
        .manage(users)
}
