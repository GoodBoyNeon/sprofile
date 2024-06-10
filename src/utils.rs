use dirs::home_dir;
use reqwest::Client;
use rocket::serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

pub enum SecretType {
    AccessToken,
    ExpiresIn,
    RefreshToken,
    ClientId,
    ClientSecret,
}

#[derive(Debug, Deserialize, Serialize)]
struct RefreshTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

pub fn read_secret(secret_type: SecretType) -> Option<String> {
    let mut file_path = home_dir()?;
    file_path.push(".sprofile");
    file_path.push(get_file_name(secret_type));

    match fs::read_to_string(file_path) {
        Ok(contents) => Some(contents),
        Err(_) => None,
    }
}
pub fn write_secret(secret_type: SecretType, secret: &str) -> io::Result<()> {
    let mut file_path = home_dir().expect("Could not find $HOME");

    file_path.push(".sprofile");
    file_path.push(get_file_name(secret_type));

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    writeln!(file, "{}", secret)?;

    Ok(())
}

fn get_file_name(secret_type: SecretType) -> String {
    match secret_type {
        SecretType::AccessToken => "access_token.txt".to_string(),
        SecretType::RefreshToken => "refresh_token.txt".to_string(),
        SecretType::ExpiresIn => "expires_in.txt".to_string(),
        SecretType::ClientId => "client_id.txt".to_string(),
        SecretType::ClientSecret => "client_secret.txt".to_string(),
    }
}

pub async fn get_access_token() -> Result<String, Box<dyn Error>> {
    let expires_in = read_secret(SecretType::ExpiresIn)
        .expect("EXPIRES_IN not found")
        .trim()
        .to_owned();

    let now = SystemTime::now();

    let expires_in_timestamp =
        UNIX_EPOCH + std::time::Duration::from_secs(expires_in.parse::<u64>()?);

    if now > expires_in_timestamp {
        let client_id = read_secret(crate::utils::SecretType::ClientId)
            .expect("CLIENT_ID not set")
            .trim()
            .to_string();
        let client_secret = read_secret(crate::utils::SecretType::ClientSecret)
            .expect("CLIENT_SECRET not set")
            .trim()
            .to_string();
        let refresh_token = read_secret(SecretType::RefreshToken)
            .expect("refresh_token not found!")
            .trim()
            .to_owned();
        let client = Client::new();

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
        ];

        let res_obj = client
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(client_id, Some(client_secret))
            .form(&params)
            .send()
            .await
            .unwrap();

        let res_st = res_obj.status();
        println!("{}", res_st);

        let res = res_obj.json::<RefreshTokenResponse>().await.unwrap();

        let _ = write_secret(SecretType::AccessToken, &res.access_token);

        let unix_timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(t) => t.as_secs(),
            Err(_) => panic!("SystemTime set to before UNIX_EPOCH"),
        };
        let _ = write_secret(
            SecretType::ExpiresIn,
            (res.expires_in + unix_timestamp).to_string().as_str(),
        );
    }

    let access_token = read_secret(SecretType::AccessToken).unwrap();

    Ok(access_token)
}
