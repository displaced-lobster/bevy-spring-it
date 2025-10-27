use bevy::prelude::{Component, Entity};
use std::marker::PhantomData;

use crate::spring_it::SpringIt;

#[derive(Component)]
pub struct SpringAnchor<S: SpringIt> {
    entity: Entity,
    _phantom: PhantomData<S>,
}

impl<S: SpringIt> SpringAnchor<S> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            _phantom: PhantomData,
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
