use bevy::prelude::*;
use bevy_spring_it::{SpringItUiPlugins, StylePositionSpring};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpringItUiPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(100.0),
                top: Val::Px(100.0),
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        },
        StylePositionSpring::new(
            UiRect {
                left: Val::Px(200.0),
                top: Val::Px(200.0),
                ..default()
            },
            1.0,
            0.0,
        ),
    ));
}
