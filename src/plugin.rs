use bevy::prelude::*;
use std::marker::PhantomData;

use crate::{
    anchor::SpringAnchor, compute_spring::ComputeSpring, spring::Spring, spring_it::SpringIt,
};

#[derive(Default)]
pub struct SpringItPlugin<S: SpringIt>(PhantomData<S>);

impl<S: SpringIt> Plugin for SpringItPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_anchored_equilibrium::<S>, update_springs::<S>),
        );
    }
}

fn update_anchored_equilibrium<S: SpringIt>(
    anchors: Query<(&S::C, &SpringAnchor<S>), Changed<S::C>>,
    mut springs: Query<&mut Spring<S>>,
) {
    for (component, anchor) in &anchors {
        if let Ok(mut spring) = springs.get_mut(anchor.entity()) {
            spring.equilibrium = spring.position(component);
        }
    }
}

fn update_springs<S: SpringIt>(time: Res<Time>, mut q_spring: Query<(&mut S::C, &mut Spring<S>)>) {
    let delta = time.delta_secs();

    for (mut component, mut spring) in &mut q_spring {
        spring.calc_motion_params(delta);

        let old_position = spring.position(&component).sub(&spring.equilibrium);
        let new_position = old_position
            .mul_f32(spring.position_position_coef)
            .add(&(spring.velocity.mul_f32(spring.position_velocity_coef)))
            .add(&spring.equilibrium);

        spring.velocity = old_position
            .mul_f32(spring.velocity_position_coef)
            .add(&(spring.velocity.mul_f32(spring.velocity_velocity_coef)));

        spring.update(&mut component, new_position);
    }
}
