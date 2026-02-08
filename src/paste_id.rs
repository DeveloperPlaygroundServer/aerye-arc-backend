use rand::prelude::*;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::fmt;

#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> PasteId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::rng();
        for _ in 0..size {
            id.push(BASE62[rng.random_range(0..63)] as char);
        }

        PasteId(Cow::Owned(id))
    }

    /// Returns the path to the paste in `upload/` corresponding to this ID.
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }
}

impl fmt::Display for PasteId<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}