use std::collections::HashMap;
use thiserror::Error;
use async_std::sync::Arc;
use std::path::{Path, PathBuf};
use std::{io, env};
use futures::AsyncReadExt;
use super::asset::{AssetLoader, AssetPath, Asset, AssetPathId};
use super::assets::{Assets, AssetEvent};
use ahash::AHasher;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::Entry;
use async_std::task;
use std::borrow::Cow;
use std::sync::RwLock;
use crate::handle::{HandleUntyped, Handle, HandleId};
use crate::asset::{AssetDynamic, SourcePathId};
use crate::source::{SourceInfo, SourceLoadState};
use crate::loader::{AssetLifecycle, LoadContext, AssetLifecycleChannel, AssetLifecycleEvent};
use bevy_reflect::Uuid;
use crossbeam_channel::TryRecvError;

#[derive(Error, Debug)]
pub enum AssetIoError {
    #[error("path not found")]
    NotFound(PathBuf),
    #[error("encountered an io error while loading asset")]
    Io(#[from] io::Error),
    #[error("failed to watch path")]
    PathWatchError(PathBuf),
}


#[derive(Error, Debug)]
pub enum AssetServerError {
    #[error("asset folder path is not a directory")]
    AssetFolderNotADirectory(String),
    #[error("no AssetLoader found for the given extension")]
    MissingAssetLoader(Option<String>),
    #[error("the given type does not match the type of the loaded asset")]
    IncorrectHandleType,
    #[error("encountered an error while loading an asset")]
    AssetLoaderError(anyhow::Error),
    // #[error("`PathLoader` encountered an error")]
    // PathLoaderError(#[from] AssetIoError),
    #[error("encountered an io error while loading asset")]
    Io(#[from] io::Error),
}

#[derive(PartialEq, Eq, Hash)]
struct TypeId {
    value: u64,
}

enum AssetLifeState
{
    None,
    Loading,
    Err,
    Complete,
}

struct AssetLifeTime {
    asset: Option<Box<dyn AssetDynamic>>,
    state: AssetLifeState,
}

struct AssetServerInternal {
    pub asset_sources: RwLock<HashMap<SourcePathId, SourceInfo>>,
    pub asset_lifecycles: RwLock<HashMap<Uuid, Box<dyn AssetLifecycle>>>,
    pub loader_by_index: RwLock<HashMap<String, usize>>,
    pub loader_list: RwLock<Vec<Arc<dyn AssetLoader>>>,
    pub root_path: Box<PathBuf>,
}

pub struct AssetServer {
    server: Arc<AssetServerInternal>,
}

impl Clone for AssetServer {
    fn clone(&self) -> Self {
        Self {
            server: self.server.clone(),
        }
    }
}

impl AssetServer {
    pub fn new() -> Self {
        let dir = env::current_dir().unwrap();
        AssetServer {
            server: Arc::new(AssetServerInternal {
                loader_list: Default::default(),
                asset_sources: Default::default(),
                asset_lifecycles: Default::default(),
                loader_by_index: Default::default(),
                root_path: Box::new(dir),
            })
        }
    }

    pub fn load<'a, T: Asset, P: Into<AssetPath<'a>> + Send>(&mut self, p: P) -> Handle<T> {
        self.load_untyped(p).typed()
    }

    pub fn load_untyped<'a, P: Into<AssetPath<'a>> + Send>(&mut self, p: P) -> HandleUntyped {
        let path: AssetPath<'a> = p.into();
        let path_id: AssetPathId = path.get_id();

        let version = {
            let mut sources = &mut self.server.asset_sources.write().unwrap();
            let source_info = match sources.entry(path_id.get_source_id()) {
                Entry::Occupied(entry) => {
                    entry.into_mut()
                }
                Entry::Vacant(entry) => entry.insert(SourceInfo {
                    committed_assets: Default::default(),
                    version: 0,
                })
            };

            if source_info.committed_assets.contains(&path_id.get_label_id()) {
                return HandleUntyped { id: path_id.into() };
            }

            source_info.version += 1;
            source_info.version
        };

        let clone = self.clone();
        let np = path.to_owned();

        task::spawn(
            async move {
                match clone.async_load(np, version).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("error: {:?}", e)
                    }
                }
            }
        );

        return HandleUntyped { id: path_id.into() };
    }

    fn get_loader_by_path<P: AsRef<Path>>(&self, path: P) -> Result<Arc<dyn AssetLoader>, AssetServerError> {
        path.as_ref().extension()
            .and_then(|e| e.to_str())
            .ok_or(AssetServerError::MissingAssetLoader(None))
            .and_then(|ex| self.get_loader_by_ext(ex))
    }

    fn get_loader_by_ext(&self, ext: &str) -> Result<Arc<dyn AssetLoader>, AssetServerError> {
        self.server.loader_by_index.read().unwrap().get(ext).map(|idx| self.server.loader_list.read().unwrap()[*idx].clone()).ok_or_else(|| AssetServerError::MissingAssetLoader(None))
    }

    pub fn register_asset_loader<T>(&mut self, file_ext: &[&str], loader: T) where T: 'static + AssetLoader {
        let mut loaders = self.server.loader_list.write().unwrap();
        loaders.push(Arc::new(loader));
        let idx = loaders.len() - 1;

        for ext in file_ext.iter() {
            self.server.loader_by_index.write().unwrap().insert(ext.to_string(), idx);
        }
    }

    pub fn register_asset<T: Asset>(&self) -> Assets<T> {
        self.server.asset_lifecycles.write().unwrap().insert(T::TYPE_UUID, Box::new(AssetLifecycleChannel::<T>::default()));
        Assets::new()
    }

    pub async fn async_load(&self, p: AssetPath<'static>, source_version: usize) -> Result<(), AssetServerError> {
        let path = p.path();
        let loader = self.get_loader_by_path(path)?;
        let mut bytes: Vec<u8> = Vec::new();

        let full_path = self.server.root_path.join(path);
        let mut file = async_std::fs::File::open(full_path).await?;
        file.read_to_end(&mut bytes).await?;

        let mut load_context = LoadContext::new(
            source_version,
            p.path(),
        );

        loader.load(&bytes, &mut load_context).await.map_err(AssetServerError::AssetLoaderError)?;

        let mut sources = self.server.asset_sources.write().unwrap();
        let source_info = sources.get_mut(&p.get_id().get_source_id()).expect("'SourceInfo' not exist");
        // 如果版本号不一样,说明以及发起了新的加载,本次加载结果需要忽略掉
        if source_version != source_info.version {
            return Ok(());
        }

        //清空加载结果记录
        source_info.committed_assets.clear();

        //todo load depends

        //dispatch asset to channel
        let asset_lifecycles = self.server.asset_lifecycles.read().unwrap();
        for (label, asset) in load_context.label_assets.iter_mut() {
            let asset_value = asset.value.take().expect("Asset should exist");
            if let Some(asset_lifecycle) = asset_lifecycles.get(&asset_value.type_uuid()) {
                let asset_path = AssetPath::new_ref(&load_context.path, label.as_deref());
                asset_lifecycle.create_asset(asset_path.into(), asset_value, load_context.version);
            } else {
                panic!(
                    "Failed to find AssetLifecycle of label '{:?}', which has an asset type {} (UUID {:?}).\
                        Are you sure this asset type has been added to your asset_server?",
                    label,
                    asset_value.type_name(),
                    asset_value.type_uuid(),
                )
            }
        }

        Ok(())
    }


    pub(crate) fn update_asset_storage<T: Asset>(&self, assets: &mut Assets<T>) {
        let asset_lifecycles = self.server.asset_lifecycles.read().unwrap();
        let asset_lifecycle = asset_lifecycles.get(&T::TYPE_UUID).unwrap();
        let mut asset_sources_guard = None;
        let channel = asset_lifecycle
            .downcast_ref::<AssetLifecycleChannel<T>>()
            .unwrap();

        loop {
            match channel.receiver.try_recv() {
                Ok(AssetLifecycleEvent::Create(result)) => {
                    // update SourceInfo if this asset was loaded from an AssetPath
                    if let HandleId::AssetPathId(id) = result.id {
                        let asset_sources = asset_sources_guard
                            .get_or_insert_with(|| self.server.asset_sources.write().unwrap());
                        if let Some(source_info) = asset_sources.get_mut(&id.get_source_id()) {
                            if source_info.version == result.version {
                                source_info.committed_assets.insert(id.get_label_id());
                            }
                        }
                    }

                    let _ = assets.set(result.id, *result.asset);
                }
                Ok(AssetLifecycleEvent::Free(handle_id)) => {
                    if let HandleId::AssetPathId(id) = handle_id {
                        let asset_sources = asset_sources_guard
                            .get_or_insert_with(|| self.server.asset_sources.write().unwrap());
                        if let Some(source_info) = asset_sources.get_mut(&id.get_source_id()) {
                            source_info.committed_assets.remove(&id.get_label_id());
                        }
                    }
                    assets.remove(handle_id);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => panic!("AssetChannel disconnected."),
            }
        }
    }

    pub fn register_asset_type<T: Asset>(&self) -> Assets<T> {
        self.server.asset_lifecycles.write().unwrap().insert(
            T::TYPE_UUID,
            Box::new(AssetLifecycleChannel::<T>::default()),
        );
        Assets::new()
    }
}
