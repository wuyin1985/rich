use bevy_ecs::schedule::{SystemStage, StageLabel};
use crate::handle::HandleId;
use bevy_app::{AppBuilder, Plugin};
use crate::asset_server::AssetServer;

#[cfg(test)]
mod tests {
    use crate::asset_server::AssetServer;
    use crate::asset::{TextAsset, TextAssetLoader, AssetPath};
    use bevy_app::{App, ScheduleRunnerSettings, ScheduleRunnerPlugin, EventWriter, AppExit};
    use crate::AssetPlugin;
    use crate::assets::{AddAsset, Assets};
    use std::{thread, time, env};
    use bevy_ecs::prelude::*;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::Duration;
    use crate::handle::{HandleUntyped, HandleId};
    use crate::handle::HandleId::AssetPathId;

    fn setup(mut server: ResMut<AssetServer>) {
        server.load_untyped("src/lib.rs");
    }

    fn run(texts: Res<Assets<TextAsset>>, mut app_exit_events: EventWriter<AppExit>) {
        let p = AssetPath::new("src/lib.rs".into(), None);
        let handle: HandleId = p.get_id().into();
        if let Some(txt) = texts.get(handle) {
            println!("{}", txt.0);
            app_exit_events.send(AppExit);
            return;
        }
        println!("waiting");
    }

    #[test]
    fn test_load_text() {
        App::build().insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))).add_plugin(ScheduleRunnerPlugin::default()).add_plugin(AssetPlugin).
            add_asset::<TextAsset>().add_asset_loader(&["txt", "rs"], TextAssetLoader {})
            .add_startup_system(setup.system()).add_system(run.system()).run();
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
            );
        //.register_type::<HandleId>();
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