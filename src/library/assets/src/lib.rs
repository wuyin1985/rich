use bevy_ecs::schedule::SystemStage;
use crate::handle::HandleId;
use bevy_app::{AppBuilder, Plugin};
use crate::asset_server::AssetServer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub mod asset;
pub mod asset_server;
pub mod handle;
mod source;
mod loader;
mod assets;


/// The names of asset stages in an App Schedule
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum AssetStage {
    LoadAssets,
    AssetEvents,
}

/// Adds support for Assets to an App. Assets are typed collections with change tracking, which are
/// added as App Resources. Examples of assets: textures, sounds, 3d models, maps, scenes
#[derive(Default)]
pub struct AssetPlugin;

pub struct AssetServerSettings {
    pub asset_folder: String,
}

impl Default for AssetServerSettings {
    fn default() -> Self {
        Self {
            asset_folder: "assets".to_string(),
        }
    }
}


impl Plugin for AssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        if app.world().get_resource::<AssetServer>().is_none() {
            let asset_server = AssetServer::new();
            app.insert_resource(asset_server);
        }

        app.add_stage_before(
            bevy_app::CoreStage::PreUpdate,
            AssetStage::LoadAssets,
            SystemStage::parallel(),
        )
            .add_stage_after(
                bevy_app::CoreStage::PostUpdate,
                AssetStage::AssetEvents,
                SystemStage::parallel(),
            )
            .register_type::<HandleId>();
            // .add_system_to_stage(
            //     bevy_app::CoreStage::PreUpdate,
            //     asset_server::free_unused_assets_system.system(),
            // );

        // #[cfg(all(
        // feature = "filesystem_watcher",
        // all(not(target_arch = "wasm32"), not(target_os = "android"))
        // ))]
        //     app.add_system_to_stage(
        //     AssetStage::LoadAssets,
        //     io::filesystem_watcher_system.system(),
        // );
    }
}