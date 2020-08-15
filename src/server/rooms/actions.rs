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
    let user_id: String = match id.identity() {
        None => String::new(),
        Some(id) => id,
    };
    let is_anon = user_id.is_empty();
    let conn = states.pool.get().unwrap();
    let room = db::Room::by_path(&info.room_path, &conn)?;
    let roles = db::helpers::list_user_roles_in_room(user_id, room.id, is_anon, &conn)?;

    Ok(HttpResponse::Ok().json(roles))
}
