use super::asserts;
use super::RouteResult;
use super::States;
use crate::db;
use crate::server::errors::ResponseError;
use actix_identity::Identity;
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
enum ActionType {
    // TODO: probably need to move somewhere else
    Message = 0,
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
}

#[derive(Deserialize, Debug)]
pub struct RoomAction {
    action_type: ActionType,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    room_path: String,
}

pub async fn change_title(
    info: Path<Info>,
    states: States,
    id: Identity,
) -> RouteResult {
    let id: i64 = match id.identity() {
        None => return Err(ResponseError::AccessError("Unauthorize")),
        Some(id) => id.parse::<i64>().unwrap_or_default(),
    };
    let conn = states.pool.get().unwrap();

    let room = db::Room::by_path(&info.room_path, &conn)?;

    let roles = db::Role::list_user_roles_by_room_id(id, room.id, &conn)?;

    Ok(HttpResponse::Ok().json(roles))
}

pub struct RoleHandler;

impl RoleHandler {}
