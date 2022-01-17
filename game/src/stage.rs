use bevy::prelude::*;
use crate::proto::PathEditor::MapConfig;

pub struct Stage {
    config: MapConfig,
    paths: Vec<StagePath>,
    queues: Vec<StageWaveQueue>,
    past_time: f32,
}

mod path_config_util {
    use bevy::math::vec3;
    use crate::prelude::Vec3;
    use crate::prelude::Vec4;
    use crate::proto::PathEditor::{MapVector3, MapVector4};

    pub fn to_vec3(v: &MapVector3) -> Vec3 {
        vec3(v.x, v.y, v.z)
    }

    pub fn to_vec4(v: &MapVector4) -> Vec4 {
        Vec4::new(v.x, v.y, v.z, v.w)
    }
}

impl Stage {
    pub fn create(config: MapConfig) -> Self {
        let paths = config.paths.iter().map(|path| {
            let points = path.points.iter().map(|p| {
                StagePathPoint {
                    pos: path_config_util::to_vec3(p.position.as_ref().unwrap()),
                    range: p.reach_range,
                }
            }).collect::<Vec<_>>();

            StagePath {
                points
            }
        }).collect::<Vec<_>>();

        let queues = config.wave_queues.iter().map(|wq| {
            let waves = wq.waves.iter().map(|ws| {
                StageWave {
                    wait_time: ws.wait_time,
                    per_spawn_unit_count: ws.per_spawn_count as _,
                    spawn_cool_down: ws.spawn_cool_down,
                    duration: ws.duration,
                    uint_name: ws.unit,
                    path_index: ws.path_index as _,
                }
            }).collect::<Vec<_>>();

            StageWaveQueue {
                waves,
                wait_time: wq.wait_time,
            }
        }).collect::<Vec<_>>();

        Stage {
            past_time: 0f32,
            config,
            paths,
            queues,
        }
    }
}

pub struct StagePathPoint {
    pos: Vec3,
    range: f32,
}

pub struct StagePath {
    points: Vec<StagePathPoint>,
}

pub struct StageWave {
    wait_time: f32,
    uint_name: u64,
    per_spawn_unit_count: u32,
    spawn_cool_down: f32,
    duration: f32,
    path_index: usize,
}

pub struct StageWaveQueue {
    waves: Vec<StageWave>,
    wait_time: f32,
}

pub fn init_stage(mut commands: Commands, config: MapConfig) {
    commands.insert_resource(Stage::create(config));
}

pub fn stage_system(mut commands: Commands,
                    mut stage: ResMut<Stage>,
                    time: Res<Time>) {
    stage.past_time += time.delta().as_secs_f32();
}