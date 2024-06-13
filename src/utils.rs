use std::io::{stdin, stdout, Write};

pub fn intro() -> (String, String) {
    let mut client_id = String::new();
    let mut client_secret = String::new();

    let intro = [
            "",
            "",
            "",
            "* * Welcome to Sprofile! * *",
            "To get started, you need to get a CLIENT_ID and CLIENT_SECRET.",
            "1. Visit the Spotify App Creation Page: https://developer.spotify.com/dashboard/create",
            "2. Fill in 'App name' and 'App description' with whatever you like",
            "3. Add 'http://localhost:8585/callback' to 'Redirect URIs'",
            "4. Check the 'Web API' option from available API/SDK list",
            "5. Hit Save",
            "6. After creating, click on your application from the spotify dashboard and go to 'Settings'",
            "7. There you can access the Client ID and Client Secret.",
            "",
            "Please enter your CLIENT ID and CLIENT SECRET below",
            ""
        ];
    for elem in intro.iter() {
        println!("{}", elem);
    }
    print!("--> Client ID: ");
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut client_id)
        .expect("Failed to read stdin");

    print!("--> Client Secret: ");
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut client_secret)
        .expect("Failed to read stdin");

    let trimmed_client_id = client_id.trim().to_owned();
    let trimmed_client_secret = client_secret.trim().to_owned();

    (trimmed_client_id, trimmed_client_secret)
}
