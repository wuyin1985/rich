use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crossbeam_queue::SegQueue;
use crate::death::Death;
use crate::game::GameState;
use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct AttrConfig {
    pub name: StringId,
    pub init: f32,
    pub max: Option<StringId>,
}

#[derive(Deserialize, Serialize)]
pub struct AttrsConfig {
    pub values: Vec<AttrConfig>,
}

impl AttrsConfig {
    pub fn parse(&mut self) {
        for v in &mut self.values {
            v.max.change_2_id();
            v.name.change_2_id();
        }
    }
}

pub struct Attr {
    value: f32,
    max_attr: Option<u64>,
}

#[derive(Component)]
pub struct Attrs {
    values: HashMap<u64, Attr>,
}

impl Attrs {
    pub fn load_from_config(config: &AttrsConfig) -> Self {
        let mut values = HashMap::new();
        for c in &config.values {
            values.insert(c.name.id(), Attr { value: c.init, max_attr: c.max.as_id() });
        }

        Self {
            values
        }
    }
}

pub enum AttrCommand {
    Add(Entity, u64, f32)
}

pub struct AttrCommandQueue {
    seg: SegQueue<AttrCommand>,
}

impl AttrCommandQueue {
    pub fn push(&self, cmd: AttrCommand) {
        self.seg.push(cmd);
    }
}

pub struct AttrPlugin;

impl Plugin for AttrPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_attr_system));
        app.add_system_set(SystemSet::on_exit(GameState::Playing).with_system(destroy_attr_system));

        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_attr_system));
    }
}

fn setup_attr_system(mut commands: Commands) {
    commands.insert_resource(
        AttrCommandQueue {
            seg: SegQueue::new(),
        }
    )
}

fn destroy_attr_system(mut commands: Commands) {
    commands.remove_resource::<AttrCommandQueue>();
}

fn update_attr_system(mut commands: Commands, queue: Res<AttrCommandQueue>, mut query: Query<(&mut Attrs)>) {
    while let Some(cmd) = queue.seg.pop() {
        match cmd {
            AttrCommand::Add(entity, name, value) => {
                let mut attrs = query.get_mut(entity).expect(format!("failed to find attrs on entity {:?}", entity).as_str());

                match attrs.values.entry(name) {

                    Entry::Occupied(o) => {
                        let attr = o.into_mut();
                        let mut v = attr.value + value;
                        let max_attr = attr.max_attr;
                        let attr_p: *mut Attr = attr;

                        if let Some(m) = max_attr {
                            let max = attrs.values.get(&m).expect(format!("failed to get max value of {}", m).as_str());
                            v = v.max(max.value);
                        }

                        if v <= 0f32 {
                            commands.entity(entity).insert(Death {});
                        }

                        unsafe {
                            (*attr_p).value = v;
                        }
                    }

                    Entry::Vacant(_) => {
                        let mut print_name = name.to_string();
                        #[cfg(feature = "debug")]
                            {
                                if let Some(s) = hashtoollib::un_hash(name) {
                                    print_name = s;
                                }
                            }
                        panic!("the attr {} not exist", print_name);
                    }
                };
            }
        }
    }
}