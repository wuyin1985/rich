use std::collections::HashMap;

use bevy_app::{Events, EventWriter, AppBuilder};

use crate::asset_server::AssetServer;
use crate::asset::{Asset, AssetLoader, AssetDynamic};
use crate::handle::{Handle, HandleId};

use bevy_ecs::{
    system::{IntoSystem, ResMut, Res},
    world::FromWorld,
};
use crate::AssetStage;

#[derive(Debug)]
pub enum AssetEvent<T: Asset> {
    Created { handle: Handle<T> },
    Modified { handle: Handle<T> },
    Removed { handle: Handle<T> },
}

#[derive(Debug)]
pub struct Assets<T: Asset> {
    assets: HashMap<HandleId, T>,
    events: Events<AssetEvent<T>>,
}

impl<T: Asset> Assets<T> {
    pub(crate) fn new() -> Self {
        Assets {
            assets: Default::default(),
            events: Default::default(),
        }
    }

    pub fn set<H: Into<HandleId>>(&mut self, handle: H, asset: T) -> Handle<T> {
        let id: HandleId = handle.into();
        self.set_untracked(id, asset);
        self.get_handle(id)
    }

    pub fn set_untracked<H: Into<HandleId>>(&mut self, handle: H, asset: T) {
        let id: HandleId = handle.into();
        if self.assets.insert(id, asset).is_some() {
            self.events.send(AssetEvent::Modified {
                handle: Handle::new(id),
            });
        } else {
            self.events.send(AssetEvent::Created {
                handle: Handle::new(id),
            });
        }
    }

    pub fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&T> {
        self.assets.get(&handle.into())
    }

    pub fn contains<H: Into<HandleId>>(&self, handle: H) -> bool {
        self.assets.contains_key(&handle.into())
    }

    pub fn get_mut<H: Into<HandleId>>(&mut self, handle: H) -> Option<&mut T> {
        let id: HandleId = handle.into();
        self.events.send(AssetEvent::Modified {
            handle: Handle::new(id),
        });
        self.assets.get_mut(&id)
    }

    pub fn get_handle<H: Into<HandleId>>(&self, handle: H) -> Handle<T> {
        Handle::new(handle.into())
    }

    pub fn get_or_insert_with<H: Into<HandleId>>(
        &mut self,
        handle: H,
        insert_fn: impl FnOnce() -> T,
    ) -> &mut T {
        let mut event = None;
        let id: HandleId = handle.into();
        let borrowed = self.assets.entry(id).or_insert_with(|| {
            event = Some(AssetEvent::Created {
                handle: Handle::new(id),
            });
            insert_fn()
        });

        if let Some(event) = event {
            self.events.send(event);
        }
        borrowed
    }

    pub fn iter(&self) -> impl Iterator<Item=(HandleId, &T)> {
        self.assets.iter().map(|(k, v)| (*k, v))
    }

    pub fn ids(&self) -> impl Iterator<Item=HandleId> + '_ {
        self.assets.keys().cloned()
    }

    pub fn remove<H: Into<HandleId>>(&mut self, handle: H) -> Option<T> {
        let id: HandleId = handle.into();
        let asset = self.assets.remove(&id);
        if asset.is_some() {
            self.events.send(AssetEvent::Removed {
                handle: Handle::new(id),
            });
        }
        asset
    }

    pub fn clear(&mut self) {
        self.assets.clear()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.assets.reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.assets.shrink_to_fit()
    }

    pub fn asset_event_system(
        mut events: EventWriter<AssetEvent<T>>,
        mut assets: ResMut<Assets<T>>,
    ) {
        events.send_batch(assets.events.drain())
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }
}


pub trait AddAsset {
    fn add_asset<T>(&mut self) -> &mut Self
        where
            T: Asset;

    fn add_asset_loader<T>(&mut self, file_ext: &[&str], loader: T) -> &mut Self
        where
            T: AssetLoader;
}


pub fn update_asset_storage_system<T: Asset + AssetDynamic>(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<T>>,
) {
    asset_server.update_asset_storage(&mut assets);
}


impl AddAsset for AppBuilder {
    fn add_asset<T>(&mut self) -> &mut Self
        where
            T: Asset,
    {
        let assets = {
            let asset_server = self.world().get_resource::<AssetServer>().unwrap();
            asset_server.register_asset_type::<T>()
        };

        self.insert_resource(assets)
            .add_system_to_stage(
                AssetStage::AssetEvents,
                Assets::<T>::asset_event_system.system(),
            )
            .add_system_to_stage(
                AssetStage::LoadAssets,
                update_asset_storage_system::<T>.system(),
            )
            //.register_type::<Handle<T>>()
            .add_event::<AssetEvent<T>>()
    }

    fn add_asset_loader<T>(&mut self, file_ext: &[&str], loader: T) -> &mut Self
        where
            T: AssetLoader,
    {
        self.world_mut()
            .get_resource_mut::<AssetServer>()
            .expect("AssetServer does not exist. Consider adding it as a resource.")
            .register_asset_loader(file_ext, loader);
        self
    }
}