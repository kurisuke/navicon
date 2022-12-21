use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicResponse {
    pub status: String,
    pub version: String,

    #[serde(rename = "$value")]
    pub data: Option<SubsonicData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubsonicData {
    License(License),
    MusicFolders(MusicFolders),
    Artists(Artists),
    Artist(Artist),
    Album(Album),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub valid: bool,
    pub email: Option<String>,
    pub license_expires: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolders {
    pub music_folder: Vec<MusicFolder>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    pub id: usize,
    pub name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artists {
    #[serde(default)]
    pub index: Vec<Index>,
    pub ignored_articles: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    #[serde(default)]
    pub artist: Vec<Artist>,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub album_count: usize,
    #[serde(default)]
    pub album: Vec<Album>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: Option<String>,
    pub artist_id: Option<String>,
    pub song_count: usize,
    pub duration: usize,
    pub created: DateTime<Utc>,
    pub year: Option<usize>,
    pub genre: Option<String>,

    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    pub id: String,
    pub parent: Option<String>,
    pub is_dir: bool,
    pub title: String,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub track: Option<usize>,
    pub year: Option<usize>,
    pub genre: Option<String>,
    pub content_type: Option<String>,
    pub duration: Option<usize>,
}
