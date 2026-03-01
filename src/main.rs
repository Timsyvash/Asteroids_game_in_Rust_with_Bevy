mod camera;
mod game;
mod player;
mod lasers;
mod asteroids;

use bevy::prelude::*;
use bevy::window::*;
#[cfg(target_os = "ios")]
use bevy_ios_iap::{IosIapPlugin, IosIapEvents, IosIapAction as IapAction};

#[cfg(not(target_os = "ios"))]
pub struct IosIapEvents;
#[cfg(not(target_os = "ios"))]
pub struct IapAction;
#[cfg(not(target_os = "ios"))]
#[cfg(target_os = "ios")]
type IapEvent = IosIapEvents;

use camera::*;
use crate::asteroids::*;
use crate::game::*;
use crate::lasers::*;
use crate::player::*;

#[cfg(target_os = "ios")]
fn iap_event_reader(mut events: EventReader<IapEvent>) {
    for event in events.read() {
        match event {
            IosIapEvents::OnProductsRetrieved(products) => {
                for p in products {
                    info!("Товар: {}", p.product_identifier);
                }
            }
            IosIapEvents::OnProductPurchaseSuccess(id) => {
                info!("Успішна покупка: {}", id);
            }
            IosIapEvents::OnProductPurchaseFailed(id, err) => {
                error!("Помилка {}: {}", id, err);
            }
            _ => {}
        }
    }
}

#[cfg(not(target_os = "ios"))]
fn iap_event_reader() {}

fn main() {
    let mut app = App::new();
    #[cfg(target_os = "ios")]
    app.add_plugins(IosIapPlugin::default());
    app.add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids".to_string(),
                resolution: WindowResolution::new(1000, 1000).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
    )
        .init_state::<GameState>()
        .insert_resource(ScoreStruct::default())
        .add_systems(Startup, (setup_camera, setup_background, count_function, max_count_function))
        .add_systems(Update, (keys, borders, lasers_player, move_lasers_player,
                              move_asteroids, collision_lasers_player_with_asteroids,
                              collision_player_with_asteroids,
                              spawn_asteroids).run_if(in_state(GameState::InGame)))
        .add_systems(Update, pause.run_if(in_state(GameState::InGame).or(in_state(GameState::Pause))))
        .add_systems(Update, (restart, start))
        .add_systems(OnEnter(GameState::Pause), hide_objects)
        .add_systems(OnExit(GameState::Pause), show_objects)
        .add_systems(OnEnter(GameState::GameOver), hide_objects)
        .add_systems(OnEnter(GameState::NotStarted), hide_objects)
        .add_systems(OnExit(GameState::NotStarted), show_objects)
        .add_systems(Update, update_count.run_if(in_state(GameState::InGame)))
        .add_systems(Update, update_max_count)
        .add_systems(Update, iap_event_reader)
        .run();
}
