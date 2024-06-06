use std::error::Error;

use ratatui::widgets::ListState;

use crate::{
    fetch::spotify::{
        get_playlists, get_recently_played, get_top_artists, get_top_tracks, get_user_profile,
        PlaylistsData, RecentlyPlayedData, TimeRange, TopArtistsData, TopTracksData, UserProfile,
    },
    ui::Panel,
};

pub enum CurrentScreen {
    Main,
    // Artist,
    // Playlist,
}

pub struct TopArtists {
    pub data: TopArtistsData,
    pub state: ListState,
}
pub struct TopTracks {
    pub data: TopTracksData,
    pub state: ListState,
}
pub struct RecentlyPlayed {
    pub data: RecentlyPlayedData,
    pub state: ListState,
}
pub struct Playlists {
    pub data: PlaylistsData,
    pub state: ListState,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_panel: Panel,

    pub user_profile: UserProfile,

    pub top_artists: TopArtists,
    pub top_tracks: TopTracks,
    pub recently_played: RecentlyPlayed,
    pub playlists: Playlists,
}

impl App {
    pub fn new(
        user_profile: UserProfile,
        top_tracks_data: TopTracksData,
        top_artists_data: TopArtistsData,
        recently_played_data: RecentlyPlayedData,
        playlists_data: PlaylistsData,
    ) -> Self {
        App {
            current_panel: Panel::RecentlyPlayed,
            current_screen: CurrentScreen::Main,
            user_profile,
            top_artists: TopArtists {
                data: top_artists_data,
                state: ListState::default(),
            },
            top_tracks: TopTracks {
                data: top_tracks_data,
                state: ListState::default(),
            },
            recently_played: RecentlyPlayed {
                data: recently_played_data,
                state: ListState::default(),
            },
            playlists: Playlists {
                data: playlists_data,
                state: ListState::default(),
            },
        }
    }
    pub fn get_current_panel(&mut self) -> &mut ListState {
        match self.current_panel {
            Panel::TopArtists => &mut self.top_artists.state,
            Panel::TopSongs => &mut self.top_tracks.state,
            Panel::RecentlyPlayed => &mut self.recently_played.state,
            Panel::Playlists => &mut self.recently_played.state,
        }
    }
}

pub async fn init_app() -> Result<App, Box<dyn Error>> {
    let user_profile = get_user_profile().await?;
    let top_tracks = get_top_tracks(TimeRange::Medium).await?;
    let top_artists = get_top_artists(TimeRange::Medium).await?;
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
