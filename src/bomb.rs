use crate::{GameState, loading::GameAssets};
/// This define the plugin handling the bomb behaviour: spawning, getting hit, exploding, despawning
use avian2d::prelude::*;
use bevy::prelude::ops::log10;
use bevy::prelude::*;

const IMPULSE_SCALING: f32 = 3_000_000.0;
const EXPLOSION_RADIUS: f32 = 120.0;

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
    bomb_query: Query<&Transform>,
    mut target_query: Query<(&Transform, &mut ExternalImpulse)>,
    mut commands: Commands,
) {
    let target = trigger.target();

    if let Ok(origin) = bomb_query.get(target) {
        let shape = Collider::circle(EXPLOSION_RADIUS);
        let rotation = 0.0;
        let direction = Dir2::X;
        let config = ShapeCastConfig::from_max_distance(0.0);
        let filter = SpatialQueryFilter::default();

        let hits = spatial_query.shape_hits(
            &shape,
            origin.translation.truncate(),
            rotation,
            direction,
            100,  // 100 should be enough for this game
            &config,
            &filter,
        );

        for hit in hits.iter() {
            if let Ok((t, mut ei)) = target_query.get_mut(hit.entity) {
                let imp = calculate_impulse_2d(origin, t, EXPLOSION_RADIUS) * IMPULSE_SCALING;
                ei.apply_impulse(imp);
            }
        }
    }
    commands.entity(target).despawn();
}

fn calculate_impulse_2d(origin: &Transform, target: &Transform, radius: f32) -> Vec2 {
    // Calculate the impulse to apply to the target based on the distance from the explosion origin
    // and the maximum radius of the explosion.

    // The impulse direction is from the origin to the target.
    // The impulse magnitude is inversely proportional to the distance
    // from the origin to the target, clamped to a maximum value.
    let origin_to_target = (target.translation - origin.translation).truncate();

    // Scale the vector by the radius of the explosion, and clamp its magnitude to [0, 1]
    let v = (origin_to_target / radius).clamp_length_min(0.0).clamp_length_max(1.0);

    // Calculate t in [0, 1] representing how far the target is from the edge of the explosion
    let t = v.length();

    // When t == 0, we want max force (f = 1), falling away to zero force at t == 1 (f = 0)
    let f = 1.0 - t;

    // Apply a logarithmic falloff to the impulse magnitude, for a more natural feel
    // https://www.desmos.com/calculator/bd8bzvmi1w
    let f = 0.5 * log10(f + 0.01) + 1.0;

    // Return a vector in the direction of v with magnitude f
    v.normalize() * f
}
