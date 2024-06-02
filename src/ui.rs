use ratatui::{
    buffer::Cell,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};
use std::collections::HashMap;

use crate::app::App;

#[derive(Hash, PartialEq, Eq)]
pub enum MyWidgetType {
    Playlist,
    Recent,
    ArtistTop,
    SongTop,
}

pub struct MyWidget {
    pub widget_type: MyWidgetType,
    pub widget_id: u64,
    pub left_neighbor: Option<u64>,
    pub right_neighbor: Option<u64>,
    pub up_neighbor: Option<u64>,
    pub down_neighbor: Option<u64>,
}

impl MyWidget {
    pub(crate) fn new(widget_type: MyWidgetType, widget_id: u64) -> Self {
        Self {
            widget_type,
            widget_id,
            up_neighbor: None,
            down_neighbor: None,
            left_neighbor: None,
            right_neighbor: None,
        }
    }
    pub(crate) fn left_neighbor(mut self, left_neighbor: Option<u64>) -> Self {
        self.left_neighbor = left_neighbor;
        self
    }
    pub(crate) fn right_neighbor(mut self, right_neighbor: Option<u64>) -> Self {
        self.right_neighbor = right_neighbor;
        self
    }
    pub(crate) fn up_neighbor(mut self, up_neighbor: Option<u64>) -> Self {
        self.up_neighbor = up_neighbor;
        self
    }
    pub(crate) fn down_neighbor(mut self, down_neighbor: Option<u64>) -> Self {
        self.down_neighbor = down_neighbor;
        self
    }
}

pub fn ui(f: &mut Frame, app: &App) -> Result<(), Box<dyn std::error::Error>> {
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

    let top_artists_rows: Vec<Row> = app
        .top_artists
        .items
        .iter()
        .enumerate()
        .map(|(i, artist)| Row::new(vec![format!("{:>2}. {}", i + 1, artist.name.clone())]))
        .collect::<Vec<Row>>();

    f.render_widget(
        Table::new(top_artists_rows, [Constraint::Percentage(100)])
            .block(create_panel_block(MyWidgetType::ArtistTop, app)),
        panel_chunk_left[0],
    );

    let top_tracks_rows: Vec<Row> = app
        .top_tracks
        .items
        .iter()
        .enumerate()
        .map(|(i, track)| Row::new(vec![format!("{:>2}. {}", i + 1, track.name.clone())]))
        .collect::<Vec<Row>>();

    f.render_widget(
        Table::new(top_tracks_rows, [Constraint::Percentage(100)])
            .block(create_panel_block(MyWidgetType::SongTop, app)),
        panel_chunk_left[1],
    );

    let recently_played_rows: Vec<Row> = app
        .recently_played
        .items
        .iter()
        .enumerate()
        .map(|(i, ph)| Row::new(vec![format!("{:>2}. {}", i + 1, ph.clone().track.name)]))
        .collect::<Vec<Row>>();

    f.render_widget(
        Table::new(recently_played_rows, [Constraint::Percentage(100)])
            .block(create_panel_block(MyWidgetType::Recent, app)),
        panel_chunk_right[0],
    );

    let playlists_rows: Vec<Row> = app
        .playlists
        .items
        .iter()
        .enumerate()
        .map(|(i, playlists)| Row::new(vec![format!("{:>2}. {}", i + 1, playlists.name)]))
        .collect::<Vec<Row>>();
    f.render_widget(
        Table::new(playlists_rows, [Constraint::Percentage(100)])
            .block(create_panel_block(MyWidgetType::Playlist, app)),
        panel_chunk_right[1],
    );

    Ok(())
}

fn create_panel_block(panel: MyWidgetType, app: &App) -> Block<'static> {
    let style = match panel == app.current_panel {
        true => Style::default().fg(Color::Green),
        false => Style::default(),
    };

    let mut title_lookup: HashMap<MyWidgetType, &str> = HashMap::new();
    title_lookup.insert(MyWidgetType::ArtistTop, " Top Artists ");
    title_lookup.insert(MyWidgetType::SongTop, " Top Songs ");
    title_lookup.insert(MyWidgetType::Recent, " Recent Songs ");
    title_lookup.insert(MyWidgetType::Playlist, " Your Playlists ");

    let title = title_lookup.get(&panel).copied().unwrap_or("");

    Block::default()
        .borders(Borders::ALL)
        .border_style(style)
        .title(title)
        .title_style(Style::new().bold().fg(Color::Cyan))
        .border_type(ratatui::widgets::BorderType::Plain)
}
