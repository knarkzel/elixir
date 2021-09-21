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
    pub threads: Vec<thread::Thread>,
}

#[derive(TemplateOnce)]
#[template(path = "search.stpl")]
pub struct Search {
    pub user: Option<User>,
}

#[derive(TemplateOnce)]
#[template(path = "search_comments.stpl", escape = false)]
pub struct SearchComments {
    pub user: Option<User>,
    pub comments: Vec<comment::Comment>,
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

#[derive(TemplateOnce)]
#[template(path = "thread_view.stpl", escape = false)]
pub struct ThreadView {
    pub user: Option<User>,
    pub thread: thread::Thread,
    pub comments: Vec<comment::Comment>,
}
