use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crossbeam_queue::SegQueue;
use crate::death::Death;
use crate::game::GameState;
use crate::prelude::*;

pub struct Attr {
    cur: f32,
    max: f32,
}

#[derive(Component)]
pub struct Attrs {
    values: HashMap<u64, Attr>,
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

pub struct AttrPlugin {}

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
                let attrs = query.get_mut(entity).expect(format!("failed to find attrs on entity {:?}", entity).as_str());
                if let Some(v) = attrs.values.get_mut(&name) {
                    v.cur += value;
                    v.cur.max(v.max);

                    if v.cur <= 0f32 {
                        commands.entity(entity).insert(Death {})
                    }
                } else {
                    let mut print_name = name.to_string().as_str();
                    #[cfg(feature = "debug")]
                        {
                            if let Some(s) = hashtoollib::un_hash(name) {
                                print_name = s.as_str();
                            }
                        }
                    panic!("the attr {} not exist", print_name);
                }
            }
        }
    }
}