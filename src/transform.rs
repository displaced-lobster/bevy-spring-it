use bevy::prelude::*;

use crate::{spring::Spring, spring_it::SpringIt};

#[derive(Component, Default)]
pub struct TransformScaleSpring;

impl TransformScaleSpring {
    pub fn new(equillibrium: Vec3, frequency: f32, damping: f32) -> Spring<Self> {
        Spring::new(equillibrium, frequency, damping)
    }
}

impl SpringIt for TransformScaleSpring {
    type C = Transform;
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

impl TransformTranslationSpring {
    pub fn new(equillibrium: Vec3, frequency: f32, damping: f32) -> Spring<Self> {
        Spring::new(equillibrium, frequency, damping)
    }
}

impl SpringIt for TransformTranslationSpring {
    type C = Transform;
    type T = Vec3;

    fn position(&self, transform: &Transform) -> Self::T {
        transform.translation
    }

    fn update(&self, transform: &mut Transform, position: Self::T) {
        transform.translation = position;
    }
}
