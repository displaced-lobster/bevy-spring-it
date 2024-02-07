use bevy::prelude::Component;

use crate::compute_spring::ComputeSpring;

pub trait SpringIt: Component + Default {
    type C: Component;
    type T: ComputeSpring + Copy + Default + Send + Sync + 'static;

    fn position(&self, component: &Self::C) -> Self::T;
    fn update(&self, component: &mut Self::C, position: Self::T);
}
