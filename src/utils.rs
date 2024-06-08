use dirs::home_dir;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::{
    error::Error,
    process::Command,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub enum SecretType {
    AccessToken,
    ExpiresIn,
    RefreshToken,
    ClientId,
    ClientSecret,
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
    let expires_in = read_secret(SecretType::ExpiresIn);

    let now = SystemTime::now();

    let expires_in_timestamp =
        UNIX_EPOCH + std::time::Duration::from_secs(expires_in.unwrap().parse::<u64>()?);

    if now > expires_in_timestamp {
        let node_path = "node";
        let server_path = "server/index.js";
        let refresh = read_secret(SecretType::RefreshToken).unwrap();

        let mut child = Command::new(node_path).args([server_path, "refresh", &refresh]).spawn().expect("Failed to start authentication server! Please ensure \"node\" installed and exists in PATH");

        thread::sleep(Duration::from_secs(2));

        let _ = child.wait().expect("Failed to wait on child process");
    }

    let access_token = read_secret(SecretType::AccessToken).unwrap();

    Ok(access_token)
}
