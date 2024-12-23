use bevy::prelude::Component;

use crate::compute_spring::ComputeSpring;

pub trait SpringIt: Component {
    type C: Component;
    type T: ComputeSpring + Copy + Default + Send + Sync + 'static;

    fn position(component: &Self::C) -> Self::T;
    fn update(component: &mut Self::C, position: Self::T);
}
