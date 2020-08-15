use super::AppStates;
use crate::db;
use actix_identity::RequestIdentity;
use actix_web::error::ErrorBadRequest;
use actix_web::web::Data;
use actix_web::FromRequest;
use actix_web::{dev, Error, HttpRequest};

use futures_util::future::{err, ok, Ready};

// Custom actix extractor.
// Maps identity info to User model.
//
// Usage: add `user: Option<User>` to route params
impl FromRequest for db::User {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        if let Some(states) = req.app_data::<Data<AppStates>>() {
            if let Some(id) = req.get_identity() {
                let conn = states.pool.get().unwrap();
                return ok(db::User::by_id(id, &conn).unwrap());
            }
        }

        err(ErrorBadRequest("Unauthorized"))
    }
}
