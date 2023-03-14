use crate::library::{LibraryItem, LibraryItemKey};

pub type LibraryRequestResult = Vec<(LibraryItemKey, LibraryItem)>;

pub enum UiEvent {
    AddLog(String),
    SetStatus(String),
    LibraryGetChildrenComplete(LibraryItemKey, LibraryRequestResult),
    LibraryFindEntriesComplete(usize, LibraryRequestResult),
}
