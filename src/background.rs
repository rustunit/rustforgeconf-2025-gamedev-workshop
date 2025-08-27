/// here we define a plugin that only loads a background image
use bevy::prelude::*;

use crate::{GameState, loading::GameAssets};

pub fn plugin(app: &mut App) {
    app.add_systems(OnExit(GameState::Loading), setup);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("background"),
        Transform::from_scale(Vec3::splat(1.5)).with_translation(Vec3::new(0., 0., -10.)),
        Sprite::from_image(assets.background.clone()),
    ));
}
