use bevy_app::{App, AppBuilder, Plugin};
use bevy_ecs::{
    system::Commands,
    prelude::*,
};

#[derive(Default)]
pub struct GamePlugin;

fn setup(mut commands: Commands) {}

fn update() {
    println!("update!")
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system()).add_system(update.system());
    }
}