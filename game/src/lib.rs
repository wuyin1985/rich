mod map;
mod proto;
mod game;
mod camera;
mod stage;
mod attacker;
mod monster;
mod table;

pub mod prelude {
    pub use bevy::prelude::*;
}

pub use game::GamePlugin;

use rand::Rng;
use crate::prelude::Vec3;

pub fn rand_position(pos: &Vec3, range: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = pos.x + rng.gen_range(0f32..range);
    let y = pos.y + rng.gen_range(0f32..range);
    let z = pos.z + rng.gen_range(0f32..range);
    Vec3::new(x, y, z)
}