use bevy::prelude::*;
use rand::*;

#[derive(Component)]
pub struct AsteroidsStruct {
    pub d: Vec2,
}

pub fn spawn_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asteroids_query: Query<&AsteroidsStruct>,
) {
    if asteroids_query.iter().len() > 12 {
        return;
    }

    let mut rng = thread_rng();
    let (x, y) = match rng.gen_range(0..4) {
        0 => (rng.gen_range(-500.0..500.0_f32), -500.0),
        1 => (rng.gen_range(-500.0..500.0_f32), 500.0),
        2 => (-500.0, rng.gen_range(-500.0..500.0_f32)),
        3 => (500.0, rng.gen_range(-500.0..500.0_f32)),
        _ => return
    };
    let index = rng.gen_range(1..4);
    let angle = rng.gen_range(0.0_f32..360.0_f32).to_radians();
    let d = Vec2::new(angle.cos(), angle.sin());

    commands.spawn((
        Sprite {
            image: asset_server.load(format!("images/asteroids/asteroid_{}.png", index)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.5),
        AsteroidsStruct {d}
    ));
}

pub fn move_asteroids(
    mut commands: Commands,
    mut asteroids_query: Query<(&mut Transform, Entity, &AsteroidsStruct)>,
) {
    for (mut asteroids_t, asteroids_e, asteroids) in asteroids_query.iter_mut() {
        asteroids_t.translation.x += asteroids.d.x * 2.0;
        asteroids_t.translation.y += asteroids.d.y * 2.0;

        let borders = asteroids_t.translation.y.abs() >= 500.0
            || asteroids_t.translation.x.abs() >= 500.0;

        if borders {
            commands.entity(asteroids_e).despawn();
        }
    }
}
