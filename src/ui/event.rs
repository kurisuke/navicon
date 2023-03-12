use crate::library::{LibraryEntryKey, LibraryItem};

type LibraryRequestResult = Result<Vec<LibraryItem>, String>;

pub enum UiEvent {
    LibraryGetChildrenComplete(Option<LibraryEntryKey>, LibraryRequestResult),
    LibraryFindEntriesComplete(usize, LibraryRequestResult),
}
