use crate::{template, TemplateOnce};
use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response},
};
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Template rendering failed")]
    TemplateError(#[from] sailfish::RenderError),
    #[error("Database failed")]
    DbError(#[from] rusqlite::Error),
    #[error("Authentication failed")]
    AuthError(#[from] rocket_auth::Error),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> Result<rocket::Response<'o>, Status> {
        let context = format!("{}", self);
        let debug = format!("{:#?}", self);
        let cause = match self {
            ApiError::TemplateError(e) => e.to_string(),
            ApiError::DbError(e) => e.to_string(),
            ApiError::AuthError(e) => e.to_string(),
        };
        let template = template::Error {
            cause,
            debug,
            context,
        };
        let body = template.render_once().unwrap();

        Response::build()
            .header(ContentType::HTML)
            .status(Status::InternalServerError)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}
