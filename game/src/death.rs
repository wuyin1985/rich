use crate::prelude::*;

#[derive(Component)]
pub struct Death {}

pub fn death_system(mut commands: Commands, query: Query<Entity, With<Death>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}