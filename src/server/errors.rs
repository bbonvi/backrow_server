use crate::db::DieselError;
use actix_http::ResponseBuilder;
use actix_web::Error as ActixError;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use diesel::result::Error as QueryError;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum ServerError {
    #[fail(display = "An internal error occurred. Please try again later")]
    InternalError,
    #[fail(display = "Bad request")]
    BadRequest,
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "Timeout")]
    Timeout,
    #[fail(display = "Validation error on field: {}", field)]
    ValidationError { field: &'static str },
    #[fail(display = "Access error. {}", _0)]
    AccessError(&'static str),
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::BadRequest => StatusCode::BAD_REQUEST,
            ServerError::NotFound => StatusCode::NOT_FOUND,
            ServerError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ServerError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ServerError::AccessError { .. } => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<DieselError> for ServerError {
    fn from(err: DieselError) -> ServerError {
        match err {
            DieselError::ConnectionError(_) => ServerError::InternalError,
            DieselError::DatabaseErrorKind(_) => ServerError::InternalError,
            DieselError::Error(query_err) => match query_err {
                QueryError::NotFound => ServerError::NotFound,
                _ => ServerError::InternalError,
            },
        }
    }
}

impl From<ActixError> for ServerError {
    fn from(_: ActixError) -> ServerError {
        ServerError::InternalError
    }
}
