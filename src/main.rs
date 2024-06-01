mod app;
mod fetch;
mod ui;
mod utils;
use anyhow::Result;
use app::{init_app, CurrentScreen};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{process::Command, thread, time::Duration};

use ui::ui;
use utils::{read_secret, SecretType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    if read_secret(SecretType::AccessToken).is_none() {
        let node_path = "node";
        let server_path = "server/index.js";

        let mut child = Command::new(node_path).arg(server_path).spawn().expect("Failed to start authentication server! Please ensure \"node\" installed and exists in PATH");

        thread::sleep(Duration::from_secs(2));

        let _ = child.wait().expect("Failed to wait on child process");
    }

    // startup
    enable_raw_mode()?;
    execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    // let _ = run().await;
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    match init_app().await {
        Ok(mut app) => {
            loop {
                t.draw(|f| {
                    ui(f, &mut app);
                });

                if let Event::Key(key) = event::read()? {
                    // dbg!()
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }
                    match app.current_screen {
                        /* Main Screen Keybinds */
                        app::CurrentScreen::Main => {
                            if let KeyCode::Char('q') = key.code {
                                break;
                            }
                        }
                        /* Artist Screen Keybinds */
                        app::CurrentScreen::Artist => {
                            if let KeyCode::Char('q') = key.code {
                                app.current_screen = CurrentScreen::Main
                            }
                        }
                        /* Playlist Screen Keybinds */
                        app::CurrentScreen::Playlist => {
                            if let KeyCode::Char('q') = key.code {
                                app.current_screen = CurrentScreen::Main
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
        }
    }

    // shutdown
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
