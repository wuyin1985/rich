use bevy::reflect::TypeUuid;
use super::table::TableDataItem;

#[derive(serde::Deserialize, serde::Serialize, TypeUuid)]
#[uuid = "9a852db2-3eb7-4c91-99ae-ec1ea92f2877"]
pub struct AttackerConfig {
    pub name: String,
    pub ability_holders: Vec<AttackAbilityHolderConfig>,
}

impl TableDataItem for AttackerConfig {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct AttackAbilityHolderConfig {
    pub ability: AbilityConfig,
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub enum AbilityConfig {
    Shoot(ShootAbility),
    Channel(ChannelAbility),
}

#[allow(dead_code)]
pub struct Attacker {}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ShootAbility {
    pub cd: f32,
    pub reload_time: f32,
    pub magazine: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ChannelAbility {
    pub total_value: f32,
    pub value_cost_speed: f32,
}


