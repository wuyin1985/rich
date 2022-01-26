use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use itertools::Itertools;
use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use bvh::Vector3;
use crate::game::GameState;
use crate::prelude::*;

#[derive(Component)]
pub struct HitBounds {
    radius: f32,
}

impl HitBounds {
    pub fn create(radius: f32) -> Self {
        HitBounds { radius }
    }
}

pub struct HitQuery {
    bvh: Option<BvhBundle>,
    task: Option<Task<BvhBundle>>,
}

impl HitQuery {
    pub fn traverse(&self, ray: &Ray) -> Vec<&HitResult> {
        let b = self.bvh.as_ref().expect("the bvh not build");
        b.bvh.traverse(ray, &b.targets)
    }

    pub fn traverse_aabb(&self, aabb: &AABB) -> Vec<&HitResult> {
        let b = self.bvh.as_ref().expect("the bvh not build");
        b.bvh.traverse_aabb(aabb, &b.targets)
    }
}

struct BvhBundle {
    bvh: BVH,
    targets: Vec<HitResult>,
}

pub struct HitResult {
    pub entity: Entity,
    radius: f32,
    position: Vector3,
    node_index: usize,
}

impl HitResult {
    pub fn create_with_entity(entity: Entity) -> Self {
        HitResult {
            entity,
            radius: Default::default(),
            position: Default::default(),
            node_index: Default::default(),
        }
    }
}


impl Bounded for HitResult {
    fn aabb(&self) -> AABB {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.position - half_size;
        let max = self.position + half_size;
        AABB::with_bounds(min, max)
    }
}

impl BHShape for HitResult {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub struct HitQueryPlugin;

impl Plugin for HitQueryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            HitQuery {
                bvh: None,
                task: None,
            }
        )

            //change to pre update remove after bug:https://github.com/bevyengine/bevy/issues/1671 fix
            .add_system_set_to_stage(CoreStage::Update,
                                     SystemSet::on_update(GameState::Playing).
                                         with_system(prepare_bvh_tree_system));
        // .add_system_set_to_stage(CoreStage::PostUpdate,
        //                          SystemSet::on_update(GameState::Playing).
        //                              with_system(build_bvh_tree_system));
    }
}


fn prepare_bvh_tree_system(
    query: Query<(Entity, &GlobalTransform, &HitBounds)>, mut worker: ResMut<HitQuery>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    let mut nodes = query.iter().enumerate().
        map(|(idx, (entity, t, hit))| {
            HitResult {
                radius: hit.radius,
                position: t.translation,
                node_index: idx,
                entity,
            }
        }).collect_vec();

    if worker.bvh.is_none() {
        worker.bvh = Some(BvhBundle { bvh: BVH::build(&mut nodes), targets: nodes });
    } else {
        let task = thread_pool.spawn(async move {
            BvhBundle { bvh: BVH::build(&mut nodes), targets: nodes }
        });

        worker.task = Some(task);
    }
}

fn build_bvh_tree_system(mut worker: ResMut<HitQuery>) {
    if let Some(task) = &mut worker.task {
        let bvh = future::block_on(task);
        worker.bvh = Some(bvh);
    }
}
