use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, time::Stopwatch, utils::HashMap};

use bevy_ecs_ldtk::LdtkWorldBundle;
use uuid::Uuid;
use virtual_joystick::{
    TintColor, VirtualJoystickAxis, VirtualJoystickBundle, VirtualJoystickInteractionArea,
    VirtualJoystickNode, VirtualJoystickType,
};

use crate::{
    game_util::{
        components::{Bolt, NamePlates, NamePlatesLocal, Rain},
        resources::{BoltPool, RainPool},
    },
    keyboard::components::KeyboardNode,
    GameStage, KeyboardState,
};

use super::player::{Enemy, Player};

const FONT_SIZE: f32 = 15.0;

const PLAYER_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const DOTS_SIZE: Vec2 = Vec2::new(10., 10.);
const LN_SIZE: Vec2 = Vec2::new(10., 10.);

pub fn spawn_player(
    commands: &mut Commands,
    id: &Uuid,
    asset_server: &Res<AssetServer>,
    next_state: &mut ResMut<NextState<GameStage>>,
    keyboard_state: &mut ResMut<NextState<KeyboardState>>,
    windows: &Query<&Window>,
) {
    if let Some(window) = windows.iter().next() {
        if window.width() < 800.0 {
            commands.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                KeyboardNode,
            ));

            //     commands
            //         .spawn(
            //             VirtualJoystickBundle::new(VirtualJoystickNode {
            //                 border_image: asset_server.load("Outline.png"),
            //                 knob_image: asset_server.load("Knob.png"),
            //                 knob_size: Vec2::new(40., 40.),
            //                 dead_zone: 0.,
            //                 id: "UniqueJoystick".to_string(),
            //                 axis: VirtualJoystickAxis::Both,
            //                 behaviour: VirtualJoystickType::Floating,
            //             })
            //             .set_color(TintColor(Color::WHITE.with_a(0.2)))
            //             .set_style(Style {
            //                 width: Val::Px(75.),
            //                 height: Val::Px(75.),
            //                 position_type: PositionType::Absolute,
            //                 right: Val::Percent(8.),
            //                 bottom: Val::Percent(8.),
            //                 ..default()
            //             }),
            //         )
            //         .insert(VirtualJoystickInteractionArea);
        }
    }

    let text = Text::from_sections([TextSection::new(
        String::new(),
        TextStyle {
            font_size: FONT_SIZE,
            color: Color::LIME_GREEN,
            ..Default::default()
        },
    )]);

    let player_image = asset_server.load("umbrella.png");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            texture: player_image,
            transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
            ..Default::default()
        })
        .insert(Player {
            last_direction: None,
            id: *id,
            target: Vec2::ZERO,
            score: 0,
            pending_inputs: Vec::new(),
            name: String::new(),
            spawn_time: None,
            death_time: None,
        })
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 1.0)),
                projection: OrthographicProjection {
                    ..Default::default()
                },
                ..Default::default()
            });
            parent
                .spawn(Text2dBundle {
                    text: text.with_alignment(TextAlignment::Center),
                    transform: Transform {
                        translation: Vec3::new(0.0, -30., 1.0),
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(NamePlatesLocal);
        });

    keyboard_state.set(KeyboardState::On);
    next_state.set(GameStage::Menu);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn spawn_enemies(
    commands: &mut Commands,
    player_id: &Uuid,
    player_pos: Option<[f32; 2]>,
    target: Option<[f32; 2]>,
    score: usize,
    enemy_name: Option<String>,
    asset_server: &Res<AssetServer>,
    spawn_time: u64,
) {
    let target = target.unwrap_or([0.0, 0.0]);
    let player_pos = player_pos.unwrap_or([0.0, 0.0]);

    if let Some(enemy_name) = enemy_name {
        let text = Text::from_sections([TextSection::new(
            format!("{}:", enemy_name),
            TextStyle {
                font_size: FONT_SIZE,
                color: Color::WHITE,
                ..Default::default()
            },
        )]);

        let player_image = asset_server.load("umbrella.png");

        let mut stopwatch = Stopwatch::new();
        stopwatch.set_elapsed(Duration::from_secs(spawn_time));

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    ..default()
                },
                texture: player_image,
                transform: Transform::from_translation(Vec3::new(
                    player_pos[0],
                    player_pos[1],
                    0.0,
                )),
                ..Default::default()
            })
            .insert(Enemy {
                id: *player_id,
                target: Vec2 {
                    x: target[0],
                    y: target[1],
                },
                score,
                name: enemy_name,
                spawn_time: stopwatch,
                pending_inputs: VecDeque::new(),
                past_pos: HashMap::new(),
            })
            .with_children(|parent| {
                parent
                    .spawn(Text2dBundle {
                        text: text.with_alignment(TextAlignment::Center),
                        transform: Transform {
                            translation: Vec3::new(0.0, -30., 0.0),
                            ..default()
                        },
                        ..Default::default()
                    })
                    .insert(NamePlates { id: *player_id });
            });
    }
}

pub fn pool_rain(
    mut commands: Commands,
    mut rain_pool: ResMut<RainPool>,
    asset_server: Res<AssetServer>,
) {
    let rain_image = asset_server.load("droplet.png");

    for _ in 0..1000 {
        let rain = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(DOTS_SIZE),
                    ..Default::default()
                },
                texture: rain_image.clone(),
                ..Default::default()
            })
            .insert(Rain)
            .insert(Visibility::Hidden)
            .id();
        rain_pool.0.push_back(rain);
    }
}

pub fn pool_bolt(
    mut commands: Commands,
    mut bolt_pool: ResMut<BoltPool>,
    asset_server: Res<AssetServer>,
) {
    let bolt_image = asset_server.load("high-voltage.png");

    for _ in 0..200 {
        let ln = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(LN_SIZE),
                    ..Default::default()
                },
                texture: bolt_image.clone(),
                ..Default::default()
            })
            .insert(Bolt)
            .insert(Visibility::Hidden)
            .id();
        bolt_pool.0.push_back(ln);
    }
}

pub fn spawn_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
        ..Default::default()
    });
}
