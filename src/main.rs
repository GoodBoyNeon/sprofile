use app::{init_app, CurrentScreen};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{stdin, stdout, Write},
    process::Command,
};
use ui::{ui, Panel};
use utils::{read_secret, write_secret, SecretType};

mod app;
mod fetch;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    if (read_secret(SecretType::ClientId).is_none())
        || (read_secret(SecretType::ClientSecret).is_none())
    {
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

        let trimmed_client_id = client_id.trim();
        let trimmed_client_secret = client_secret.trim();

        let _ = write_secret(SecretType::ClientId, trimmed_client_id);
        let _ = write_secret(SecretType::ClientSecret, trimmed_client_secret);
    }

    if read_secret(SecretType::AccessToken).is_none() {
        let node_path = "node";
        let server_path = "server/index.js";

        let mut child = Command::new(node_path)
            .args([server_path, "auth"])
            .spawn()
            .expect(
                "Failed to start authentication server! Ensure node installed and exists in PATH",
            );

        let _ = child.wait().expect("Failed to wait on child process");
    }

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    match init_app().await {
        Ok(mut app) => {
            loop {
                let _ = terminal.draw(|f| {
                    let _ = ui(f, &mut app);
                });
                if let Event::Key(key) = event::read()? {
                    // dbg!();
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('k') => {
                            let i = match app.get_current_panel().selected() {
                                Some(i) => {
                                    if i == 0 {
                                        0
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            app.get_current_panel().select(Some(i));
                        }
                        KeyCode::Char('j') => {
                            let i = match app.get_current_panel().selected() {
                                Some(i) => i + 1,
                                None => 0,
                            };
                            app.get_current_panel().select(Some(i));
                        }
                        _ => {}
                    }
                    match app.current_screen {
                        /* Main Screen Keybinds */
                        CurrentScreen::Main => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('1') => app.current_panel = Panel::TopArtists,
                            KeyCode::Char('2') => app.current_panel = Panel::TopSongs,
                            KeyCode::Char('3') => app.current_panel = Panel::RecentlyPlayed,
                            KeyCode::Char('4') => app.current_panel = Panel::Playlists,
                            _ => {}
                        },
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
        }
    }

    // shutdown
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}
