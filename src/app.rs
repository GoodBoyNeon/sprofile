use std::error::Error;

use crate::{
    fetch::spotify::{
        get_top_artists, get_top_tracks, get_user_profile, TopArtists, TopTracks, UserProfile,
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
}

impl App {
    pub fn new(user_profile: UserProfile, top_tracks: TopTracks, top_artists: TopArtists) -> Self {
        App {
            current_panel: MyWidgetType::Recent,
            current_screen: CurrentScreen::Main,
            user_profile,
            top_artists,
            top_tracks,
        }
    }
}

pub async fn init_app() -> Result<App, Box<dyn Error>> {
    let user_profile = get_user_profile().await?;
    let top_tracks = get_top_tracks(crate::fetch::spotify::TimeRange::Short).await?;
    let top_artists = get_top_artists(crate::fetch::spotify::TimeRange::Short).await?;
    Ok(App::new(user_profile, top_tracks, top_artists))
}
