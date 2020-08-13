extern crate bcrypt;
use bcrypt::{hash, verify};

use super::asserts;
use crate::db;
use crate::server::errors::ResponseError;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AuthForm {
    username: String,
    password: String,
}

pub async fn get(pool: web::Data<db::DbPool>, id: Identity) -> Result<HttpResponse, ResponseError> {
    let conn = pool.get().unwrap();

    if let Some(uuid) = id.identity() {
        let user = db::User::by_id(Uuid::parse_str(&uuid).unwrap(), &conn)?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

pub async fn log_out(id: Identity) -> Result<HttpResponse, ResponseError> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize, Debug)]
pub struct DiscordRedirect {
    code: String,
}

pub async fn sign_in_discord(
    pool: web::Data<db::DbPool>,
    info: web::Query<DiscordRedirect>,
    id: Identity,
) -> Result<HttpResponse, ResponseError> {
    let conn = pool.get().unwrap();

    // let user = db::User::by_name(&info.username.clone(), &conn)?;

    let discord_user = super::auth::get_discord_user(info.code.clone())
        .await
        .map_err(|err| {
            error!("{}", err);
            ResponseError::InternalError
        })?;

    let user = match db::User::by_discord_id(discord_user.id.to_owned(), &conn) {
        Ok(user) => user,
        Err(user_err) => {
            if !db::helpers::is_not_found_error(&user_err) {
                return Err(ResponseError::InternalError);
            }
            // We use discord ID as `username` to avoid validation troubles.
            // And set discord name to `nickname` instead, so user couldn't see numbers in his name.
            db::NewUser {
                username: &discord_user.id.to_owned(),
                nickname: Some(discord_user.username),
                discord_id: Some(discord_user.id.to_owned()),
                ..Default::default()
            }
            .create(&conn)?
        }
    };

    id.remember(user.id.to_string());
    Ok(HttpResponse::Ok().json(user))
}

pub async fn sign_in(
    pool: web::Data<db::DbPool>,
    form: web::Form<AuthForm>,
    id: Identity,
) -> Result<HttpResponse, ResponseError> {
    let conn = pool.get().unwrap();

    let user = db::User::by_name(&form.username.clone(), &conn)?;
    let password = user.password.clone();

    if password.is_none() {
        return Err(ResponseError::BadRequestMessage(
            "Account uses passwordless authentication",
        ));
    }

    let is_password_valid = verify(&form.password, &password.unwrap()).map_err(|err| {
        error!("{}", err);
        ResponseError::InternalError
    })?;

    if is_password_valid {
        id.remember(user.id.to_string());
        return Ok(HttpResponse::Ok().json(user));
    }

    Err(ResponseError::AccessError("Password is invalid"))
}

pub async fn sign_up(
    pool: web::Data<db::DbPool>,
    form: web::Form<AuthForm>,
    id: Identity,
) -> Result<HttpResponse, ResponseError> {
    let conn = pool.get().unwrap();

    if db::User::by_name(&form.username, &conn).is_ok() {
        return Err(ResponseError::BadRequestMessage(
            "User with this name already exists",
        ));
    }

    if !asserts::valid_username(form.username.clone()) {
        return Err(ResponseError::BadRequestMessage(
            "This username is not allowed",
        ));
    }

    let hashed_password = hash(form.password.clone(), 10).unwrap();

    let user = db::NewUser {
        username: &form.username,
        password: Some(hashed_password),
        ..Default::default()
    }
    .create(&conn)?;

    id.remember(user.id.to_string());

    Ok(HttpResponse::Ok().json(user))
}
