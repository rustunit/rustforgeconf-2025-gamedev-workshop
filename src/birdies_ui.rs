/// Here we only worry about the UI part that show the player
/// how many piggies are still left to hit out of how many that were spawned
use bevy::prelude::*;

use crate::{GameState, birdies::BirdResources};

#[derive(Component, Debug, Clone, Copy)]
pub struct BirdyUI;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update.run_if(in_state(GameState::Playing)));
}

fn update(
    query: Single<Entity, (With<Text>, With<BirdyUI>)>,
    birdies: Res<BirdResources>,
    mut text: TextUiWriter,
) {
    if birdies.is_changed() {
        let e = *query;
        *text.text(e, 0) = format!("Birdies: {}", birdies.birds - birdies.birds_spawned);
    }
}
