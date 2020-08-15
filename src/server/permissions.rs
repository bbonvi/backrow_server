use crate::db;
use crate::diesel::prelude::PgConnection;
use db::{Room, User};
use serde_repr::*;

#[derive(Deserialize_repr, Debug)]
#[repr(i8)]
pub enum ActionType {
    ChangeTitle,
    ChangePath,
    ChangePublic,
    DeleteRoom,
    PasswordCreate,
    PasswordUpdate,
    PasswordDelete,
    EmoteCreate,
    EmoteUpdate,
    EmoteDelete,
    RoleCreate,
    RoleUpdate,
    RoleDelete,
    VideoAdd,
    VideoDelete,
    VideoMove,
    PlayerPause,
    PlayerResume,
    PlayerRewind,
    MessageCreate,
    MessageRead,
    MessageDelete,
    MessageHistory,
    UserKick,
    UserBan,
    UserUnban,
    UserTimeout,
}

pub struct AssertPermission {
    user: Option<User>,
    room: Room,
}

impl User {
    pub fn assert_permission(user: Option<User>, room: Room) -> AssertPermission {
        AssertPermission { user, room }
    }
}

impl AssertPermission {
    pub fn new(user: Option<User>, room: Room) -> AssertPermission {
        AssertPermission { user, room }
    }

    pub fn is_allowed(
        &self,
        action_type: ActionType,
        conn: &PgConnection,
    ) -> Result<bool, db::DieselError> {
        let user_id = self.user.map(|u| u.id);

        // Get user roles sorted by `position`, which indicates role's priority.
        let roles = db::helpers::list_user_roles_in_room(user_id, self.room.id, &conn)?;

        // Loop roles until find the one where PermissionState is not `unset`.
        // Eventually it will fallback on `everyone` role.
        for role in roles {
            let permission = match action_type {
                ChangeTitle => role.title_update,
                ChangePath => role.path_update,
                ChangePublic => role.public_update,
                DeleteRoom => role.room_delete,
                PasswordCreate => role.password_create,
                PasswordUpdate => role.password_update,
                PasswordDelete => role.password_delete,
                EmoteCreate => role.emote_create,
                EmoteUpdate => role.emote_update,
                EmoteDelete => role.emote_delete,
                RoleCreate => role.role_create,
                RoleUpdate => role.role_update,
                RoleDelete => role.role_delete,
                VideoAdd => role.video_create,
                VideoDelete => role.video_delete,
                VideoMove => role.video_move,
                PlayerPause => role.player_pause,
                PlayerResume => role.player_resume,
                PlayerRewind => role.player_rewind,
                MessageCreate => role.message_create,
                MessageRead => role.message_read,
                MessageDelete => role.message_delete,
                MessageHistory => role.message_history_read,
                UserKick => role.user_kick,
                UserBan => role.user_ban,
                UserUnban => role.user_unban,
                UserTimeout => role.user_timeout,
            };

            match permission {
                db::PermissionState::Allowed => return Ok(true),
                db::PermissionState::Forbidden => return Ok(false),
                db::PermissionState::Unset => continue,
            }
        }

        Ok(false)
    }
}
