//! This component holds velocity information about an entity in the form of a
//! two dimensional vector. This information encodes both speed and direction.
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Velocity(pub f32, pub f32);

impl Component for Velocity {
    // TODO: investigate storage types. This component in particular should
    // probably use `VecStorage`.
    type Storage = DenseVecStorage<Self>;
}
