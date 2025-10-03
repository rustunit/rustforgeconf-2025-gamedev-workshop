use std::f32;

/// This plugin loads the level once entering `Playing`
/// state from hardcoded elements.
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, loading::GameAssets, piggies::PiggySpawn};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    // ground
    let mut vertices = Vec::new();
    let mut x = -768.;
    const A: f32 = 50.;
    const B: f32 = f32::consts::TAU / (768. * 2.);
    while x <= 768. {
        let y = A * (B * (x + 950.)).sin();
        vertices.push(Vec2::new(x, y));
        x += 0.05;
    }

    commands.spawn((
        Name::new("ground"),
        StateScoped(GameState::Playing),
        Transform::from_translation(Vec3::new(0., -200., 0.)),
        RigidBody::Static,
        Collider::polyline(vertices, None),
    ));

    commands.spawn((
        Name::new("foundation"),
        StateScoped(GameState::Playing),
        Transform::from_translation(Vec3::new(50., -230., 0.)),
        RigidBody::Static,
        Collider::rectangle(200.0, 20.0)
    ));

    commands.spawn((
        Name::new("foundation2"),
        StateScoped(GameState::Playing),
        Transform::from_translation(Vec3::new(450., -215., 0.)),
        RigidBody::Static,
        Collider::rectangle(200.0, 20.0)
    ));

    let boxes = vec![
        // house 1
        Vec2::new(0., -180.),
        Vec2::new(0., -140.),
        Vec2::new(0., -100.),
        Vec2::new(0., -60.),
        // Vec2::new(0., 30.),
        //
        Vec2::new(100., -180.),
        Vec2::new(100., -140.),
        Vec2::new(100., -100.),
        Vec2::new(100., -60.),
        // Vec2::new(100., 30.),
        // house 2
        Vec2::new(400., -180.),
        Vec2::new(400., -140.),
        Vec2::new(400., -100.),
        Vec2::new(400., -60.),
        Vec2::new(400., -20.),
        //
        Vec2::new(500., -180.),
        Vec2::new(500., -140.),
        Vec2::new(500., -100.),
        Vec2::new(500., -60.),
        Vec2::new(500., -20.),
    ];

    let piggies = vec![
        // house 1
        Vec2::new(50., -130.),
        // house 2
        Vec2::new(450., -130.),
    ];

    for b in boxes {
        commands.spawn((
            Name::new("box"),
            StateScoped(GameState::Playing),
            Transform::from_translation(Vec3::new(b.x, b.y, 0.)),
            RigidBody::Dynamic,
            Collider::rectangle(40., 40.),
            children![(
                Transform::from_scale(Vec3::new(0.3, 0.3, 1.)),
                Sprite::from_image(assets.kiste.clone()),
            )],
        ));
    }

    commands.spawn((
        Name::new("roof"),
        StateScoped(GameState::Playing),
        Transform::from_translation(Vec3::new(50., 50., 0.)),
        RigidBody::Dynamic,
        Collider::triangle(Vec2::new(-80., 0.), Vec2::new(0., 80.), Vec2::new(80., 0.)),
        children![(
            Transform::from_scale(Vec3::new(1.1, 1.1, 1.)).with_translation(Vec3::new(0., 40., 0.)),
            Sprite::from_image(assets.roof.clone()),
        )],
    ));

    commands.trigger(PiggySpawn(piggies));
}
