use crate::env;
use crate::vars::*;
use regex::Regex;

fn in_range(string: &str, min: usize, max: usize) -> bool {
    let char_count = string.chars().count();
    char_count >= min && char_count <= max
}

fn matches(string: &str, reg_string: &str) -> bool {
    let re = Regex::new(reg_string).unwrap();
    re.is_match(&string)
}

pub fn valid_origin(req: &actix_web::HttpRequest) -> bool {
    let app_origin = env::APP_ORIGIN.clone();
    let origin = super::helpers::get_origin(&req);
    origin.contains(&app_origin)
}

pub fn valid_username(username: &String) -> bool {
    in_range(username, USERNAME_MIN_LEN, USERNAME_MAX_LEN) && matches("^[a-zA-Z0-9_]+$", &username)
}

pub fn valid_password(password: &String) -> bool {
    in_range(password, PASSWORD_MIN_LEN, PASSWORD_MAX_LEN)
}

pub fn valid_nickname(nickname: String) -> bool {
    in_range(&nickname, NICKNAME_MIN_LEN, NICKNAME_MAX_LEN)
}

pub fn valid_email(email: String) -> bool {
    in_range(&email, 4, 100)
        && matches(
            &email,
            &r#"^[a-zA-Z0-9!#$%&'*+-/=?^_`{|}~."\(\),:;<>@\[\]]+$""#,
        )
}

pub fn valid_room_name(name: String) -> bool {
    in_range(&name, ROOM_NAME_MIN_LEN, ROOM_NAME_MAX_LEN)
}

pub fn valid_room_path(username: &String) -> bool {
    in_range(username, USERNAME_MIN_LEN, USERNAME_MAX_LEN) && matches(username, r"^[a-zA-Z0-9_]+$")
}
