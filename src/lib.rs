pub mod migrations;
pub mod routes;
pub mod template;

pub use chrono::prelude::*;
pub use rocket::response::content::Html;
pub use rocket::*;
pub use rocket::{form::Form, response::Redirect};
pub use rocket_auth::{Auth, Login, Signup, User, Users};
pub use routes::*;
pub use rusqlite::params;
pub use sailfish::TemplateOnce;

#[macro_export]
macro_rules! redirect_error {
    ($value:expr) => {
        if let Err(e) = $value {
            return Redirect::to(uri!(crate::error_page(e.to_string())));
        }
    };
}

#[get("/error/<cause>")]
pub fn error_page(cause: String, user: Option<User>) -> Html<String> {
    let template = template::Error { user, cause };
    Html(template.render_once().unwrap())
}

#[rocket_sync_db_pools::database("main")]
pub struct DbConn(rusqlite::Connection);

pub const URL: &str = "database/main.sqlite";
