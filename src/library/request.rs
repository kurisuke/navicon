use super::LibraryEntryKey;

pub enum LibraryRequest {
    GetChildren(Option<LibraryEntryKey>),
    FindEntries(usize, FindType, String),
}

pub enum FindType {
    Any,
    Artist,
    Album,
    Song,
}
