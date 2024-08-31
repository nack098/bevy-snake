use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, window::{PresentMode, PrimaryWindow}};
use rand::Rng;

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 700.;
const GRID_SIZE: i32 = 28;
const GRID_SQUARE_SIZE: f32 = WINDOW_WIDTH / GRID_SIZE as f32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(GameTime(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_movement_input, handle_movement, handle_eat_food.after(handle_movement), spawn_food,check_for_death.after(handle_movement), position_translation))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Food;

#[derive(Resource)]
struct GameTime(Timer);

fn setup(mut commands: Commands) {
    println!("Setting up...");

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
           sprite: Sprite {
               color: Color::MIDNIGHT_BLUE,
               ..default()
           },
           transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
           ..default()
        },
        SnakeHead {
            direction: Direction::Up
        },
        Position {
            x: 0,
            y: 0,
        }
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
            ..default()
        },
        SnakeSegment,
        Position {
            x: 0,
            y: -1
        }
    ));
}

fn spawn_food(mut commands: Commands, food: Query<&Food>) {
    if food.is_empty() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED, ..default()
                }, 
                transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)), 
                ..default()
            },
            Food,
            Position {
                x: rand::thread_rng().gen_range(0..GRID_SIZE),
                y: rand::thread_rng().gen_range(0..GRID_SIZE)
            }
        ));
    }
}

fn handle_movement_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut SnakeHead>) {
    let mut head = query.iter_mut().next().unwrap();

    if keys.pressed(KeyCode::W) && head.direction != Direction::Down {
        head.direction = Direction::Up;
    } else if keys.pressed(KeyCode::S) && head.direction != Direction::Up {
        head.direction = Direction::Down;
    } else if keys.pressed(KeyCode::A) && head.direction != Direction::Right {
        head.direction = Direction::Left;
    } else if keys.pressed(KeyCode::D) && head.direction != Direction::Left {
        head.direction = Direction::Right;
    }
}

fn handle_movement(mut query: Query<(&mut SnakeHead, &mut Position), (With<SnakeHead>, Without<SnakeSegment>)>,
                   mut segment_query: Query<&mut Position, (With<SnakeSegment>, Without<SnakeHead>)>,
                   time: Res<Time>,
                   mut timer: ResMut<GameTime>
                   ) 
{
    if timer.0.tick(time.delta()).just_finished() {
        let tuple = query.iter_mut().next().unwrap();
        let head = tuple.0;
        let mut pos = tuple.1;
        
        let prev_transform = pos.clone();

        match head.direction {
            Direction::Up => {
                pos.y += 1;
            }
            Direction::Down => {
                pos.y -= 1;
            }
            Direction::Left => {
                pos.x -= 1;
            }
            Direction::Right => {
                pos.x += 1;
            }
        }

        let mut prev_translation = prev_transform;
        for mut segment in segment_query.iter_mut() {
            let prev = segment.clone();
            segment.x = prev_translation.x;
            segment.y = prev_translation.y;

            prev_translation = prev;
        }
    }
}

fn handle_eat_food(mut commands: Commands, head_query: Query<&Position, With<SnakeHead>>, food_query: Query<(Entity, &Position), With<Food>>) {
    let head_pos = head_query.single();

    for food in food_query.iter() {
        if head_pos.x == food.1.x && head_pos.y == food.1.y {
            commands.entity(food.0).despawn();
            commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            ..default()
                        },
                        transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                        ..default()
                    },
                    SnakeSegment,
                    Position {
                        x: -1,
                        y: -1
                    }
           ));
        }
    }
}

fn check_for_death(mut commands: Commands, 
                   entity_query: Query<Entity, Without<Camera2d>>, 
                   head_query: Query<&Position, With<SnakeHead>>, 
                   segments_query: Query<&Position, With<SnakeSegment>>) {
    let head = head_query.single();
    for segment in segments_query.iter() {
        if head.x == segment.x && head.y == segment.y {
            for entity in entity_query.iter() {
                commands.entity(entity).despawn();
            }

            commands.spawn((
                SpriteBundle {
                    sprite : Sprite {
                        color: Color::MIDNIGHT_BLUE,
                        ..default()
                    },
                    transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                    ..default()
                },
                SnakeHead {
                    direction: Direction::Up
                },
                Position {
                    x: 0,
                    y: 0
                }
            ));

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                    ..default()
                },
                SnakeSegment,
                Position {
                    x: 0,
                    y: -1
                }
            ));
        }
    }
}

fn position_translation(windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_single().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, GRID_SIZE as f32),
            convert(pos.y as f32, window.height() as f32, GRID_SIZE as f32),
            0.0,
        );
    }

}
