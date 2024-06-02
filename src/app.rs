use std::error::Error;

use crate::{
    fetch::spotify::{
        get_playlists, get_recently_played, get_top_artists, get_top_tracks, get_user_profile,
        Playlists, RecentlyPlayed, TopArtists, TopTracks, UserProfile,
    },
    ui::MyWidgetType,
};

pub enum CurrentScreen {
    Main,
    Artist,
    Playlist,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_panel: MyWidgetType,
    pub user_profile: UserProfile,
    pub top_artists: TopArtists,
    pub top_tracks: TopTracks,
    pub recently_played: RecentlyPlayed,
    pub playlists: Playlists,
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
            current_panel: MyWidgetType::Recent,
            current_screen: CurrentScreen::Main,
            user_profile,
            top_artists,
            top_tracks,
            recently_played,
            playlists,
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
