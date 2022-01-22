use std::collections::HashMap;
use crate::prelude::*;

#[derive(Component)]
pub struct HierarchyNameMap {
    pub values: HashMap<u64, Entity>,
}

impl HierarchyNameMap {
    pub fn create() -> Self {
        HierarchyNameMap { values: Default::default() }
    }
}

#[derive(Component)]
pub struct HierarchyNameMapInitTag {}

fn collect_names(map: &mut HashMap<u64, Entity>,
                 entity: Entity,
                 children_query: &Query<&Children>,
                 name_query: &Query<&Name>,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            if let Ok(name) = name_query.get(*child) {
                let id = hashtoollib::hash(name.as_str());
                map.insert(id, *child);
            }
        }

        for child in children.iter() {
            collect_names(map, *child, children_query, name_query);
        }
    }
}

//attacker system relay on the GLTF node name to find fire points, sfx attach
pub fn init_node_name_system(
    mut hierarchy_query: Query<(Entity, &mut HierarchyNameMap), (Without<HierarchyNameMapInitTag>, With<Children>)>,
    children_query: Query<&Children>,
    name_query: Query<&Name>,
) {
    for (entity, mut hierarchy) in hierarchy_query.iter_mut() {
        let  map = &mut hierarchy.values;
        collect_names(map, entity, &children_query, &name_query);
    }
}

