use std::ops::Deref;
use std::sync::Mutex;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::tasks::ComputeTaskPool;
use serde::{Serialize, Deserialize};
use crate::attrs::AttrsConfig;
use crate::stage::MapStage;
use crate::table::TableDataItem;

#[derive(Deserialize, Serialize, TypeUuid)]
#[uuid = "9a852db2-3eb7-4c91-99ae-ec1ea92f2877"]
pub struct MonsterConfig {
    pub name: String,
    pub move_speed: f32,
    pub asset: String,
    pub attrs: AttrsConfig,
}

impl TableDataItem for MonsterConfig {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn parse(&mut self) {
        self.attrs.parse();
    }
}

#[derive(Component, Debug)]
pub struct MoveWithMapPath {
    pub road_index: usize,
    pub target_point_index: usize,
    pub speed: f32,
}

#[derive(Component)]
pub struct MoveWithPathEnded {}

pub fn move_by_map_path_system(commands: Commands,
                               pool: Res<ComputeTaskPool>,
                               mut query: Query<(Entity, &mut MoveWithMapPath, &mut Transform), Without<MoveWithPathEnded>>,
                               stage: Res<MapStage>,
                               time: Res<Time>) {
    let delta = time.delta_seconds();
    let stage = stage.deref();
    let cs = Mutex::new(commands);

    query.par_for_each_mut(&pool, 64, |(entity, mut move_with, mut transform)| {
        let road = &stage.roads[move_with.road_index];
        if move_with.target_point_index > road.points.len() - 1 {
            let mut g = cs.lock().unwrap();
            g.entity(entity).insert(MoveWithPathEnded {});
            return;
        }

        let next_point = &road.points[move_with.target_point_index];
        let dir = (next_point.pos - transform.translation).normalize();
        let move_dis = move_with.speed * delta;
        transform.translation += dir * move_dis;
        let range = move_dis;
        //if reach
        if Vec3::distance_squared(transform.translation, next_point.pos) <= (range * range) {
            move_with.target_point_index += 1;
        }
    });
}