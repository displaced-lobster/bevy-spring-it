use bevy::prelude::*;

use crate::{anchor::SpringAnchor, spring::Spring, spring_it::SpringIt};

#[derive(Default, Reflect)]
pub struct TransformScaleSpring;

impl TransformScaleSpring {
    pub fn new(equilibrium: Vec3, frequency: f32, damping: f32) -> Spring<Self> {
        Spring::new(equilibrium, frequency, damping)
    }
}

impl SpringIt for TransformScaleSpring {
    type C = Transform;
    type T = Vec3;

    fn position(transform: &Transform) -> Self::T {
        transform.scale
    }

    fn update(transform: &mut Transform, position: Self::T) {
        transform.scale = position;
    }
}

#[derive(Default, Reflect)]
pub struct TransformTranslationSpring;

impl TransformTranslationSpring {
    pub fn new(equilibrium: Vec3, frequency: f32, damping: f32) -> Spring<Self> {
        Spring::new(equilibrium, frequency, damping)
    }
}

impl SpringIt for TransformTranslationSpring {
    type C = Transform;
    type T = Vec3;

    fn position(transform: &Transform) -> Self::T {
        transform.translation
    }

    fn update(transform: &mut Transform, position: Self::T) {
        transform.translation = position;
    }
}

pub type TransformScaleSpringAnchor = SpringAnchor<TransformScaleSpring>;
pub type TransformTranslationSpringAnchor = SpringAnchor<TransformTranslationSpring>;
