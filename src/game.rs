use bevy::prelude::*;
use crate::asteroids::*;
use crate::lasers::*;
use crate::player::*;

#[derive(Resource, PartialEq, Eq, Debug, Clone, Hash, States, Default)]
pub enum GameState {
    #[default]
    NotStarted,
    InGame,
    GameOver,
    Pause,
}

#[derive(Component)]
pub struct BackgroundStruct;

#[derive(Component)]
pub struct GameOverStruct;

#[derive(Component)]
pub struct PauseStruct;

#[derive(Component)]
pub struct GameStartStruct;

#[derive(Component)]
pub struct CountStruct;

#[derive(Component)]
pub struct MaxCountStruct;

#[derive(Resource, Default)]
pub struct ScoreStruct {
    pub score: u32,
    pub max_score: u32,
}

#[derive(Component)]
pub struct CountTextStruct;

#[derive(Component)]
pub struct MaxCountTextStruct;

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("images/background/background.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        BackgroundStruct
    ));
}

pub fn collision_lasers_player_with_asteroids(
    mut commands: Commands,
    lasers_player_query: Query<(Entity, &Transform), With<LasersStruct>>,
    asteroids_query: Query<(Entity, &Transform), With<AsteroidsStruct>>,
    mut count: ResMut<ScoreStruct>,
    asset_server: Res<AssetServer>
) {
   'laser: for (laser_entity, laser_transform) in lasers_player_query.iter() {
        for (asteroid_entity, asteroid_transform) in asteroids_query.iter() {
            let distance = laser_transform.translation.distance(asteroid_transform.translation);
            if distance < 40.0 {
                commands.entity(laser_entity).despawn();
                commands.entity(asteroid_entity).despawn();
                count.score += 1;
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/explosion_asteroids.ogg")),
                    PlaybackSettings::ONCE,
                ));
                continue 'laser;
            }
        }
    }
}

pub fn collision_player_with_asteroids(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<PlayerStruct>>,
    asteroids_query: Query<(Entity, &Transform), With<AsteroidsStruct>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    lasers_player_query: Query<Entity, With<LasersStruct>>,
    mut count: ResMut<ScoreStruct>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single() {
        for (asteroid_entity, asteroid_transform) in asteroids_query.iter() {
            let distance = player_transform.translation.distance(asteroid_transform.translation);
            if distance < 40.0 {
                commands.entity(player_entity).despawn();
                commands.entity(asteroid_entity).despawn();
                for laser_entity in lasers_player_query.iter() {
                    commands.entity(laser_entity).despawn();
                }

                next_state.set(GameState::GameOver);
                if count.score > count.max_score {
                    count.max_score = count.score;
                }
                commands.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    GameOverStruct
                )).with_children(|p| {
                    p.spawn((
                        Text::new("Гра закінчилася! Натисніть на R для перезапуску!"),
                        TextFont {
                            font: asset_server.load("fonts/Montserrat-Italic-VariableFont_wght.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/game_over_music.ogg")),
                    PlaybackSettings::ONCE,
                ));
            }
        }
    }
}

pub fn pause(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    key_code: Res<ButtonInput<KeyCode>>,
    pause_query: Query<Entity, With<PauseStruct>>,
) {
    if key_code.just_pressed(KeyCode::KeyQ) {
        if *state.get() == GameState::InGame {
            next_state.set(GameState::Pause);
            commands.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                PauseStruct
            )).with_children(|p| {
                p.spawn((
                    Text::new("Пауза!"),
                    TextFont {
                        font: asset_server.load("fonts/Montserrat-Italic-VariableFont_wght.ttf"),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        } else if *state.get() == GameState::Pause {
            next_state.set(GameState::InGame);
            for pause_entity in pause_query.iter() {
                commands.entity(pause_entity).despawn();
            }
        }
    }
}

pub fn restart(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    key_code: Res<ButtonInput<KeyCode>>,
    game_over_query: Query<Entity, With<GameOverStruct>>,
    mut count: ResMut<ScoreStruct>,
) {
    if key_code.just_pressed(KeyCode::KeyR) {
        if *state.get() == GameState::GameOver {
            count.score = 0;
            next_state.set(GameState::InGame);

            for game_over_entity in game_over_query.iter() {
                commands.entity(game_over_entity).despawn();
            }

            commands.spawn((
                Sprite {
                    image: asset_server.load("images/player/player.png"),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.5),
                PlayerStruct,
            ));
        }
    }
}

pub fn start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    key_code: Res<ButtonInput<KeyCode>>,
    game_start_query: Query<Entity, With<GameStartStruct>>,
) {
    if *state.get() == GameState::NotStarted && game_start_query.is_empty() {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GameStartStruct
        )).with_children(|p| {
            p.spawn((
                Text::new("Натисніть S для початку гри!"),
                TextFont {
                    font: asset_server.load("fonts/Montserrat-Italic-VariableFont_wght.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    }

    if key_code.just_pressed(KeyCode::KeyS) && *state.get() == GameState::NotStarted {
        next_state.set(GameState::InGame);
        for game_start_entity in game_start_query.iter() {
            commands.entity(game_start_entity).despawn();
        }

        commands.spawn((
            Sprite {
                image: asset_server.load("images/player/player.png"),
                ..default()
            },
            PlayerStruct,
        ));
    }
}

pub fn hide_objects(
   mut param_set: ParamSet<(
    Query<&mut Visibility, With<PlayerStruct>>,
    Query<&mut Visibility, With<AsteroidsStruct>>,
    Query<&mut Visibility, With<LasersStruct>>,
   )>
) {
    for mut vis in param_set.p0().iter_mut() {
        *vis = Visibility::Hidden;
    }
    for mut vis in param_set.p1().iter_mut() {
        *vis = Visibility::Hidden;
    }
    for mut vis in param_set.p2().iter_mut() {
        *vis = Visibility::Hidden;
    }
}

pub fn show_objects(
    mut param_set: ParamSet<(
        Query<&mut Visibility, With<PlayerStruct>>,
        Query<&mut Visibility, With<AsteroidsStruct>>,
        Query<&mut Visibility, With<LasersStruct>>,
    )>
) {
    for mut vis in param_set.p0().iter_mut() {
        *vis = Visibility::Visible;
    }
    for mut vis in param_set.p1().iter_mut() {
        *vis = Visibility::Visible;
    }
    for mut vis in param_set.p2().iter_mut() {
        *vis = Visibility::Visible;
    }
}

pub fn count_function(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(5.0),
            ..default()
        },
        CountStruct
    )).with_children(|p| {
        p.spawn((
            Text::new("Очки: 0"),
            TextFont {
                font: asset_server.load("fonts/Montserrat-Italic-VariableFont_wght.ttf"),
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
            CountTextStruct,
        ));
    });
}

pub fn update_count(
    count: Res<ScoreStruct>,
    mut count_query: Query<&mut Text, With<CountTextStruct>>,
) {
    for mut text in count_query.iter_mut() {
        **text = format!("Очки: {}", count.score);
    }
}

pub fn max_count_function(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(50.0),
            ..default()
        },
        MaxCountStruct
    )).with_children(|p| {
        p.spawn((
            Text::new("Макс. к-сть очок: 0"),
            TextFont {
                font: asset_server.load("fonts/Montserrat-Italic-VariableFont_wght.ttf"),
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
            MaxCountTextStruct,
        ));
    });
}

pub fn update_max_count(
    count: Res<ScoreStruct>,
    mut max_count_query: Query<&mut Text, With<MaxCountTextStruct>>,
) {
    for mut text in max_count_query.iter_mut() {
        **text = format!("Макс. к-сть очок: {}", count.max_score);
    }
}
 