/// This define the plugin handling the bomb behaviour: spawning, getting hit, exploding, despawning
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, loading::GameAssets};

#[derive(Event, Debug, Clone)]
pub struct BombSpawn(pub Vec<Vec2>);

pub fn plugin(app: &mut App) {
    app.add_observer(on_spawn);
}

fn on_spawn(trigger: Trigger<BombSpawn>, mut commands: Commands, assets: Res<GameAssets>) {
    for p in &trigger.event().0 {
        commands
            .spawn((
                Name::new("bomb"),
                StateScoped(GameState::Playing),
                Transform::from_translation(Vec3::new(p.x, p.y, 0.)),
                Sensor,
                Collider::rectangle(30., 30.),
                CollisionEventsEnabled,
                ExternalImpulse::new(Vec2::new(0.0, 0.0)),  // TODO: remove this
                children![(
                    Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
                    Sprite::from_image(assets.bomb.clone()),
                )],
            ))
            .observe(on_bomb_collision);
    }
}

fn on_bomb_collision(
    trigger: Trigger<OnCollisionStart>,
    spatial_query: SpatialQuery,
    mut query: Query<(&Transform, &mut ExternalImpulse)>,
    mut commands: Commands,
) {
    let target = trigger.target();

    // Shape properties
    if let Ok((origin, _)) = query.get(target) {
        let shape = Collider::circle(50.0);
        let rotation = 0.0;
        let direction = Dir2::X;

        // Configuration for the shape cast
        let config = ShapeCastConfig::from_max_distance(0.0);
        let filter = SpatialQueryFilter::default();

        // Cast shape and get up to 20 hits
        let hits = spatial_query.shape_hits(&shape, origin.translation.truncate(), rotation, direction, 100, &config, &filter);

        // Print hits
        for hit in hits.iter() {
            info!("Hit: {:?}", hit);

            if let Ok(mut x) = query.get_mut(hit.entity) {
                info!("BAR");
                x.1.apply_impulse(Vec2::new(0.0, 1000.0));
            }
        }
    }
    commands.entity(target).despawn();
}
