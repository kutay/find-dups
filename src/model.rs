use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub(crate) struct FileEntry {
    pub path: String,
    pub hash: String,
    pub created: String,
}

impl Ord for FileEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.created.cmp(&other.created)
    }
}

impl PartialOrd for FileEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FileEntry {
    fn eq(&self, other: &Self) -> bool {
        self.created == other.created
    }
}

pub(crate) struct SearchOptions {
    pub(crate) folder: String,
    pub(crate) skip_hidden_folders: bool,
}
