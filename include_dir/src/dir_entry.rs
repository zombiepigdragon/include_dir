use crate::{compress::Compression, Dir, File};
use std::path::Path;

/// A directory entry, roughly analogous to [`std::fs::DirEntry`].
#[derive(Debug, Clone, PartialEq)]
pub enum DirEntry<'a, C: Compression = crate::compress::None> {
    /// A directory.
    Dir(Dir<'a, C>),
    /// A file.
    File(File<'a, C>),
}

impl<'a, C: Compression> DirEntry<'a, C> {
    /// The [`DirEntry`]'s full path.
    pub fn path(&self) -> &'a Path {
        match self {
            DirEntry::Dir(d) => d.path(),
            DirEntry::File(f) => f.path(),
        }
    }

    /// Try to get this as a [`Dir`], if it is one.
    pub fn as_dir(&self) -> Option<&Dir<'a, C>> {
        match self {
            DirEntry::Dir(d) => Some(d),
            DirEntry::File(_) => None,
        }
    }

    /// Try to get this as a [`File`], if it is one.
    pub fn as_file(&self) -> Option<&File<'a, C>> {
        match self {
            DirEntry::File(f) => Some(f),
            DirEntry::Dir(_) => None,
        }
    }

    /// Get this item's sub-items, if it has any.
    pub fn children(&self) -> &'a [DirEntry<'a, C>] {
        match self {
            DirEntry::Dir(d) => d.entries(),
            DirEntry::File(_) => &[],
        }
    }
}
