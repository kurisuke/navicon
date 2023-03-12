use super::LibraryItemKey;

pub enum LibraryRequest {
    GetChildren(Option<LibraryItemKey>),
    FindEntries(usize, FindType, String),
}

pub enum FindType {
    Any,
    Artist,
    Album,
    Song,
}
