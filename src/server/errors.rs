use crate::db::DieselError;
use actix_http::ResponseBuilder;
use actix_web::Error as ActixError;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as QueryError};

use failure::Fail;

#[derive(Fail, Debug)]
pub enum ServerError {
    #[fail(display = "internal error")]
    InternalError,
    #[fail(display = "bad request")]
    BadRequest,
    #[fail(display = "not found")]
    NotFound,
    #[fail(display = "timeout")]
    Timeout,
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
