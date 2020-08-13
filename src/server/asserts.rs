use crate::env;

pub fn valid_origin(req: &actix_web::HttpRequest) -> bool {
    let app_origin = env::APP_ORIGIN.clone();
    let origin = super::helpers::get_origin(&req);
    origin.contains(&app_origin)
}

