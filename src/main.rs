use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const WORLD_BOUNDS: f32 = 100.0;
const PLAYER_SPEED: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sat Runner".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_systems((
            move_system,
            spawn_falling_dots_system,
            move_falling_dots_system,
        ))
        .insert_resource(DotTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::GRAY;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(WORLD_BOUNDS * 2.0, WORLD_BOUNDS * 2.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
        ..default()
    });

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
            ..Default::default()
        })
        .insert(Player { moving: false })
        .insert(Target::default())
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
                ..Default::default()
            });
        });
}

pub fn move_system(
    mut query: Query<(&mut Transform, &mut Target, &mut Player)>,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut windows: Query<&mut Window>,
    touches: Res<Touches>,
) {
    for (mut t, mut tg, mut p) in query.iter_mut() {
        if mouse.pressed(MouseButton::Left) || mouse.pressed(MouseButton::Right) {
            for window in windows.iter_mut() {
                if let Some(cursor) = window.cursor_position() {
                    let (camera, camera_transform) = camera_query.single();
                    let click_position =
                        get_click_position(&window, camera, camera_transform, cursor);
                    tg.x = click_position.x;
                    tg.y = click_position.y;
                    p.moving = true;
                }
            }
        }

        for touch in touches.iter() {
            let touch_pos = touch.position();
            let (camera, camera_transform) = camera_query.single();

            for window in windows.iter_mut() {
                let touch_position =
                    get_touch_position(&window, camera, camera_transform, touch_pos);
                tg.x = touch_position.x;
                tg.y = touch_position.y;
                p.moving = true;
            }
        }

        if p.moving {
            let current_position = Vec2::new(t.translation.x, t.translation.y);
            let direction = Vec2::new(tg.x, tg.y) - current_position;
            let distance_to_target = direction.length();

            if distance_to_target > 0.0 {
                let normalized_direction = direction / distance_to_target;
                let movement = normalized_direction * PLAYER_SPEED;

                let new_position = current_position + movement;

                if new_position.x.abs() <= WORLD_BOUNDS && new_position.y.abs() <= WORLD_BOUNDS {
                    if movement.length() < distance_to_target {
                        t.translation += Vec3::new(movement.x, movement.y, 0.1);
                    } else {
                        t.translation = Vec3::new(tg.x, tg.y, 0.1);
                        p.moving = false;
                    }
                } else {
                    p.moving = false;
                }
            } else {
                p.moving = false;
            }
        }
    }
}

fn spawn_falling_dots_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<DotTimer>,
) {
    let y_position = WORLD_BOUNDS;
    let x_position: f32 = rand::random::<f32>() * WORLD_BOUNDS * 2.0 - WORLD_BOUNDS; // Random position within the world bounds
    let speed: f32 = 10.0 + rand::random::<f32>() * 3.0; // Random speed between 1.0 and 4.0

    if timer.0.tick(time.delta()).just_finished() {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(x_position, y_position, 0.1)),
                ..Default::default()
            })
            .insert(FallingDot { speed });
    }
}

fn move_falling_dots_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &FallingDot)>,
) {
    for (entity, mut transform, dot) in query.iter_mut() {
        transform.translation.y -= dot.speed * time.delta_seconds();

        if transform.translation.y < -WORLD_BOUNDS {
            commands.entity(entity).despawn();
        }
    }
}

pub fn get_click_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_position: Vec2,
) -> Vec2 {
    let screen_size = Vec2::new(window.width(), window.height());
    let screen_position = cursor_position / screen_size;
    let clip_position = (screen_position - Vec2::new(0.5, 0.5)) * 2.0;
    let mut click_position = camera
        .projection_matrix()
        .inverse()
        .project_point3(clip_position.extend(0.0));
    click_position = *camera_transform * click_position;
    click_position.truncate()
}

pub fn get_touch_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_position: Vec2,
) -> Vec2 {
    let screen_size = Vec2::new(window.width(), window.height());
    let screen_position = Vec2::new(
        cursor_position.x / screen_size.x,
        1.0 - (cursor_position.y / screen_size.y),
    );
    let clip_position = (screen_position - Vec2::new(0.5, 0.5)) * 2.0;
    let mut touch_position = camera
        .projection_matrix()
        .inverse()
        .project_point3(clip_position.extend(0.0));
    touch_position = *camera_transform * touch_position;
    touch_position.truncate()
}

#[derive(Component)]
pub struct Player {
    pub moving: bool,
}

#[derive(Default, Reflect, Component)]
pub struct Target {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
struct FallingDot {
    speed: f32,
}

#[derive(Resource)]
struct DotTimer(Timer);
