use crate::*;
use rocket_auth::User;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    pub user: Option<User>,
}

#[derive(TemplateOnce)]
#[template(path = "register.stpl")]
pub struct Register {
    pub user: Option<User>,
}

#[derive(TemplateOnce)]
#[template(path = "login.stpl")]
pub struct Login {
    pub user: Option<User>,
}

#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
pub struct Error {
    pub user: Option<User>,
    pub cause: String,
}

#[derive(TemplateOnce)]
#[template(path = "thread_create.stpl")]
pub struct ThreadCreate {
    pub user: Option<User>,
}
