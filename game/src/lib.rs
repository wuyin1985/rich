extern crate core;

mod map;
mod proto;
mod game;
mod camera;
mod stage;
mod attacker;
mod monster;
mod table;
mod node;
mod force;
mod sfx;

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

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum StringId {
    Str(String),
    Id(u64),
}

impl StringId {
    pub fn change_2_id(&mut self) {
        *self = match self {
            StringId::Str(s) => {
                let id = hashtoollib::hash(s.as_str());
                Self::Id(id)
            }
            StringId::Id(id) => {
                Self::Id(*id)
            }
        }
    }

    pub fn id(&self) -> u64 {
        match self {
            StringId::Str(s) => {
                panic!("the string {} id not convert into id yet", s.as_str());
            }
            StringId::Id(id) => {
                *id
            }
        }
    }
}