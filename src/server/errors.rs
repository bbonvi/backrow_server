use crate::db::DieselError;
use actix_http::ResponseBuilder;
use actix_web::Error as ActixError;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use diesel::result::Error as QueryError;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum ResponseError {
    #[fail(display = "An internal error occurred. Please try again later")]
    InternalError,
    #[fail(display = "Bad request")]
    BadRequest,
    #[fail(display = "{}", _0)]
    BadRequestMessage(&'static str),
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "Timeout")]
    Timeout,
    #[fail(display = "Validation error on field: {}", field)]
    ValidationError { field: &'static str },
    #[fail(display = "Access error. {}", _0)]
    AccessError(&'static str),
}

impl error::ResponseError for ResponseError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ResponseError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::BadRequest => StatusCode::BAD_REQUEST,
            ResponseError::BadRequestMessage { .. } => StatusCode::BAD_REQUEST,
            ResponseError::NotFound => StatusCode::NOT_FOUND,
            ResponseError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ResponseError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ResponseError::AccessError { .. } => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<DieselError> for ResponseError {
    fn from(err: DieselError) -> ResponseError {
        match err {
            DieselError::ConnectionError(_) => ResponseError::InternalError,
            DieselError::DatabaseErrorKind(_) => ResponseError::InternalError,
            DieselError::Error(query_err) => match query_err {
                QueryError::NotFound => ResponseError::NotFound,
                _ => ResponseError::InternalError,
            },
        }
    }
}

impl From<ActixError> for ResponseError {
    fn from(_: ActixError) -> ResponseError {
        ResponseError::InternalError
    }
}
