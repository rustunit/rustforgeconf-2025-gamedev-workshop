/// This plugin defines the sling shot mechanic of the birds.
/// The `Sling` resource holds the position and velocity of the sling.
/// This way we can worry about seperating working on the trajectory visualizing and the sling shot input mechanics.
use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{GameState, MainCamera, loading::GameAssets};

static FORCE_SCALE: Vec2 = Vec2::splat(5.0);

#[derive(Resource)]
struct Sling {
    pub pos: Vec2,
    pub drag_pos: Option<Vec2>,
}

#[derive(Event)]
struct OnShoot {
    pub velocity: Vec2,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Sling {
        pos: Vec2::new(-300., -40.),
        drag_pos: None,
    });

    app.add_systems(OnEnter(GameState::Playing), setup);
    app.add_systems(
        Update,
        (on_mouse_input, touch_input).run_if(in_state(GameState::Playing)),
    );

    app.add_observer(on_shoot);
}

#[derive(Component)]
struct SlingEntity;

fn setup(mut commands: Commands, sling: Res<Sling>, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("sling"),
        SlingEntity,
        StateScoped(GameState::Playing),
        Transform::from_translation(sling.pos.extend(0.)),
        children![(
            Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
            Sprite::from_image(assets.bevy.clone()),
        )],
    ));
}

fn touch_input(mut commands: Commands, touches: Res<Touches>) {
    // todo
}

fn on_mouse_input(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut sling: ResMut<Sling>,
    mut sling_ent: Single<&mut Transform, With<SlingEntity>>,
) {
    let (camera, camera_transform) = &*camera;

    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = get_cursor_pos(&*window, camera, camera_transform) {
            let dist = (cursor_pos - sling.pos).length();
            if dist < 20.0 {
                sling.drag_pos = Some(cursor_pos);
            }
        }
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        if sling.drag_pos.is_some() {
            if let Some(cursor_pos) = get_cursor_pos(&*window, camera, camera_transform) {
                sling.drag_pos = Some(cursor_pos);
            }
        }
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        if let Some(drag_pos) = sling.drag_pos {
            let velocity = FORCE_SCALE * (sling.pos - drag_pos);
            commands.trigger(OnShoot { velocity });
        }
        sling.drag_pos = None;
    }

    let render_pos = sling.drag_pos.unwrap_or(sling.pos);
    **sling_ent = Transform::from_translation(render_pos.extend(0.0));
}

fn on_shoot(
    event: Trigger<OnShoot>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    sling: Res<Sling>,
) {
    commands.spawn((
        Name::new("bird"),
        StateScoped(GameState::Playing),
        Transform::from_translation(sling.pos.extend(0.)),
        RigidBody::Dynamic,
        LinearVelocity(event.velocity),
        Collider::circle(20.),
        children![(
            Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
            Sprite::from_image(assets.bevy.clone()),
        )],
    ));
}

fn get_cursor_pos(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let cursor_pos = window.cursor_position()?;
    let cursor_pos = camera
        .viewport_to_world(camera_transform, cursor_pos)
        .ok()?
        .origin
        .truncate();
    Some(cursor_pos)
}
