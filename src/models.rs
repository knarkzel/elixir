use super::schema::post;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "post"]
pub struct PostForm<'a> {
    pub content: &'a str,
}
