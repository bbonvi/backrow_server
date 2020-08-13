use crate::env;
use crate::vars;
use regex::Regex;

pub fn valid_origin(req: &actix_web::HttpRequest) -> bool {
    let app_origin = env::APP_ORIGIN.clone();
    let origin = super::helpers::get_origin(&req);
    origin.contains(&app_origin)
}

pub fn valid_username(username: String) -> bool {
    let char_count = username.chars().count();
    if char_count < vars::USERNAME_MIN_LEN || char_count > vars::USERNAME_MAX_LEN {
        return false;
    }

    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    re.is_match(&username)
}

pub fn valid_nickname(nickname: String) -> bool {
    let char_count = nickname.chars().count();
    char_count >= vars::NICKNAME_MIN_LEN && char_count <= vars::NICKNAME_MAX_LEN
}

pub fn valid_email(email: String) -> bool {
    let char_count = email.chars().count();
    if char_count < 4 || char_count > 40 {
        return false;
    }

    let re = Regex::new(r#"^[a-zA-Z0-9!#$%&'*+-/=?^_`{|}~."\(\),:;<>@\[\]]+$""#).unwrap();
    re.is_match(&email)
}
