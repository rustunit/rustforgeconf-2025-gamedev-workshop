/// Here we only worry about the UI part that show the player
/// how many piggies are still left to hit out of how many that were spawned
use bevy::prelude::*;

use crate::{GameState, birdies::BirdResources};

#[derive(Component, Debug, Clone, Copy)]
struct BirdyUI;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
    app.add_systems(Update, update.run_if(in_state(GameState::Playing)));
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            bottom: Val::Px(0.),
            ..default()
        },
        StateScoped(GameState::Playing),
        BirdyUI,
        Text::new("Birds: 0/3".to_string()),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Right).with_no_wrap(),
    ));
}

fn update(
    query: Single<Entity, (With<Text>, With<BirdyUI>)>,
    birdies: Res<BirdResources>,
    mut text: TextUiWriter,
) {
    if birdies.is_changed() {
        let e = *query;
        *text.text(e, 0) = format!("Birdies: {}/{}", birdies.birds_spawned, birdies.birds);
    }
}
