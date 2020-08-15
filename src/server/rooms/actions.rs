use super::asserts;
use super::RouteResult;
use super::States;
use crate::db;
use crate::server::errors::ResponseError;
use actix_identity::Identity;
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use serde::Deserialize;
use serde_repr::*;

#[derive(Deserialize_repr, Debug)]
#[repr(i8)]
enum ActionType {
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

#[derive(Deserialize, Debug)]
pub struct RoomAction {
    action_type: ActionType,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    room_path: String,
}

pub async fn list_user_roles(info: Path<Info>, states: States, id: Identity) -> RouteResult {
    let id: String = match id.identity() {
        None => String::new(),
        Some(id) => id,
    };
    let is_anon = id.is_empty();

    let conn = states.pool.get().unwrap();

    let room = db::Room::by_path(&info.room_path, &conn)?;

    let roles = db::helpers::list_user_roles_in_room(id, room.id, is_anon, &conn)?;

    Ok(HttpResponse::Ok().json(roles))
}

pub struct RoleHandler;

impl RoleHandler {}
