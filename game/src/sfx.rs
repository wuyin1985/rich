use crate::prelude::*;
use crate::StringId;
use crate::table::{TableData, TableDataItem};

#[derive(serde::Deserialize, serde::Serialize, Component)]
pub struct SfxGroupConfig {
    pub name: String,
    pub sfx_list: Vec<SfxConfig>,
}

impl TableDataItem for SfxGroupConfig {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn parse(&mut self) {
        for sfx in &mut self.sfx_list {
            sfx.parse();
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum SfxVisualType {
    Default,
    Range,
    Chain,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SfxVisual {
    pub asset: String,
    pub visual_type: SfxVisualType,
}

impl SfxVisual {
    pub fn parse(&mut self) {}
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SfxSound {
    pub id: StringId,
}

impl SfxSound {
    pub fn parse(&mut self) {
        self.id.change_2_id();
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum SfxConfig {
    Visual(SfxVisual),
    Sound(SfxSound),
}

impl SfxConfig {
    pub fn parse(&mut self) {
    }
}

#[derive(serde::Deserialize, serde::Serialize, Component)]
pub struct SfxRuntime {
    pub config: SfxConfig,
}

#[derive(serde::Deserialize, serde::Serialize, Component)]
pub struct CreateSfxReq {
    pub id: u64,
}

pub fn create_sfx_system(
    req_query: Query<(Entity, &CreateSfxReq)>,
    table: Res<TableData<SfxConfig>>,
) {}
