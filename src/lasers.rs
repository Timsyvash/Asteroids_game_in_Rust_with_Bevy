use bevy::prelude::*;
use crate::player::PlayerStruct;

#[derive(Component)]
pub struct LasersStruct {
    pub d: Vec2,
}

pub fn lasers_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<&mut Transform, With<PlayerStruct>>,
    key_code: Res<ButtonInput<KeyCode>>
) {
    if key_code.just_pressed(KeyCode::Space) {
        if let Ok(t_lasers) = player_query.single_mut() {
            let angle = t_lasers.rotation.to_euler(EulerRot::ZYX).0 + 90_f32.to_radians();
            let d = Vec2::new(angle.cos(), angle.sin());
            commands.spawn((
                Sprite {
                    image: asset_server.load("images/lasers/laser.png"),
                    ..default()
                },
                *t_lasers,
                LasersStruct {d}
            ));
            commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/laser_player_music.ogg")),
                PlaybackSettings::ONCE,
            ));
        }
    }
}

pub fn move_lasers_player(
    mut commands: Commands,
    mut lasers_query: Query<(&mut Transform, Entity, &LasersStruct)>,
) {
    for (mut lasers_t, lasers_e, lasers) in lasers_query.iter_mut() {
        lasers_t.translation.x += lasers.d.x * 15.0;
        lasers_t.translation.y += lasers.d.y * 15.0;

        let borders = lasers_t.translation.y.abs() >= 500.0
            || lasers_t.translation.x.abs() >= 500.0;

        if borders {
            commands.entity(lasers_e).despawn();
        }
    }
}