use reqwest::Client;
use serde::Deserialize;
use std::{error::Error, u8};

use crate::utils::get_access_token;

const BASE_URL: &str = "https://api.spotify.com/v1";

#[derive(Deserialize, Debug)]
pub struct Followers {
    pub total: u32,
    pub href: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    pub display_name: String,
    pub followers: Followers,
}

pub async fn get_user_profile() -> Result<UserProfile, Box<dyn Error>> {
    let token = get_access_token().await?;

    let client = Client::new();
    let url = format!("{}/{}", BASE_URL, "me");

    let res = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<UserProfile>()
        .await?;

    Ok(res)
}

#[derive(Deserialize, Debug)]
pub enum TimeRange {
    Short,
    Medium,
    Long,
}
#[derive(Deserialize, Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    // pub followers: Followers,
    // pub popularity: u8,
    pub uri: String,
    // pub genres: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct Track {
    pub id: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: u16,
    pub duration_ms: u32,
    pub explicit: bool,
    pub href: String,
    // pub is_playable: bool,
    pub name: String,
    pub popularity: u8,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub is_local: bool,
}
#[derive(Deserialize, Debug)]
pub struct TopTracks {
    pub href: String,
    pub limit: u8,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<Track>,
}

#[derive(Deserialize, Debug)]
pub struct TopArtists {
    pub href: String,
    pub limit: u8,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<Artist>,
}

pub async fn get_top_artists(time_range: TimeRange) -> Result<TopArtists, Box<dyn Error>> {
    let token = get_access_token().await?;

    let client = Client::new();
    let url = format!("{}/{}/{}/{}", BASE_URL, "me", "top", "artists",);

    let res = client
        .get(url)
        .query(&[
            (
                "time_range",
                match time_range {
                    TimeRange::Long => "long_term",
                    TimeRange::Medium => "medium_term",
                    TimeRange::Short => "short_term",
                },
            ),
            ("limit", "50"),
            ("offset", "0"),
        ])
        .bearer_auth(token)
        .send()
        .await?
        .json::<TopArtists>()
        .await?;

    Ok(res)
}
pub async fn get_top_tracks(time_range: TimeRange) -> Result<TopTracks, Box<dyn Error>> {
    let token = get_access_token().await?;

    let client = Client::new();
    let url = format!("{}/{}/{}/{}", BASE_URL, "me", "top", "tracks");

    let res = client
        .get(url)
        .query(&[
            (
                "time_range",
                match time_range {
                    TimeRange::Long => "long_term",
                    TimeRange::Medium => "medium_term",
                    TimeRange::Short => "short_term",
                },
            ),
            ("limit", "50"),
            ("offset", "0"),
        ])
        .bearer_auth(token)
        .send()
        .await?
        .json::<TopTracks>()
        .await?;

    Ok(res)
}

#[derive(Deserialize, Debug)]
pub struct PlayHistory {
    pub track: Track,
    pub played_at: String,
}
#[derive(Deserialize, Debug)]
pub struct RecentlyPlayed {
    pub href: String,
    pub limit: u8,
    pub next: Option<String>,
    pub items: Vec<PlayHistory>,
}
pub async fn get_recently_played() -> Result<RecentlyPlayed, Box<dyn Error>> {
    let token = get_access_token().await?;

    let client = Client::new();
    let url = format!("{}/{}/{}/{}", BASE_URL, "me", "player", "recently-played");

    let res = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("limit", "50")])
        .send()
        .await?
        .json::<RecentlyPlayed>()
        .await?;

    Ok(res)
}

#[derive(Deserialize, Debug)]
pub struct SimplifiedTrack {
    pub href: String,
    pub total: u32,
}
#[derive(Deserialize, Debug)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub public: Option<bool>,
    pub snapshot_id: String,
    // pub tracks: Option<Vec<Option<SimplifiedTrack>>>,
    pub uri: String,
}
#[derive(Deserialize, Debug)]
pub struct Playlists {
    pub href: String,
    pub limit: u8,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<SimplifiedPlaylist>,
}

pub async fn get_playlists() -> Result<Playlists, Box<dyn Error>> {
    let token = get_access_token().await?;

    let client = Client::new();
    let url = format!("{}/{}/{}", BASE_URL, "me", "playlists");

    let res = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("limit", "50")])
        .send()
        .await?
        .json::<Playlists>()
        .await?;

    Ok(res)
}
