use crate::*;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    pub posts: Vec<models::Post>,
}
