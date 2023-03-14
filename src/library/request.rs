use super::LibraryItemKey;

pub enum LibraryRequest {
    GetChildren(LibraryItemKey),
    FindEntries(usize, FindType, String),
    Shutdown,
}

pub enum FindType {
    Any,
    Artist,
    Album,
    Song,
}
