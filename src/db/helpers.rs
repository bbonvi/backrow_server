use super::DieselError;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;

/// Check whether or not DieselError is emited by unique constraint error
pub fn is_unique_constraint_error(err: &DieselError) -> bool {
    if let DieselError::Error(db_err) = err {
        if let Error::DatabaseError(db_err, _) = db_err {
            if let DatabaseErrorKind::UniqueViolation = db_err {
                return true;
            }
        }
    }

    if let DieselError::DatabaseErrorKind(db_err) = err {
        if let DatabaseErrorKind::UniqueViolation = db_err {
            return true;
        }
    }
    false
}

pub fn is_not_found_error(err: &DieselError) -> bool {
    if let DieselError::Error(db_error) = err {
        if let Error::NotFound = db_error {
            return true;
        }
    }

    false
}
