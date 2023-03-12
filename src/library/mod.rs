mod cache;
mod request;

use crate::{conn::Connection, subsonic::Id};

use self::cache::LibraryCache;

use color_eyre::{eyre::bail, Result};

pub struct Library {
    conn: Connection,
    cache: LibraryCache,
}

impl Library {
    pub fn new(conn: Connection) -> Library {
        Library {
            conn,
            cache: LibraryCache::new(),
        }
    }

    pub fn get_children(
        &mut self,
        key: &LibraryItemKey,
    ) -> Result<Vec<(LibraryItemKey, LibraryItem)>> {
        if let Some(children) = self.cache.get_children(key) {
            Ok(children)
        } else {
            match key {
                LibraryItemKey::Root => {
                    let resp = self.conn.get_artists()?;
                    self.cache.update_root(resp);
                }
                LibraryItemKey::Artist(artist_id) => {
                    let resp = self.conn.get_artist(artist_id)?;
                    self.cache.update_artist(resp, artist_id);
                }
                LibraryItemKey::Album(album_id) => {
                    let resp = self.conn.get_album(album_id)?;
                    self.cache.update_album(resp, album_id);
                }
                LibraryItemKey::Song(_) => {}
            }
            if let Some(children) = self.cache.get_children(key) {
                Ok(children)
            } else {
                bail!("empty children")
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum LibraryItemKey {
    Root,
    Artist(Id),
    Album(Id),
    Song(Id),
}

pub enum LibraryItem {
    Artist(Artist),
    Album(Album),
    Song(Song),
}

impl std::fmt::Display for LibraryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibraryItem::Artist(artist) => write!(f, "{}", artist.name),
            LibraryItem::Album(album) => write!(f, "{}", album.name),
            LibraryItem::Song(song) => write!(f, "{}", song),
        }
    }
}

#[derive(Clone)]
pub struct Artist {
    pub name: SearchString,
}

#[derive(Clone)]
pub struct Album {
    pub name: SearchString,
}

#[derive(Clone)]
pub struct Song {
    pub title: SearchString,
    pub track_number: Option<usize>,
    pub duration: Option<usize>,
}

impl std::fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} [{}:{:02}]",
            self.track_number.unwrap_or_default(),
            self.title,
            self.duration.unwrap_or_default() / 60,
            self.duration.unwrap_or_default() % 60
        )
    }
}

#[derive(Clone)]
pub struct SearchString {
    display: String,
    search: String,
}

impl SearchString {
    fn contains(&self, other: &SearchString) -> bool {
        self.search.contains(&other.search)
    }
}

impl From<&str> for SearchString {
    fn from(value: &str) -> Self {
        SearchString {
            display: value.to_string(),
            search: value.to_lowercase(),
        }
    }
}

impl std::fmt::Display for SearchString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}
