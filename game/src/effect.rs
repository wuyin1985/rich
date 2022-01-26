use serde::{Serialize, Deserialize};
use crate::attrs::{AttrCommand, AttrCommandQueue};
use crate::prelude::*;
use crate::StringId;
use crate::table::{TableData, TableDataItem};

#[derive(Deserialize, Serialize)]
pub enum EffectConfig {
    Hurt(f32),
}

#[derive(Deserialize, Serialize)]
pub struct EffectsConfig {
    pub name: StringId,
    pub values: Vec<EffectConfig>,
}

impl TableDataItem for EffectsConfig {
    fn get_name(&self) -> &str {
        &self.name.str()
    }

    fn parse(&mut self) {
        self.name.change_2_id();
    }
}

pub struct EffectCommand {
    pub id: u64,
    pub target: Entity,
}

pub fn handle_effect_system(In(cmds): In<Vec<EffectCommand>>,
                            table: Res<TableData<EffectsConfig>>,
                            attr_commands: Res<AttrCommandQueue>,
) {
    for cmd in &cmds {
        let cfgs = table.index(cmd.id);
        for cfg in &cfgs.values {
            match cfg {
                EffectConfig::Hurt(hurt_value) => {
                    attr_commands.push(AttrCommand::Add(
                        cmd.target, 0, *hurt_value,
                    ))
                }
            }
        }
    }
}