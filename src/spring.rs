use bevy::prelude::{Component, Reflect, ReflectComponent};

use crate::spring_it::SpringIt;

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Spring<S: SpringIt> {
    pub equilibrium: S::T,
    pub frequency: f32,
    pub damping: f32,
    pub(crate) velocity: S::T,
    pub(crate) position_position_coef: f32,
    pub(crate) position_velocity_coef: f32,
    pub(crate) velocity_position_coef: f32,
    pub(crate) velocity_velocity_coef: f32,
}

impl<S: SpringIt> Spring<S> {
    pub fn new(equilibrium: S::T, frequency: f32, damping: f32) -> Self {
        Self {
            equilibrium,
            frequency,
            damping,
            velocity: S::T::default(),
            position_position_coef: 0.0,
            position_velocity_coef: 0.0,
            velocity_position_coef: 0.0,
            velocity_velocity_coef: 0.0,
        }
    }

    pub(crate) fn calc_motion_params(&mut self, delta_time: f32) {
        let epsilon = 0.0001;
        let damping = if self.damping < 0.0 {
            0.0
        } else {
            self.damping
        };
        let frequency = if self.frequency < 0.0 {
            0.0
        } else {
            self.frequency
        };

        if frequency < epsilon {
            self.position_position_coef = 1.0;
            self.position_velocity_coef = 0.0;
            self.velocity_position_coef = 0.0;
            self.velocity_velocity_coef = 1.0;
        } else if damping > 1.0 + epsilon {
            // over damped
            let za = -frequency * damping;
            let zb = frequency * (damping * damping - 1.0).sqrt();
            let z1 = za - zb;
            let z2 = za + zb;

            let e1 = (z1 * delta_time).exp();
            let e2 = (z2 * delta_time).exp();

            let inv_2_zb = 1.0 / (2.0 * zb);

            let e1_over_2_zb = e1 * inv_2_zb;
            let e2_over_2_zb = e2 * inv_2_zb;

            let z1e1_over_2_zb = z1 * e1_over_2_zb;
            let z2e2_over_2_zb = z2 * e2_over_2_zb;

            self.position_position_coef = e1_over_2_zb * z2 - z2e2_over_2_zb + e2;
            self.position_velocity_coef = -e1_over_2_zb + e2_over_2_zb;
            self.velocity_position_coef = (z1e1_over_2_zb - z2e2_over_2_zb + e2) * z2;
            self.velocity_velocity_coef = -z1e1_over_2_zb + z2e2_over_2_zb;
        } else if damping < 1.0 - epsilon {
            // under damped
            let omega_zeta = frequency * damping;
            let alpha = frequency * (1.0 - damping * damping).sqrt();

            let exp_term = (-omega_zeta * delta_time).exp();
            let cos_term = (alpha * delta_time).cos();
            let sin_term = (alpha * delta_time).sin();

            let inv_alpha = 1.0 / alpha;

            let exp_sin = exp_term * sin_term;
            let exp_cos = exp_term * cos_term;
            let exp_omega_zeta_over_alpha = exp_term * omega_zeta * sin_term * inv_alpha;

            self.position_position_coef = exp_cos + exp_omega_zeta_over_alpha;
            self.position_velocity_coef = exp_sin * inv_alpha;
            self.velocity_position_coef = -exp_sin * alpha - omega_zeta * exp_omega_zeta_over_alpha;
            self.velocity_velocity_coef = exp_cos - exp_omega_zeta_over_alpha;
        } else {
            // critically damped
            let exp_term = (-frequency * delta_time).exp();
            let time_exp = delta_time * exp_term;
            let time_exp_freq = time_exp * frequency;

            self.position_position_coef = time_exp_freq + exp_term;
            self.position_velocity_coef = time_exp;
            self.velocity_position_coef = -frequency * time_exp_freq;
            self.velocity_velocity_coef = -time_exp_freq + exp_term;
        }
    }

    pub fn position(&self, component: &S::C) -> S::T {
        S::position(component)
    }

    pub fn update(&mut self, component: &mut S::C, position: S::T) {
        S::update(component, position);
    }

    pub fn with_damping(self, damping: f32) -> Self {
        Self { damping, ..self }
    }

    pub fn with_equilibrium(self, equilibrium: S::T) -> Self {
        Self {
            equilibrium,
            ..self
        }
    }

    pub fn with_frequency(self, frequency: f32) -> Self {
        Self { frequency, ..self }
    }
}
