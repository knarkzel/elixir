use crate::{template, TemplateOnce};
use chrono::{prelude::*, Utc};
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
    #[error("Invalid user input provided")]
    InvalidInput,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> Result<rocket::Response<'o>, Status> {
        let context = format!("{}", self);
        let debug = format!("{:#?}", self);
        let cause = match self {
            ApiError::TemplateError(e) => e.to_string(),
            ApiError::DbError(e) => e.to_string(),
            ApiError::AuthError(e) => e.to_string(),
            ApiError::InvalidInput => String::from("Bad input detected!"),
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

pub fn time_ago(published: &str) -> String {
    let previous = published.parse::<DateTime<Utc>>().unwrap();
    let current = Utc::now();
    let delta = current.signed_duration_since(previous);

    let days = delta.num_days();
    if days == 0 {
        let hours = delta.num_hours();
        if hours == 0 {
            let minutes = delta.num_minutes();
            let suffix = match minutes {
                1 => "minute ago",
                _ => "minutes ago",
            };
            format!("{} {}", minutes, suffix)
        } else {
            let suffix = match hours {
                1 => "hour ago",
                _ => "hours ago",
            };
            format!("{} {}", hours, suffix)
        }
    } else {
        let suffix = match days {
            1 => "day ago",
            _ => "days ago",
        };
        format!("{} {}", days, suffix)
    }
}

pub fn migrations() -> Result<(), Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open(crate::URL)?;
    for migration in glob::glob("migrations/*.sql")? {
        let sql = std::fs::read_to_string(migration?)?;
        conn.execute(&sql, [])?;
    }
    Ok(())
}
