use crate::library::{LibraryItem, LibraryItemKey};

type LibraryRequestResult = Result<Vec<LibraryItem>, String>;

pub enum UiEvent {
    LibraryGetChildrenComplete(Option<LibraryItemKey>, LibraryRequestResult),
    LibraryFindEntriesComplete(usize, LibraryRequestResult),
}
