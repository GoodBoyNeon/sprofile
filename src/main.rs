use anyhow::Result;
use app::{init_app, CurrentScreen};
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io::stdout, process::Command};
use ui::{ui, Panel};
use utils::{read_secret, SecretType};

mod app;
mod fetch;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    if read_secret(SecretType::AccessToken).is_none() {
        let node_path = "node";
        let server_path = "server/index.js";

        let mut child = Command::new(node_path).arg(server_path).spawn().expect("Failed to start authentication server! Please ensure \"node\" installed and exists in PATH");

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
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
