use super::DieselError;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error;
use super::Role;
use crate::diesel::prelude::PgConnection;

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

pub fn list_user_roles_in_room(
    user_id: String,
    room_id: String,
    is_anon: bool,
    conn: &PgConnection,
) -> Result<Vec<Role>, DieselError> {
    let mut assigned_user_roles = Role::list_user_roles_by_room_id(user_id.clone(), room_id.clone(), conn)?;
    let mut generic_room_roles = Role::list_generic_room_roles(room_id.clone(), is_anon, conn)?;

    assigned_user_roles.append(&mut generic_room_roles);

    Ok(assigned_user_roles)
}
