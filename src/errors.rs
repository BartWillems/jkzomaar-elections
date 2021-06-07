use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use std::convert::From;

#[derive(Debug, Display)]
pub(crate) enum ElectionError {
    // User tries to vote with a UUID that does not exist
    InvalidBallot,
    AlreadyVoted,
    #[display(fmt = "Conflict: {}", _0)]
    Conflict(String),
    InvalidCandidate,
    NotFound,
    #[display(fmt = "Unknown error occured: {}", _0)]
    UnkownError(String),
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
}

impl ResponseError for ElectionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ElectionError::InvalidBallot => HttpResponse::Unauthorized().json("Invalid Ballot"),
            ElectionError::AlreadyVoted => HttpResponse::Conflict().json("Already Voted"),
            ElectionError::Conflict(ref message) => HttpResponse::Conflict().json(message),
            ElectionError::InvalidCandidate => {
                HttpResponse::BadRequest().json("Candidate does not exist")
            }
            ElectionError::NotFound => HttpResponse::NotFound().json("Not Found"),
            ElectionError::UnkownError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ElectionError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<sqlx::Error> for ElectionError {
    fn from(error: sqlx::Error) -> ElectionError {
        log::debug!("SQLX error: {:?}", error);
        match error {
            sqlx::Error::Database(err) => {
                err.downcast_ref::<sqlx::postgres::PgDatabaseError>().into()
            }
            sqlx::Error::RowNotFound => ElectionError::NotFound,
            _ => ElectionError::UnkownError(error.to_string()),
        }
    }
}

impl From<&sqlx::postgres::PgDatabaseError> for ElectionError {
    fn from(error: &sqlx::postgres::PgDatabaseError) -> ElectionError {
        log::debug!("Postgres error: {:?}", error);
        match error.code() {
            // Foreign Key Violation | Check Violation
            "23503" | "23514" => ElectionError::BadRequest(error.to_string()),
            // Unique Violation
            "23505" => ElectionError::Conflict(error.to_string()),
            _ => ElectionError::UnkownError(error.to_string()),
        }
    }
}
