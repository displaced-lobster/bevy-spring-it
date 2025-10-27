use bevy::ecs::component::{Component, Mutable};

use crate::compute_spring::ComputeSpring;

pub trait SpringIt: Send + Sync + 'static {
    type C: Component<Mutability = Mutable>;
    type T: ComputeSpring + Copy + Default + Send + Sync + 'static;

    fn position(component: &Self::C) -> Self::T;
    fn update(component: &mut Self::C, position: Self::T);
}
