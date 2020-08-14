use crate::env;
use reqwest::Error;
use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DiscordTokenResponse {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Deserialize, Debug)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: usize,
    pub locale: String,
    pub mfa_enabled: bool,
    pub premium_type: usize,
}

// #[tokio::main]
pub async fn get_discord_user(code: String) -> Result<DiscordUser, Error> {
    let client = reqwest::Client::new();

    // Get user token
    let params = [
        ("code", &code),
        ("client_id", &env::DISCORD_CLIENT_ID.clone()),
        ("client_secret", &env::DISCORD_CLIENT_SECRET.clone()),
        ("grant_type", &String::from("authorization_code")),
        ("redirect_uri", &env::DISCORD_REDIRECT_URL.clone()),
        ("scope", &String::from("identify")),
    ];
    let res = client
        .post("https://discord.com/api/oauth2/token")
        .form(&params)
        .send()
        .await?
        .json::<DiscordTokenResponse>()
        .await?;

    // Get user info, using token
    let user = client
        .get("https://discord.com/api/v6/users/@me")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", res.access_token),
        )
        .send()
        .await?
        .json::<DiscordUser>()
        .await?;

    Ok(user)
}
