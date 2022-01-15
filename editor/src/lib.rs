use game::prelude::*;
use bevy_inspector_egui;
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct EditorPlugin {}

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new());
    }
}

