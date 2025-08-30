use crate::{GameState, loading::GameAssets};
/// This define the plugin handling the bomb behaviour: spawning, getting hit, exploding, despawning
use avian2d::prelude::*;
use bevy::prelude::ops::log10;
use bevy::prelude::*;

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

    let r = 100.0;

    // Shape properties
    if let Ok(origin) = bomb_query.get(target) {
        let shape = Collider::circle(r);
        let rotation = 0.0;
        let direction = Dir2::X;

        // Configuration for the shape cast
        let config = ShapeCastConfig::from_max_distance(0.0);
        let filter = SpatialQueryFilter::default();

        // Cast shape and get up to 20 hits
        let hits = spatial_query.shape_hits(
            &shape,
            origin.translation.truncate(),
            rotation,
            direction,
            100,
            &config,
            &filter,
        );

        // Print hits
        for hit in hits.iter() {
            info!("Hit: {:?}", hit);

            if let Ok((t, mut ei)) = target_query.get_mut(hit.entity) {
                let imp = calculate_impulse(origin, t, r) * 5_000_000.0;
                ei.apply_impulse(imp);
            }
        }
    }
    commands.entity(target).despawn();
}

fn calculate_impulse(o: &Transform, t: &Transform, radius: f32) -> Vec2 {
    info!("origin={o:?}, t={t:?}");

    let v = (t.translation - o.translation).truncate();
    info!("v1={v:?}");

    let v = v / radius;
    let v = v.clamp_length_min(0.0);
    let v = v.clamp_length_max(1.0);
    //let d = v.length();

    // when |v| == 1, we want 0 force
    // when |v| == 0, we want max force
    let v_norm = v.normalize();
    let v = v_norm - v;

    //info!("v={v:?}, d={d:?}");
    info!("v2={v:?}");

    let d = 0.5 * log10(1.0 - v.length()) + 1.0;
    info!("d={d:?}");

    v * d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_impulse() {
        dbg!(calculate_impulse(
            &Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            &Transform::from_translation(Vec3::new(1.0, 1.0, 0.0)),
            1.0
        ));
    }
}
