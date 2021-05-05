use crate::asset::{Asset, AssetPath, AssetDynamic, AssetPathId};
use std::collections::HashMap;
use crate::handle::{Handle, HandleId};
use std::path::Path;
use downcast_rs::{Downcast, impl_downcast};
use crossbeam_channel::{Receiver, Sender};

pub struct LoadedAsset<T: Asset> {
    pub(crate) value: Option<T>,
    pub(crate) dependencies: Vec<AssetPath<'static>>,
}

impl<T: Asset> LoadedAsset<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            dependencies: Vec::new(),
        }
    }

    pub fn with_dependency(mut self, asset_path: AssetPath) -> Self {
        self.dependencies.push(asset_path.to_owned());
        self
    }

    pub fn with_dependencies(mut self, asset_path: Vec<AssetPath<'static>>) -> Self {
        self.dependencies.extend(asset_path);
        self
    }
}

pub(crate) struct BoxedLoadedAsset {
    pub(crate) value: Option<Box<dyn AssetDynamic>>,
    pub(crate) dependencies: Vec<AssetPath<'static>>,
}

impl<T: Asset> From<LoadedAsset<T>> for BoxedLoadedAsset {
    fn from(asset: LoadedAsset<T>) -> Self {
        BoxedLoadedAsset {
            value: asset.value.map(|value| Box::new(value) as Box<dyn AssetDynamic>),
            dependencies: asset.dependencies,
        }
    }
}

pub struct LoadContext<'a> {
    pub(crate) label_assets: HashMap<Option<String>, BoxedLoadedAsset>,
    pub(crate) version: usize,
    pub(crate) path: &'a Path,
}

impl<'a> LoadContext<'a> {
    pub(crate) fn new(version: usize, path: &'a Path) -> Self {
        Self {
            label_assets: Default::default(),
            version,
            path,
        }
    }

    pub fn path(&self) -> &Path { &self.path }

    pub fn has_labeled_asset(&self, label: &str) -> bool {
        self.label_assets.contains_key(&Some(label.to_string()))
    }

    pub fn set_default_asset<T: Asset>(&mut self, asset: LoadedAsset<T>) -> Handle<T> {
        self.label_assets.insert(None, asset.into());
        Handle::new(AssetPath::new_ref(self.path(), None).into())
    }

    pub fn set_labeled_asset<T: Asset>(&mut self, label: &str, asset: LoadedAsset<T>) -> Handle<T> {
        self.label_assets.insert(Some(label.to_string()), asset.into());
        Handle::new(AssetPath::new_ref(self.path(), Some(label)).into())
    }
}


pub struct AssetLoadResult<T: AssetDynamic> {
    pub asset: Box<T>,
    pub id: HandleId,
    pub version: usize,
}

pub enum AssetLifecycleEvent<T: AssetDynamic> {
    Create(AssetLoadResult<T>),
    Free(HandleId),
}

#[derive(Debug)]
pub struct AssetLifecycleChannel<T: AssetDynamic> {
    pub sender: Sender<AssetLifecycleEvent<T>>,
    pub receiver: Receiver<AssetLifecycleEvent<T>>,
}

pub trait AssetLifecycle: Downcast + Send + Sync + 'static {
    fn create_asset(&self, id: HandleId, asset: Box<dyn AssetDynamic>, version: usize);
    fn free_asset(&self, id: HandleId);
}
impl_downcast!(AssetLifecycle);

impl<T: AssetDynamic> AssetLifecycle for AssetLifecycleChannel<T> {
    fn create_asset(&self, id: HandleId, asset: Box<dyn AssetDynamic>, version: usize) {
        if let Ok(asset) = asset.downcast::<T>() {
            self.sender.send(AssetLifecycleEvent::Create(AssetLoadResult {
                id,
                asset,
                version,
            })).unwrap();
        } else {
            panic!("Failed to downcast asset to {}", std::any::type_name::<T>())
        }
    }

    fn free_asset(&self, id: HandleId) {
        self.sender.send(AssetLifecycleEvent::Free(id)).unwrap();
    }
}

impl<T: AssetDynamic> Default for AssetLifecycleChannel<T> {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        AssetLifecycleChannel { sender, receiver }
    }
}

