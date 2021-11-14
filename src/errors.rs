use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError Trait converts error messages to http responses
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try again"),
            ServiceError::BadRequest(ref message) => { 
                HttpResponse::BadRequest().json(message) 
            }
            ServiceError::Unauthorized => { 
                HttpResponse::Unauthorized().json("Unauthorized") 
            }
        }
    }
}

// we can return early in our handlers if uuid provided by the user is not valid and provide a
// custom message
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into()) // changes the &str into String
    } 
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this should help to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message().into());
                    return ServiceError::BadRequest(message.to_string());
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError
        }
    }
}
