use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(Component, Debug)]
pub struct MoveWithMapPath {
    path_index: usize,
    target_point_index: usize,
}

// #[derive(Deserialize, Serialize, TypeUuid)]
// #[uuid = "9a852db2-3eb7-4c91-99ae-ec1ea92f2877"]
// pub struct MonsterConfig {
//     pub name: String,
//     pub ability_holders: Vec<AttackAbilityHolderConfig>,
// }
//
// impl TableDataItem for MonsterConfig {
//     fn get_name(&self) -> &str {
//         &self.name
//     }
// }
