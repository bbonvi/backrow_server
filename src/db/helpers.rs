use super::DieselError;
use super::Role;
use crate::diesel::prelude::PgConnection;
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

pub fn get_highest_user_role(
    user_id: Option<String>,
    room_id: String,
    conn: &PgConnection,
) -> Result<Role, DieselError> {
    let list = list_user_roles_in_room(user_id, room_id, conn)?;
    // There will always be at least one role
    let first = list.first().unwrap().to_owned();
    Ok(first)
}

/// user_id is None if user is anonymous
pub fn list_user_roles_in_room(
    user_id: Option<String>,
    room_id: String,
    conn: &PgConnection,
) -> Result<Vec<Role>, DieselError> {
    let is_anon = user_id.is_none();

    let mut generic_room_roles = Role::list_generic_room_roles(room_id.clone(), is_anon, conn)?;
    match user_id {
        Some(id) => {
            let mut assigned_user_roles = Role::list_user_roles_by_room_id(id, room_id, conn)?;
            assigned_user_roles.append(&mut generic_room_roles);
            Ok(assigned_user_roles)
        }
        None => Ok(generic_room_roles),
    }
}
