use bevy::prelude::{App, Plugin};

use crate::{
    plugin::SpringItPlugin,
    spring::Spring,
    transform::{TransformScaleSpring, TransformTranslationSpring},
    ui::NodePositionSpring,
};

pub struct SpringItTransformPlugins;

impl Plugin for SpringItTransformPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SpringItPlugin::<TransformScaleSpring>::default(),
            SpringItPlugin::<TransformTranslationSpring>::default(),
        ))
        .register_type::<Spring<TransformScaleSpring>>()
        .register_type::<Spring<TransformTranslationSpring>>();
    }
}

pub struct SpringItUiPlugins;

impl Plugin for SpringItUiPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpringItPlugin::<NodePositionSpring>::default());
    }
}
