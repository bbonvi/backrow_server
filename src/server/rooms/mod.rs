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
    ChangeTitle = 0,
}

#[derive(Deserialize, Debug)]
pub struct RoomAction {
    // TODO: probably need to move somewhere else
    Message,
    ChangeTitle = 0,
    ChangePath,
    ChangePublic,
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
pub struct Info {
    room_path: String,
}

pub async fn action(
    form: Json<RoomAction>,
    info: Path<Info>,
    states: States,
    id: Identity,
) -> RouteResult {
    let id: i64 = match id.identity() {
        None => return Err(ResponseError::AccessError("Unauthorize")),
        Some(id) => id.parse::<i64>().unwrap_or_default(),
    };

    let conn = states.pool.get().unwrap();

    let rooms = db::Room::list(&conn)?;

    Ok(HttpResponse::Ok().json(rooms))
}

#[derive(Deserialize, Debug)]
pub struct CreateRoom {
    title: String,
    path: String,
}

pub async fn create(states: States, id: Identity, form: Json<CreateRoom>) -> RouteResult {
    use crate::diesel::Connection;

    let id: i64 = match id.identity() {
        None => return Err(ResponseError::AccessError("Unauthorize")),
        Some(id) => id.parse::<i64>().unwrap_or_default(),
    };

    if !asserts::valid_room_name(&form.title) {
        return Err(ResponseError::BadRequestMessage("Invalid room name"));
    }

    if !asserts::valid_room_path(&form.path) {
        return Err(ResponseError::BadRequestMessage("Invalid room path"));
    }

    let conn = states.pool.get().unwrap();

    if db::Room::by_path(&form.path, &conn).is_ok() {
        return Err(ResponseError::BadRequestMessage(
            "Room with this path already exists",
        ));
    }

    let user = db::User::by_id(id, &conn)?;
    let room: Result<db::Room, db::DieselError> = conn.transaction(|| {
        // create room
        let room = db::NewRoom {
            title: &form.title,
            path: &form.path,
            ..Default::default()
        }
        .create(&conn)?;

        // initialize default roles.
        let _ = db::NewRole::everyone(room.id).create(&conn)?;
        let _ = db::NewRole::anonymous(room.id).create(&conn)?;
        let _ = db::NewRole::stranger(room.id).create(&conn)?;
        let _ = db::NewRole::administator(room.id).create(&conn)?;
        let owner_role = db::NewRole::owner(room.id).create(&conn)?;

        // assign owner role to user
        let _ = db::NewUserRole {
            role_id: owner_role.id,
            user_id: user.id,
        }
        .create(&conn);

        Ok(room)
    });

    let room = room?;

    Ok(HttpResponse::Ok().json(room))
}

// TODO: pagination
pub async fn list(states: States) -> RouteResult {
    let conn = states.pool.get().unwrap();

    let rooms = db::Room::list(&conn)?;

    Ok(HttpResponse::Ok().json(rooms))
}
