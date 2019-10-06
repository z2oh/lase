//! This component holds position and movement information about an entity. This
//! includes the previous position of the entity, the current position of the
//! entity, and the entity's current velocity. If an entity updates position
//! successfully (i.e. moves with no collisions), then `pos - old_pos` should be
//! equal to `velocity`, modulo some floating point error.
use amethyst::ecs::{Component, DenseVecStorage};

use crate::vector::prelude::*;

#[derive(Debug)]
pub struct RelativeLocomotor {
    pub old_pos: Point2,
    pub pos: Point2,
    pub velocity: Vec2,
}

impl RelativeLocomotor {
    pub fn with_pos(pos: Point2) -> Self {
        Self::with_velocity(pos, Vec2::zeros())
    }

    pub fn with_velocity(pos: Point2, velocity: Vec2) -> Self {
        Self {
            old_pos: pos,
            pos,
            velocity,
        }
    }
}

impl Component for RelativeLocomotor {
    // TODO: investigate storage types. This component in particular should
    // probably use `VecStorage`.
    type Storage = DenseVecStorage<Self>;
}
