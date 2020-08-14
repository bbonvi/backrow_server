use super::DieselError;
use crate::schema::rooms;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: i64,
    pub title: String,
    pub path: String,

    #[serde(skip_serializing)]
    pub is_public: bool,

    #[serde(skip_serializing)]
    pub is_deleted: bool,

    #[serde(skip_serializing)]
    pub password: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,

    #[serde(skip_serializing)]
    pub last_login: Option<NaiveDateTime>,

    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
}

impl Room {
    pub fn by_id(room_id: i64, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        rooms
            .filter(id.eq(room_id))
            .first::<Room>(conn)
            .map_err(|err| {
                error!("Couldn't query room by id {:?}: {}", room_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn by_path(path_query: &str, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        rooms
            .filter(path.eq(path_query))
            .first::<Room>(conn)
            .map_err(|err| {
                error!("Couldn't query room by path {:?}: {}", path_query, err);
                err
            })
            .map_err(From::from)
    }

    pub fn list(conn: &PgConnection) -> Result<std::vec::Vec<Room>, DieselError> {
        use crate::schema::rooms::dsl::*;

        // TODO: pagination
        const LIMIT: i64 = 100;

        rooms
            .limit(LIMIT)
            .load(conn)
            .map_err(|err| {
                error!("Couldn't query rooms: {}", err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::rooms::dsl::*;

        diesel::delete(rooms.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove room {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        diesel::update(rooms)
            .set(self)
            .get_result::<Room>(conn)
            .map_err(|err| {
                error!("Couldn't update room {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(Insertable, AsChangeset, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "rooms"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewRoom<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub is_public: bool,
}

impl<'a> Default for NewRoom<'a> {
    fn default() -> Self {
        Self {
            title: "",
            path: "",
            is_public: true,
        }
    }
}

impl<'a> NewRoom<'a> {
    pub fn create(&self, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        diesel::insert_into(rooms)
            .values(self)
            .get_result::<Room>(conn)
            .map_err(|err| {
                error!("Couldn't create room {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
