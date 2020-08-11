use super::DieselError;
use crate::schema::roles;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

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

    pub title_update: i32,
    pub path_update: i32,
    pub public_update: i32,
    pub room_delete: i32,
    pub audit_log_read: i32,
    pub embed_links: i32,
    pub ping_everyone: i32,

    pub password_create: i32,
    pub password_update: i32,
    pub password_delete: i32,

    pub emote_create: i32,
    pub emote_update: i32,
    pub emote_delete: i32,
    pub emote_view: i32,

    pub role_create: i32,
    pub role_delete: i32,
    pub role_update: i32,
    pub role_view: i32,

    pub video_create: i32,
    pub video_delete: i32,
    pub video_watch: i32,
    pub video_move: i32,
    pub video_iframe: i32,
    pub video_raw: i32,

    pub player_pause: i32,
    pub player_resume: i32,
    pub player_rewind: i32,

    pub subtitles_file: i32,
    pub subtitles_embed: i32,

    pub message_create: i32,
    pub message_read: i32,
    pub message_history_read: i32,
    pub message_timeout: i32,

    pub user_kick: i32,
    pub user_ban: i32,
    pub user_unban: i32,
    pub user_timeout: i32,

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

    pub title_update: i32,
    pub path_update: i32,
    pub public_update: i32,
    pub room_delete: i32,
    pub audit_log_read: i32,
    pub embed_links: i32,
    pub ping_everyone: i32,

    pub password_create: i32,
    pub password_update: i32,
    pub password_delete: i32,

    pub emote_create: i32,
    pub emote_update: i32,
    pub emote_delete: i32,
    pub emote_view: i32,

    pub role_create: i32,
    pub role_delete: i32,
    pub role_update: i32,
    pub role_view: i32,

    pub video_create: i32,
    pub video_delete: i32,
    pub video_watch: i32,
    pub video_move: i32,
    pub video_iframe: i32,
    pub video_raw: i32,

    pub player_pause: i32,
    pub player_resume: i32,
    pub player_rewind: i32,

    pub subtitles_file: i32,
    pub subtitles_embed: i32,

    pub message_create: i32,
    pub message_read: i32,
    pub message_history_read: i32,
    pub message_timeout: i32,

    pub user_kick: i32,
    pub user_ban: i32,
    pub user_unban: i32,
    pub user_timeout: i32,
}

impl<'a> Default for NewRole<'a> {
    /// You should always explicitly specify `room_id`, never use default value
    fn default() -> NewRole<'a> {
        NewRole {
            // default room_id should NOT be used!
            room_id: Uuid::new_v4(),
            name: "",
            color: None,
            is_default: false,
            position: 999,

            title_update: -1,
            path_update: -1,
            public_update: -1,
            room_delete: -1,
            audit_log_read: -1,
            embed_links: -1,
            ping_everyone: -1,

            password_create: -1,
            password_update: -1,
            password_delete: -1,

            emote_create: -1,
            emote_update: -1,
            emote_delete: -1,
            emote_view: -1,

            role_create: -1,
            role_delete: -1,
            role_update: -1,
            role_view: -1,

            video_create: -1,
            video_delete: -1,
            video_watch: -1,
            video_move: -1,
            video_iframe: -1,
            video_raw: -1,

            player_pause: -1,
            player_resume: -1,
            player_rewind: -1,
            
            subtitles_file: -1,
            subtitles_embed: -1,

            message_create: -1,
            message_read: -1,
            message_history_read: -1,
            message_timeout: -1,

            user_kick: -1,
            user_ban: -1,
            user_unban: -1,
            user_timeout: -1,
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
