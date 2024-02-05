use bevy::prelude::*;
use std::marker::PhantomData;

use crate::{compute_spring::ComputeSpring, spring::Spring, spring_it::SpringIt};

#[derive(Default)]
pub struct SpringItPlugin<C: Component, S: SpringIt<C>>(PhantomData<(C, S)>);

impl<C: Component, S: SpringIt<C>> Plugin for SpringItPlugin<C, S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_springs::<C, S>);
    }
}

fn update_springs<C: Component, S: SpringIt<C>>(
    time: Res<Time>,
    mut q_spring: Query<(&mut C, &mut Spring<C, S>)>,
) {
    let delta = time.delta_seconds();

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
