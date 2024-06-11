use crate::utils::{read_secret, write_secret, SecretType};
// use anyhow::Result;
use core::panic;
use reqwest::Client;
use rocket::{
    get,
    http::Status,
    launch,
    response::Redirect,
    routes,
    serde::{Deserialize, Serialize},
    Shutdown, State,
};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const REDIRECT_URI: &str = "http://localhost:8585/callback";

#[derive(Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
}

#[get("/login")]
fn login() -> Redirect {
    let client_id = read_secret(crate::utils::SecretType::ClientId)
        .expect("CLIENT_ID not set")
        .trim()
        .to_string();
    let redirect_uri = REDIRECT_URI.to_string();

    let scope = "user-top-read user-read-recently-played playlist-read-private";

    let params = serde_urlencoded::to_string([
        ("response_type", "code"),
        ("client_id", client_id.as_str()),
        ("scope", scope),
        ("redirect_uri", redirect_uri.as_str()),
    ]);

    let auth_url = format!(
        "https://accounts.spotify.com/authorize?{}",
        params.expect("Error occured: Bad Parameters")
    );

    Redirect::to(auth_url)
}

#[get("/callback?<code>")]
async fn callback(
    client: &State<Client>,
    shutdown: Shutdown,
    code: String,
) -> Result<&str, Status> {
    let client_id = read_secret(crate::utils::SecretType::ClientId)
        .expect("CLIENT_ID not set")
        .trim()
        .to_string();
    let client_secret = read_secret(crate::utils::SecretType::ClientSecret)
        .expect("CLIENT_SECRET not set")
        .trim()
        .to_string();
    let redirect_uri = REDIRECT_URI.to_string();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", &redirect_uri),
        // ("client_id", &client_id),
        // ("client_secret", &client_secret),
    ];

    // let combined_str = format!("{}:{}", client_id, client_secret);
    // let encoded_str = base64::encode(&combined_str);

    let res_obj = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        // .header("Authorization", format!("Basic {}", encoded_str))
        .form(&params)
        .send()
        .await
        .unwrap();

    let res_st = res_obj.status();
    println!("{}", res_st);
    let res = res_obj.json::<TokenResponse>().await.unwrap();

    let _ = write_secret(
        crate::utils::SecretType::AccessToken,
        &res.access_token.to_string(),
    );
    let _ = write_secret(
        crate::utils::SecretType::RefreshToken,
        &res.refresh_token.to_string(),
    );
    let unix_timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(t) => t.as_secs(),
        Err(_) => panic!("SystemTime set to before UNIX_EPOCH"),
    };
    let _ = write_secret(
        SecretType::ExpiresIn,
        (res.expires_in + unix_timestamp).to_string().as_str(),
    );

    shutdown.notify();
    Ok("Authorization Successful. You can close this window.")
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .manage(Client::new())
        .mount("/", routes![login, callback])
}
