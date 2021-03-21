use std::collections::HashMap;
use thiserror::Error;
use async_std::sync::Arc;
use std::path::{Path, PathBuf};
use std::{io, env};
use futures::AsyncReadExt;
use super::asset::{AssetLoader, AssetPath, Asset, AssetPathId, Assets};
use ahash::AHasher;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::Entry;
use async_std::task;
use std::borrow::Cow;
use std::sync::RwLock;

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
    asset: Option<Box<dyn Asset>>,
    state: AssetLifeState,
}

struct AssetServerInternal {
    pub asset_life_collection: RwLock<HashMap<AssetPathId, AssetLifeTime>>,
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
                asset_life_collection: Default::default(),
                loader_by_index: Default::default(),
                root_path: Box::new(dir),
            })
        }
    }

    pub fn load<'a, P: Into<AssetPath<'a>> + Send>(&mut self, p: P) {
        let pp: AssetPath<'a> = p.into();
        let path = pp.path();
        let path_id: AssetPathId = path.into();

        let mut collection = &mut self.server.asset_life_collection.write().unwrap();
        let life = match collection.entry(path_id) {
            Entry::Occupied(entry) => {
                return;
            }
            Entry::Vacant(entry) => entry.insert(AssetLifeTime {
                asset: None,
                state: AssetLifeState::Loading,
            })
        };

        let clone = self.clone();
        let np = pp.to_owned();

        let handle = task::spawn(
            async move {
                match clone.async_load(np).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("error: {:?}", e)
                    }
                }
            }
        );
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

    pub async fn async_load(&self, p: AssetPath<'static>) -> Result<(), AssetServerError> {
        let path = p.path();
        let loader = self.get_loader_by_path(path)?;
        let mut bytes: Vec<u8> = Vec::new();

        let full_path = self.server.root_path.join(path);
        let mut file = async_std::fs::File::open(full_path).await?;
        file.read_to_end(&mut bytes).await?;
        loader.load(&bytes).await.map_err(AssetServerError::AssetLoaderError)
    }

    // pub fn register_asset_type<T: Asset>(&self) -> Assets<T> {
    //     self.server.asset_lifecycles.write().insert(
    //         T::TYPE_UUID,
    //         Box::new(AssetLifecycleChannel::<T>::default()),
    //     );
    //     Assets::new(self.server.asset_ref_counter.channel.sender.clone())
    // }
}
