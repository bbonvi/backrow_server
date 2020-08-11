use super::DieselError;
use crate::schema::users;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

fn is_false(x: &bool) -> bool {
    !x
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing)]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<i32>,

    #[serde(skip_serializing_if = "is_false")]
    pub is_admin: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let user = serde_json::to_string(self).unwrap();
        write!(f, "{}", user)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "users"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub nickname: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub color: Option<String>,
    pub image_id: Option<i32>,
}
impl<'a> Default for NewUser<'a> {
    fn default() -> Self {
        Self {
            username: "",
            nickname: None,
            email: None,
            password: None,
            color: None,
            image_id: None,
        }
    }
}
impl<'a> fmt::Display for NewUser<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let user = serde_json::to_string(self).unwrap();
        write!(f, "{}", user)
    }
}

// TODO: Remove later
// type AllColumns = (users::id, users::username, users::nickname);
// pub const ALL_COLUMNS: AllColumns = (users::id, users::username, users::nickname);
// type All = diesel::dsl::Select<users::table, AllColumns>;
// type WithName<'a> = diesel::dsl::Eq<users::username, &'a str>;
// type ByName<'a> = diesel::dsl::Filter<All, WithName<'a>>;

impl User {
    pub fn by_id(user_id: Uuid, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .map_err(|err| {
                error!("Couldn't query user by id {:?}: {}", user_id, err);
                err
            })
            .map_err(From::from)
    }
    pub fn by_name(name: &str, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        users
            .filter(username.eq(name))
            .first::<User>(conn)
            .map_err(|err| {
                error!("Couldn't query user by name {:?}: {}", name, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove user {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        diesel::update(users)
            .set(self)
            .get_result::<User>(conn)
            .map_err(|err| {
                error!("Couldn't update user {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

impl<'a> NewUser<'a> {
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<User, DieselError> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(self)
            .get_result::<User>(conn)
            .map_err(|err| {
                error!("Couldn't create user {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
