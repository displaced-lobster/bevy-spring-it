use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use crate::{
    plugin::SpringItPlugin,
    transform::{TransformScaleSpring, TransformTranslationSpring},
    ui::NodePositionSpring,
};

pub struct SpringItTransformPlugins;

impl PluginGroup for SpringItTransformPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpringItPlugin::<TransformScaleSpring>::default())
            .add(SpringItPlugin::<TransformTranslationSpring>::default())
    }
}

pub struct SpringItUiPlugins;

impl PluginGroup for SpringItUiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(SpringItPlugin::<NodePositionSpring>::default())
    }
}
