#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod template;

pub use rocket::response::content::Html;
pub use sailfish::TemplateOnce;
