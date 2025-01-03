use bevy::{color::palettes::css::RED, prelude::*};
use bevy_spring_it::{NodePositionSpring, SpringItUiPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpringItUiPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(100.0),
            top: Val::Px(100.0),
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            ..default()
        },
        BackgroundColor(RED.into()),
        NodePositionSpring::new(
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
