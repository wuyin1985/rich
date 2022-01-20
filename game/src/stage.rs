use bevy::prelude::*;
use std::ops::DerefMut;

use crate::map::MapConfigAsset;
use crate::monster::{MonsterConfig, MoveWithMapPath};
use crate::proto::PathEditor::{MapConfig, PathWayPointData};
use crate::rand_position;
use crate::table::TableData;

pub struct MapStage {
    pub paths: Vec<MapStagePath>,
    queues: Vec<StageWaveQueue>,
    waiting_queues: Vec<usize>,
    working_queues: Vec<MapStageWorkingQueue>,
    past_time: f32,
}

#[derive(Default)]
pub struct MapStageWorkingQueue {
    queue_idx: usize,
    waiting_wave_idx: usize,
    working_waves: Vec<MapStageWorkingWave>,
}

#[derive(Default)]
pub struct MapStageWorkingWave {
    work_time: f32,
    wave_idx: usize,
    spawn_cool_down: f32,
}

mod path_config_util {
    use bevy::math::vec3;

    use crate::prelude::Vec3;
    use crate::prelude::Vec4;
    use crate::proto::PathEditor::{MapVector3, MapVector4};

    pub fn to_vec3(v: &MapVector3) -> Vec3 {
        vec3(v.x, v.y, v.z)
    }

    #[allow(dead_code)]
    pub fn to_vec4(v: &MapVector4) -> Vec4 {
        Vec4::new(v.x, v.y, v.z, v.w)
    }
}

impl MapStage {
    fn line_space(start: Vec3, stop: Vec3, nstep: u32) -> Vec<Vec3>
    {
        let delta = (stop - start) / ((nstep - 1) as f32);
        return (0..(nstep))
            .map(|i| start + (i as f32) * delta)
            .collect();
    }

    fn get_start_stop(point: Vec3, range: f32) -> (Vec3, Vec3) {

    }

    pub fn create(config: &MapConfig) -> Self {
        config.paths.iter().enumerate().for_each(|(path_idx, path)| {
            let max_range = path.points.iter().fold(0f32, |ranger_of_points, point| {
                point.reach_range.max(ranger_of_points)
            });

            const PER_PATH_RANGE_DELTA: f32 = 0.25f32;
            let road_count = (max_range / PER_PATH_RANGE_DELTA).ceil() as u32;
            for t in path.points.windows(2) {
                let [a, b]: &[PathWayPointData; 2] = t.try_into().unwrap();
            }
        });

        for cp in &config.paths {
            for t in cp.points.windows(2) {}
        }
        let paths = config.paths.iter().map(|path| {
            let points = path.points.iter().map(|p| {
                MapStagePathPoint {
                    pos: path_config_util::to_vec3(p.position.as_ref().unwrap()),
                    range: p.reach_range,
                }
            }).collect::<Vec<_>>();

            MapStagePath {
                points
            }
        }).collect::<Vec<_>>();

        let mut waiting_queues = Vec::new();

        let queues = config.wave_queues.iter().enumerate().map(|(queue_idx, wq)| {
            let waves = wq.waves.iter().map(|ws| {
                MapStageWave {
                    wait_time: ws.wait_time,
                    per_spawn_unit_count: ws.per_spawn_count as _,
                    spawn_cool_down: ws.spawn_cool_down,
                    duration: ws.duration,
                    uint_name: ws.unit,
                    path_index: ws.path_index as _,
                }
            }).collect::<Vec<_>>();

            waiting_queues.push(queue_idx);

            StageWaveQueue {
                waves,
                wait_time: wq.wait_time,
            }
        }).collect::<Vec<_>>();

        MapStage {
            past_time: 0f32,
            paths,
            queues,
            waiting_queues,
            working_queues: Default::default(),
        }
    }
}

pub struct MapStagePathPoint {
    pub pos: Vec3,
    pub range: f32,
}

pub struct MapStagePath {
    pub points: Vec<MapStagePathPoint>,
}

#[allow(dead_code)]
pub struct MapStageWave {
    wait_time: f32,
    uint_name: u64,
    per_spawn_unit_count: u32,
    spawn_cool_down: f32,
    duration: f32,
    path_index: usize,
}

pub struct StageWaveQueue {
    waves: Vec<MapStageWave>,
    wait_time: f32,
}

pub fn init_stage_system(mut commands: Commands, res: Res<Assets<MapConfigAsset>>) {
    let (_, config) = res.iter().next().expect("no map config loaded");
    commands.insert_resource(MapStage::create(&config.config));
}

pub fn update_stage_system(mut commands: Commands,
                           mut map_stage: ResMut<MapStage>,
                           map_assets: Res<Assets<MapConfigAsset>>,
                           asset_server: Res<AssetServer>,
                           monster_table: Res<TableData<MonsterConfig>>,
                           time: Res<Time>) {
    let map_stage = map_stage.deref_mut();
    let delta = time.delta().as_secs_f32();
    map_stage.past_time += delta;

    let (_, config_asset) = map_assets.iter().next().expect("no map config loaded");
    let config = &config_asset.config;

    //check waiting
    map_stage.waiting_queues.retain(|queue_idx| {
        let past = map_stage.past_time >= map_stage.queues[*queue_idx].wait_time;
        if past {
            map_stage.working_queues.push(MapStageWorkingQueue { queue_idx: *queue_idx, waiting_wave_idx: 0, working_waves: Default::default() });
        }
        !past
    });

    //check working
    for i in (0..map_stage.working_queues.len()).rev() {
        let working = &mut map_stage.working_queues[i];
        let mut finish = false;
        let queue = &map_stage.queues[working.queue_idx];
        if working.waiting_wave_idx > queue.waves.len() - 1 {
            finish = working.working_waves.len() == 0;
        } else {
            let waiting_wave = &queue.waves[working.waiting_wave_idx];
            if map_stage.past_time >= waiting_wave.wait_time {
                //add wave
                working.working_waves.push(MapStageWorkingWave { wave_idx: working.waiting_wave_idx, work_time: 0f32, spawn_cool_down: 0f32 });
                working.waiting_wave_idx += 1;
            }
        }

        if finish {
            map_stage.working_queues.remove(i);
        }
    };

    //update wave
    for queue in &mut map_stage.working_queues {
        let mut ww = std::mem::take(&mut queue.working_waves);
        for i in (0..ww.len()).rev() {
            let wave = &mut ww[i];
            wave.work_time += delta;

            let wave_config = &config.wave_queues[queue.queue_idx].waves[wave.wave_idx];
            if wave.spawn_cool_down <= 0f32 {
                wave.spawn_cool_down = wave_config.spawn_cool_down;
                //spawn
                let path = &config.paths[wave_config.path_index as usize];
                if path.points.len() == 0 {
                    panic!("the path {} point is zero", wave_config.path_index);
                }
                let first_point = path_config_util::to_vec3(path.points[0].position.as_ref().unwrap());
                let monster_config = monster_table.index(wave_config.unit);

                let gltf: Handle<Scene> = asset_server.load(&monster_config.asset);
                for _ in 0..wave_config.per_spawn_count {
                    let pos = rand_position(&first_point, 3f32);
                    commands.spawn_bundle(
                        (
                            Transform::from_translation(pos),
                            GlobalTransform::identity(),
                            MoveWithMapPath { path_index: wave_config.path_index as usize, target_point_index: 1, speed: monster_config.move_speed },
                        )
                    ).with_children(|parent| {
                        parent.spawn_scene(gltf.clone());
                    });
                }
            } else {
                wave.spawn_cool_down -= delta;
            }

            if wave.work_time >= wave_config.duration {
                ww.remove(i);
            }
        }

        queue.working_waves = ww;
    }
}