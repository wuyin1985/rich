use crate::prelude::*;
use bevy::reflect::TypeUuid;
use crate::StringId;
use crate::table::TableData;
use super::table::TableDataItem;

#[derive(serde::Deserialize, serde::Serialize, TypeUuid)]
#[uuid = "9a852db2-3eb7-4c91-99ae-ec1ea92f2877"]
pub struct AttackerConfig {
    pub abilities: Vec<AbilityConfig>,
    pub name: String,
    pub asset: String,
}

impl TableDataItem for AttackerConfig {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn parse(&mut self) {
        for ac in &mut self.abilities {
            ac.parse();
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum AbilityConfig {
    Shoot(ShootAbilityConfig),
    Channel(ChannelAbilityConfig),
}

impl AbilityConfig {
    pub fn parse(&mut self) {
        match self {
            AbilityConfig::Shoot(s) => {
                s.fire_node.change_2_id();
            }
            AbilityConfig::Channel(c) => {
                c.fire_node.change_2_id();
            }
        }
    }
}

pub struct Attacker {}

pub struct AttackerRef {
    target: Entity,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Component)]
pub struct ShootAbilityConfig {
    pub cd: f32,
    pub reload_time: f32,
    pub magazine: u32,
    pub fire_node: StringId,
}

#[derive(Component)]
pub struct ShootAbilityRuntime {
    pub fire_node: Option<Entity>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Component)]
pub struct ChannelAbilityConfig {
    pub total_value: f32,
    pub value_cost_speed: f32,
    pub fire_node: StringId,
}

#[derive(Component)]
pub struct ChannelAbilityRuntime {
    pub fire_node: Option<Entity>,
}

#[derive(Component)]
pub struct CreateAttackerReq {
    id: u64,
}

pub fn spawn_attacker_system(mut commands: Commands,
                             query: Query<(Entity, &CreateAttackerReq)>,
                             table: Res<TableData<AttackerConfig>>,
                             asset_server: Res<AssetServer>) {
    for (entity, req) in query.iter() {
        let config = table.index(req.id);
        commands.entity(entity).remove::<CreateAttackerReq>().with_children(|parent| {
            parent.spawn_scene(asset_server.load(&config.asset));
        }).with_children(|child_builder| {
            for ability in &config.abilities {
                let mut cmd = child_builder.spawn();
                match ability {
                    AbilityConfig::Shoot(ab) => {
                        cmd.insert_bundle(
                            (
                                ab.clone(),
                                ShootAbilityRuntime { fire_node: None }
                            )
                        );
                    }

                    AbilityConfig::Channel(ab) => {
                        cmd.insert_bundle(
                            (
                                ab.clone(),
                                ChannelAbilityRuntime { fire_node: None }
                            )
                        );
                    }
                }
            }
        });
    }
}

pub fn update_shoot_ability_system(query: Query<(&ShootAbilityConfig, &ShootAbilityRuntime)>) {}

