use crate::{app::App, controllers::fetch::TimeRange};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List},
    Frame,
};
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum Panel {
    Playlists,
    RecentlyPlayed,
    TopArtists,
    TopTracks,
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

    let username = &app.user_profile.display_name;
    let title = format!(" {} ", username);

    let title_blk = Block::default()
        .style(Style::default())
        .title(title.to_string())
        .title_style(Style::new().fg(Color::Black).bold().bg(Color::Blue))
        .title_alignment(Alignment::Center);

    f.render_widget(title_blk, main_chunks[0]);

    let footer_blk = Block::default()
        .style(Style::default())
        .title("F1: all time / F2: 6 months / F3: 4 weeks / 1-2-3-4: change panel / q: quit")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().dark_gray());

    f.render_widget(footer_blk, main_chunks[2]);

    // Main screen
    let panel_chunk_container = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
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

    let hl_style = Style::default().bold().on_dark_gray();

    let top_artists: Vec<String> = app
        .top_artists
        .data
        .items
        .iter()
        .enumerate()
        .map(|(i, artist)| format!("{:>2}. {}", i + 1, artist.name.clone()))
        .collect::<Vec<String>>();

    f.render_stateful_widget(
        List::new(top_artists)
            .block(create_panel_block(Panel::TopArtists, app))
            .highlight_style(hl_style)
            .repeat_highlight_symbol(true),
        panel_chunk_left[0],
        &mut app.top_artists.state,
    );

    let top_tracks_rows: Vec<String> = app
        .top_tracks
        .data
        .items
        .iter()
        .enumerate()
        .map(|(i, track)| format!("{:>2}. {}", i + 1, track.name.clone()))
        .collect::<Vec<String>>();
    f.render_stateful_widget(
        List::new(top_tracks_rows)
            .block(create_panel_block(Panel::TopTracks, app))
            .highlight_style(hl_style)
            .repeat_highlight_symbol(true),
        panel_chunk_left[1],
        &mut app.top_tracks.state,
    );

    let recently_played_rows: Vec<String> = app
        .recently_played
        .data
        .items
        .iter()
        .enumerate()
        .map(|(i, ph)| format!("{:>2}. {}", i + 1, ph.track.name))
        .collect::<Vec<String>>();
    f.render_stateful_widget(
        List::new(recently_played_rows)
            .block(create_panel_block(Panel::RecentlyPlayed, app))
            .highlight_style(hl_style)
            .repeat_highlight_symbol(true),
        panel_chunk_right[0],
        &mut app.recently_played.state,
    );

    let playlists_rows: Vec<String> = app
        .playlists
        .data
        .items
        .iter()
        .enumerate()
        .map(|(i, playlists)| format!("{:>2}. {}", i + 1, playlists.name))
        .collect::<Vec<String>>();
    f.render_stateful_widget(
        List::new(playlists_rows)
            .block(create_panel_block(Panel::Playlists, app))
            .highlight_style(hl_style)
            .repeat_highlight_symbol(true),
        panel_chunk_right[1],
        &mut app.playlists.state,
    );

    Ok(())
}

fn create_panel_block(panel: Panel, app: &App) -> Block<'static> {
    let block_style = match panel == app.current_panel {
        true => Style::default(),
        false => Style::default().gray(),
    };
    let border_style = match panel == app.current_panel {
        true => Style::default().fg(Color::Green),
        false => Style::default(),
    };
    let border_type = match panel == app.current_panel {
        true => BorderType::Thick,
        false => BorderType::Plain,
    };
    let block_name_style = match panel == app.current_panel {
        true => Style::new().bold().fg(Color::Cyan),
        false => Style::new().fg(Color::Cyan),
    };

    let mut title_lookup: HashMap<Panel, &str> = HashMap::new();

    title_lookup.insert(Panel::TopArtists, "Top Artists (1)");
    title_lookup.insert(Panel::TopTracks, "Top Songs (2)");
    title_lookup.insert(Panel::RecentlyPlayed, "Recent Songs (3)");
    title_lookup.insert(Panel::Playlists, "Your Playlists (4)");

    let block_name = title_lookup.get(&panel).copied().unwrap_or("");

    let base_blk = Block::default()
        .title(block_name)
        .style(block_style)
        .title_style(block_name_style)
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(border_style)
        .border_type(border_type);

    return match panel {
        Panel::TopArtists => base_blk
            .title(
                Line::styled(
                    "<all time>",
                    get_right_title_style(TimeRange::Long, Panel::TopArtists, app),
                )
                .right_aligned(),
            )
            .title(
                Line::styled(
                    "<6months>",
                    get_right_title_style(TimeRange::Medium, Panel::TopArtists, app),
                )
                .right_aligned(),
            )
            .title(
                Line::styled(
                    "<4weeks>",
                    get_right_title_style(TimeRange::Short, Panel::TopArtists, app),
                )
                .right_aligned(),
            ),
        Panel::TopTracks => base_blk
            .title(
                Line::styled(
                    "<all time>",
                    get_right_title_style(TimeRange::Long, Panel::TopTracks, app),
                )
                .right_aligned(),
            )
            .title(
                Line::styled(
                    "<6month>",
                    get_right_title_style(TimeRange::Medium, Panel::TopTracks, app),
                )
                .right_aligned(),
            )
            .title(
                Line::styled(
                    "<4week>",
                    get_right_title_style(TimeRange::Short, Panel::TopTracks, app),
                )
                .right_aligned(),
            ),
        _ => base_blk,
    };
}
fn get_right_title_style(time_range: TimeRange, panel: Panel, app: &App) -> Style {
    if panel != app.current_panel {
        Style::default().not_bold().white()
    } else {
        match panel {
            Panel::TopArtists => match app.top_artists.cur_time_range == time_range {
                true => Style::default().green().bold(),
                false => Style::default().not_bold(),
            },
            Panel::TopTracks => match app.top_tracks.cur_time_range == time_range {
                true => Style::default().green().bold(),
                false => Style::default().not_bold(),
            },
            _ => Style::default(),
        }
    }
}
