use std::collections::{HashMap, HashSet};

use color_eyre::Result;

use crate::subsonic::{SubsonicData, SubsonicResponse};

use super::{Album, Artist, LibraryEntry, LibraryEntryKey, LibraryItem, SearchString, Song};

pub struct LibraryCache {
    indexes: HashMap<String, HashSet<LibraryEntryKey>>,
    entries: HashMap<String, LibraryEntry>,
}

impl LibraryCache {
    pub fn new() -> LibraryCache {
        LibraryCache {
            indexes: HashMap::new(),
            entries: HashMap::new(),
        }
    }

    pub fn update_root(&mut self, resp: SubsonicResponse) -> Result<()> {
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
                            item: LibraryItem::Artist(Artist {
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

    pub fn get_children(&self, parent: Option<LibraryEntryKey>) -> Vec<&LibraryEntryKey> {
        self.entries
            .iter()
            .filter(|(_, e)| parent == e.parent)
            .map(|(k, _)| k)
            .collect()
    }

    pub fn find_artist(&self, contains: &str) -> Vec<&LibraryEntryKey> {
        let contains: SearchString = contains.into();
        self.entries
            .iter()
            .filter(|(_, e)| {
                if let LibraryItem::Artist(artist) = &e.item {
                    artist.name.contains(&contains)
                } else {
                    false
                }
            })
            .map(|(k, _)| k)
            .collect()
    }

    pub fn update_artist(
        &mut self,
        resp: SubsonicResponse,
        artist_id: &LibraryEntryKey,
    ) -> Result<()> {
        if let Some(SubsonicData::Artist(artist)) = resp.data {
            let mut album_ids = vec![];
            for album in artist.album {
                album_ids.push(album.id.clone());
                self.entries.insert(
                    album.id.clone(),
                    LibraryEntry {
                        item: LibraryItem::Album(Album {
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
                    item: LibraryItem::Artist(Artist {
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
                if let LibraryItem::Album(album) = &e.item {
                    album.name.contains(&contains)
                } else {
                    false
                }
            })
            .map(|(k, _)| k)
            .collect()
    }

    pub fn update_album(
        &mut self,
        resp: SubsonicResponse,
        album_id: &LibraryEntryKey,
    ) -> Result<()> {
        if let Some(SubsonicData::Album(album)) = resp.data {
            let mut song_ids = vec![];
            for song in album.song {
                song_ids.push(song.id.clone());
                self.entries.insert(
                    song.id.clone(),
                    LibraryEntry {
                        item: LibraryItem::Song(Song {
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
                    item: LibraryItem::Album(Album {
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
                if let LibraryItem::Song(song) = &e.item {
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
                LibraryItem::Artist(artist) => artist.name.contains(&contains),
                LibraryItem::Album(album) => album.name.contains(&contains),
                LibraryItem::Song(song) => song.title.contains(&contains),
            })
            .map(|(k, _)| k)
            .collect()
    }

    pub fn get_item(&self, id: &LibraryEntryKey) -> Option<&LibraryItem> {
        self.entries.get(id).map(|e| &e.item)
    }
}
