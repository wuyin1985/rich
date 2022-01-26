use bevy::prelude::*;
use std::ops::{Deref, DerefMut};
use itertools::Itertools;
use crate::attrs::Attrs;
use crate::hit_query::HitBounds;

use crate::map::MapConfigAsset;
use crate::monster::{MonsterConfig, MoveWithMapPath};
use crate::proto::PathEditor::{MapConfig, PathWayPointData};
use crate::rand_position;
use crate::table::TableData;

pub struct MapStage {
    pub roads: Vec<MapStageRoad>,
    path_2_road: Vec<StagePath2RoadMap>,
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
    spawn_road_idx: usize,
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
    pub fn create(config: &MapConfig) -> Self {
        fn get_dir(points: &Vec<PathWayPointData>, idx_a: usize, idx_b: usize, last_dir: Option<Vec3>) ->
        (Vec3, (Vec3, f32), (Vec3, f32)) {
            let pa = &points[idx_a];
            let pb = &points[idx_b];
            let a = path_config_util::to_vec3(pa.position.as_ref().unwrap());
            let b = path_config_util::to_vec3(pb.position.as_ref().unwrap());
            let horizon = (b.x - a.x).abs() > (b.z - a.z).abs();
            let mut dir = match horizon {
                true => {
                    Vec3::new(0f32, 0f32, 1f32)
                }
                false => {
                    Vec3::new(1f32, 0f32, 0f32)
                }
            };

            if let Some(ld) = last_dir {
                if ld.dot(dir) < 0f32 {
                    dir = -dir;
                }
            }

            (dir, (a, pa.reach_range), (b, pb.reach_range))
        }

        fn get_line_start_stop(pos: &Vec3, dir: &Vec3, range: &f32) -> (Vec3, Vec3) {
            let add = (*dir) * (*range);
            (*pos - add / 2f32, *pos + add / 2f32)
        }

        fn line_space(start: Vec3, stop: Vec3, nstep: u32) -> Vec<Vec3>
        {
            assert!(nstep > 1, "nstep must big than 1");
            let delta = (stop - start) / ((nstep - 1) as f32);
            return (0..(nstep))
                .map(|i| start + (i as f32) * delta)
                .collect();
        }

        let mut path_2_road_map = Vec::new();
        let mut all_roads = Vec::new();

        config.paths.iter().for_each(|path| {
            let mut spawn_lines = Vec::new();
            let point_count = path.points.len();
            if point_count > 1 {
                let (line_dir, (a, a_len), _) = get_dir(&path.points, 0, 1, None);
                spawn_lines.push((a, line_dir, a_len))
            }

            let mut last_dir = None;

            let extends = path.points.windows(3).map(|t| {
                let ((a, _), (b, b_len), (c, _)) = t.iter().map(|pd| {
                    let mut p = path_config_util::to_vec3(&pd.position.as_ref().unwrap());
                    p.y = 0f32;
                    (p, pd.reach_range)
                }).next_tuple().unwrap();

                let normal: Vec3 = (a - b).normalize() + (c - b).normalize();
                last_dir = Some(normal);
                (b, normal, b_len)
            });

            spawn_lines.extend(extends);

            if point_count > 1 {
                let (line_dir, _, (b, b_len)) = get_dir(&path.points, point_count - 2, point_count - 1, last_dir);
                spawn_lines.push((b, line_dir, b_len))
            }

            assert_eq!(point_count, spawn_lines.len(), "point count not equal");

            let max_range = path.points.iter().fold(0f32, |ranger_of_points, point| {
                point.reach_range.max(ranger_of_points)
            });

            const PER_PATH_RANGE_DELTA: f32 = 0.25f32;
            let road_count = (max_range / PER_PATH_RANGE_DELTA).ceil() as u32;

            let road_points = spawn_lines.iter().map(|(pos, dir, len)| {
                let (start, stop) = get_line_start_stop(pos, dir, len);
                line_space(start, stop, road_count)
            }).collect_vec();

            let roads = (0..road_count).map(|i| {
                let points_row = road_points.iter().map(|col_of_points| {
                    MapStageRoadPoint { pos: col_of_points[i as usize] }
                }).collect::<Vec<_>>();
                MapStageRoad { points: points_row }
            }).collect_vec();

            path_2_road_map.push(StagePath2RoadMap { start_idx: all_roads.len(), count: roads.len() });
            all_roads.extend(roads);
        });

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
            roads: all_roads,
            path_2_road: path_2_road_map,
            queues,
            waiting_queues,
            working_queues: Default::default(),
        }
    }
}

struct StagePath2RoadMap {
    pub start_idx: usize,
    pub count: usize,
}

pub struct MapStageRoad {
    pub points: Vec<MapStageRoadPoint>,
}

pub struct MapStageRoadPoint {
    pub pos: Vec3,
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


#[cfg(feature = "debug")]
pub fn draw_stage_roads(map_stage: Res<MapStage>, mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>) {
    let map_stage = map_stage.deref();
    for road in &map_stage.roads {
        for t in road.points.windows(2) {
            lines.line_colored(t[0].pos, t[1].pos, 0.0f32, Color::GREEN);
        }
    }
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
                working.working_waves.push(MapStageWorkingWave {
                    spawn_road_idx: 0,
                    wave_idx: working.waiting_wave_idx,
                    work_time: 0f32,
                    spawn_cool_down: 0f32,
                });
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
                let path_2_road = &map_stage.path_2_road[wave_config.path_index as usize];
                let road_idx = path_2_road.start_idx + wave.spawn_road_idx;
                wave.spawn_road_idx = (wave.spawn_road_idx + 1) % path_2_road.count;

                let road = &map_stage.roads[road_idx];
                let first_point = rand_position(&road.points[0].pos, 0.25f32);
                let monster_config = monster_table.index(wave_config.unit);

                let gltf: Handle<Scene> = asset_server.load(&monster_config.asset);
                for _ in 0..wave_config.per_spawn_count {
                    let pos = rand_position(&first_point, 3f32);
                    commands.spawn_bundle(
                        (
                            Transform::from_translation(pos),
                            GlobalTransform::identity(),
                            MoveWithMapPath {
                                road_index: road_idx as usize,
                                target_point_index: 1,
                                speed: monster_config
                                    .move_speed,
                            },
                            HitBounds::create(0.2f32),
                            Attrs::load_from_config(&monster_config.attrs),
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