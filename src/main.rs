extern crate rocket;

use app::{init_app, CurrentScreen};
use controllers::{fetch::TimeRange, server};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use secret::{read_secret, write_secret, SecretType};
use std::io::stdout;
use ui::{ui, Panel};
use utils::intro;

mod app;
mod controllers;
mod secret;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    if (read_secret(SecretType::ClientId).is_none())
        || (read_secret(SecretType::ClientSecret).is_none())
    {
        let (client_id, client_secret) = intro();
        let _ = write_secret(SecretType::ClientId, &client_id);
        let _ = write_secret(SecretType::ClientSecret, &client_secret);
    }

    if read_secret(SecretType::AccessToken).is_none() {
        server::launch().await;
    }

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    match init_app().await {
        Ok(mut app) => {
            loop {
                let _ = terminal.draw(|f| {
                    let _ = ui(f, &mut app);
                });
                if let Event::Key(key) = event::read()? {
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
                            KeyCode::Char('1') => {
                                app.current_panel = Panel::TopArtists;
                                futures::future::ready(()).await
                            }
                            KeyCode::Char('2') => {
                                app.current_panel = Panel::TopTracks;
                                futures::future::ready(()).await
                            }
                            KeyCode::Char('3') => {
                                app.current_panel = Panel::RecentlyPlayed;
                                futures::future::ready(()).await
                            }
                            KeyCode::Char('4') => {
                                app.current_panel = Panel::Playlists;
                                futures::future::ready(()).await
                            }
                            KeyCode::F(1) => match app.current_panel {
                                Panel::TopArtists => {
                                    app.top_artists.cur_time_range = TimeRange::Long;
                                    app.update_top_artist_data(TimeRange::Long).await
                                }
                                Panel::TopTracks => {
                                    app.top_tracks.cur_time_range = TimeRange::Long;
                                    app.update_top_tracks_data(TimeRange::Long).await
                                }
                                _ => {}
                            },
                            KeyCode::F(2) => match app.current_panel {
                                Panel::TopArtists => {
                                    app.top_artists.cur_time_range = TimeRange::Medium;
                                    app.update_top_artist_data(TimeRange::Medium).await
                                }
                                Panel::TopTracks => {
                                    app.top_tracks.cur_time_range = TimeRange::Medium;
                                    app.update_top_tracks_data(TimeRange::Medium).await
                                }
                                _ => {}
                            },
                            KeyCode::F(3) => match app.current_panel {
                                Panel::TopArtists => {
                                    app.top_artists.cur_time_range = TimeRange::Short;
                                    app.update_top_artist_data(TimeRange::Short).await
                                }
                                Panel::TopTracks => {
                                    app.top_tracks.cur_time_range = TimeRange::Short;
                                    app.update_top_tracks_data(TimeRange::Short).await
                                }
                                _ => {}
                            },
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
