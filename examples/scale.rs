use bevy::prelude::*;
use bevy_spring_it::{SpringItTransformPlugins, TransformScaleSpring};

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
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 10.0)));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid {
            half_size: Vec3::splat(1.0),
        })),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        TransformScaleSpring::new(Vec3::splat(3.0), 1.0, 0.0),
    ));
}
