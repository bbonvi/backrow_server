use super::RouteResult;
use super::States;
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
    id: Identity,
    user: Option<User>,
) -> RouteResult {
    let user_id = user.map(|u| u.id);
    let conn = states.pool.get().unwrap();

    let room = db::Room::by_path(info.room_path.clone(), &conn)?;
    let roles = db::helpers::list_user_roles_in_room(user_id, room.id, &conn)?;

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
    id: Identity,
) -> RouteResult {
    let user_id = id.identity();
    let conn = states.pool.get().unwrap();

    let room = db::Room::by_path(info.room_path.clone(), &conn)?;
    let roles = db::helpers::list_user_roles_in_room(user_id, room.id, &conn)?;

    Ok(HttpResponse::Ok().json(roles))
}
