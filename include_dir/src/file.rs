use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    path::Path,
};

use once_cell::sync::OnceCell;

use crate::compress::Compression;

/// A file with its contents stored in a `&'static [u8]`.
#[derive(Clone, PartialEq, Eq)]
pub struct File<'a, C: Compression = crate::compress::None> {
    path: &'a str,
    compressed: &'a [u8],
    decompressed: &'a OnceCell<Cow<'a, [u8]>>,
    #[cfg(feature = "metadata")]
    metadata: Option<crate::Metadata>,
    _compression: PhantomData<C>,
}

impl<'a, C: Compression> File<'a, C> {
    /// Create a new [`File`].
    pub const fn new(
        path: &'a str,
        compressed: &'a [u8],
        decompressed: &'a OnceCell<Cow<'a, [u8]>>,
    ) -> Self {
        File {
            path,
            compressed,
            decompressed,
            #[cfg(feature = "metadata")]
            metadata: None,
            _compression: PhantomData,
        }
    }

    /// The full path for this [`File`], relative to the directory passed to
    /// [`crate::include_dir!()`].
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// The file's raw contents.
    pub fn contents(&self) -> &[u8] {
        self.decompressed
            .get_or_init(|| C::decompress(self.compressed))
    }

    /// The file's contents interpreted as a string.
    pub fn contents_utf8(&self) -> Option<&str> {
        std::str::from_utf8(self.contents()).ok()
    }
}

#[cfg(feature = "metadata")]
impl<'a, C: Compression> File<'a, C> {
    /// Set the [`Metadata`] associated with a [`File`].
    pub const fn with_metadata(self, metadata: crate::Metadata) -> Self {
        let File { path, compressed, decompressed, .. } = self;

        File {
            path,
            compressed,
            decompressed,
            metadata: Some(metadata),
            _compression: PhantomData,
        }
    }

    /// Get the [`File`]'s [`Metadata`], if available.
    pub fn metadata(&self) -> Option<&crate::Metadata> {
        self.metadata.as_ref()
    }
}

impl<'a, C: Compression> Debug for File<'a, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let File {
            path,
            compressed,
            decompressed,
            #[cfg(feature = "metadata")]
            metadata,
            _compression,
        } = self;

        let mut d = f.debug_struct("File");

        d.field("path", path);
        d.field(
            "contents",
            &match decompressed.get() {
                Some(contents) => format!("<{} bytes>", contents.len()),
                None => format!("<{} bytes (compressed)>", compressed.len()),
            },
        );

        #[cfg(feature = "metadata")]
        d.field("metadata", metadata);

        d.finish()
    }
}
