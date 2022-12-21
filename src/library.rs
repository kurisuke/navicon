use std::collections::{HashMap, HashSet};

use color_eyre::Result;

use crate::{conn::Connection, subsonic::SubsonicData};

pub struct Library {
    indexes: HashMap<String, HashSet<LibraryEntryKey>>,
    entries: HashMap<String, LibraryEntry>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            indexes: HashMap::new(),
            entries: HashMap::new(),
        }
    }

    pub fn update_root(&mut self, conn: &Connection) -> Result<()> {
        let resp = conn.get_artists()?;
        if let Some(SubsonicData::Artists(artists)) = resp.data {
            for index in artists.index {
                let mut index_artists = HashSet::new();
                for artist in index.artist {
                    index_artists.insert(artist.id.clone());
                    self.entries.insert(
                        artist.id,
                        LibraryEntry {
                            parent: None,
                            children: vec![],
                            item: Item::Artist(Artist {
                                name: artist.name.as_str().into(),
                            }),
                        },
                    );
                }
                self.indexes.insert(index.name, index_artists);
            }
        }
        Ok(())
    }

    pub fn find_artist(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        let contains: SearchString = contains.into();
        self.entries
            .iter()
            .filter(|(_, e)| {
                if let Item::Artist(artist) = &e.item {
                    artist.name.contains(&contains)
                } else {
                    false
                }
            })
            .map(|(k, _)| k)
            .collect()
    }

    pub fn update_artist(&mut self, conn: &Connection, artist_id: &LibraryEntryKey) -> Result<()> {
        let resp = conn.get_artist(artist_id)?;
        if let Some(SubsonicData::Artist(artist)) = resp.data {
            let mut album_ids = vec![];
            for album in artist.album {
                album_ids.push(album.id.clone());
                self.entries.insert(
                    album.id.clone(),
                    LibraryEntry {
                        item: Item::Album(Album {
                            name: album.name.as_str().into(),
                        }),
                        parent: Some(artist_id.clone()),
                        children: vec![],
                    },
                );
            }

            self.entries.insert(
                artist.id,
                LibraryEntry {
                    parent: None,
                    children: album_ids,
                    item: Item::Artist(Artist {
                        name: artist.name.as_str().into(),
                    }),
                },
            );
        }
        Ok(())
    }

    pub fn find_album(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        let contains: SearchString = contains.into();
        self.entries
            .iter()
            .filter(|(_, e)| {
                if let Item::Album(album) = &e.item {
                    album.name.contains(&contains)
                } else {
                    false
                }
            })
            .map(|(k, _)| k)
            .collect()
    }

    pub fn update_album(&mut self, conn: &Connection, album_id: &LibraryEntryKey) -> Result<()> {
        let resp = conn.get_album(album_id)?;
        if let Some(SubsonicData::Album(album)) = resp.data {
            let mut song_ids = vec![];
            for song in album.song {
                song_ids.push(song.id.clone());
                self.entries.insert(
                    song.id.clone(),
                    LibraryEntry {
                        item: Item::Song(Song {
                            title: song.title.as_str().into(),
                            track_number: song.track,
                            duration: song.duration,
                        }),
                        parent: Some(album_id.clone()),
                        children: vec![],
                    },
                );
            }

            self.entries.insert(
                album.id,
                LibraryEntry {
                    parent: None,
                    children: song_ids,
                    item: Item::Album(Album {
                        name: album.name.as_str().into(),
                    }),
                },
            );
        }
        Ok(())
    }

    pub fn find_song(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        let contains: SearchString = contains.into();
        self.entries
            .iter()
            .filter(|(_, e)| {
                if let Item::Song(song) = &e.item {
                    song.title.contains(&contains)
                } else {
                    false
                }
            })
            .map(|(k, _)| k)
            .collect()
    }

    pub fn find_entry(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        let contains: SearchString = contains.into();
        self.entries
            .iter()
            .filter(|(_, e)| match &e.item {
                Item::Artist(artist) => artist.name.contains(&contains),
                Item::Album(album) => album.name.contains(&contains),
                Item::Song(song) => song.title.contains(&contains),
            })
            .map(|(k, _)| k)
            .collect()
    }

    pub fn get_item(&self, id: &LibraryEntryKey) -> Option<&Item> {
        self.entries.get(id).map(|e| &e.item)
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
