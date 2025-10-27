use bevy::ecs::component::{Component, Mutable};

use crate::compute_spring::ComputeSpring;

pub trait SpringIt: Component {
    type C: Component<Mutability = Mutable>;
    type T: ComputeSpring + Copy + Default + Send + Sync + 'static;

    fn position(component: &Self::C) -> Self::T;
    fn update(component: &mut Self::C, position: Self::T);
}
