use std::borrow::Cow;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::str;
use std::sync::mpsc::Sender;

use ahash::AHasher;
use anyhow::{Error, Result};
use async_trait::async_trait;
use bevy_app::{Events, EventWriter};
use bevy_ecs::system::ResMut;
use bevy_reflect::{TypeUuid, TypeUuidDynamic};
use downcast_rs::{Downcast, impl_downcast};

use crate::handle::{Handle, HandleId};
use crate::loader::{LoadContext, LoadedAsset};

pub trait AssetDynamic: Downcast + TypeUuidDynamic + Send + Sync + 'static {}
impl_downcast!(AssetDynamic);

pub trait Asset: TypeUuid + AssetDynamic {}

impl<T> Asset for T where T: TypeUuid + AssetDynamic + TypeUuidDynamic {}

impl<T> AssetDynamic for T where T: Send + Sync + 'static + TypeUuidDynamic {}

pub struct AssetPath<'a> {
    path: Cow<'a, Path>,
    label: Option<Cow<'a, str>>,
}

impl<'a> AssetPath<'a> {
    pub fn new_ref(path: &'a Path, label: Option<&'a str>) -> AssetPath<'a> {
        AssetPath {
            path: Cow::Borrowed(path),
            label: label.map(|val| Cow::Borrowed(val)),
        }
    }

    pub fn new(path: PathBuf, label: Option<String>) -> AssetPath<'a> {
        AssetPath {
            path: Cow::Owned(path),
            label: label.map(|val| Cow::Owned(val)),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|label| label.as_ref())
    }

    pub fn get_id(&self) -> AssetPathId {
        AssetPathId::from(self)
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

fn get_hasher() -> AHasher {
    AHasher::new_with_keys(42, 23)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourcePathId(u64);

impl<'a> From<&'a Path> for SourcePathId {
    fn from(value: &'a Path) -> Self {
        let mut hasher = get_hasher();
        value.hash(&mut hasher);
        SourcePathId(hasher.finish())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct LabelId(u64);

impl<'a> From<Option<&'a str>> for LabelId {
    fn from(value: Option<&'a str>) -> Self {
        let mut hashse = get_hasher();
        value.hash(&mut hashse);
        LabelId(hashse.finish())
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct AssetPathId(SourcePathId, LabelId);

impl AssetPathId {
    pub fn get_source_id(&self) -> SourcePathId {
        self.0
    }

    pub fn get_label_id(&self) -> LabelId {
        self.1
    }
}


impl<'a, 'b> From<&'a AssetPath<'b>> for AssetPathId {
    fn from(value: &'a AssetPath<'b>) -> Self {
        AssetPathId(
            SourcePathId::from(value.path()),
            LabelId::from(value.label()),
        )
    }
}



#[derive(Debug, TypeUuid)]
#[uuid = "7435ff9a-3b73-4928-bee6-c47e25d3f942"]
pub struct TextAsset(String);

#[async_trait]
pub trait AssetLoader: Send + Sync + 'static {
    async fn load<'a>(&self, bytes: &[u8], load_context: &'a mut LoadContext) -> Result<(), anyhow::Error>;
}

pub struct TextAssetLoader {}


#[derive(Copy, Eq, PartialEq, Clone, Debug)]
pub struct TestUtf8Error {
    pub(super) valid_up_to: usize,
    pub(super) error_len: Option<u8>,
}

#[async_trait]
impl AssetLoader for TextAssetLoader {
    async fn load<'a>(&self, bytes: &[u8], load_context: &'a mut LoadContext) -> Result<(), anyhow::Error> {
        let strs = str::from_utf8(bytes)?;
        println!("{}", strs);
        load_context.set_default_asset(LoadedAsset::new(TextAsset(strs.to_string())));
        Ok(())
    }
}

