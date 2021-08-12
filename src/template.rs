use crate::*;

#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
pub struct Error {
    pub cause: String,
    pub debug: String,
    pub context: String,
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    pub user: Option<User>,
    pub threads: Vec<root::ThreadListing>,
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
#[template(path = "thread_create.stpl")]
pub struct ThreadCreate {
    pub user: Option<User>,
}
