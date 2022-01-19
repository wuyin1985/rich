use std::ops::Deref;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::tasks::ComputeTaskPool;
use serde::{Serialize, Deserialize};
use crate::stage::MapStage;
use crate::table_data::TableDataItem;

#[derive(Deserialize, Serialize, TypeUuid)]
#[uuid = "9a852db2-3eb7-4c91-99ae-ec1ea92f2877"]
pub struct MonsterConfig {
    pub name: String,
    pub move_speed: f32,
    pub asset: String,
}

impl TableDataItem for MonsterConfig {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Debug)]
pub struct MoveWithMapPath {
    pub path_index: usize,
    pub target_point_index: usize,
    pub speed: f32,
}

pub fn move_by_map_path_system(pool: Res<ComputeTaskPool>,
                               mut query: Query<(&mut MoveWithMapPath, &mut Transform)>,
                               stage: Res<MapStage>,
                               time: Res<Time>) {
    let delta = time.delta_seconds();
    let stage = stage.deref();
    query.par_for_each_mut(&pool, 64, |(mut move_with, mut transform)| {
        let path = &stage.paths[move_with.path_index];
        if move_with.target_point_index > path.points.len() - 1 {
            return;
        }

        let next_point = &path.points[move_with.target_point_index];
        let dir = (next_point.pos - transform.translation).normalize();
        let move_dis = move_with.speed * delta;
        transform.translation += dir * move_dis;
        let range = next_point.range.max(move_dis);
        //if reach
        if Vec3::distance_squared(transform.translation, next_point.pos) <= (range * range) {
            move_with.target_point_index += 1;
        }
    });
}