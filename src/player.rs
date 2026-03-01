use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerStruct;

pub fn keys(
    key_code: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerStruct>>
) {
    for mut transform_player in player_query.iter_mut() {
        let ale = transform_player.rotation.to_euler(EulerRot::ZYX).0 + 90_f32.to_radians();
        let d = Vec2::new(ale.cos(), ale.sin());

        if key_code.pressed(KeyCode::ArrowDown) {
            transform_player.translation.x -= d.x * 5.0;
            transform_player.translation.y -= d.y * 5.0;
        } else if key_code.pressed(KeyCode::ArrowUp) {
            transform_player.translation.x += d.x * 5.0;
            transform_player.translation.y += d.y * 5.0;
        }

        if key_code.pressed(KeyCode::ArrowLeft) {
            transform_player.rotate_z(3_f32.to_radians());
        } else if key_code.pressed(KeyCode::ArrowRight) {
            transform_player.rotate_z(-3_f32.to_radians());
        }
    }
}

pub fn borders(
    mut player_query: Query<&mut Transform, With<PlayerStruct>>
) {
    for mut player_transform in player_query.iter_mut() {
        if player_transform.translation.x >= 475.0 {
            player_transform.translation.x = -475.0;
        } else if player_transform.translation.x <= -475.0 {
            player_transform.translation.x = 475.0;
        }
        if player_transform.translation.y >= 475.0 {
            player_transform.translation.y = -475.0;
        } else if player_transform.translation.y <= -475.0 {
            player_transform.translation.y = 475.0;
        }
    }
}
