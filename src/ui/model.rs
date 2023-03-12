use crate::library::{LibraryEntryKey, LibraryItem};

struct Model {
    context: Context,
    log: String,
}

impl Model {
    pub fn new() -> Model {
        Model {
            context: Context::Library(LibraryContext {
                this: None,
                children: Resolvable::Pending,
                parent: None,
            }),
            log: String::new(),
        }
    }
}

enum Context {
    Library(LibraryContext),
    Search(SearchContext),
}

struct LibraryContext {
    this: Option<LibraryItem>,
    children: Resolvable<Vec<LibraryItem>>,
    parent: Option<LibraryEntryKey>,
}

struct SearchContext {
    query: String,
    results: Resolvable<Vec<LibraryItem>>,
}

enum Resolvable<T> {
    Pending,
    Resolved(T),
    Error,
}
