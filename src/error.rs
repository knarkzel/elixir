use crate::{template, TemplateOnce};
use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response},
};
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Template rendering failed: {0:?}.")]
    TemplateError(#[from] sailfish::RenderError),
    #[error("Database failed: {0:?}.")]
    DbError(#[from] rusqlite::Error),
    #[error("Authentication failed: {0:?}.")]
    AuthError(#[from] rocket_auth::Error),
    #[error("Resource was not found.")]
    NotFound,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> Result<rocket::Response<'o>, Status> {
        let template = template::Error {
            cause: self.to_string(),
        };
        let body = template.render_once().unwrap();
        Response::build()
            .header(ContentType::HTML)
            .status(Status::InternalServerError)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}
