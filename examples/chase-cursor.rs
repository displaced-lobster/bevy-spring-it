use bevy::{color::palettes::css::RED, prelude::*};
use bevy_spring_it::{Spring, SpringItTransformPlugins, TransformTranslationSpring};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpringItTransformPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, chase_cursor)
        .run();
}

#[derive(Component)]
struct ChaseCamera;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: RED.into(),
                rect: Some(Rect {
                    min: Vec2::new(-50.0, -50.0),
                    max: Vec2::new(50.0, 50.0),
                }),
                ..default()
            },
            ..default()
        },
        TransformTranslationSpring::new(Vec3::ZERO, 7.0, 0.4),
        ChaseCamera,
    ));
}

fn chase_cursor(
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_spring: Query<&mut Spring<TransformTranslationSpring>, With<ChaseCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some([x, y, _]) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.to_array())
    {
        for mut spring in &mut q_spring {
            spring.equilibrium = Vec3::new(x, y, 0.0);
        }
    }
}
