use crate::db;
use crate::server::errors::ResponseError;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use super::asserts;


#[derive(Deserialize, Debug)]
pub struct SignUpForm {
    username: String,
    password: String,
}

pub async fn sign_up(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    form: web::Form<SignUpForm>,
) -> Result<HttpResponse, ResponseError> {
    let conn = pool.get().unwrap();

    if db::User::by_name(&form.username, &conn).is_ok() {
        return Err(ResponseError::BadRequestMessage("User with this name already exists"))
    }

    if !asserts::valid_username(form.username.clone()) {
        return Err(ResponseError::BadRequestMessage("This username is not allowed"))
    }

    let user = db::NewUser {
        username: &form.username,
        password: Some(form.password.clone()),
        ..Default::default()
    }.create(&conn)?;

    Ok(HttpResponse::Ok().json(user))
}
