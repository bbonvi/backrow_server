use crate::db;
use crate::diesel::prelude::PgConnection;
use db::{Role, Room, User};
use serde_repr::*;

#[derive(Deserialize_repr, Debug, Clone)]
/// Some actions require context.
///
/// e.g. we can not modify role that higher in position than highest requestor's role.
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
    /// This permission should be dealt within a context of role position.
    RoleUpdate(Role),
    /// This permission should be dealt within a context of role position.
    RoleDelete(Role),
    RoleView,
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
    /// This permission should be dealt within a context of user position.
    UserKick(Option<User>),
    /// This permission should be dealt within a context of user position.
    UserBan(Option<User>),
    UserUnban,
    /// This permission should be dealt within a context of user position.
    UserTimeout(Option<User>),
}

pub struct AssertPermission {
    user: Option<User>,
    room: Room,
}

impl User {
    pub fn is_allowed(
        &self,
        room: &Room,
        action_type: ActionType,
        conn: &PgConnection,
    ) -> Result<bool, db::DieselError> {
        AssertPermission::new(Some(self), room).is_allowed(action_type, conn)
    }
    pub fn is_anonymous_allowed(
        room: &Room,
        action_type: ActionType,
        conn: &PgConnection,
    ) -> Result<bool, db::DieselError> {
        AssertPermission::new(None, room).is_allowed(action_type, conn)
    }
}

impl AssertPermission {
    pub fn new(user: Option<&User>, room: &Room) -> AssertPermission {
        AssertPermission {
            user: user.map(|u| u.clone()),
            room: room.clone(),
        }
    }

    pub fn is_allowed(
        self,
        action_type: ActionType,
        conn: &PgConnection,
    ) -> Result<bool, db::DieselError> {
        let user_id = self.user.map(|u| u.id.clone());

        // Get user roles sorted by `position`, which indicates role's priority.
        let user_roles =
            db::helpers::list_user_roles_in_room(user_id, self.room.id.to_owned(), &conn)?;

        // Loop over roles until role with not `unset` permission is found.
        // It'll fallback to `everyone` in the worst case.
        for user_role in user_roles {
            let permission = match &action_type {
                ActionType::ChangeTitle => user_role.title_update,
                ActionType::ChangePath => user_role.path_update,
                ActionType::ChangePublic => user_role.public_update,
                ActionType::DeleteRoom => user_role.room_delete,
                ActionType::PasswordCreate => user_role.password_create,
                ActionType::PasswordUpdate => user_role.password_update,
                ActionType::PasswordDelete => user_role.password_delete,
                ActionType::EmoteCreate => user_role.emote_create,
                ActionType::EmoteUpdate => user_role.emote_update,
                ActionType::EmoteDelete => user_role.emote_delete,
                ActionType::RoleCreate => user_role.role_create,
                ActionType::RoleUpdate(requested_role) => {
                    if requested_role.position > user_role.position {
                        db::PermissionState::Forbidden
                    } else {
                        user_role.role_update
                    }
                }
                ActionType::RoleDelete(requested_role) => {
                    if requested_role.position > user_role.position {
                        db::PermissionState::Forbidden
                    } else {
                        user_role.role_delete
                    }
                }
                ActionType::RoleView => user_role.role_view,
                ActionType::VideoAdd => user_role.video_create,
                ActionType::VideoDelete => user_role.video_delete,
                ActionType::VideoMove => user_role.video_move,
                ActionType::PlayerPause => user_role.player_pause,
                ActionType::PlayerResume => user_role.player_resume,
                ActionType::PlayerRewind => user_role.player_rewind,
                ActionType::MessageCreate => user_role.message_create,
                ActionType::MessageRead => user_role.message_read,
                ActionType::MessageDelete => user_role.message_delete,
                ActionType::MessageHistory => user_role.message_history_read,
                ActionType::UserKick(requested_user) => {
                    // TODO: remove code duplications!
                    // Possibly move to User struct.
                    let requested_role = db::helpers::get_highest_user_role(
                        requested_user.map(|u| u.id),
                        self.room.id.to_owned(),
                        &conn,
                    )?;

                    if requested_role.position > user_role.position {
                        db::PermissionState::Forbidden
                    } else {
                        user_role.user_kick
                    }
                }
                ActionType::UserBan(requested_user) => {
                    let requested_role = db::helpers::get_highest_user_role(
                        requested_user.map(|u| u.id),
                        self.room.id.to_owned(),
                        &conn,
                    )?;

                    if requested_role.position > user_role.position {
                        db::PermissionState::Forbidden
                    } else {
                        user_role.user_ban
                    }
                }
                ActionType::UserUnban => user_role.user_unban,
                ActionType::UserTimeout(requested_user) => {
                    let requested_role = db::helpers::get_highest_user_role(
                        requested_user.map(|u| u.id),
                        self.room.id.to_owned(),
                        &conn,
                    )?;

                    if requested_role.position > user_role.position {
                        db::PermissionState::Forbidden
                    } else {
                        user_role.user_timeout
                    }
                }
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
