use super::DieselError;
use crate::schema::users;
use actix_web::web;

use crate::diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::users::dsl as users_dsl;

#[derive(Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub color: String,
    pub image_id: Option<i32>,
    pub is_admin: bool,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(AsExpression, Insertable, Debug, Associations, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: Option<String>,
}

impl User {
    pub fn create(new_user: NewUser, conn: &PgConnection) -> Result<User, DieselError> {
        diesel::insert_into(self::users::table)
            .values(&new_user)
            .get_result(conn)
            .map_err(|err| {
                error!("couldn't create user: {}", err);
                err
            })
            .map_err(From::from)
    }
}
