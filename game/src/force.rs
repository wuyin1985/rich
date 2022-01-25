use std::ops::{Deref, DerefMut};
use bvh::aabb::AABB;
use bvh::ray::Ray;
use bvh::Vector3;
use crate::hit_query::{HitQuery, HitResult};
use crate::prelude::*;
use crate::{StringId, StringIdOptionCopy};
use crate::attrs::{AttrCommand, AttrCommandQueue};
use crate::table::{TableData, TableDataItem};

#[derive(Clone, Copy)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum HitTargetSelect {
    Target,
    Circle(f32),
    RayFromStart(f32),
}


#[derive(Clone, Copy)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Movement {
    Immediate,
    Line(f32),
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct ForceConfig {
    pub name: StringId,
    pub select: HitTargetSelect,
    pub movement: Movement,
    pub self_sfx: Option<StringId>,
    pub fire_sfx: Option<StringId>,
    pub hit_sfx: Option<StringId>,
}

impl TableDataItem for ForceConfig {
    fn get_name(&self) -> &str {
        &self.name.str()
    }

    fn parse(&mut self) {
        self.name.change_2_id();
        self.self_sfx.change_2_id();
        self.fire_sfx.change_2_id();
        self.hit_sfx.change_2_id();
    }
}

#[derive(Component)]
pub struct CreateForceReq {
    id: u64,
}

#[derive(Component)]
pub enum ForceTarget {
    Entity(Entity),
    Position(Vec3),
}

#[derive(Component)]
struct Force {
    pub select: HitTargetSelect,
    pub self_sfx: Option<u64>,
    pub fire_sfx: Option<u64>,
    pub hit_sfx: Option<u64>,
}

#[derive(Component)]
struct ForceMoveImmediate {}

#[derive(Component)]
struct ForceMoveLine {
    speed: f32,
}

fn create_force_system(
    mut commands: Commands,
    query: Query<(Entity, &CreateForceReq, &ForceTarget, &Transform)>,
    table: Res<TableData<ForceConfig>>,
) {
    for (entity, req, target, transform) in query.iter() {
        let config = table.index(req.id);
        let mut cmds = commands.entity(entity);
        cmds.insert(
            Force {
                select: config.select,
                hit_sfx: config.hit_sfx.as_id(),
                fire_sfx: config.fire_sfx.as_id(),
                self_sfx: config.self_sfx.as_id(),
            }
        );

        match config.movement {
            Movement::Immediate => {
                cmds.insert(ForceMoveImmediate {});
            }
            Movement::Line(speed) => {
                cmds.insert(ForceMoveLine { speed });
            }
        }
    }
}

fn update_force_immediate(mut commands: Commands,
                          mut query: Query<(Entity, &Force, &ForceTarget, &mut Transform), With<ForceMoveImmediate>>,
                          global_transform_query: Query<&GlobalTransform>,
                          hit_query: Res<HitQuery>,
                          attr_commands: Res<AttrCommandQueue>,
) {
    let bvh = hit_query.deref();
    for (entity, force, target, mut transform) in query.iter_mut() {
        let transform = transform.deref_mut();
        let target_pos = match target {
            ForceTarget::Entity(e) => {
                let p = global_transform_query.get(*e).expect(format!("failed to find global transform for entity {:?}", e).as_str());
                p.translation
            }
            ForceTarget::Position(p) => {
                *p
            }
        };

        let start_pos = transform.translation;
        transform.translation = target_pos;

        let target_as_result = HitResult::create_with_entity(entity);

        let result_list = match force.select {
            HitTargetSelect::Target => {
                vec![&target_as_result]
            }

            HitTargetSelect::Circle(radius) => {
                let ld = Vector3::new(target_pos.x - radius, target_pos.y - radius, target_pos.z - radius);
                let ru = Vector3::new(target_pos.x + radius, target_pos.y + radius, target_pos.z + radius);
                bvh.traverse_aabb(&AABB { min: ld, max: ru })
            }

            //todo impl len
            HitTargetSelect::RayFromStart(len) => {
                let dir = (transform.translation - start_pos);
                bvh.traverse(&Ray::new(start_pos, dir))
            }
        };

        for ret in result_list {
            attr_commands.push(AttrCommand::Add(
                ret.entity, 0, -1f32,
            ))
        }
    }
}
