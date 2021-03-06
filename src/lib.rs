pub mod routes;
pub mod template;
pub mod utils;

pub use ammonia::clean;
pub use chrono::prelude::*;
pub use utils::ApiError;
pub use rocket::response::content::Html;
pub use rocket::*;
pub use rocket::{form::Form, response::Redirect};
pub use rocket_auth::{Auth, Login, Signup, User, Users};
pub use routes::*;
pub use rusqlite::params;
pub use sailfish::TemplateOnce;

#[rocket_sync_db_pools::database("main")]
pub struct Db(rusqlite::Connection);

pub const URL: &str = "main.sqlite";

pub type ApiResult<T> = std::result::Result<T, ApiError>;
