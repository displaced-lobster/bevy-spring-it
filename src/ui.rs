use bevy::prelude::{Component, Style, UiRect, Val};

use crate::{compute_spring::ComputeSpring, spring::Spring, spring_it::SpringIt};

impl ComputeSpring for UiRect {
    fn add(&self, other: &Self) -> Self {
        UiRect {
            left: self.left.add(&other.left),
            right: self.right.add(&other.right),
            top: self.top.add(&other.top),
            bottom: self.bottom.add(&other.bottom),
        }
    }

    fn add_f32(&self, v: f32) -> Self {
        UiRect {
            left: self.left.add_f32(v),
            right: self.right.add_f32(v),
            top: self.top.add_f32(v),
            bottom: self.bottom.add_f32(v),
        }
    }

    fn mul_f32(&self, v: f32) -> Self {
        UiRect {
            left: self.left.mul_f32(v),
            right: self.right.mul_f32(v),
            top: self.top.mul_f32(v),
            bottom: self.bottom.mul_f32(v),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        UiRect {
            left: self.left.sub(&other.left),
            right: self.right.sub(&other.right),
            top: self.top.sub(&other.top),
            bottom: self.bottom.sub(&other.bottom),
        }
    }
}

impl ComputeSpring for Val {
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Val::Px(px1), Val::Px(px2)) => Val::Px(px1 + px2),
            (Val::Percent(percent1), Val::Percent(percent2)) => Val::Percent(percent1 + percent2),
            (Val::Vw(vw1), Val::Vw(vw2)) => Val::Vw(vw1 + vw2),
            (Val::Vh(vh1), Val::Vh(vh2)) => Val::Vh(vh1 + vh2),
            (Val::VMin(vmin1), Val::VMin(vmin2)) => Val::VMin(vmin1 + vmin2),
            (Val::VMax(vmax1), Val::VMax(vmax2)) => Val::VMax(vmax1 + vmax2),
            (Val::Auto, Val::Auto) => Val::Auto,
            _ => *self,
        }
    }

    fn add_f32(&self, v: f32) -> Self {
        match self {
            Val::Px(px) => Val::Px(px + v),
            Val::Percent(percent) => Val::Percent(percent + v),
            Val::Vw(vw) => Val::Vw(vw + v),
            Val::Vh(vh) => Val::Vh(vh + v),
            Val::VMin(vmin) => Val::VMin(vmin + v),
            Val::VMax(vmax) => Val::VMax(vmax + v),
            Val::Auto => Val::Auto,
        }
    }

    fn mul_f32(&self, v: f32) -> Self {
        match self {
            Val::Px(px) => Val::Px(px * v),
            Val::Percent(percent) => Val::Percent(percent * v),
            Val::Vw(vw) => Val::Vw(vw * v),
            Val::Vh(vh) => Val::Vh(vh * v),
            Val::VMin(vmin) => Val::VMin(vmin * v),
            Val::VMax(vmax) => Val::VMax(vmax * v),
            Val::Auto => Val::Auto,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        match (self, other) {
            (Val::Px(px1), Val::Px(px2)) => Val::Px(px1 - px2),
            (Val::Percent(percent1), Val::Percent(percent2)) => Val::Percent(percent1 - percent2),
            (Val::Vw(vw1), Val::Vw(vw2)) => Val::Vw(vw1 - vw2),
            (Val::Vh(vh1), Val::Vh(vh2)) => Val::Vh(vh1 - vh2),
            (Val::VMin(vmin1), Val::VMin(vmin2)) => Val::VMin(vmin1 - vmin2),
            (Val::VMax(vmax1), Val::VMax(vmax2)) => Val::VMax(vmax1 - vmax2),
            (Val::Auto, Val::Auto) => Val::Auto,
            _ => *self,
        }
    }
}

#[derive(Component, Default)]
pub struct StylePositionSpring;

impl StylePositionSpring {
    pub fn new(equillibrium: UiRect, frequency: f32, damping: f32) -> Spring<Self> {
        Spring::new(equillibrium, frequency, damping)
    }
}

impl SpringIt for StylePositionSpring {
    type C = Style;
    type T = UiRect;

    fn position(&self, style: &Style) -> Self::T {
        UiRect {
            left: style.left,
            right: style.right,
            top: style.top,
            bottom: style.bottom,
        }
    }

    fn update(&self, style: &mut Style, position: Self::T) {
        style.left = position.left;
        style.right = position.right;
        style.top = position.top;
        style.bottom = position.bottom;
    }
}
