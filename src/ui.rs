use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, List, Paragraph, Row, Table},
    Frame,
};
use std::collections::HashMap;

use crate::app::App;

#[derive(Hash, PartialEq, Eq)]
pub enum Panel {
    Playlists,
    RecentlyPlayed,
    TopArtists,
    TopSongs,
}

pub fn ui(f: &mut Frame, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    let title_blk = Block::default().style(Style::default());

    let username = &app.user_profile.display_name;

    let title_text = Paragraph::new(
        Text::styled(username, Style::default().fg(Color::Cyan).bold())
            .alignment(Alignment::Center),
    )
    .block(title_blk);

    f.render_widget(title_text, main_chunks[0]);

    // Main screen
    let panel_chunk_container = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_chunks[1]);

    let panel_chunk_left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(panel_chunk_container[0]);

    let panel_chunk_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(panel_chunk_container[1]);

    /* PANELS */
    let top_artists: Vec<String> = app
        .top_artists
        .items
        .iter()
        .enumerate()
        .map(|(i, artist)| format!("{:>2}. {}", i + 1, artist.name.clone()))
        .collect::<Vec<String>>();
    // app.top_artists_state.select(Some(1));
    f.render_stateful_widget(
        List::new(top_artists)
            .block(create_panel_block(Panel::TopArtists, app))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        panel_chunk_left[0],
        &mut app.top_artists_state,
    );

    let top_tracks_rows: Vec<String> = app
        .top_tracks
        .items
        .iter()
        .enumerate()
        .map(|(i, track)| format!("{:>2}. {}", i + 1, track.name.clone()))
        .collect::<Vec<String>>();
    // app.top_tracks_state.select(Some(1));
    f.render_stateful_widget(
        List::new(top_tracks_rows)
            .block(create_panel_block(Panel::TopSongs, app))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        panel_chunk_left[1],
        &mut app.top_tracks_state,
    );

    let recently_played_rows: Vec<String> = app
        .recently_played
        .items
        .iter()
        .enumerate()
        .map(|(i, ph)| format!("{:>2}. {}", i + 1, ph.track.name))
        .collect::<Vec<String>>();
    // app.recently_played_state.select(Some(1));
    f.render_stateful_widget(
        List::new(recently_played_rows)
            .block(create_panel_block(Panel::RecentlyPlayed, app))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        panel_chunk_right[0],
        &mut app.recently_played_state,
    );

    let playlists_rows: Vec<String> = app
        .playlists
        .items
        .iter()
        .enumerate()
        .map(|(i, playlists)| format!("{:>2}. {}", i + 1, playlists.name))
        .collect::<Vec<String>>();
    // app.playlists_state.select(Some(1));
    f.render_stateful_widget(
        List::new(playlists_rows)
            .block(create_panel_block(Panel::Playlists, app))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        panel_chunk_right[1],
        &mut app.playlists_state,
    );

    Ok(())
}

fn create_panel_block(panel: Panel, app: &App) -> Block<'static> {
    let border_style = match panel == app.current_panel {
        true => Style::default().fg(Color::Green),
        false => Style::default(),
    };
    let title_style = match panel == app.current_panel {
        true => Style::new().bold().fg(Color::Cyan),
        false => Style::new().fg(Color::Cyan),
    };

    let mut title_lookup: HashMap<Panel, &str> = HashMap::new();

    title_lookup.insert(Panel::TopArtists, "[ 1 ] Top Artists");
    title_lookup.insert(Panel::TopSongs, "[ 2 ] Top Songs");
    title_lookup.insert(Panel::RecentlyPlayed, "[ 3 ] Recent Songs");
    title_lookup.insert(Panel::Playlists, "[ 4 ] Your Playlists");

    let title = title_lookup.get(&panel).copied().unwrap_or("");

    Block::default()
        .title(title)
        .title_style(title_style)
        .borders(Borders::ALL)
        .border_style(border_style)
        .border_type(ratatui::widgets::BorderType::Rounded)
}
