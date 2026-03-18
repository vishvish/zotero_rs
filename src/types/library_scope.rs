//! Library scope identifiers for user and group endpoints.

/// Scope used to select the Zotero library namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibraryScope {
    /// User library scope.
    User(u64),
    /// Group library scope.
    Group(u64),
}

impl LibraryScope {
    /// Returns the API path prefix for the selected scope.
    pub fn path_prefix(self) -> String {
        match self {
            Self::User(id) => format!("users/{id}"),
            Self::Group(id) => format!("groups/{id}"),
        }
    }
}
