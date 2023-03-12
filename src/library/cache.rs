use std::collections::{HashMap, HashSet};

use crate::subsonic::{Id, SubsonicData, SubsonicResponse};

use super::{Album, Artist, LibraryItem, LibraryItemKey, Song};

pub struct LibraryCache {
    indexes: HashMap<String, HashSet<LibraryItemKey>>,
    artists: HashMap<Id, CacheEntry<Artist>>,
    albums: HashMap<Id, CacheEntry<Album>>,
    songs: HashMap<Id, CacheEntry<Song>>,
}

impl LibraryCache {
    pub fn new() -> LibraryCache {
        LibraryCache {
            indexes: HashMap::new(),
            artists: HashMap::new(),
            albums: HashMap::new(),
            songs: HashMap::new(),
        }
    }

    pub fn update_root(&mut self, resp: SubsonicResponse) {
        if let Some(SubsonicData::Artists(artists)) = resp.data {
            for index in artists.index {
                let mut index_artists = HashSet::new();
                for artist in index.artist {
                    index_artists.insert(LibraryItemKey::Artist(artist.id.clone()));
                    self.artists.insert(
                        artist.id,
                        CacheEntry {
                            parent: None,
                            children: vec![],
                            item: Artist {
                                name: artist.name.as_str().into(),
                            },
                        },
                    );
                }
                self.indexes.insert(index.name, index_artists);
            }
        }
    }

    pub fn update_artist(&mut self, resp: SubsonicResponse, artist_id: &Id) {
        if let Some(SubsonicData::Artist(artist)) = resp.data {
            let mut album_ids = vec![];
            for album in artist.album {
                album_ids.push(album.id.clone());
                self.albums.insert(
                    album.id,
                    CacheEntry {
                        item: Album {
                            name: album.name.as_str().into(),
                        },
                        parent: Some(artist_id.clone()),
                        children: vec![],
                    },
                );
            }

            self.artists.insert(
                artist.id,
                CacheEntry {
                    parent: None,
                    children: album_ids,
                    item: Artist {
                        name: artist.name.as_str().into(),
                    },
                },
            );
        }
    }

    pub fn update_album(&mut self, resp: SubsonicResponse, album_id: &Id) {
        if let Some(SubsonicData::Album(album)) = resp.data {
            let mut song_ids = vec![];
            for song in album.song {
                song_ids.push(song.id.clone());
                self.songs.insert(
                    song.id,
                    CacheEntry {
                        item: Song {
                            title: song.title.as_str().into(),
                            track_number: song.track,
                            duration: song.duration,
                        },
                        parent: Some(album_id.clone()),
                        children: vec![],
                    },
                );
            }

            self.albums.insert(
                album.id,
                CacheEntry {
                    parent: None,
                    children: song_ids,
                    item: Album {
                        name: album.name.as_str().into(),
                    },
                },
            );
        }
    }

    pub(crate) fn get_children(
        &self,
        key: &LibraryItemKey,
    ) -> Option<Vec<(LibraryItemKey, LibraryItem)>> {
        match key {
            LibraryItemKey::Root => {
                if self.artists.is_empty() {
                    None
                } else {
                    Some(
                        self.artists
                            .iter()
                            .map(|(k, v)| {
                                (
                                    LibraryItemKey::Artist(k.clone()),
                                    LibraryItem::Artist(v.item.clone()),
                                )
                            })
                            .collect(),
                    )
                }
            }
            LibraryItemKey::Artist(artist_id) => {
                if let Some(artist_entry) = self.artists.get(artist_id) {
                    if artist_entry.children.is_empty() {
                        None
                    } else {
                        Some(
                            artist_entry
                                .children
                                .iter()
                                .filter_map(|album_id| {
                                    self.albums.get(album_id).map(|album_entry| {
                                        (
                                            LibraryItemKey::Album(album_id.clone()),
                                            LibraryItem::Album(album_entry.item.clone()),
                                        )
                                    })
                                })
                                .collect(),
                        )
                    }
                } else {
                    None
                }
            }
            LibraryItemKey::Album(album_id) => {
                if let Some(album_entry) = self.artists.get(album_id) {
                    if album_entry.children.is_empty() {
                        None
                    } else {
                        Some(
                            album_entry
                                .children
                                .iter()
                                .filter_map(|song_id| {
                                    self.songs.get(song_id).map(|song_entry| {
                                        (
                                            LibraryItemKey::Song(song_id.clone()),
                                            LibraryItem::Song(song_entry.item.clone()),
                                        )
                                    })
                                })
                                .collect(),
                        )
                    }
                } else {
                    None
                }
            }
            LibraryItemKey::Song(_) => None,
        }
    }
}

struct CacheEntry<T> {
    parent: Option<Id>,
    children: Vec<Id>,
    item: T,
}
