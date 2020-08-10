use diesel::result::{ConnectionError, DatabaseErrorKind, Error};
use std::fmt;

#[derive(Debug)]
pub enum DieselError {
    /// Errors which can occur during Connection::establish
    ConnectionError(ConnectionError),
    /// The kind of database error that occurred
    DatabaseErrorKind(DatabaseErrorKind),
    /// Represents all the ways that a query can fail
    Error(Error),
}

impl From<ConnectionError> for DieselError {
    fn from(err: ConnectionError) -> DieselError {
        DieselError::ConnectionError(err)
    }
}
impl From<DatabaseErrorKind> for DieselError {
    fn from(err: DatabaseErrorKind) -> DieselError {
        DieselError::DatabaseErrorKind(err)
    }
}
impl From<Error> for DieselError {
    fn from(err: Error) -> DieselError {
        DieselError::Error(err)
    }
}
impl From<DieselError> for std::io::Error {
    fn from(err: DieselError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

impl fmt::Display for DieselError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DieselError::ConnectionError(ref err) => write!(f, "{}", err),
            DieselError::DatabaseErrorKind(ref err) => write!(f, "{:?}", err),
            DieselError::Error(ref err) => write!(f, "{}", err),
        }
    }
}
