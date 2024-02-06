use bevy::prelude::*;
use bevy_spring_it::{SpringItTransformPlugins, TransformScaleSpring, TransformTranslationSpring};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpringItTransformPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            material: materials.add(Color::RED.into()),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..default()
        },
        TransformScaleSpring::new(Vec3::splat(2.0), 0.5, 0.0),
        TransformTranslationSpring::new(Vec3::ZERO, 1.0, 0.0),
    ));
}
