use super::RouteResult;
use super::States;
use crate::db;
use crate::db::{Role, Room, User};
use crate::server::permissions::{ActionType, AssertPermission};
use actix_identity::Identity;
use actix_web::web::Path;
use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    room_path: String,
}

pub async fn list_user_roles(info: Path<Info>, states: States, id: Identity) -> RouteResult {
    let user_id = id.identity();
    let conn = states.pool.get().unwrap();

    let room = db::Room::by_path(&info.room_path, &conn)?;
    let roles = db::helpers::list_user_roles_in_room(user_id, room.id, &conn)?;

    Ok(HttpResponse::Ok().json(roles))
}
