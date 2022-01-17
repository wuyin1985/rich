use bevy::reflect::TypeUuid;
use super::ability::{ChannelAbility, ShootAbility};
use super::table_data::TableDataItem;

use bevy::{
    reflect::{
        serde::{ReflectDeserializer, ReflectSerializer},
        DynamicStruct, TypeRegistry,
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeSeed;

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(TypeUuid)]
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

#[derive(Reflect)]
pub struct StringID {
    str: Option<String>,
    id: u64,
}

impl StringID {
    pub fn str(v: String) -> Self {
        Self { str: Some(v), id: 0 }
    }

    pub fn id(v: u64) -> Self {
        Self { str: None, id: v }
    }
}

#[derive(Reflect)]
pub struct TestConfig {
    id: StringID,
    value: u32,
    level: u32,
    asset_id: StringID,
}


pub fn test_reflect(type_registry: Res<TypeRegistry>) {
    let type_registry = type_registry.read();

    let origin = TestConfig {
        id: StringID::str("haha".to_string()),
        value: 1u32,
        level: 3,
        asset_id: StringID::str("main".to_string()),
    };

    let serializer = ReflectSerializer::new(&origin, &type_registry);
    let ron_string =
        ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default()).unwrap();
    info!("{}\n", ron_string);

    // Dynamic properties can be deserialized
    let reflect_deserializer = ReflectDeserializer::new(&type_registry);
    let mut deserializer = ron::de::Deserializer::from_str(&ron_string).unwrap();
    let reflect_value = reflect_deserializer.deserialize(&mut deserializer).unwrap();

    let new_value = reflect_value.downcast_ref::<TestConfig>();
    info!("haha");
}