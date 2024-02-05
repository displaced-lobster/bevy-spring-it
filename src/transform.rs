use bevy::prelude::*;

use crate::spring_it::SpringIt;

#[derive(Component, Default)]
pub struct TransformScaleSpring;

impl SpringIt<Transform> for TransformScaleSpring {
    type T = Vec3;

    fn position(&self, transform: &Transform) -> Self::T {
        transform.scale
    }

    fn update(&self, transform: &mut Transform, position: Self::T) {
        transform.scale = position;
    }
}

#[derive(Component, Default)]
pub struct TransformTranslationSpring;

impl SpringIt<Transform> for TransformTranslationSpring {
    type T = Vec3;

    fn position(&self, transform: &Transform) -> Self::T {
        transform.translation
    }

    fn update(&self, transform: &mut Transform, position: Self::T) {
        transform.translation = position;
    }
}
