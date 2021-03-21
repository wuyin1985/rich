use async_trait::async_trait;
use std::str;
use anyhow::{Result, Error};
use std::borrow::Cow;
use std::path::Path;
use ahash::AHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use base::*;
use crate::handle::{HandleId, RefChange};

pub trait Asset: Send + Sync + 'static {
    fn get_type_id(&self);
}

pub struct AssetPath<'a> {
    path: Cow<'a, Path>,
    label: Option<Cow<'a, str>>,
}

impl<'a> AssetPath<'a> {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn to_owned(&self) -> AssetPath<'static> {
        AssetPath {
            path: Cow::Owned(self.path.to_path_buf()),
            label: self
                .label
                .as_ref()
                .map(|value| Cow::Owned(value.to_string())),
        }
    }
}

impl<'a> From<&'a str> for AssetPath<'a> {
    fn from(p: &'a str) -> Self {
        let mut parts = p.split('#');
        let path = Path::new(parts.next().expect("Path must be set."));
        let label = parts.next();
        AssetPath {
            path: Cow::Borrowed(path),
            label: label.map(|label| Cow::Borrowed(label)),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct AssetPathId(u64);

impl<'a> From<&'a Path> for AssetPathId {
    fn from(value: &'a Path) -> Self {
        let mut hash = AHasher::new_with_keys(42, 23);
        value.hash(&mut hash);
        AssetPathId(hash.finish())
    }
}

#[derive(Debug)]
pub struct Assets<T: Asset> {
    assets: HashMap<HandleId, T>,
    pub ref_change_sender: Sender<RefChange>,
}

#[derive(Debug, TypeUuid)]
#[uuid = "7233c597-ccfa-411f-bd59-9af349432ada"]
pub struct TextAsset {}

impl Asset for TextAsset {
    fn get_type_id(&self) {}
}

#[async_trait]
pub trait AssetLoader: Send + Sync + 'static {
    async fn load(&self, bytes: &[u8]) -> Result<(), anyhow::Error>;
}

pub struct TextAssetLoader {}


#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub struct TestUtf8Error {
    pub(super) valid_up_to: usize,
    pub(super) error_len: Option<u8>,
}

#[async_trait]
impl AssetLoader for TextAssetLoader {
    async fn load(&self, bytes: &[u8]) -> Result<(), anyhow::Error> {
        let strs = str::from_utf8(bytes)?;
        println!("{}", strs);
        Ok(())
    }
}

