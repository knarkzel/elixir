pub mod template;
pub mod migrations;

pub use rocket::response::content::Html;
pub use sailfish::TemplateOnce;

pub const URL: &str = "database/main.sqlite";
