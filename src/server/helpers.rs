pub fn get_origin(req: &actix_web::HttpRequest) -> String {
    String::from(
        req.headers()
            .get("origin")
            .map(|o| o.to_str().unwrap_or_default())
            .unwrap_or_default(),
    )
}

