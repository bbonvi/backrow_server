use super::DieselError;
use crate::schema::rooms;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: Uuid,
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

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let room = serde_json::to_string(self).unwrap();
        write!(f, "{}", room)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "rooms"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewRoom<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub is_public: Option<bool>,
    pub is_deleted: Option<bool>,
}
impl<'a> Default for NewRoom<'a> {
    fn default() -> Self {
        Self {
            title: "",
            path: "",
            is_public: None,
            is_deleted: None,
        }
    }
}
impl<'a> fmt::Display for NewRoom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let new_room = serde_json::to_string(self).unwrap();
        write!(f, "{}", new_room)
    }
}

impl Room {
    pub fn byid(room_id: Uuid, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        rooms
            .filter(id.eq(room_id))
            .first::<Room>(conn)
            .map(|room| {
                debug!("Room {} has been queried by id {:?}", room, room_id);
                room
            })
            .map_err(|err| {
                error!("Couldn't query room by id {:?}: {}", room_id, err);
                err
            })
            .map_err(From::from)
    }
    pub fn by_path(path: &str, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        rooms
            .filter(path.eq(path))
            .first::<Room>(conn)
            .map(|room| {
                debug!("Room {} has been queried by path {:?}", room, path);
                room
            })
            .map_err(|err| {
                error!("Couldn't query room by path {:?}: {}", path, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::rooms::dsl::*;

        diesel::delete(rooms.filter(id.eq(self.id)))
            .execute(conn)
            .map(|room| {
                debug!("Room {} has been removed", self);
                room
            })
            .map_err(|err| {
                error!("Couldn't remove room {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        diesel::update(rooms)
            .set(self)
            .get_result::<Room>(conn)
            .map(|room| {
                debug!("Room has been updated: {}", room);
                room
            })
            .map_err(|err| {
                error!("Couldn't update room {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

impl<'a> NewRoom<'a> {
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<Room, DieselError> {
        use crate::schema::rooms::dsl::*;

        diesel::insert_into(rooms)
            .values(self)
            .get_result::<Room>(conn)
            .map(|room| {
                debug!("Room has been created: {}", room);
                room
            })
            .map_err(|err| {
                error!("Couldn't create room {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
