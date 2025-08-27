/// This define the plugin handling the piggie behaviour: spawning, getting hit, despawning
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, loading::GameAssets};

#[derive(Event, Debug, Clone)]
pub struct PiggySpawn(pub Vec<Vec2>);

#[derive(Resource, Clone, Debug)]
pub struct PiggyResources {
    pub pigs: usize,
    pub pigs_spawned: usize,
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_spawn);
}

fn on_spawn(trigger: Trigger<PiggySpawn>, mut commands: Commands, assets: Res<GameAssets>) {
    for p in &trigger.event().0 {
        commands
            .spawn((
                Name::new("piggy"),
                StateScoped(GameState::Playing),
                Transform::from_translation(Vec3::new(p.x, p.y, 0.)),
                Sensor,
                Collider::rectangle(30., 30.),
                CollisionEventsEnabled,
                children![(
                    Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
                    Sprite::from_image(assets.piggy.clone()),
                )],
            ))
            .observe(on_piggy_collision);
    }

    commands.insert_resource(PiggyResources {
        pigs: trigger.event().0.len(),
        pigs_spawned: trigger.event().0.len(),
    });
}

fn on_piggy_collision(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut piggies: ResMut<PiggyResources>,
) {
    let target = trigger.target();
    commands.entity(target).despawn();
    piggies.pigs = piggies.pigs.saturating_sub(1);
}
