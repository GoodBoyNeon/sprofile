use std::error::Error;

use ratatui::widgets::ListState;

use crate::{
    fetch::spotify::{
        get_playlists, get_recently_played, get_top_artists, get_top_tracks, get_user_profile,
        Playlists, RecentlyPlayed, TopArtists, TopTracks, UserProfile,
    },
    ui::Panel,
};

// pub struct PanelStates {
//     pub top_artists_state:
//     pub top_tracks_state:
//     pub recently_played_state:
//     pub playlists_state:
// }

pub enum CurrentScreen {
    Main,
    Artist,
    Playlist,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_panel: Panel,
    pub user_profile: UserProfile,
    pub top_artists: TopArtists,
    pub top_tracks: TopTracks,
    pub recently_played: RecentlyPlayed,
    pub playlists: Playlists,
    pub top_tracks_state: ListState,
    pub top_artists_state: ListState,
    pub recently_played_state: ListState,
    pub playlists_state: ListState,
}

impl App {
    pub fn new(
        user_profile: UserProfile,
        top_tracks: TopTracks,
        top_artists: TopArtists,
        recently_played: RecentlyPlayed,
        playlists: Playlists,
    ) -> Self {
        App {
            current_panel: Panel::RecentlyPlayed,
            current_screen: CurrentScreen::Main,
            user_profile,
            top_artists,
            top_tracks,
            recently_played,
            playlists,
            top_tracks_state: ListState::default(),
            top_artists_state: ListState::default(),
            recently_played_state: ListState::default(),
            playlists_state: ListState::default(),
        }
    }
    pub fn get_current_panel_state(&mut self) -> &mut ListState {
        match self.current_panel {
            Panel::TopArtists => &mut self.top_artists_state,
            Panel::TopSongs => &mut self.top_tracks_state,
            Panel::RecentlyPlayed => &mut self.recently_played_state,
            Panel::Playlists => &mut self.recently_played_state,
        }
    }
}

pub async fn init_app() -> Result<App, Box<dyn Error>> {
    let user_profile = get_user_profile().await?;
    let top_tracks = get_top_tracks(crate::fetch::spotify::TimeRange::Short).await?;
    let top_artists = get_top_artists(crate::fetch::spotify::TimeRange::Short).await?;
    let recently_played = get_recently_played().await?;
    let playlists = get_playlists().await?;
    Ok(App::new(
        user_profile,
        top_tracks,
        top_artists,
        recently_played,
        playlists,
    ))
}
