/// This plugin defines the sling shot mechanic of the birds.
/// The `Sling` resource holds the position and velocity of the sling.
/// This way we can worry about seperating working on the trajectory visualizing and the sling shot input mechanics.
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ui_anchor::{AnchorPoint, AnchorUiConfig, AnchoredUiNodes};

use crate::{GameState, birdies::BirdResources, birdies_ui::BirdyUI, loading::GameAssets};

#[derive(Resource)]
struct Sling {
    pub pos: Vec2,
    pub velocity: Vec2,
}

#[derive(Event)]
pub struct OnShoot;

pub fn plugin(app: &mut App) {
    app.insert_resource(Sling {
        pos: Vec2::new(-300., -40.),
        velocity: Vec2::new(1000., 200.),
    });

    app.add_systems(OnEnter(GameState::Playing), setup);
    app.add_systems(
        Update,
        (on_mouse_input, touch_input).run_if(in_state(GameState::Playing)),
    );

    app.add_observer(on_shoot);
}

fn setup(mut commands: Commands, sling: Res<Sling>, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("sling"),
        StateScoped(GameState::Playing),
        Transform::from_translation(sling.pos.extend(0.)),
        LinearVelocity(sling.velocity),
        children![(
            Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
            Sprite::from_image(assets.bevy.clone()),
        )],
        AnchoredUiNodes::spawn_one((
            AnchorUiConfig {
                anchorpoint: AnchorPoint::middle(),
                offset: Some(Vec3::new(0.0, 50., 0.0)),
                ..Default::default()
            },
            Node {
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
            BorderRadius::all(Val::Px(2.)),
            Outline::default(),
            Children::spawn_one((
                BirdyUI,
                Text::new("Birds: 3".to_string()),
                TextFont {
                    font_size: 20.0,
                    ..Default::default()
                },
                TextLayout::new_with_justify(JustifyText::Right).with_no_wrap(),
            )),
        )),
    ));
}

fn touch_input(mut commands: Commands, touches: Res<Touches>) {
    if touches.any_just_pressed() {
        commands.trigger(OnShoot);
    }
}

fn on_mouse_input(mut commands: Commands, mouse_buttons: Res<ButtonInput<MouseButton>>) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        commands.trigger(OnShoot);
    }
}

fn on_shoot(
    _: Trigger<OnShoot>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    sling: Res<Sling>,
    mut birds: ResMut<BirdResources>,
) {
    if birds.birds_spawned < birds.birds {
        birds.birds_spawned += 1;

        commands.spawn((
            Name::new("bird"),
            StateScoped(GameState::Playing),
            Transform::from_translation(sling.pos.extend(0.)),
            RigidBody::Dynamic,
            LinearVelocity(sling.velocity),
            Collider::circle(20.),
            children![(
                Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
                Sprite::from_image(assets.bevy.clone()),
            )],
        ));
    }
}
