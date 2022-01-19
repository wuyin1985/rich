use bevy::app::Plugin;
use bevy::prelude::*;
use std::ops::Deref;
use crate::{monster, stage};
use crate::attacker_config::AttackerConfig;
use crate::camera::LookTransformPlugin;
use crate::map::{MapConfigAsset, MapConfigAssetLoader};
use crate::monster::MonsterConfig;
use crate::prelude::App;
use crate::table_data::{TableData, TableDataItem};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Result,
}

pub struct GamePlugin {}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
            {
                hashtoollib::load_reverse_dict("assets/config/hash.json");
            }

        app.insert_resource(
            WindowDescriptor {
                title: "Rich".to_string(),
                width: 960.,
                height: 540.,
                vsync: true,
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .add_plugin(LookTransformPlugin)

            .add_state(GameState::Loading)
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(start_load))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_load_finish))

            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(stage::init_stage_system))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(stage::update_stage_system))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(monster::move_by_map_path_system))

            .add_system(bevy::input::system::exit_on_esc_system)
            .init_asset_loader::<MapConfigAssetLoader>()
            .add_asset::<MapConfigAsset>();

        load_table::<AttackerConfig>(app, "assets/config/ron/attacker.ron");
        load_table::<MonsterConfig>(app, "assets/config/ron/monster.ron");
    }
}

fn load_table<T>(app: &mut App, path: &str) where T: TableDataItem {
    app.insert_resource(TableData::<T>::load_from_file(path));
}

fn start_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<MapConfigAsset> = asset_server.load("config/map/Map_pb.map");
    commands.insert_resource(handle);

    commands.spawn_bundle((
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0f32, 180.0f32.to_radians(), 0.0f32)),
        GlobalTransform::identity()
    )).with_children(|parent| {
        parent.spawn_scene(asset_server.load("gltf/Map_export.glb#Scene0"));
    });
}

fn check_load_finish(mut commands: Commands, map: Res<Assets<MapConfigAsset>>,
                     mut state: ResMut<State<GameState>>,
                     map_handle: Res<Handle<MapConfigAsset>>,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<StandardMaterial>>) {
    if let Some(asset) = map.get(map_handle.deref()) {
        let config = &asset.config;

        //camera
        {
            let c = config.camera.as_ref().unwrap();
            let pos = c.position.as_ref().unwrap();

            let rot = c.rotation.as_ref().unwrap();
            let transform = Transform::from_matrix(
                Mat4::from_scale_rotation_translation(Vec3::ONE,
                                                      Quat::from_xyzw(rot.x, rot.y, rot.z, rot.w),
                                                      Vec3::new(pos.x, pos.y, pos.z)));

            commands.spawn_bundle(PerspectiveCameraBundle {
                transform,
                ..Default::default()
            });

            //let look_at = c.look_at.as_ref().unwrap();
            // commands.spawn_bundle(OrbitCameraBundle::new(
            //     OrbitCameraController::default(),
            //     PerspectiveCameraBundle::default(),
            //     Vec3::new(pos.x, pos.y, pos.z),
            //     Vec3::new(look_at.x, look_at.y, look_at.z),
            // ));
        }

        //light
        {
            const HALF_SIZE: f32 = 1.0;
            let light_config = config.light.as_ref().unwrap();
            let color = light_config.color.as_ref().unwrap();
            let pos = light_config.position.as_ref().unwrap();
            // let look_at = light_config.look_at.as_ref().unwrap();
            // let transform = Transform::from_xyz(pos.x, pos.y, pos.z).looking_at(Vec3::new(look_at.x, look_at.y, look_at.z), Vec3::Y);
            let rot = light_config.rotation.as_ref().unwrap();
            let transform = Transform::from_matrix(
                Mat4::from_scale_rotation_translation(Vec3::ONE,
                                                      Quat::from_xyzw(rot.x, rot.y, rot.z, rot.w),
                                                      Vec3::new(pos.x, pos.y, pos.z)));

            commands.spawn_bundle(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    shadow_projection: OrthographicProjection {
                        left: -HALF_SIZE,
                        right: HALF_SIZE,
                        bottom: -HALF_SIZE,
                        top: HALF_SIZE,
                        near: -10.0 * HALF_SIZE,
                        far: 10.0 * HALF_SIZE,
                        ..Default::default()
                    },
                    shadow_depth_bias: light_config.shadow_bias,
                    shadow_normal_bias: light_config.shadow_normal_bias,
                    shadows_enabled: true,
                    color: Color::rgb(color.x, color.y, color.z),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            });
        }

        //path point
        {
            let ps = &config.paths;

            let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
            let material = materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..Default::default()
            });

            for path in ps {
                for point in &path.points {
                    let pos = point.position.as_ref().unwrap();

                    commands.spawn_bundle(PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform::from_xyz(pos.x, pos.y + 0.5f32, pos.z),
                        ..Default::default()
                    });
                }
            }
        }

        state.set(GameState::Playing).expect(format!("failed to switch game state to {:?}", GameState::Playing).as_str());
    }
}


