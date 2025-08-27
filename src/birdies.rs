use bevy::prelude::*;

use crate::GameState;

#[derive(Resource, Clone, Debug)]
pub struct BirdResources {
    pub birds: usize,
    pub birds_spawned: usize,
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
}

fn setup(mut commands: Commands) {
    commands.insert_resource(BirdResources {
        birds: 3,
        birds_spawned: 0,
    });
}
