use bevy::{
    app::PluginGroupBuilder,
    prelude::{PluginGroup, Style, Transform},
};

use crate::{
    plugin::SpringItPlugin,
    transform::{TransformScaleSpring, TransformTranslationSpring},
    ui::StylePositionSpring,
};

pub struct SpringItTransformPlugins;

impl PluginGroup for SpringItTransformPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpringItPlugin::<Transform, TransformScaleSpring>::default())
            .add(SpringItPlugin::<Transform, TransformTranslationSpring>::default())
    }
}

pub struct SpringItUiPlugins;

impl PluginGroup for SpringItUiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpringItPlugin::<Style, StylePositionSpring>::default())
    }
}
