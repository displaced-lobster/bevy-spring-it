use bevy::prelude::Component;

use crate::compute_spring::ComputeSpring;

pub trait SpringIt<C: Component>: Component + Default {
    type T: ComputeSpring + Copy + Default + Send + Sync + 'static;

    fn position(&self, component: &C) -> Self::T;
    fn update(&self, component: &mut C, position: Self::T);
}
