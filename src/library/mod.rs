mod cache;

use color_eyre::Result;

use crate::conn::Connection;

use self::cache::LibraryCache;

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

    pub fn update_root(&mut self) -> Result<()> {
        let resp = self.conn.get_artists()?;
        self.cache.update_root(resp)?;
        Ok(())
    }

    pub fn get_children(&self, parent: Option<LibraryEntryKey>) -> Vec<&LibraryEntryKey> {
        self.cache.get_children(parent)
    }

    pub fn find_artist(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        self.cache.find_artist(contains)
    }

    pub fn update_artist(&mut self, artist_id: &LibraryEntryKey) -> Result<()> {
        let resp = self.conn.get_artist(artist_id)?;
        self.cache.update_artist(resp, artist_id)?;
        Ok(())
    }

    pub fn find_album(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        self.cache.find_album(contains)
    }

    pub fn update_album(&mut self, album_id: &LibraryEntryKey) -> Result<()> {
        let resp = self.conn.get_album(album_id)?;
        self.cache.update_album(resp, album_id)?;
        Ok(())
    }

    pub fn find_song(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        self.cache.find_song(contains)
    }

    pub fn find_entry(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        self.cache.find_entry(contains)
    }

    pub fn get_item(&self, id: &LibraryEntryKey) -> Option<&Item> {
        self.cache.get_item(id)
    }
}

pub type LibraryEntryKey = String;

struct LibraryEntry {
    parent: Option<String>,
    children: Vec<String>,
    item: Item,
}

pub enum Item {
    Artist(Artist),
    Album(Album),
    Song(Song),
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Artist(artist) => write!(f, "{}", artist.name),
            Item::Album(album) => write!(f, "{}", album.name),
            Item::Song(song) => write!(f, "{}", song),
        }
    }
}

pub struct Artist {
    pub name: SearchString,
}

pub struct Album {
    pub name: SearchString,
}

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
