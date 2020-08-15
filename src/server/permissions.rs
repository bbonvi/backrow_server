use crate::db;
use crate::diesel::prelude::PgConnection;
use db::{Room, User};
use serde_repr::*;

#[derive(Deserialize_repr, Debug, Clone)]
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
        self,
        action_type: ActionType,
        conn: &PgConnection,
    ) -> Result<bool, db::DieselError> {
        let user_id = self.user.map(|u| u.id.clone());

        // Get user roles sorted by `position`, which indicates role's priority.
        let roles = db::helpers::list_user_roles_in_room(user_id, self.room.id.to_owned(), &conn)?;

        // Loop over roles until role with not `unset` permission is found.
        // It'll fallback to everyone in the worst case.
        for role in roles {
            let permission = match &action_type {
                ActionType::ChangeTitle => role.title_update,
                ActionType::ChangePath => role.path_update,
                ActionType::ChangePublic => role.public_update,
                ActionType::DeleteRoom => role.room_delete,
                ActionType::PasswordCreate => role.password_create,
                ActionType::PasswordUpdate => role.password_update,
                ActionType::PasswordDelete => role.password_delete,
                ActionType::EmoteCreate => role.emote_create,
                ActionType::EmoteUpdate => role.emote_update,
                ActionType::EmoteDelete => role.emote_delete,
                ActionType::RoleCreate => role.role_create,
                ActionType::RoleUpdate => role.role_update,
                ActionType::RoleDelete => role.role_delete,
                ActionType::VideoAdd => role.video_create,
                ActionType::VideoDelete => role.video_delete,
                ActionType::VideoMove => role.video_move,
                ActionType::PlayerPause => role.player_pause,
                ActionType::PlayerResume => role.player_resume,
                ActionType::PlayerRewind => role.player_rewind,
                ActionType::MessageCreate => role.message_create,
                ActionType::MessageRead => role.message_read,
                ActionType::MessageDelete => role.message_delete,
                ActionType::MessageHistory => role.message_history_read,
                ActionType::UserKick => role.user_kick,
                ActionType::UserBan => role.user_ban,
                ActionType::UserUnban => role.user_unban,
                ActionType::UserTimeout => role.user_timeout,
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
