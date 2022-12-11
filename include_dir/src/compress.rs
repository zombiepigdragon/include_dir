//! Support for compressing embedded files and extracting them lazily at runtime.
#![allow(missing_copy_implementations, missing_debug_implementations)] // most types in this module can't be instantiated

use std::borrow::Cow;

/// Trait for a method of performing compression.
pub trait Compression {
    /// Perform decompression.
    fn decompress(data: &[u8]) -> Cow<'_, [u8]>;
}

/// Do not do compression on embedded files.
pub enum None {}

impl Compression for None {
    fn decompress(data: &[u8]) -> Cow<'_, [u8]> {
        Cow::Borrowed(data)
    }
}

/// Do zstd compression on embedded files.
#[cfg(feature = "compression-zstd")]
pub enum Zstd {}

#[cfg(feature = "compression-zstd")]
impl Compression for Zstd {
    fn decompress(data: &[u8]) -> Cow<'_, [u8]> {
        zstd::decode_all(data).unwrap().into()
    }
}

/// Do zstd compression on embedded files.
#[cfg(feature = "compression-lz4")]
pub enum Lz4 {}

#[cfg(feature = "compression-lz4")]
impl Compression for Lz4 {
    fn decompress(data: &[u8]) -> Cow<'_, [u8]> {
        lz4_compression::decompress::decompress(data)
            .unwrap()
            .into()
    }
}
