use bevy::{
    color::palettes::css::{RED, YELLOW},
    prelude::*,
};
use bevy_spring_it::{
    SpringItTransformPlugins, TransformTranslationSpring, TransformTranslationSpringAnchor,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SpringItTransformPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player_block)
        .run();
}

#[derive(Component)]
struct PlayerBlock;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let entity = commands
        .spawn((
            Sprite {
                color: RED.into(),
                rect: Some(Rect {
                    min: Vec2::new(-10.0, -10.0),
                    max: Vec2::new(10.0, 10.0),
                }),
                ..default()
            },
            TransformTranslationSpring::new(Vec3::ZERO, 7.0, 0.4),
        ))
        .id();

    commands.spawn((
        PlayerBlock,
        Sprite {
            color: YELLOW.into(),
            rect: Some(Rect {
                min: Vec2::new(-50.0, -50.0),
                max: Vec2::new(50.0, 50.0),
            }),
            ..default()
        },
        TransformTranslationSpringAnchor::new(entity),
    ));
}

fn move_player_block(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Transform, With<PlayerBlock>>,
) {
    const MAX: f32 = 1000.0;
    const SPEED: f32 = 200.0;

    let mut movement = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement += Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement -= Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        movement -= Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        movement += Vec2::X;
    }

    player.translation += movement.extend(0.0) * SPEED * time.delta_secs();
    player.translation.x = player.translation.x.clamp(-MAX, MAX);
    player.translation.y = player.translation.y.clamp(-MAX, MAX);
}
