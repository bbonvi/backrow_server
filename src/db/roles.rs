use super::DieselError;
use crate::schema::roles;

use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use std::io::Write;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Integer"]
pub enum PermissionState {
    Unset = -1,
    Forbidden = 0,
    Allowed = 1,
}

impl Default for PermissionState {
    fn default() -> PermissionState {
        PermissionState::Unset
    }
}

impl From<i32> for PermissionState {
    fn from(value: i32) -> PermissionState {
        match value {
            -1 => PermissionState::Unset,
            0 => PermissionState::Forbidden,
            1 => PermissionState::Allowed,
            _ => PermissionState::Unset,
        }
    }
}
impl<ST, DB> FromSql<ST, DB> for PermissionState
where
    i32: FromSql<ST, DB>,
    DB: Backend,
{
    fn from_sql(value: Option<&<DB as Backend>::RawValue>) -> deserialize::Result<Self> {
        <i32 as FromSql<ST, DB>>::from_sql(value).map(PermissionState::from)
    }
}
impl<DB> ToSql<Integer, DB> for PermissionState
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        <i32 as ToSql<Integer, DB>>::to_sql(&(*self as i32), out)
    }
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "roles"]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: Uuid,
    pub room_id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub is_default: bool,
    pub position: i32,

    pub title_update: PermissionState,
    pub path_update: PermissionState,
    pub public_update: PermissionState,
    pub room_delete: PermissionState,
    pub audit_log_read: PermissionState,
    pub embed_links: PermissionState,
    pub ping_everyone: PermissionState,

    pub password_create: PermissionState,
    pub password_update: PermissionState,
    pub password_delete: PermissionState,

    pub emote_create: PermissionState,
    pub emote_update: PermissionState,
    pub emote_delete: PermissionState,
    pub emote_view: PermissionState,

    pub role_create: PermissionState,
    pub role_delete: PermissionState,
    pub role_update: PermissionState,
    pub role_view: PermissionState,

    pub video_create: PermissionState,
    pub video_delete: PermissionState,
    pub video_watch: PermissionState,
    pub video_move: PermissionState,
    pub video_iframe: PermissionState,
    pub video_raw: PermissionState,

    pub player_pause: PermissionState,
    pub player_resume: PermissionState,
    pub player_rewind: PermissionState,

    pub subtitles_file: PermissionState,
    pub subtitles_embed: PermissionState,

    pub message_create: PermissionState,
    pub message_read: PermissionState,
    pub message_history_read: PermissionState,
    pub message_timeout: PermissionState,

    pub user_kick: PermissionState,
    pub user_ban: PermissionState,
    pub user_unban: PermissionState,
    pub user_timeout: PermissionState,

    pub created_at: NaiveDateTime,
}

impl Role {
    pub fn list_by_room_id(room_id_query: Uuid, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        roles
            .filter(room_id.eq(room_id_query))
            .first::<Role>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query role by room_id {:?}: {}",
                    room_id_query, err
                );
                err
            })
            .map_err(From::from)
    }
    pub fn by_id(role_id: Uuid, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        roles
            .filter(id.eq(role_id))
            .first::<Role>(conn)
            .map_err(|err| {
                error!("Couldn't query role by id {:?}: {}", role_id, err);
                err
            })
            .map_err(From::from)
    }
    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::roles::dsl::*;

        diesel::delete(roles.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't delete role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        diesel::update(roles)
            .set(self)
            .get_result::<Role>(conn)
            .map_err(|err| {
                error!("Couldn't update role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "roles"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewRole<'a> {
    /// You should always explicitly specify `room_id`, never use default value
    pub room_id: Uuid,
    pub name: &'a str,
    pub color: Option<&'a str>,
    pub is_default: bool,
    pub position: i32,

    pub title_update: PermissionState,
    pub path_update: PermissionState,
    pub public_update: PermissionState,
    pub room_delete: PermissionState,
    pub audit_log_read: PermissionState,
    pub embed_links: PermissionState,
    pub ping_everyone: PermissionState,

    pub password_create: PermissionState,
    pub password_update: PermissionState,
    pub password_delete: PermissionState,

    pub emote_create: PermissionState,
    pub emote_update: PermissionState,
    pub emote_delete: PermissionState,
    pub emote_view: PermissionState,

    pub role_create: PermissionState,
    pub role_delete: PermissionState,
    pub role_update: PermissionState,
    pub role_view: PermissionState,

    pub video_create: PermissionState,
    pub video_delete: PermissionState,
    pub video_watch: PermissionState,
    pub video_move: PermissionState,
    pub video_iframe: PermissionState,
    pub video_raw: PermissionState,

    pub player_pause: PermissionState,
    pub player_resume: PermissionState,
    pub player_rewind: PermissionState,

    pub subtitles_file: PermissionState,
    pub subtitles_embed: PermissionState,

    pub message_create: PermissionState,
    pub message_read: PermissionState,
    pub message_history_read: PermissionState,
    pub message_timeout: PermissionState,

    pub user_kick: PermissionState,
    pub user_ban: PermissionState,
    pub user_unban: PermissionState,
    pub user_timeout: PermissionState,
}

impl<'a> Default for NewRole<'a> {
    /// `room_id` and `name` should always be specified.
    /// Never use default values them!
    fn default() -> NewRole<'a> {
        NewRole {
            room_id: Uuid::default(),
            name: "",

            color: None,
            is_default: false,
            position: 999,

            title_update: PermissionState::default(),
            path_update: PermissionState::default(),
            public_update: PermissionState::default(),
            room_delete: PermissionState::default(),
            audit_log_read: PermissionState::default(),
            embed_links: PermissionState::default(),
            ping_everyone: PermissionState::default(),

            password_create: PermissionState::default(),
            password_update: PermissionState::default(),
            password_delete: PermissionState::default(),

            emote_create: PermissionState::default(),
            emote_update: PermissionState::default(),
            emote_delete: PermissionState::default(),
            emote_view: PermissionState::default(),

            role_create: PermissionState::default(),
            role_delete: PermissionState::default(),
            role_update: PermissionState::default(),
            role_view: PermissionState::default(),

            video_create: PermissionState::default(),
            video_delete: PermissionState::default(),
            video_watch: PermissionState::default(),
            video_move: PermissionState::default(),
            video_iframe: PermissionState::default(),
            video_raw: PermissionState::default(),

            player_pause: PermissionState::default(),
            player_resume: PermissionState::default(),
            player_rewind: PermissionState::default(),

            subtitles_file: PermissionState::default(),
            subtitles_embed: PermissionState::default(),

            message_create: PermissionState::default(),
            message_read: PermissionState::default(),
            message_history_read: PermissionState::default(),
            message_timeout: PermissionState::default(),

            user_kick: PermissionState::default(),
            user_ban: PermissionState::default(),
            user_unban: PermissionState::default(),
            user_timeout: PermissionState::default(),
        }
    }
}

impl<'a> NewRole<'a> {
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<Role, DieselError> {
        use crate::schema::roles::dsl::*;

        diesel::insert_into(roles)
            .values(self)
            .get_result::<Role>(conn)
            .map_err(|err| {
                error!("Couldn't create role {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
