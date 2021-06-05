use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use std::convert::From;

#[derive(Debug, Display)]
pub(crate) enum ElectionError {
    // User tries to vote with a UUID that does not exist
    InvalidUuid,
    AlreadyVoted,
    DuplicateBallotCreation,
    InvalidCandidate,
}

impl ResponseError for ElectionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ElectionError::InvalidUuid => HttpResponse::Unauthorized().json("Invalid UUID"),
            ElectionError::AlreadyVoted => HttpResponse::Conflict().json("Already Voted"),
            ElectionError::DuplicateBallotCreation => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ElectionError::InvalidCandidate => {
                HttpResponse::BadRequest().json("Candidate does not exist")
            }
        }
    }
}
