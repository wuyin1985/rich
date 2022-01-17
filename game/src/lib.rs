mod map;
mod proto;
mod game;
mod camera;
mod stage;
mod ability;
mod attacker_config;
mod monster;
mod table_data;

pub mod prelude {
    pub use bevy::prelude::*;
}

pub use game::GamePlugin;


