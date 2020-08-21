use super::RouteResult;
use super::States;
use crate::server::errors::ResponseError;
use crate::db;
use crate::db::{Role, Room, User};
use crate::server::permissions::{ActionType, AssertPermission};
use actix_identity::Identity;
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Url {
    room_path: String,
}

type Info = Path<Url>;

pub async fn list_user_roles(
    info: Info,
    states: States,
    user: Option<User>,
) -> RouteResult {
    let user_id = user.map(|u| u.id);
    let conn = states.pool.get().unwrap();

    let room = db::Room::by_path(info.room_path.clone(), &conn)?;
    let roles = db::helpers::list_user_roles_in_room(user_id, room.id, &conn)?;
    

    Ok(HttpResponse::Ok().json(roles))
}

pub async fn list_room_roles(
    info: Info,
    states: States,
    user: Option<User>,
) -> RouteResult {
    let conn = states.pool.get().unwrap();

    let room = db::Room::by_path(info.room_path.clone(), &conn)?;

    match user {
        Some(u) => {
            if !u.is_allowed(&room, ActionType::RoleCreate, &conn)? {
                return Err(ResponseError::AccessError("Not allowed to view roles"))
            }
        }
        None => {
            if !db::User::is_anonymous_allowed(&room, ActionType::RoleCreate, &conn)? {
                return Err(ResponseError::AccessError("Not allowed to view roles"))
            }
        }
    }

    let roles = db::Role::list_by_room_id(room.id, &conn)?;

    Ok(HttpResponse::Ok().json(roles))
}

#[derive(Deserialize, Debug)]
pub struct CreateRole {
    name: String,
}

pub async fn create_role(
    info: Info,
    json: Json<CreateRole>,
    states: States,
    user: User,
) -> RouteResult {
    let user_id = user.id.to_owned();
    let conn = states.pool.get().unwrap();

    let role_name = json.name.to_owned();

    let room = db::Room::by_path(info.room_path.clone(), &conn)?;
    if !user.is_allowed(&room, ActionType::RoleCreate, &conn)? {
        return Err(ResponseError::AccessError("Not allowed to create role"))
    }

    let roles = db::helpers::list_user_roles_in_room(Some(user_id), room.id.clone(), &conn)?;
    let role = db::NewRole::new(&role_name, &room.id).create(&conn)?;

    Ok(HttpResponse::Ok().json(role))
}
